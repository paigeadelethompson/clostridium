use sdl2::video::Window;

pub struct Display {
    pub height: u8,
    pub width: u8,
    pub window: sdl2::video::Window
}

pub fn init_video() -> Window {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap()
}