use crate::math::{Constraint, Size};
use crate::style::Color;
use crate::widget::{downcast, Element, IWidget, PaintContext, RenderElement, Widget};

pub struct ColoredBox {
    pub color: Color,
    pub child: Widget,
}

struct ColoredBoxElement {
    color: Color,
    child: Element,
}

impl IWidget for ColoredBox {
    fn create_element(&self) -> Element {
        let child = self.child.create_element();
        Element::new(self.widget_id(), ColoredBoxElement {
            color: self.color,
            child,
        })
    }

    fn update_element(&self, element: &mut Element) {
        let inner = downcast::<ColoredBoxElement>(element.inner.as_mut());
        if inner.color != self.color {
            inner.color = self.color;
            element.needs_repaint = true;
        }

        if self.child.widget_id() != inner.child.widget_id {
            inner.child = self.child.create_element();
            element.needs_layout = true;
        } else {
            self.child.update_element(&mut inner.child);
        }
    }
}

impl RenderElement for ColoredBoxElement {
    fn layout(&self, constraint: Constraint) -> Size {
        self.child.layout(constraint)
    }

    fn paint(&self, size: Size, context: &mut PaintContext) {
        context
            .rect()
            .with_color(self.color)
            .with_bounds(size)
            .fill();
    }
}