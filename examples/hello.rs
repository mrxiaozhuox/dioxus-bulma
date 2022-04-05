use dioxus::prelude::*;
use dioxus_bulma::{
    columns::{Column, Columns},
    elements::{notification::Notification, Button},
    Colors,
};

fn main() {
    dioxus::desktop::launch(App)
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let cdn_url = dioxus_bulma::get_bulma_cdn();
    cx.render(rsx! {
        link { rel: "stylesheet", href: "{cdn_url}" }
        br {}
        div {
            class: "container",
        }
    })
}
