use fltk::{prelude::*, *};
use fltk_anchor::{Anchor, Anchored};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);


    let mut win = window::Window::new(438, 193, 565, 518, "Fl2Rust test");

    let mut button1 = button::Button::new(10, 10, 80, 25, "Button");
    let mut button2 = button::Button::new(475, 10, 80, 25, "Button2");
    let mut button3 = button::Button::new(475, 483, 80, 25, "Button3");
    let mut button4 = button::Button::new(10, 483, 80, 25, "Button4");

    let mut slider = valuator::Slider::new(10, 45, 545, 30, "Slider");
    slider.set_type(valuator::SliderType::Horizontal);

    let mut text_display = text::TextDisplay::new(10, 105, 545, 372, None);

    button1.set_anchor(Anchor::Left | Anchor::Top);
    button2.set_anchor(Anchor::Right | Anchor::Top);
    button3.set_anchor(Anchor::Right | Anchor::Bottom);
    button4.set_anchor(Anchor::Left | Anchor::Bottom);

    slider.set_anchor(Anchor::Left | Anchor::Top | Anchor::Right);

    text_display.set_anchor(Anchor::Left | Anchor::Top | Anchor::Right | Anchor::Bottom);

    win.end();
    win.make_resizable(true);
    win.show();

    app.run().unwrap();
}
