use fastrand;

const ROWS: usize = 64;
const COLUMNS: usize = 64;

#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RainType {
    Snow = 3,
    Rain = 4,
    Hale = 5,
}

impl RainType {
    pub fn from_temperature(temp: f32) -> Self {
        if temp > 5.0 {
            RainType::Rain
        } else if temp >= 0.0 {
            RainType::Hale
        } else {
            RainType::Snow
        }
    }


}

pub struct Rain {
    y: [i16; ROWS],
    x: [i16; ROWS * COLUMNS],
    row_state: [RainType; ROWS], // state of entire y row, cause each row will be updated once it passes the bottom
    angle: f64,
    pub temperature: f32,
    pub x_velocity: i16, // this is same cause of wind, but maybe i should account for weight or sth
    x_limit: i16,        // resolution + some threshold. put as i16, to avoid casting
    y_limit: i16,
    y_spacing: i16,      // row interval; used as the re-entry point so wrapping rows don't leave a gap
    // y_velocity: u16, // this will be determined by y_state
    current_state: RainType, // this is what each updates to after passing the bottom
}

// Relation of y to x -> if 1 : 2
// y[0] -> (x[0], x[1]), y[1] -> (x[2], x[3])
// y[i] -> j = i*2 ; (x[j], x[j+1])

fn compute_angle(x_velocity: i16, rain_type: RainType) -> f64 {
    let y_vel = rain_type as i8 as f64;
    - (x_velocity as f64 / y_vel).atan().to_degrees()
}

impl Rain {
    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn new(x_limit: i16, y_limit: i16) -> Self {
        let temperature = 15.0f32;
        let current_state = RainType::from_temperature(temperature);

        let spacing = (y_limit + ROWS as i16 - 1) / ROWS as i16; // ceiling division: spacing * ROWS >= y_limit
        let jitter = spacing / 4;
        let mut y = [0i16; ROWS];
        for (i, y) in y.iter_mut().enumerate() {
            *y = -(y_limit / 2) + i as i16 * spacing + fastrand::i16(-jitter..jitter);
        }

        // Randomize x positions independently per particle
        let mut x = [0i16; ROWS * COLUMNS];
        for x in x.iter_mut() {
            *x = fastrand::i16(0..x_limit);
        }

        Rain {
            y,
            x,
            row_state: [current_state; ROWS],
            angle: compute_angle(0, current_state),
            temperature,
            x_velocity: 0,
            x_limit,
            y_limit,
            y_spacing: spacing,
            current_state,
        }
    }

    pub fn adjust_temperature(&mut self, delta: f32) {
        self.temperature += delta;
        self.current_state = RainType::from_temperature(self.temperature);
        self.angle = compute_angle(self.x_velocity, self.current_state);
    }

    pub fn adjust_wind(&mut self, delta: i16) {
        self.x_velocity = (self.x_velocity + delta).clamp(-10, 10);
        self.angle = compute_angle(self.x_velocity, self.current_state);
    }

    pub fn iter_particles(&self) -> impl Iterator<Item = (i16, i16, RainType)> + '_ {
        self.x
            .chunks(COLUMNS)
            .zip(self.y.iter())
            .zip(self.row_state.iter())
            .flat_map(|((x_chunk, &y), &rain_type)| {
                x_chunk.iter().map(move |&x| (x, y, rain_type))
            })
    }

    // written branchlessly, as through godbolt, I found the compiler wouldn't autovectorize it
    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))] // conditional compilation step as the compiler didn't want to use ymm
    pub fn update(&mut self) {
        // the loop below gets completely unrolled since we only have 32 elements
        for (y, rain_type) in self.y.iter_mut().zip(&mut self.row_state) {
            let is_limit = (*y > self.y_limit) as u8;
            *y = if is_limit == 1 {
                -fastrand::i16(1..=self.y_spacing)
            } else {
                *y + *rain_type as i8 as i16
            };
            let next_state =
                (self.current_state as u8 - *rain_type as u8) * is_limit + *rain_type as u8;
            *rain_type = unsafe { std::mem::transmute(next_state) };
        }
        for x in self.x.iter_mut() {
            *x += self.x_velocity;
            let mut addition = if *x > self.x_limit {
                -(self.x_limit + 5)
            } else if self.current_state == RainType::Snow {
                fastrand::bool() as i16 * 2 - 1 
            } else { 0 }; // too big
            addition = if *x < -5 { self.x_limit + 5 } else { addition };
            *x += addition;
        }
    }
}
// cleaner but less vectorizable y loop
// for (y, rain_type) in self.y.iter_mut().zip(&mut self.row_state) {
//     *y += *rain_type as i8 as i16;
//     if *y  > self.y_limit {
//         *y = -fastrand::i16(1..=self.y_spacing);
//         *rain_type = self.current_state;
//     }
// }

// same for x
// for x in self.x.iter_mut() {
//     *x += self.x_velocity;
//     if *x > self.x_limit { // The compiler should be able to make this branchless itself
//         *x = -5;
//     } else if *x < -5 {
//         *x = self.x_limit;
//     }
// }
