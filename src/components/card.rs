use dioxus::prelude::*;

#[derive(Props)]
pub struct CardProps<'a> {
    #[props(default)]
    header: Element<'a>,

    #[props(default)]
    content: Element<'a>,

    #[props(default)]
    image: Element<'a>,

    #[props(default)]
    footer: Element<'a>,
}

pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "card",
            cx.props.header.as_ref().and_then(|_| cx.render(rsx! {
                header {
                    class: "card-header",
                    &cx.props.header
                }
            }))
            cx.props.image.as_ref().and_then(|_| cx.render(rsx! {
                div {
                    class: "card-header",
                    &cx.props.image
                }
            }))
            cx.props.content.as_ref().and_then(|_| cx.render(rsx! {
                div {
                    class: "card-header",
                    &cx.props.content
                }
            }))
            cx.props.footer.as_ref().and_then(|_| cx.render(rsx! {
                footer {
                    class: "card-header",
                    &cx.props.footer
                }
            }))
        }
    })
}
