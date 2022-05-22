use std::env::args;

use fltk::{app::{App, Screen, set_font_size}, window::Window, prelude::{WidgetBase, WindowExt, DisplayExt, WidgetExt}, text::{TextDisplay, TextBuffer}};
use fltk_theme::{ColorTheme, color_themes, WidgetScheme, SchemeType};

fn main() {
    let mut arguments = args();
    arguments.next();
    let text = arguments.next().unwrap();

    let app = App::default();

    ColorTheme::new(color_themes::DARK_THEME).apply();
    WidgetScheme::new(SchemeType::Fluent).apply();
    set_font_size(20);

    let screens = Screen::all_screens();
    let screen = screens.first().expect("Couldn't get screen"); // Boo hoo single screen setup

    let width = 300;
    let height = 80;
    let mut win = Window::new(screen.w()-width, 0, width, height, "");
    win.set_border(false);

    let mut buf = TextBuffer::default();
    let mut display = TextDisplay::default_fill();
    buf.set_text(&text);
    display.set_buffer(buf);

    win.show();
    app.run().unwrap();
}
