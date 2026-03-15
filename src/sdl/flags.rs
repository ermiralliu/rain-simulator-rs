use bitflags::bitflags;

bitflags! {
    pub struct WindowInitConfig: u64 {
        const FULLSCREEN           = 0x0000000000000001;
        const OPENGL               = 0x0000000000000002;
        const OCCLUDED             = 0x0000000000000004;
        const HIDDEN               = 0x0000000000000008;
        const BORDERLESS           = 0x0000000000000010;
        const RESIZABLE            = 0x0000000000000020;
        const MINIMIZED            = 0x0000000000000040;
        const MAXIMIZED            = 0x0000000000000080;
        const MOUSE_GRABBED        = 0x0000000000000100;
        const INPUT_FOCUS          = 0x0000000000000200;
        const MOUSE_FOCUS          = 0x0000000000000400;
        const EXTERNAL             = 0x0000000000000800;
        const MODAL                = 0x0000000000001000;
        const HIGH_PIXEL_DENSITY   = 0x0000000000002000;
        const MOUSE_CAPTURE        = 0x0000000000004000;
        const MOUSE_RELATIVE_MODE  = 0x0000000000008000;
        const ALWAYS_ON_TOP        = 0x0000000000010000;
        const UTILITY              = 0x0000000000020000;
        const TOOLTIP              = 0x0000000000040000;
        const POPUP_MENU           = 0x0000000000080000;
        const KEYBOARD_GRABBED     = 0x0000000000100000;
        const VULKAN               = 0x0000000010000000;
        const METAL                = 0x0000000020000000;
        const TRANSPARENT          = 0x0000000040000000;
        const NOT_FOCUSABLE        = 0x0000000080000000;
    }
}

bitflags! {
    pub struct AppInitConfig: u32 {
        const AUDIO    = 0x00000010;
        const VIDEO    = 0x00000020;
        const JOYSTICK = 0x00000200;
        const HAPTIC   = 0x00001000;
        const GAMEPAD  = 0x00002000;
        const EVENTS   = 0x00004000;
        const SENSOR   = 0x00008000;
        const CAMERA   = 0x00010000;
    }
}
