use crate::math::{Constraint, Size};
use crate::widget::{downcast, Element, IWidget, PaintContext, RenderElement, Widget};

pub struct SizedBox {
    pub size: Size,
    pub child: Widget,
}

struct SizedBoxElement {
    size: Size,
    child: Element,
}

impl IWidget for SizedBox {
    fn create_element(&self) -> Element {
        Element::new(self.widget_id(), SizedBoxElement {
            size: self.size,
            child: self.child.create_element(),
        })
    }

    fn update_element(&self, element: &mut Element) {
        let mut inner = downcast::<SizedBoxElement>(element.inner.as_mut());
        if inner.size != self.size {
            inner.size = self.size;
            element.needs_layout = true;
        }
        if inner.child.widget_id != self.child.widget_id() {
            inner.child = self.child.create_element();
            element.needs_layout = true;
        } else {
            self.child.update_element(&mut inner.child);
        }
    }
}

impl RenderElement for SizedBoxElement {
    fn layout(&self, constraint: Constraint) -> Size {
        constraint.max_size()
    }

    fn paint(&self, size: Size, context: &mut PaintContext) {
        self.child.paint(size, context);
    }
}