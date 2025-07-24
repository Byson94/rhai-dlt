use rhai::{Engine, Array, Map};
use crate::widgetnode::WidgetNode;

pub fn register_all_widgets(engine: &mut Engine) {
    engine.register_type::<WidgetNode>();

    // Primitive widgets
    engine.register_fn("label", |text: &str| {
        WidgetNode::Label(text.to_string())
    });

    engine.register_fn("box", |props: Map, children: Array| {
        WidgetNode::Box {
            props,
            children: children.into_iter().map(|v| v.cast()).collect(),
        }
    });

    engine.register_fn("centerbox", |props: Map, children: Array| {
        WidgetNode::CenterBox {
            props,
            children: children.into_iter().map(|v| v.cast()).collect(),
        }
    });

    engine.register_fn("button", |props: Map, children: Array| {
        let children = children
            .into_iter()
            .map(|v| {
                if v.is::<WidgetNode>() {
                    v.cast::<WidgetNode>()
                } else if v.is::<String>() {
                    WidgetNode::Label(v.cast::<String>())
                } else if v.is::<&str>() {
                    WidgetNode::Label(v.cast::<String>())
                } else {
                    panic!("Unsupported child type in button");
                }
            })
            .collect::<Vec<_>>();

        WidgetNode::Button { props, children }
    });


    engine.register_fn("image", |props: Map| {
        WidgetNode::Image { props }
    });

    engine.register_fn("input", |props: Map| {
        WidgetNode::Input { props }
    });

    engine.register_fn("progress", |props: Map| {
        WidgetNode::Progress { props }
    });

    engine.register_fn("spacer", |props: Map| {
        WidgetNode::Spacer { props }
    });

    engine.register_fn("slider", |props: Map| {
        WidgetNode::Slider { props }
    });

    engine.register_fn("revealer", |props: Map, children: Array| {
        WidgetNode::Revealer {
            props,
            children: children.into_iter().map(|v| v.cast()).collect(),
        }
    });

    engine.register_fn("scroll", |props: Map, children: Array| {
        WidgetNode::Scroll {
            props,
            children: children.into_iter().map(|v| v.cast()).collect(),
        }
    });

    engine.register_fn("calendar", |props: Map| {
        WidgetNode::Calendar { props }
    });

    engine.register_fn("graph", |props: Map| {
        WidgetNode::Graph { props }
    });

    engine.register_fn("include", |path: &str| {
        // TODO: load and eval another config file
        WidgetNode::Include(path.to_string())
    });

    engine.register_fn("defstyle", |style: &str| {
        WidgetNode::DefStyle(style.to_string())
    });

    engine.register_fn("eventbox", |props: Map, children: Array| {
        WidgetNode::EventBox {
            props,
            children: children.into_iter().map(|v| v.cast()).collect(),
        }
    });

    // --- Top-level macros ---

    engine.register_fn("defwidget", |name: &str, node: WidgetNode| {
        WidgetNode::DefWidget { 
            name: name.to_string(),
            node: Box::new(node),
        } 
    });

    engine.register_fn("defwindow", |name: &str, props: Map, widget: String| {
        WidgetNode::DefWindow {
            name: name.to_string(),
            props,
            widget: widget.to_string(),
        }
    });

    engine.register_fn("poll", |var: &str, interval: &str, cmd: &str, initial: &str| {
        WidgetNode::Poll {
            var: var.to_string(),
            interval: interval.to_string(),
            cmd: cmd.to_string(),
            initial: initial.to_string(),
        }
    });

    engine.register_fn("listen", |var: &str, signal: &str| {
        WidgetNode::Listen {
            var: var.to_string(),
            signal: signal.to_string(),
        }
    });

    engine.register_fn("enter", |children: Array| {
        WidgetNode::Enter(children.into_iter().map(|v| v.cast()).collect())
    });
}
