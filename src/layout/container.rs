use dioxus::prelude::*;

#[derive(Props)]
pub struct ContainerProps<'a> {
    #[props(default)]
    widescreen: bool,

    #[props(default)]
    fullhd: bool,

    #[props(default)]
    max_desktop: bool,

    #[props(default)]
    max_widescreen: bool,

    #[props(default)]
    fluid: bool,

    children: Element<'a>,
}

pub fn Container<'a>(cx: Scope<'a, ContainerProps<'a>>) -> Element {

    let extra_class = if cx.props.widescreen {
        "is-widescreen"
    } else if cx.props.fullhd {
        "is-fullhd"
    } else if cx.props.max_desktop {
        "is-max-desktop"
    } else if cx.props.max_widescreen {
        "is-max-widescreen"
    } else if cx.props.fluid {
        "is-fluid"
    } else {
        ""
    };

    cx.render(rsx! {
        div {
            class: "container {extra_class}",
            &cx.props.children
        }
    })
}
