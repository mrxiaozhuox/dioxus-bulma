use dioxus::prelude::*;

use crate::{Colors, Sizes};

#[derive(Props)]
pub struct TagProps<'a> {
    #[props(optional)]
    color: Option<Colors>,

    #[props(default)]
    is_light: bool,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(default)]
    is_rounded: bool,

    #[props(default)]
    deletable: bool,

    children: Element<'a>,
}

pub fn Tag<'a>(cx: Scope<'a, TagProps<'a>>) -> Element {
    let visible = use_state(&cx, || true);
    if !visible.get() {
        return None;
    }

    let mut extra_class = String::new();

    if cx.props.color.is_some() {
        extra_class += &format!(" is-{}", cx.props.color.as_ref().unwrap().to_string());
    }

    if cx.props.is_light {
        extra_class += " is-light";
    }

    if cx.props.size.is_some() {
        extra_class += &format!(" is-{}", cx.props.size.as_ref().unwrap().to_string());
    }

    if cx.props.is_rounded {
        extra_class += " is-rounded";
    }

    if cx.props.deletable {
        let delete_button_size = match cx.props.size.as_ref().unwrap_or(&Sizes::Normal) {
            Sizes::Small => "small",
            Sizes::Normal => "small",
            Sizes::Medium => "normal",
            Sizes::Large => "medium",
        };
        cx.render(rsx! {
            span {
                class: "tag {extra_class}",
                &cx.props.children
                button {
                    class: "delete is-{delete_button_size}",
                    onclick: move |_| {
                        visible.set(false);
                    }
                }
            }
        })
    } else {
        cx.render(rsx! {
            span {
                class: "tag {extra_class}",
                &cx.props.children
            }
        })
    }
}
