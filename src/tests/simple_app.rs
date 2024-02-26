use crate::context::Context;
use crate::math::Size;
use crate::style::Color;
use crate::widget::basic::{ColoredBox, Leaf, SizedBox};
use crate::widget::Widget;

#[test]
fn test_simple_app() {
    let app = crate::run(build);
}

fn build(context: Context) -> Widget {
    SizedBox {
        size: Size::square(100.0),
        child: ColoredBox {
            color: Color::BLACK,
            child: Leaf.into(),
        }.into(),
    }.into()
}