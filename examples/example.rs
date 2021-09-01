use fltk::{prelude::*, *};
use fltk_anchor::{Anchor, Anchored};

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    button::Button::new(10, 10, 80, 40, "Click").with_anchor(Anchor::Left | Anchor::Top | Anchor::Bottom);
    win.end();
    win.make_resizable(true);
    win.show();
    a.run().unwrap();
}