use dioxus::prelude::*;

use crate::{Colors, Sizes};

#[derive(Props, PartialEq)]
pub struct ProgressProps {
    #[props(default)]
    max: u16,
    #[props(default)]
    value: u16,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,
}

pub fn Progress(cx: Scope<ProgressProps>) -> Element {
    let mut extra_class = String::new();

    if cx.props.size.is_none() {
        extra_class += &format!(" is-{}", cx.props.size.as_ref().unwrap().to_string());
    }

    if cx.props.color.is_none() {
        extra_class += &format!(" is-{}", cx.props.color.as_ref().unwrap().to_string());
    }

    cx.render(rsx! {
        progress {
            class: "progress {extra_class}",
            value: "{cx.props.value}",
        }
    })
}
