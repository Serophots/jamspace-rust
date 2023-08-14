mod whiteboard_window;
mod tools;

use sfml::graphics::{Color, Font, PrimitiveType, RenderStates, RenderTarget, Text, Transformable, Vertex};
use sfml::system::{Clock, Time, Vector2f, Vector2i, Vector2u};
use sfml::window;
use sfml::window::{Event, Key};
use whiteboard_window::WhiteboardWindow;
use crate::DrawTool::PEN;
use crate::tools::DrawingTool;
use crate::tools::rounded_line::RoundedLine;
use crate::tools::single_point::SinglePoint;

macro_rules! resource {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $path)
    }
}

#[derive(Clone, Copy)]
enum DrawTool {
    PEN
}

fn construct_drawing(defining_points: Vec<Vector2f>, tool: DrawTool, color: Color) -> Option<Box<dyn DrawingTool>> {
    let defining_points_len = defining_points.len();

    if defining_points_len == 0 { return None }

    match tool {
        PEN => {
            match defining_points_len {
                1 => {
                    return Some(Box::new(SinglePoint::new(defining_points.clone(), color)));
                },
                1.. => {
                    return Some(Box::new(RoundedLine::new(defining_points.clone(), color)));
                },
                _ => {
                    unreachable!()
                }
            }
        }
    }
}

fn main() {
    let window_size = Vector2u::new(800,600);
    let mut window = WhiteboardWindow::new(window_size, "Jamspace");

    //State
    let mut primitive_type = PrimitiveType::TRIANGLE_STRIP;
    let mut mouse_coords = Vector2f::default();
    let mut drawing_timer = Clock::start();

    //Drawing
    let mut drawing = false;
    let mut drawing_tool = PEN;
    let default_render_states = RenderStates::default();
    let mut defining_points: Vec<Vector2f> = Vec::new();
    let mut drawings: Vec<Box<dyn DrawingTool>> = Vec::new();

    //Load fonts
    let font_roboto = Font::from_file(resource!("Roboto-Regular.ttf")).expect("Failed to load font: Roboto-Regular.ttf");

    //Space bar text
    let mut space_message = Text::default();
    space_message.set_string("Press space to toggle skeleton mode :>");
    space_message.set_font(&font_roboto);
    space_message.set_character_size(20);
    space_message.set_position((0., 0.));
    space_message.set_fill_color(Color::WHITE);
    space_message.set_outline_color(Color::BLACK);
    space_message.set_outline_thickness(3.);

    //FPS text
    let mut fps_message = Text::default();
    fps_message.set_font(&font_roboto);
    fps_message.set_character_size(20);
    fps_message.set_position((0., 20.));
    fps_message.set_fill_color(Color::WHITE);
    fps_message.set_outline_color(Color::BLACK);
    fps_message.set_outline_thickness(3.);

    //Mouse position
    let mut mouse_message = Text::default();
    mouse_message.set_font(&font_roboto);
    mouse_message.set_character_size(20);
    mouse_message.set_position((0., 40.));
    mouse_message.set_fill_color(Color::WHITE);
    mouse_message.set_outline_color(Color::BLACK);
    mouse_message.set_outline_thickness(3.);

    //Create UI
    // let mut ui = UIBuilder::new();
    //
    // let controls_frame = ui.add_element(Box::new(Frame::new(
    //     UDim2::new((0.25, 0.), (0.8, -50.)),
    //     UDim2::new((0.5, 0.), (0.2, 0.)),
    //     Vector2f::default(), window_size.as_other(),
    //     Color::BLACK,
    // )));
    // let controls_frame_pos = controls_frame.get_positioning();
    // let controls_frame_size = controls_frame.get_sizing();
    //
    //
    // let draw_tool_button = ui.add_element(Box::new(Button::new(
    //     UDim2::new((0., 5.), (0., 5.)), UDim2::new((1./4., -10.), (1., -10.)),
    //     controls_frame_pos, controls_frame_size,
    //     || {
    //         println!("Draw hover");
    //     },
    //     || {
    //         println!("Draw click");
    //     },
    //     Color::WHITE
    // )));
    // ui.add_element(Box::new(Button::new(
    //     UDim2::new((1./4., 5.), (0., 5.)), UDim2::new((1./4., -10.,), (1., -10.)),
    //     controls_frame_pos, controls_frame_size,
    //     || {
    //         println!("Shape hover");
    //     },
    //     || {
    //         println!("Shape click");
    //     },
    //     Color::WHITE
    // )));

    //TODO: Develop a parental hierarchy system to the UI
    //When checking for hover, we can eliminate the children of a parent who isn't hovered


    'main: loop{
        //Event loop
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, ..} => break 'main,
                Event::MouseButtonPressed { button: window::mouse::Button::Left, x, y } => {
                    let mouse = Vector2i::new(x,y);
                    let map_coords = window.window.map_pixel_to_coords(mouse, &*window.view);
                    defining_points.push(map_coords);

                    drawing_timer.restart();
                    drawing = true;
                },
                Event::MouseButtonReleased { button: window::mouse::Button::Left, .. } => {
                    match construct_drawing(defining_points.clone(), drawing_tool, Color::RED) {
                        Some(drawing) => drawings.push(drawing),
                        None => {}
                    }
                    defining_points.clear(); //Note that this method has no effect on the allocated capacity of the vector   :))) sounds fun
                    drawing = false;
                },
                Event::MouseMoved {x,y} => {
                    let mouse_pixels = Vector2i::new(x,y);
                    mouse_coords = window.window.map_pixel_to_coords(mouse_pixels, &*window.view);

                    if drawing && drawing_timer.elapsed_time() > Time::seconds(0.01) && match defining_points.last() {
                        None => true,
                        Some(last) => {
                            (mouse_coords - *last).length_sq() > 10. * 10.
                        }
                    } {
                        drawing_timer.restart();
                        defining_points.push(mouse_coords);
                    }
                },
                Event::KeyPressed { code: Key::Space, .. } => {
                    if primitive_type == PrimitiveType::TRIANGLE_STRIP {
                        primitive_type = PrimitiveType::LINE_STRIP
                    }else{
                        primitive_type = PrimitiveType::TRIANGLE_STRIP
                    }
                }
                _ => {}
            }
        }

        //Window render
        window.clear(Color::rgb(230,230,230));

        //Unfixed drawing
        window.set_fixed(false);

        for drawing in &drawings {
            window.window.draw_primitives(
                &*drawing.get_rendered_vertexes(),
                primitive_type,
                &default_render_states,
            )
        }

        if defining_points.len() > 0 { //double check but I dont wanna clone every frame :woozy_face:
            match construct_drawing(defining_points.clone(), drawing_tool, Color::GREEN) {
                Some(drawing) => {
                    window.window.draw_primitives(
                        &*drawing.get_rendered_vertexes(),
                        primitive_type,
                        &default_render_states
                    )
                }
                None => {}
            }
        }

        //Fixed drawing
        window.set_fixed(true);

        fps_message.set_string(&format!("FPS: {}", window.framerate));
        mouse_message.set_string(&format!("MOUSE: {},{}", mouse_coords.x.round(), mouse_coords.y.round()));
        window.draw(&space_message);
        window.draw(&fps_message);
        window.draw(&mouse_message);

        // window.draw(&ui);

        //Display
        window.display();
    }
}