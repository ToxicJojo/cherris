use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

struct GameUI {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl GameUI {
    pub fn new(canvas: Canvas<Window>, event_pump: EventPump) -> GameUI {
        GameUI { canvas, event_pump }
    }
}
