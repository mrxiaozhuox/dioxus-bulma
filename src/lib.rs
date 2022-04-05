#![allow(non_snake_case)]

pub mod columns;
pub mod elements;
pub mod components;

/// include bulma with css content
pub fn get_bulma_css() -> &'static str {
    include_str!("./assets/bulma.min.css")
}

/// get bulma cdn url
pub fn get_bulma_cdn() -> &'static str {
    "https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css"
}

pub enum Colors {
    Primary,
    Link,
    Info,
    Success,
    
}