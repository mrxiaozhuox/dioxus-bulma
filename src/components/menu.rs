use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct MenuProps {

}

pub fn Menu(cx: Scope) -> Element {
    cx.parent();
    cx.render(rsx! {
        aside {
            class: "menu",

        }
    })
}