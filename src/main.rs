use rhai_dlt::parser::parse_widget_code;
use std::fs;

fn main() {
    let input = fs::read_to_string("test.rhai")
        .expect("Should have been able to read the file");

    match parse_widget_code(&input) {
        Ok(output) => {
            println!("{}", output); // optional
            fs::write("./eww/eww.yuck", output)
                .expect("Should be able to write the file");
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
