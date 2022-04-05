use dioxus::prelude::*;

#[derive(Props)]
pub struct ImageProps<'a> {
    #[props(optional)]
    size: Option<u8>,

    #[props(optional)]
    ratio: Option<(u8, u8)>,

    #[props(default)]
    is_fullwidth: bool,

    src: &'a str,
}

pub fn Image<'a>(cx: Scope<'a, ImageProps<'a>>) -> Element {
    let mut class_name = "image".to_string();

    if let Some(size) = cx.props.size {
        class_name = format!("{class_name} is-{size}x{size}");
    }

    if let Some(ratio) = cx.props.ratio {
        let a = ratio.0;
        let b = ratio.1;
        class_name = format!("{class_name} is-{a}by{b}");
    }

    if cx.props.is_fullwidth {
        class_name += " is-fullwidth";
    }

    cx.render(rsx! {
        figure {
            class: "{class_name}",
            img {
                src: "{cx.props.src}"
            }
        }
    })
}
