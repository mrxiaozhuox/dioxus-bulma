pub mod button;

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
