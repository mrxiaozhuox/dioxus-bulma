mod button;
mod image;
mod notification;
mod progress;
pub mod tag;

pub use button::{Button, ButtonProps, ButtonState};
pub use image::{Image, ImageProps};
pub use notification::{Notification, NotificationProps};
pub use progress::{Progress, ProgressProps};

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