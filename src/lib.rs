#![allow(non_snake_case)]

pub mod columns;
pub mod components;
pub mod elements;
pub mod layout;

pub mod prelude {
    pub use crate::{columns::*, components::*, elements::*, layout::*};
    pub use crate::{get_bulma_cdn, get_bulma_css, Colors, Sizes};
}

/// include bulma with css content
pub fn get_bulma_css() -> &'static str {
    include_str!("./assets/bulma.min.css")
}

/// get bulma cdn url
pub fn get_bulma_cdn() -> &'static str {
    "https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css"
}

#[derive(Clone, PartialEq)]
pub enum Colors {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Black,
    Dark,
    Light,
    White,
}

impl ToString for Colors {
    fn to_string(&self) -> String {
        match self {
            Colors::Primary => "primary",
            Colors::Link => "link",
            Colors::Info => "info",
            Colors::Success => "success",
            Colors::Warning => "warning",
            Colors::Danger => "danger",
            Colors::Black => "black",
            Colors::Dark => "dark",
            Colors::Light => "light",
            Colors::White => "white",
        }
        .to_string()
    }
}

#[derive(PartialEq)]
pub enum Sizes {
    Small,
    Normal,
    Medium,
    Large,
}

impl ToString for Sizes {
    fn to_string(&self) -> String {
        match self {
            Sizes::Small => "small",
            Sizes::Normal => "normal",
            Sizes::Medium => "medium",
            Sizes::Large => "large",
        }
        .to_string()
    }
}

impl Default for Sizes {
    fn default() -> Self {
        Self::Normal
    }
}
