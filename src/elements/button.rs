use dioxus::prelude::*;

use crate::{Colors, Sizes};

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
