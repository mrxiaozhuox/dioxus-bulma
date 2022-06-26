use dioxus::prelude::*;

use crate::Sizes;

#[derive(Clone)]
pub enum Separator {
    Arrow,
    Bullet,
    Dot,
    Succeeds,
}

impl ToString for Separator {
    fn to_string(&self) -> String {
        match self {
            Separator::Arrow => "arrow",
            Separator::Bullet => "bullet",
            Separator::Dot => "dot",
            Separator::Succeeds => "succeeds",
        }.to_string()
    }
}

#[derive(Props)]
pub struct BreadcurmbProps<'a> {
    #[props(default)]
    centered: bool,
    #[props(default)]
    right: bool,

    #[props(optional)]
    separator: Option<Separator>,
    
    #[props(optional)]
    size: Option<Sizes>,

    children: Element<'a>,
}

pub fn Breadcurmb<'a>(cx: Scope<'a, BreadcurmbProps<'a>>) -> Element {

    let mut extra_class = String::new();
    
    if cx.props.centered {
        extra_class += "is-centered";
    } else if cx.props.right {
        extra_class += "is-right";
    }

    if cx.props.separator.is_some() {
        let separator = cx.props.separator.as_ref().unwrap().to_string();
        extra_class += &format!("has-{}-separator", separator);
    }

    if cx.props.size.is_some() {
        let size = cx.props.size.as_ref().unwrap().to_string();
        extra_class += &format!("is-{}", size);
    }

    cx.render(rsx! {
        nav {
            class: "breadcrumb {extra_class}",
            &cx.props.children
        }    
    })
}