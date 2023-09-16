use std::process;
use cli;

fn main() {
    loop {
        let prompt = cli::prompt(">");
        match prompt.as_str() {
            "exit" => process::exit(0),
            &_ => {
                println!("{:?}", prompt);
            },
        }
    }
}
