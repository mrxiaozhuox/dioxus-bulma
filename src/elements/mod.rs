pub mod button;
pub mod image;

pub use button::{Button, ButtonState};

use dioxus::prelude::*;

#[inline_props]
pub fn Block<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            class: "block",
            children
        }
    })
}

#[inline_props]
pub fn Box<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            class: "box",
            children
        }
    })
}

#[inline_props]
pub fn Content<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            class: "content",
            children
        }
    })
}