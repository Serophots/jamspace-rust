mod whiteboard_window;

use sfml::graphics::{CircleShape, Color, Font, PrimitiveType, RenderStates, RenderTarget, Shape, Text, Transformable, Vertex};
use sfml::system::{Clock, Time, Vector2f, Vector2i, Vector2u};
use sfml::window::{Event, Key};
use sfml::window::mouse::Button;
use whiteboard_window::WhiteboardWindow;

macro_rules! resource {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $path)
    }
}
fn main() {
    let mut window = WhiteboardWindow::new(Vector2u::new(800,600), "Jamspace");

    //State
    let mut drawing_timer = Clock::start();
    let mut drawing = false;
    let mut drawing_points: Vec<Vertex> = Vec::new();

    let mut drawn_lines: Vec<Vec<Vertex>> = Vec::new();


    //Load fonts
    let font_roboto = Font::from_file(resource!("Roboto-Regular.ttf")).expect("Failed to load font: Roboto-Regular.ttf");

    //FPS text
    let mut fps_message = Text::default();
    fps_message.set_font(&font_roboto);
    fps_message.set_character_size(20);
    fps_message.set_position((0., 0.));
    fps_message.set_fill_color(Color::WHITE);
    fps_message.set_outline_color(Color::BLACK);
    fps_message.set_outline_thickness(3.);

    //Create entities
    let mut a = CircleShape::new(10., 12);
    a.set_fill_color(Color::rgb(255,0,0));
    a.set_position(Vector2f::new(0., 0.,));
    let mut b = CircleShape::new(10., 12);
    b.set_fill_color(Color::rgb(255,0,0));
    b.set_position(Vector2f::new(15., 0.,));


    'main: loop{
        //Event loop
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, ..} => break 'main,
                Event::MouseButtonPressed { button: Button::Left, x, y } => {
                    let mouse = Vector2i::new(x,y);
                    let map_coords = window.window.map_pixel_to_coords(mouse, &*window.view);

                    drawing_points.push(Vertex::new(map_coords, Color::RED, Vector2f::default()));

                    drawing_timer.restart();
                    drawing = true;
                },
                Event::MouseButtonReleased { button: Button::Left, x,y } => {
                    let mouse = Vector2i::new(x,y);
                    let map_coords = window.window.map_pixel_to_coords(mouse, &*window.view);

                    drawing_points.push(Vertex::new(map_coords, Color::RED, Vector2f::default()));

                    drawn_lines.push(drawing_points.clone());
                    drawing_points.clear();

                    drawing = false;
                },
                Event::MouseMoved {x,y} => {
                    if drawing && drawing_timer.elapsed_time() > Time::seconds(0.05) {
                        drawing_timer.restart();

                        let mouse = Vector2i::new(x,y);
                        let map_coords = window.window.map_pixel_to_coords(mouse, &*window.view);

                        drawing_points.push(Vertex::new(map_coords, Color::RED, Vector2f::default()));

                    }
                }
                _ => {}
            }
        }

        //Game logic

        //Window render
        window.clear(Color::rgb(70,70,70));

        //Fixed drawing
        window.set_fixed(true);

        fps_message.set_string(&format!("FPS: {}", window.framerate));
        window.draw(&fps_message);

        //Unfixed drawing
        window.set_fixed(false);

        window.window.draw_primitives(&*drawing_points, PrimitiveType::LINE_STRIP, &RenderStates::default());

        for line in &drawn_lines {
            window.window.draw_primitives(&*line, PrimitiveType::LINE_STRIP, &RenderStates::default());
        }


        //Display
        window.display();
    }
    println!("End event loop")

}