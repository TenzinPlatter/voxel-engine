pub mod render;

use beryllium::{
    video::GlWindow, *
};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// make window float with my niri setup
const WINDOW_TITLE: &str = "(float)";

pub fn init_sdl_and_win() -> (Sdl, GlWindow) {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    #[cfg(target_os = "macos")]
    sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE).unwrap();

    let win_args = video::CreateWinArgs {
        title: WINDOW_TITLE,
        width: WIDTH,
        height: HEIGHT,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let win = sdl.create_gl_window(win_args).expect("Failed to create window");
    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    (sdl, win)
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}
