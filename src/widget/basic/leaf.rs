use crate::math::{Constraint, Size};
use crate::widget::{Element, IWidget, PaintContext, RenderElement};

pub struct Leaf;

impl IWidget for Leaf {
    fn create_element(&self) -> Element {
        Element::new(self.widget_id(), LeafElement)
    }

    fn update_element(&self, _element: &mut Element) {
    }
}

struct LeafElement;

impl RenderElement for LeafElement {
    fn layout(&self, _constraint: Constraint) -> Size {
        Size::ZERO
    }

    fn paint(&self, _size: Size, _context: &mut PaintContext) {}
}