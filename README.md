# fltk-anchor

An anchoring mechanism for fltk-rs widgets, useful when resizing the parent to override FLTK's default resizing defaults.

## Usage
```toml
[dependencies]
fltk = 1.1
fltk-anchor = "0.1"
```

## Example

```rust
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
```
This indicates to fltk that when resizing, the button shouldn't move left, top nor bottom (the height is fixed). However, the right side which is not anchored, will cause the button's width to be modified when resizing.
