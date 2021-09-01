#![allow(non_upper_case_globals)]

use fltk::{enums::Event, prelude::{WidgetExt, WidgetBase}};

bitflags::bitflags! {
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
        let x = self.x();
        let y = self.y();
        let w = self.w();
        let h = self.h();
        self.handle(move |s, ev| match ev {
            Event::Resize => {
                if anchor == Anchor::Left {
                    s.resize(x, s.y(), s.w(), s.h());
                } else if anchor == Anchor::Right {
                    s.resize(s.x(), s.y(), w, s.h());
                } else if anchor == Anchor::Top {
                    s.resize(s.x(), y, s.w(), s.h());
                } else if anchor == Anchor::Bottom {
                    s.resize(s.x(), s.y(), s.w(), h);
                } else if anchor == Anchor::Left | Anchor::Right {
                    s.resize(x, s.y(), w, s.h());
                } else if anchor == Anchor::Top | Anchor::Right {
                    s.resize(s.x(), y, w, s.h());
                } else if anchor == Anchor::Bottom | Anchor::Right {
                    s.resize(s.x(), s.y(), w, h);
                } else if anchor == Anchor::Top | Anchor::Left {
                    s.resize(x, y, s.w(), s.h());
                } else if anchor == Anchor::Bottom | Anchor::Left {
                    s.resize(x, s.y(), s.w(), h);
                } else if anchor == Anchor::Top | Anchor::Bottom {
                    s.resize(s.x(), y, s.w(), h);
                } else if anchor == Anchor::Top | Anchor::Bottom | Anchor::Left {
                    s.resize(x, y, s.w(), h);
                } else if anchor == Anchor::Top | Anchor::Bottom | Anchor::Right {
                    s.resize(s.x(), y, w, h);
                } else if anchor == Anchor::Top | Anchor::Left | Anchor::Right {
                    s.resize(x, y, w, s.h());
                } else if anchor == Anchor::Bottom | Anchor::Left | Anchor::Right {
                    s.resize(x, s.y(), w, h);
                } else if anchor == Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right {
                    s.resize(x, y, w, h);
                } else {
                    //
                }
                true
            }
            _ => false,
        });
    }

    fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.set_anchor(anchor);
        self
    }
}
