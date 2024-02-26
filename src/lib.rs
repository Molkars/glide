#![feature(trait_upcasting)]

mod context;
mod widget;
mod math;
mod style;

#[cfg(test)]
mod tests;

pub fn run(build: fn(context::Context) -> widget::Widget) {
    todo!()
}