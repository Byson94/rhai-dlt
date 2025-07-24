use rhai::{Dynamic, Map};
use crate::widgetnode::WidgetNode;
use std::collections::HashMap;

pub fn to_pretty_yuck(node: &WidgetNode) -> String {
    to_pretty_yuck_inner(node, 0)
}

fn to_pretty_yuck_inner(node: &WidgetNode, indent: usize) -> String {
    let ind = "  ".repeat(indent);
    let newline = "\n";

    match node {
        WidgetNode::Label(text) => format!("{ind}(label \"{}\")", escape_str(text)),

        WidgetNode::Box { props, children } => {
            format_widget("box", props, children, indent)
        }

        WidgetNode::CenterBox { props, children } => {
            format_widget("centerbox", props, children, indent)
        }

        WidgetNode::Button { props, children } => {
            format_widget("button", props, children, indent)
        }

        WidgetNode::Image { props } => format!("{ind}(image{})", format_props(props)),
        WidgetNode::Input { props } => format!("{ind}(input{})", format_props(props)),
        WidgetNode::Progress { props } => format!("{ind}(progress{})", format_props(props)),
        WidgetNode::Spacer { props } => format!("{ind}(spacer{})", format_props(props)),
        WidgetNode::Slider { props } => format!("{ind}(slider{})", format_props(props)),
        WidgetNode::Calendar { props } => format!("{ind}(calendar{})", format_props(props)),
        WidgetNode::Graph { props } => format!("{ind}(graph{})", format_props(props)),

        WidgetNode::Scroll { props, children } => {
            format_widget("scroll", props, children, indent)
        }

        WidgetNode::Revealer { props, children } => {
            format_widget("revealer", props, children, indent)
        }

        WidgetNode::EventBox { props, children } => {
            format_widget("eventbox", props, children, indent)
        }

        WidgetNode::Include(path) => format!("{ind}(include \"{}\")", escape_str(path)),
        WidgetNode::DefStyle(style) => format!("{ind}(defstyle \"{}\")", escape_str(style)),

        WidgetNode::DefWidget { name, node } => {
            let inner_yuck = to_pretty_yuck_inner(node, indent + 1);
            // TODO: fill the [] with params
            format!("{ind}(defwidget {name} []{newline}{inner_yuck}{newline}{ind})")
        }

        WidgetNode::DefWindow { name, props, widget } => {
            let ind = "  ".repeat(indent);
            let mut out = format!("{ind}(defwindow {name}");
            out.push_str(&format_window_special_props(props, indent + 1));
            out.push_str(&format!(" ({widget})"));
            out.push('\n');
            out.push_str(&format!("{ind})"));
            out
        }

        WidgetNode::Poll { var, interval, cmd, initial } => {
            format!(
                "{ind}(defpoll {} :interval \"{}\" :initial \"{}\" \"{}\")",
                var, interval, initial, cmd
            )
        }

        WidgetNode::Listen { var, signal } => {
            format!("{ind}(deflisten {} :initial \"\" \"{}\")", var, signal)
        }


        WidgetNode::Enter(children) => {
            let mut out = String::new();
            for child in children {
                out.push_str(&to_pretty_yuck_inner(child, 0));
                out.push('\n');
            }
            out.trim_end().to_string()
        }
    }
}

// Helper to format any widget with props and children
fn format_widget(name: &str, props: &Map, children: &[WidgetNode], indent: usize) -> String {
    let ind = "  ".repeat(indent);
    let newline = "\n";

    if children.is_empty() {
        format!("{ind}({}{})", name, format_props(props))
    } else {
        let mut out = format!("{ind}({}{}", name, format_props(props));
        for child in children {
            out.push('\n');
            out.push_str(&to_pretty_yuck_inner(child, indent + 1));
        }
        out.push('\n');
        out.push_str(&format!("{ind})"));
        out
    }
}

fn format_window_special_props(props: &Map, indent: usize) -> String {
    let mut out = String::new();
    let ind = "  ".repeat(indent);

    // Map special keys like "geometry" and "reserve" to their widget names
    let special_map: HashMap<&str, &str> = HashMap::from([
        ("geometry", "geometry"),
        ("reserve", "struts"),
    ]);

    for (key, val) in props.iter() {
        if let Some(widget_name) = special_map.get(key.as_str()) {
            if val.is::<Map>() {
                let map = val.clone().cast::<Map>();
                out.push_str(&format!(" :{} ({})", key, format_map_as_widget(widget_name, &map)));
                continue;
            }
        }

        // Fallback for normal props
        out.push_str(" :");
        out.push_str(key);
        out.push(' ');
        out.push_str(&format_dynamic(val));
    }

    out
}


fn format_map_as_widget(widget: &str, map: &Map) -> String {
    let mut out = format!("{widget}");
    for (k, v) in map.iter() {
        out.push_str(" :");
        out.push_str(k);
        out.push(' ');
        out.push_str(&format_dynamic(v));
    }
    out
}


// Convert props Map to `:key value` string
fn format_props(props: &Map) -> String {
    let mut out = String::new();
    for (key, val) in props.iter() {
        out.push_str(" :");
        out.push_str(key);
        out.push(' ');
        out.push_str(&format_dynamic(val));
    }
    out
}

// Convert a Rhai Dynamic to Yuck-compatible string
fn format_dynamic(val: &Dynamic) -> String {
    if val.is::<bool>() || val.is::<i64>() || val.is::<f64>() {
        val.to_string()
    } else if val.is::<String>() {
        format!("\"{}\"", escape_str(&val.clone().cast::<String>()))
    } else if val.is::<Map>() {
        let map = val.clone().cast::<Map>();
        let mut inner = String::from("#{");
        for (k, v) in map.iter() {
            inner.push_str(&format!(" \"{}\": {},", k, format_dynamic(v)));
        }
        if inner.ends_with(',') {
            inner.pop(); // remove trailing comma
        }
        inner.push('}');
        inner
    } else {
        // fallback
        format!("\"{}\"", val.to_string())
    }
}

// Escape quotes or special characters if needed
fn escape_str(input: &str) -> String {
    input.replace('"', "\\\"")
}
