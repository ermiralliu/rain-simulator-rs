mod bindings;
pub mod flags; // we expose this so you can build the bitset nicely

use bindings::{
    IMG_Load, SDL_CreateRenderer, SDL_CreateTextureFromSurface, SDL_CreateWindow, SDL_Delay,
    SDL_DestroyRenderer, SDL_DestroySurface, SDL_DestroyTexture, SDL_DestroyWindow, SDL_Event,
    SDL_FRect, SDL_GetTicks, SDL_Init, SDL_PollEvent, SDL_Quit, SDL_RenderClear, SDL_RenderPresent,
    SDL_RenderTexture, SDL_RenderTextureRotated, SDL_Renderer, SDL_SetRenderDrawColor,
    SDL_Texture, SDL_Window,
};
use flags::{AppInitConfig, WindowInitConfig};

use std::{
    ffi::{CStr, c_int},
    marker::PhantomData,
    ptr::null,
};

pub type FloatRect = SDL_FRect;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

const SDL_EVENT_QUIT: u32 = 0x100;
const SDL_EVENT_KEY_DOWN: u32 = 0x300;

pub enum Scancode {
    Up,
    Down,
    Left,
    Right,
    Other(i32),
}

pub enum Event {
    Quit,
    KeyDown(Scancode),
    Other,
}

pub struct Sdl; // proof of initialization ----- token pattern ---- enforce initialization at
// compile time

impl Sdl {
    pub fn init(flags: AppInitConfig) -> Option<Self> {
        // we can implement a proper Result here if we get the error from SDL_GetError
        // But let's leave that for later
        let is_init = unsafe { SDL_Init(flags.bits()) };
        if is_init { Option::Some(Sdl) } else { None }
    }

    pub fn create_renderer<'sdl>(
        &'sdl self,
        title: &CStr, // c"stringstuff" - no copies
        width: u32,
        height: u32,
        flags: WindowInitConfig,
    ) -> Option<Renderer<'sdl>> {
        // This can also surface errors more cleanly
        if width > 40000 || height > 40000 {
            panic!("No way you actually meant to put the resolution that high");
        }
        let window = unsafe {
            SDL_CreateWindow(
                title.as_ptr(),
                width as c_int,
                height as c_int,
                flags.bits(),
            )
        };
        if window.is_null() {
            return Option::None;
        }
        let renderer = unsafe { SDL_CreateRenderer(window, null()) }; // I should maybe allow for naming here
        if renderer.is_null() {
            return Option::None;
        }
        Some(Renderer {
            window,
            renderer,
            _lifetime: PhantomData,
        }) //
    }

    #[inline]
    pub fn ticks() -> u64 {
        unsafe { SDL_GetTicks() }
    }

    #[inline]
    pub fn delay(ms: u32) {
        unsafe { SDL_Delay(ms) };
    }

    pub fn poll_event(&self) -> Option<Event> {
        let mut raw = SDL_Event { padding: [0u8; 128] };
        if !unsafe { SDL_PollEvent(&mut raw) } {
            return None;
        }
        match unsafe { raw.r#type } {
            SDL_EVENT_QUIT => Some(Event::Quit),
            SDL_EVENT_KEY_DOWN => {
                let scancode = unsafe { raw.key.scancode };
                Some(Event::KeyDown(match scancode {
                    79 => Scancode::Right,
                    80 => Scancode::Left,
                    81 => Scancode::Down,
                    82 => Scancode::Up,
                    _ => Scancode::Other(scancode),
                }))
            }
            _ => Some(Event::Other),
        }
    }
}

impl Drop for Sdl {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}

pub struct Renderer<'sdl> {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    _lifetime: PhantomData<&'sdl Sdl>,
}

impl<'sdl> Renderer<'sdl> {
    // here I'm passing the sdl initialization token (parent) as a parameter, but for Textures, I'm returning them from the
    // parent. I need to unify the API

    pub fn create_texture(&self, texture_name: &CStr) -> (Texture<'_>, (f32, f32)) {
        let (raw_texture, size) = unsafe {
            let sur = IMG_Load(texture_name.as_ptr());
            let size = ((*sur).w as f32, (*sur).h as f32);
            let texture = SDL_CreateTextureFromSurface(self.renderer, sur);
            SDL_DestroySurface(sur);
            (texture, size)
        };
        (Texture { raw_texture, _phantom: PhantomData }, size)
    }

    #[inline]
    pub fn render_texture<'ren>(
        &'sdl self,
        texture: &Texture<'ren>,
        dest_rect: FloatRect,
    ) where
        'sdl: 'ren, // seems accurate to me, so it shouldn't cause too many problems
    {
        unsafe {
            SDL_RenderTexture(self.renderer, texture.raw_texture, null(), &dest_rect as *const SDL_FRect);
        }
    }

    #[inline]
    pub fn render_texture_rotated<'ren>(
        &'sdl self,
        texture: &Texture<'ren>,
        dest_rect: FloatRect,
        angle: f64, // degrees, clockwise; center defaults to middle of dest_rect (null center)
    ) where
        'sdl: 'ren,
    {
        unsafe {
            SDL_RenderTextureRotated(self.renderer, texture.raw_texture, null(), &dest_rect as *const SDL_FRect, angle, null(), 0);
        }
    }

    #[inline]
    pub fn clear(&self) {
        unsafe { SDL_RenderClear(self.renderer) };
    }

    #[inline]
    pub fn present(&self) {
        unsafe { SDL_RenderPresent(self.renderer) };
    }

    #[inline]
    pub fn set_draw_color(&self, color: Color) {
        unsafe { SDL_SetRenderDrawColor(self.renderer, color.r, color.g, color.b, color.a) };
    }

}

impl<'sdl> Drop for Renderer<'sdl> {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyRenderer(self.renderer);
            SDL_DestroyWindow(self.window);
        }
    }
}

pub struct Texture<'ren> {
    raw_texture: *mut SDL_Texture,
    _phantom: PhantomData<&'ren ()>,
}

impl<'ren> Drop for Texture<'ren> {
    fn drop(&mut self) {
        unsafe { SDL_DestroyTexture(self.raw_texture) };
    }
}
