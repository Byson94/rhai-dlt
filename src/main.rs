use rhai_dlt::parser::parse_widget_code;
use std::fs;

fn main() {
    let input = fs::read_to_string("test.rhai")
        .expect("Should have been able to read the file");

    let result = parse_widget_code(&input);
    println!("Result: {:#?}", result);
}
