pub mod sdl;
pub mod rain;

use sdl::{Color, Event, FloatRect, Scancode, Sdl};
use sdl::flags::{AppInitConfig, WindowInitConfig};
use rain::{Rain, RainType};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const BLACK: Color = Color::new(0, 0, 0, 255);

fn main() {
    let sdl = Sdl::init(AppInitConfig::VIDEO | AppInitConfig::EVENTS)
        .expect("SDL init failed");

    let renderer = sdl
        .create_renderer(c"Rain Simulator", WIDTH, HEIGHT, WindowInitConfig::RESIZABLE)
        // .create_renderer(c"Rain Simulator", WIDTH, HEIGHT, WindowInitConfig::RESIZABLE) //
        // WindowInitConfig::None() is better when you don't want resizing
       .expect("Renderer creation failed");

    let (rain_texture, rain_size) = renderer.create_texture(c"assets/rain.png");
    let (snow_texture, snow_size) = renderer.create_texture(c"assets/snow.png");
    let (hail_texture, hail_size) = renderer.create_texture(c"assets/hail.png");

    let mut rain = Rain::new(WIDTH as i16, HEIGHT as i16);

    const FRAME_MS: u64 = 1000 / 60; // ~16 ms target per frame

    'main: loop {
        let frame_start = Sdl::ticks();

        // --- events ---
        while let Some(event) = sdl.poll_event() {
            match event {
                Event::Quit => break 'main,
                Event::KeyDown(scancode) => match scancode {
                    Scancode::Up    => rain.adjust_temperature(1.0),  // warmer
                    Scancode::Down  => rain.adjust_temperature(-1.0), // colder
                    Scancode::Right => rain.adjust_wind(1),           // wind right
                    Scancode::Left  => rain.adjust_wind(-1),          // wind left
                    Scancode::Other(_) => {}
                },
                Event::Other => {}
            }
        }

        // --- update ---
        unsafe { rain.update() };

        // --- render ---
        renderer.set_draw_color(BLACK);
        renderer.clear();

        let angle = rain.angle();
        for (x, y, rain_type) in rain.iter_particles() {
            let (texture, (w, h)) = match rain_type {
                RainType::Rain => (&rain_texture, rain_size),
                RainType::Snow => (&snow_texture, snow_size),
                RainType::Hale => (&hail_texture, hail_size),
            };
            let dest = FloatRect { x: x as f32, y: y as f32, w, h };
            renderer.render_texture_rotated(texture, dest, angle);
        }

        renderer.present();

        let elapsed = Sdl::ticks() - frame_start;
        if elapsed < FRAME_MS {
            Sdl::delay((FRAME_MS - elapsed) as u32);
        }
    }
}
