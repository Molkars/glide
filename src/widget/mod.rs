use std::any::{Any, TypeId};
use crate::math::{Constraint, Size};
use crate::style::Color;

pub mod basic;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct WidgetId(TypeId);

pub struct Element {
    pub widget_id: WidgetId,
    pub needs_layout: bool,
    pub needs_repaint: bool,
    inner: Box<dyn RenderElement>,
}

impl Element {
    pub fn new(widget_id: WidgetId, inner: impl RenderElement) -> Self {
        Self {
            widget_id,
            needs_layout: true,
            needs_repaint: true,
            inner: Box::new(inner),
        }
    }
}

pub fn downcast<T: RenderElement>(inner: &mut dyn Any) -> &mut T {
    inner.downcast_mut::<T>().expect("downcast failed")
}

pub trait RenderElement: Any {
    fn layout(&self, constraint: Constraint) -> Size;
    fn paint(&self, size: Size, context: &mut PaintContext);
}

impl RenderElement for Element {
    #[inline]
    fn layout(&self, constraint: Constraint) -> Size {
        self.inner.layout(constraint)
    }

    #[inline]
    fn paint(&self, size: Size, context: &mut PaintContext) {
        self.inner.paint(size, context)
    }
}

pub struct Widget(Box<dyn IWidget>);

impl<T: IWidget> From<T> for Widget {
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

impl IWidget for Widget {
    #[inline]
    fn create_element(&self) -> Element {
        Element::new(self.widget_id(), self.0.create_element())
    }

    #[inline]
    fn update_element(&self, element: &mut Element) {
        self.0.update_element(element);
    }
}

pub trait IWidget: Any {
    fn widget_id(&self) -> WidgetId {
        WidgetId(self.type_id())
    }

    fn create_element(&self) -> Element;
    fn update_element(&self, element: &mut Element);
}

pub struct PaintContext;

impl PaintContext {
    pub fn rect(&mut self) -> RectBuilder {
        RectBuilder {
            context: self,
            color: Color::BLACK,
            bounds: Size::INFINITY,
        }
    }

    pub fn fill_rect(&mut self, color: Color, bounds: Size) {
        // ...
    }
}

pub struct RectBuilder<'a> {
    pub context: &'a mut PaintContext,
    pub color: Color,
    pub bounds: Size,
}

impl<'a> RectBuilder<'a> {
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_bounds(mut self, bounds: Size) -> Self {
        self.bounds = bounds;
        self
    }

    pub fn fill(self) {
        self.context.fill_rect(self.color, self.bounds);
    }
}