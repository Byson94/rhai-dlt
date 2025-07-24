use rhai::{Engine, Scope, EvalAltResult};
use crate::builtins::register_all_widgets;
use crate::widgetnode::WidgetNode;
use crate::transpiler::to_pretty_yuck;

pub fn parse_widget_code(code: &str) -> Result<String, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.set_max_expr_depths(128, 128);
    register_all_widgets(&mut engine);
    let mut scope = Scope::new();
    match engine.eval_with_scope::<WidgetNode>(&mut scope, code) {
        Ok(widget) => {
            println!("{:#?}", widget);
            Ok(to_pretty_yuck(&widget))
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            let pos = e.position();
            if !pos.is_none() {
                eprintln!("At: line {}, column {}", pos.line().unwrap_or(0), pos.position().unwrap_or(0));
            }

            Err(e)
        }
    }
}
