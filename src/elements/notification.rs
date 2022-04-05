use dioxus::prelude::*;

use crate::Colors;

#[derive(Props)]
pub struct NotificationProps<'a> {
    #[props(optional)]
    color: Option<Colors>,

    #[props(default)]
    is_light: bool,

    #[props(default)]
    is_deletable: bool,

    children: Element<'a>,
}

pub fn Notification<'a>(cx: Scope<'a, NotificationProps<'a>>) -> Element {
    let closed = use_state(&cx, || false);
    if *closed.get() {
        return None;
    }

    let mut class_name = "notification".to_string();

    if let Some(color) = &cx.props.color {
        let color_name = color.to_string();
        class_name = format!("{class_name} is-{color_name}");
    }

    if cx.props.is_light {
        class_name += " is-light";
    }

    if cx.props.is_deletable {
        cx.render(rsx! {
            div {
                class: "{class_name}",
                button {
                    class: "delete",
                    onclick: move |_| { closed.set(true); }
                }
                &cx.props.children
            }
        })
    } else {
        cx.render(rsx! {
            div {
                class: "{class_name}",
                &cx.props.children
            }
        })
    }
}
