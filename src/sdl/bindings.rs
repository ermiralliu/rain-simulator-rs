#![allow(non_camel_case_types)]
use std::ffi::c_char;

pub enum SDL_Window {}
pub enum SDL_Renderer {}
pub enum SDL_Texture {}

// Only the fields we need; layout matches SDL3's SDL_Surface exactly.
// flags: u32, format: u32, w: i32, h: i32 — always at these offsets.
#[repr(C)]
pub struct SDL_Surface {
    pub flags: u32,
    pub format: u32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
pub struct SDL_FRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub r#type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub window_id: u32,
    pub which: u32,
    pub scancode: i32,
    pub key: u32,
    pub r#mod: u16,
    pub raw: u16,
    pub down: bool,
    pub repeat: bool,
}

#[repr(C)]
pub union SDL_Event {
    pub r#type: u32,
    pub key: SDL_KeyboardEvent,
    pub padding: [u8; 128],
}

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
    pub fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> bool;
    pub fn SDL_RenderPresent(renderer: *mut SDL_Renderer) -> bool;
    pub fn SDL_SetRenderDrawColor(renderer: *mut SDL_Renderer, r: u8, g: u8, b: u8, a: u8) -> bool;

    // Texture
    pub fn SDL_DestroySurface(surface: *mut SDL_Surface);
    pub fn SDL_CreateTextureFromSurface(renderer: *mut SDL_Renderer, surface: *mut SDL_Surface) -> *mut SDL_Texture;
    pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);
    pub fn SDL_RenderTexture(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture, srcrect: *const SDL_FRect, dstrect: *const SDL_FRect) -> bool;
    pub fn SDL_RenderTextureRotated(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture, srcrect: *const SDL_FRect, dstrect: *const SDL_FRect, angle: f64, center: *const std::ffi::c_void, flip: i32) -> bool;

    // Events
    pub fn SDL_PollEvent(event: *mut SDL_Event) -> bool;

    // Time
    pub fn SDL_GetTicks() -> u64; // milliseconds since SDL_Init
    pub fn SDL_Delay(ms: u32);
}

#[link(name = "SDL3_image")]
unsafe extern "C" {
    // Surface (for texture)
    pub fn IMG_Load(name: *const c_char) -> *mut SDL_Surface;
}
