use dioxus::prelude::*;

use crate::{Colors, Sizes};

#[derive(PartialEq)]
pub enum ButtonState {
    Normal,
    Hover,
    Focus,
    Active,
    Loading,
    Static,
    Disabled,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(optional)]
    color: Option<Colors>,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(default)]
    state: ButtonState,

    #[props(default)]
    is_light: bool,

    #[props(default)]
    is_outlined: bool,

    #[props(default)]
    is_inverted: bool,

    #[props(default)]
    is_rounded: bool,

    #[props(default)]
    is_fullwidth: bool,

    children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let mut class_name = "button".to_string();

    if let Some(color) = &cx.props.color {
        let color_name = color.to_string();
        class_name = format!("{class_name} is-{color_name}");
    }

    if let Some(size) = &cx.props.size {
        let size_name = size.to_string();
        class_name = format!("{class_name} is-{size_name}");
    }

    if cx.props.is_light {
        class_name += " is-light";
    }

    if cx.props.is_outlined {
        class_name += " is-outlined";
    }

    if cx.props.is_inverted {
        class_name += " is-inverted";
    }

    if cx.props.is_rounded {
        class_name += " is-rounded";
    }

    if cx.props.is_fullwidth {
        class_name += " is-fullwidth";
    }

    let state = &cx.props.state;
    let mut disabled = "false";
    if *state != ButtonState::Normal {
        match state {
            ButtonState::Normal => {}
            ButtonState::Hover => {
                class_name += " is-hovered";
            }
            ButtonState::Focus => {
                class_name += " is-focused";
            }
            ButtonState::Active => {
                class_name += " is-active";
            }
            ButtonState::Loading => {
                class_name += " is-loading";
            }
            ButtonState::Static => {
                class_name += " is-static";
            }
            ButtonState::Disabled => {
                disabled = "true";
            }
        }
    }

    cx.render(rsx! {
        button {
            class: "{class_name}",
            disabled: "{disabled}",
            &cx.props.children
        }
    })
}
