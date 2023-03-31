/*!
# fltk-anchor

An anchoring mechanism for fltk-rs widgets, useful when resizing the parent to override FLTK's default resizing defaults.

## Usage
```toml,no_run
[dependencies]
fltk = 1.1
fltk-anchor = "0.1"
```

## Example

```rust,no_run
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
```
This indicates to fltk that when resizing, the button has a fixed size and position, while the input fills the remaining part of the window.
*/

#![allow(non_upper_case_globals)]
#![allow(clippy::needless_doctest_main)]

use fltk::{
    enums::Event,
    prelude::{WidgetBase, WidgetExt},
};

bitflags::bitflags! {
    #[derive(Debug, Clone, Ord, Hash, PartialOrd, Eq, PartialEq, Copy)]
    pub struct Anchor: i32 {
        const None = 0;
        const Left = 10;
        const Top = 100;
        const Right = 1000;
        const Bottom = 10000;
    }
}

pub trait Anchored<T>
where
    T: WidgetExt + WidgetBase,
{
    fn set_anchor(&mut self, anchor: Anchor);
    fn with_anchor(self, anchor: Anchor) -> Self
    where
        Self: Sized;
}

impl<T> Anchored<T> for T
where
    T: WidgetExt + WidgetBase,
{
    fn set_anchor(&mut self, anchor: Anchor) {
        let parent = self.top_window().unwrap();
        let p_w = parent.w();
        let p_h = parent.h();
        let x = self.x();
        let y = self.y();
        let w = self.w();
        let h = self.h();
        let d_w = p_w - w;
        let d_h = p_h - h;
        let d_x = p_w - x;
        let d_y = p_h - y;
        self.handle(move |s, ev| match ev {
            Event::Resize => {
                let p_w = parent.w();
                let p_h = parent.h();
                if anchor == Anchor::Left {
                    s.resize(x, p_h - d_y, w, h);
                } else if anchor == Anchor::Right {
                    s.resize(p_w - d_x, p_h - d_y, w, h);
                } else if anchor == Anchor::Top {
                    s.resize(p_w - d_x, y, w, h);
                } else if anchor == Anchor::Bottom {
                    s.resize(p_w - d_x, p_h - d_y, w, h);
                } else if anchor == Anchor::Left | Anchor::Right { // ?
                    s.resize(x, p_h - d_y, p_w - d_w, h);
                } else if anchor == Anchor::Top | Anchor::Right {
                    s.resize(p_w - d_x, y, w, h);
                } else if anchor == Anchor::Bottom | Anchor::Right {
                    s.resize(p_w - d_x, p_h - d_y, w, h);
                } else if anchor == Anchor::Top | Anchor::Left {
                    s.resize(x, y, w, h);
                } else if anchor == Anchor::Bottom | Anchor::Left {
                    s.resize(x, p_h - d_y, w, h);
                } else if anchor == Anchor::Top | Anchor::Bottom { // ?
                    s.resize(p_w - d_x, y, w, p_h - d_h);
                } else if anchor == Anchor::Top | Anchor::Bottom | Anchor::Left {
                    s.resize(x, y, w, p_h - d_h);
                } else if anchor == Anchor::Top | Anchor::Bottom | Anchor::Right { // ?
                    s.resize(s.x(), y, p_w - d_w, p_h - d_h);
                } else if anchor == Anchor::Top | Anchor::Left | Anchor::Right {
                    s.resize(x, y, p_w - d_w, h);
                } else if anchor == Anchor::Bottom | Anchor::Left | Anchor::Right { // ?
                    s.resize(x, s.y(),  p_w - d_w, p_h - d_h);
                } else if anchor == Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right {
                    s.resize(x, y, p_w - d_w, p_h - d_h);
                } else { // ? None
                    s.resize(p_w - d_x, p_h - d_y, w, h);
                }
                false
            }
            _ => false,
        });
    }

    fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.set_anchor(anchor);
        self
    }
}
