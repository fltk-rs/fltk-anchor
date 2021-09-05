use fltk::{prelude::*, *};
use fltk_anchor::{Anchor, Anchored};

const PADDING: i32 = 8;

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);

    button::Button::new(PADDING, PADDING, 80, 40, "Click").with_anchor(Anchor::Left | Anchor::Top);

    input::MultilineInput::new(
        PADDING,
        PADDING * 2 + 40,
        400 - PADDING * 2,
        300 - 40 - PADDING * 3,
        "",
    )
    .with_anchor(Anchor::Left | Anchor::Right | Anchor::Top | Anchor::Bottom);

    win.end();

    win.make_resizable(true);
    win.show();

    a.run().unwrap();
}
