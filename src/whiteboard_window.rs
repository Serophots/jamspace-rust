use std::env;
use sfml::graphics::{Color, Drawable, RectangleShape, RenderTarget, RenderWindow, View};
use sfml::SfBox;
use sfml::system::{Clock, Vector2f, Vector2u};
use sfml::window::{ContextSettings, Event, Style};
use sfml::window::mouse::Button;

pub struct WhiteboardWindow<'a> {
    pub(crate) window: RenderWindow,

    pub view: SfBox<View>,
    view_rectangle: RectangleShape<'a>,

    right_down: bool,
    right_down_last_pos: Vector2f,
    zoom: f32,

    framerate_clock: SfBox<Clock>,
    previous_frame_time: f32,
    pub framerate: i32,
    pub delta_time: f32,
}

impl<'a> WhiteboardWindow<'a> {
    pub fn new(size: Vector2u, title: &str) -> Self {

        //Anti aliasing (cli args)
        let context_settings = ContextSettings {
            antialiasing_level: match env::args().nth(1) {
                Some(arg_str) => match arg_str.parse::<u32>() {
                    Ok(arg) => arg,
                    Err(e) => panic!("Failed to set anti aliasing level. ParseIntError: {}", e)
                },
                None => {
                    println!("Failed to set anti aliasing level");
                    0
                }
            },
            ..Default::default()
        };

        //Window
        let window = RenderWindow::new(
            (size.x, size.y),
            title,
            Style::CLOSE,
            &context_settings
        );
        // window.set_vertical_sync_enabled(true);

        //View
        let view = View::new(Vector2f::new(size.x as f32 / 2.,size.y as f32 / 2.), size.as_other());
        let mut view_rectangle = RectangleShape::new();
        view_rectangle.set_size(size.as_other());

        Self {
            window,
            view,
            view_rectangle,

            right_down: false,
            right_down_last_pos: Vector2f::default(),
            zoom: 1.,

            framerate_clock: Clock::start(),
            previous_frame_time: 0.,
            framerate: 0,
            delta_time: 0.,
        }
    }

    pub fn set_fixed(&mut self, fixed: bool) {
        if fixed {
            let default_view = View::new(self.window.default_view().center(), self.window.default_view().size());
            self.window.set_view(&default_view);
        } else {
            self.window.set_view(&*self.view);
        }
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        let option_event = self.window.poll_event();

        if let Some(event) = option_event {
            match event {
                Event::MouseWheelScrolled { delta, .. } => {
                    if delta < 1. {
                        self.view.zoom(1.1);
                        self.zoom *= 1.1;
                    }else{
                        self.view.zoom(1. / 1.1);
                        self.zoom *= 1. / 1.1;
                    }
                },
                Event::MouseMoved {x, y} => {
                    if self.right_down {
                        let mouse_pos = Vector2f::new(x as f32, y as f32);
                        let delta_pos = mouse_pos - self.right_down_last_pos;

                        self.view.move_(-delta_pos * self.zoom);

                        self.right_down_last_pos = mouse_pos;
                    }
                },
                Event::MouseButtonPressed { button: Button::Right, x,y } => {
                    let mouse_pos = Vector2f::new(x as f32, y as f32);
                    self.right_down_last_pos = mouse_pos;
                    self.right_down = true;
                },
                Event::MouseButtonReleased { button: Button::Right, .. } => {
                    self.right_down = false;
                },
                _ => {}
            }
        }

        option_event
    }
    pub fn clear(&mut self, color: Color) {
        self.window.clear(color);
        self.set_fixed(false);
        self.window.draw(&self.view_rectangle);
    }
    pub fn display(&mut self) {
        self.window.display();

        let current_time = self.framerate_clock.elapsed_time().as_seconds();
        self.delta_time = current_time - self.previous_frame_time;
        self.framerate = (1. / self.delta_time).round() as i32;
        self.previous_frame_time = current_time;
    }
    pub fn draw(&mut self, object: &dyn Drawable) {
        self.window.draw(object);
    }
}