#![allow(non_camel_case_types)]
use std::ffi::c_char;

pub enum SDL_Window {}
pub enum SDL_Renderer {}
pub enum SDL_Surface {}
pub enum SDL_Texture {}

#[link(name = "SDL3")]
unsafe extern "C" {
    // Main
    pub fn SDL_Init(flags: u32) -> bool;
    pub fn SDL_Quit();
    
    // Window
    pub fn SDL_CreateWindow(
        title: *const c_char,
        w: i32,
        h: i32,
        flags: u64,
    ) -> *mut SDL_Window;
    pub fn SDL_DestroyWindow(window: *mut SDL_Window);
    // Renderer
    pub fn SDL_CreateRenderer(window: *mut SDL_Window, name: *const c_char) -> *mut SDL_Renderer;
    pub fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);

    // Texture
    pub fn SDL_DestroySurface(surface: *mut SDL_Surface);
    pub fn SDL_CreateTextureFromSurface(renderer: *mut SDL_Renderer, surface: *mut SDL_Surface) -> *mut SDL_Texture;
    pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);
}

#[link(name = "SDL3-image")]
unsafe extern "C" {
    // Surface (for texture)
    pub fn IMG_Load(name: *const c_char) -> *mut SDL_Surface;
}

