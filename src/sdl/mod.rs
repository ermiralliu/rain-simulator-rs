mod bindings;
pub mod flags; // we expose this so you can build the bitset nicely

use bindings::{
    IMG_Load, SDL_CreateRenderer, SDL_CreateTextureFromSurface, SDL_CreateWindow,
    SDL_DestroyRenderer, SDL_DestroySurface, SDL_DestroyWindow, SDL_Init, SDL_Quit, SDL_Renderer,
    SDL_Texture, SDL_Window,
};
use flags::{AppInitConfig, WindowInitConfig};

use std::{
    ffi::{CStr, c_int},
    marker::PhantomData,
    ptr::null,
};

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

    pub fn create_texture(&self, texture_name: &CStr) -> Texture<'_> {
        let raw_texture = unsafe {
            let sur = IMG_Load(texture_name.as_ptr());
            let texture = SDL_CreateTextureFromSurface(self.renderer, sur);
            SDL_DestroySurface(sur);
            texture
        };
        Texture {
            raw_texture,
            _phantom: PhantomData
        }
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
