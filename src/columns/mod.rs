use dioxus::prelude::*;

#[derive(Props)]
pub struct ColumnsProps<'a> {
    #[props(default)]
    is_mobile: bool,

    #[props(default)]
    is_gapless: bool,

    #[props(default)]
    is_multiline: bool,

    #[props(default)]
    is_centered: bool,

    #[props(default)]
    is_vcentered: bool,

    #[props(optional)]
    variable_gap: Option<u8>,

    // #[props(optional)]
    // custom_class: Option<String>,

    children: Element<'a>,
}

pub fn Columns<'a>(cx: Scope<'a, ColumnsProps<'a>>) -> Element {
    let mut class_name = "columns".to_string();

    if cx.props.is_mobile {
        class_name += " is-mobile";
    }

    if cx.props.is_gapless {
        class_name += " is-gapless";
    }

    if cx.props.is_multiline {
        class_name += " is-multiline";
    }

    if cx.props.is_centered {
        class_name += " is-centered";
    }

    if cx.props.is_vcentered {
        class_name += " is-vcentered";
    }

    if let Some(num) = cx.props.variable_gap {
        if (0..=8).contains(&num) {
            class_name = format!("{class_name} is-variable is-{num}");
        }
    }

    // if let Some(class) = &cx.props.custom_class {
    //     class_name += class;
    // }

    cx.render(rsx! {
        div {
            class: "{class_name}",
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct ColumnProps<'a> {
    #[props(default)]
    is_narrow: bool,

    #[props(optional)]
    size: Option<u8>,

    #[props(optional)]
    offset: Option<u8>,

    // #[props(optional)]
    // custom_class: Option<String>,

    children: Element<'a>,
}

pub fn Column<'a>(cx: Scope<'a, ColumnProps<'a>>) -> Element {
    let mut class_name = "column".to_string();

    if cx.props.is_narrow {
        class_name += " is-narrow";
    }

    if let Some(num) = cx.props.size {
        if (0..12).contains(&num) {
            class_name = format!("{class_name} is-{num}");
        }
    }

    if let Some(num) = cx.props.offset {
        if (0..12).contains(&num) {
            class_name = format!("{class_name} is-offset-{num}");
        }
    }

    // if let Some(class) = &cx.props.custom_class {
    //     class_name += class;
    // }

    cx.render(rsx! {
        div {
            class: "{class_name}",
            &cx.props.children
        }
    })
}
