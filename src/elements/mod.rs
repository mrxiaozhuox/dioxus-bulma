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

#[derive(Props)]
pub struct ContentProps<'a> {
    #[props(optional)]
    size: Option<crate::Sizes>,

    children: Element<'a>,
}

pub fn Content<'a>(cx: Scope<'a, ContentProps<'a>>) -> Element {
    let extra_class = if cx.props.size.is_some() {
        cx.props.size.as_ref().unwrap().to_string()
    } else {
        String::new()
    };
    cx.render(rsx! {
        div {
            class: "content {extra_class}",
            &cx.props.children
        }
    })
}
