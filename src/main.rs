#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
extern crate conrod_winit;
#[macro_use]
extern crate clap;
extern crate find_folder;
extern crate glium;
extern crate rand;

mod number;
mod support;

use clap::App;
use conrod_core::{
    color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
};
use glium::Surface;
use number::{create_secret_number, Number};
use std::io;

macro_rules! make_color {
    ($r:expr, $g:expr, $b:expr) => {
        color::Color::Rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        color::Color::Rgba(
            $r as f32 / 255.0,
            $g as f32 / 255.0,
            $b as f32 / 255.0,
            $a as f32 / 255.0,
        )
    };
}

/// 藏青 - #2e4e7e
pub const CANG_QING: color::Color = make_color!(46, 78, 126);
/// 漆黑 - #161823
pub const QI_HEI: color::Color = make_color!(22, 24, 35);
/// 玄色 - #622a1d
pub const XUAN_SE: color::Color = make_color!(98, 42, 29);

widget_ids! {
    struct Ids {
        canvas,
        canvas_x_scrollbar,
        canvas_y_scrollbar,
        introduce_title,
        introduce_text,
        button,
        text_show,
        text_input,
    }
}

enum State {
    Introduce,
    Game,
}

/// This struct holds all of the variables used to demonstrate application data being passed
/// through the widgets. If some of these seem strange, that's because they are! Most of these
/// simply represent the aesthetic state of different parts of the GUI to offer visual feedback
/// during interaction with the widgets.
struct DemoApp {
    state: State,
    /// Background color (for demonstration of button and sliders).
    bg_color: conrod_core::Color,
    /// The widget border width (we'll use this to demo Bordering
    /// and number_dialer).
    border_width: f64,
    text_input: String,
    text_result: String,

    secret_number: [u32; 4],
    step: i32,
}

#[derive(Copy, Clone, Debug)]
struct Fonts {
    yh_id: conrod_core::text::font::Id,

    space_mono_regular_id: conrod_core::text::font::Id,
    space_mono_bold_id: conrod_core::text::font::Id,
}

impl DemoApp {
    /// Constructor for the Demonstration Application model.
    fn new() -> DemoApp {
        DemoApp {
            state: State::Introduce,
            bg_color: CANG_QING,
            border_width: 1.0,
            text_input: "".to_string(),
            text_result: "".to_string(),
            secret_number: create_secret_number(),
            step: 0,
        }
    }
}

fn main() {
    let yaml = load_yaml!("../assets/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let width: u32 = matches
        .value_of("width")
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    let height: u32 = matches
        .value_of("height")
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    let is_terminal: bool = matches
        .value_of("terminal")
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    if is_terminal {
        terminal_mode();
    } else {
        gui_mode(width, height);
    }
}

fn terminal_mode() {
    let rules = include_str!("../assets/rules.txt");
    println!("{}", rules);
    let secret_number = create_secret_number();
    let mut step = 0;

    loop {
        println!("Guess the number!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // see the answer and end the game
        if guess.trim() == "answer" {
            let mut answer = 0;
            for i in secret_number.iter() {
                answer = answer * 10 + i;
            }
            println!("{}", answer);
            break;
        }

        let guess_number = match Number::new(guess.as_str()) {
            Ok(num) => num,
            Err(be) => {
                println!("{}", be);
                continue;
            }
        };

        step += 1;
        let (a, b) = guess_number.ab_check(secret_number);
        println!(
            "Step: {}. You guessed: {}. {}A{}B",
            step, guess_number.value, a, b
        );
        if a == 4 {
            break;
        }
    }
}

fn gui_mode(width: u32, height: u32) {
    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("AB Guessing Game")
        .with_dimensions((width, height).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([width as f64, height as f64]).build();

    // Generate the widget identifiers.
    let mut ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let yh_id = ui
        .fonts
        .insert_from_file(assets.join("fonts/msyh.ttf"))
        .unwrap();
    let space_mono_regular_id = ui
        .fonts
        .insert_from_file(assets.join("fonts/space-mono/SpaceMono-Regular.ttf"))
        .unwrap();
    let space_mono_bold_id = ui
        .fonts
        .insert_from_file(assets.join("fonts/space-mono/SpaceMono-Bold.ttf"))
        .unwrap();
    let fonts = Fonts {
        yh_id: yh_id,
        space_mono_regular_id: space_mono_regular_id,
        space_mono_bold_id: space_mono_bold_id,
    };

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // Our demonstration app that we'll control with our GUI.
    let mut app = DemoApp::new();

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }
        // We'll set all our widgets in a single function called `set_widgets`.
        {
            let mut ui = ui.set_widgets();
            set_widgets(&mut ui, &mut app, &mut ids, fonts);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

/// Set all `Widget`s within the User Interface.
///
/// The first time this gets called, each `Widget`'s `State` will be initialised and cached within
/// the `Ui` at their given indices. Every other time this get called, the `Widget`s will avoid any
/// allocations by updating the pre-existing cached state. A new graphical `Element` is only
/// retrieved from a `Widget` in the case that it's `State` has changed in some way.
fn set_widgets(ui: &mut conrod_core::UiCell, app: &mut DemoApp, ids: &mut Ids, fonts: Fonts) {
    match app.state {
        State::Introduce => set_introduce_widgets(ui, app, ids, fonts),
        State::Game => set_game_widgets(ui, app, ids, fonts),
    }
}

fn set_introduce_widgets(
    ui: &mut conrod_core::UiCell,
    app: &mut DemoApp,
    ids: &mut Ids,
    fonts: Fonts,
) {
    // We can use this `Canvas` as a parent Widget upon which we can place other widgets.
    widget::Canvas::new()
        .border(app.border_width)
        .pad(30.0)
        .color(app.bg_color)
        .scroll_kids()
        .set(ids.canvas, ui);
    widget::Scrollbar::x_axis(ids.canvas)
        .auto_hide(true)
        .set(ids.canvas_y_scrollbar, ui);
    widget::Scrollbar::y_axis(ids.canvas)
        .auto_hide(true)
        .set(ids.canvas_x_scrollbar, ui);

    widget::Text::new("AB Guessing Game")
        .font_id(fonts.yh_id)
        .font_size(38)
        .color(QI_HEI)
        .mid_top_with_margin_on(ids.canvas, 30.0)
        .center_justify()
        .line_spacing(2.5)
        .set(ids.introduce_title, ui);

    let rules = include_str!("../assets/rules.txt");
    widget::Text::new(rules)
        .font_id(fonts.yh_id)
        .font_size(30)
        .color(QI_HEI)
        .mid_top_with_margin_on(ids.canvas, 120.0)
        .center_justify()
        .line_spacing(2.5)
        .set(ids.introduce_text, ui);

    if widget::Button::new()
        .w_h(200.0, 50.0)
        .mid_top_of(ids.canvas)
        .down_from(ids.introduce_text, 45.0)
        // .rgb(0.4, 0.75, 0.6)
        .color(app.bg_color)
        .border(app.border_width)
        .label("START")
        .set(ids.button, ui)
        .was_clicked()
    {
        app.state = State::Game;
    }
}

fn set_game_widgets(ui: &mut conrod_core::UiCell, app: &mut DemoApp, ids: &mut Ids, fonts: Fonts) {
    // We can use this `Canvas` as a parent Widget upon which we can place other widgets.
    widget::Canvas::new()
        .border(app.border_width)
        .pad(30.0)
        .color(app.bg_color)
        .scroll_kids()
        .set(ids.canvas, ui);
    widget::Scrollbar::x_axis(ids.canvas)
        .auto_hide(true)
        .set(ids.canvas_y_scrollbar, ui);
    widget::Scrollbar::y_axis(ids.canvas)
        .auto_hide(true)
        .set(ids.canvas_x_scrollbar, ui);

    // A text box in which we can mutate a single line of text, and trigger reactions via the
    // `Enter`/`Return` key.
    let text = &mut app.text_input;
    let result = &mut app.text_result;
    for event in widget::TextBox::new(text)
        .font_id(fonts.space_mono_regular_id)
        .font_size(36)
        .w_h(130.0, 40.0)
        .top_left_of(ids.canvas)
        .border(app.border_width)
        .border_color(app.bg_color.invert().plain_contrast())
        .color(app.bg_color.invert())
        .set(ids.text_input, ui)
    {
        match event {
            widget::text_box::Event::Enter => {
                if (*text).trim() == "answer" {
                    let answer: String = app.secret_number.iter().map(|i| i.to_string()).collect();
                    *result = (*result).to_string() + "\n" + answer.as_str();
                } else {
                    match Number::new((*text).as_str()) {
                        Ok(num) => {
                            app.step += 1;
                            let (a, b) = num.ab_check(app.secret_number);
                            let ll = format!("{}. {}. {}A{}B", app.step, *text, a, b);
                            *result = (*result).to_string() + "\n" + ll.as_str();
                        }
                        Err(be) => {
                            *result = (*result).to_string() + "\n" + be.to_string().as_str();
                        }
                    };
                }
            }
            widget::text_box::Event::Update(string) => {
                *text = string;
            }
        }
    }

    widget::Text::new(result)
        .font_id(fonts.space_mono_regular_id)
        .font_size(28)
        .color(QI_HEI)
        .top_left_of(ids.canvas)
        .right_from(ids.text_input, 35.0)
        .left_justify()
        .set(ids.text_show, ui);
}
