use std::io;
use std::io::Write;
use std::collections::HashMap;

pub fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut user_input = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();
    return String::from(user_input.trim());
}

pub fn parse_kwargs(input: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut args = input.split_whitespace();
    let mut pending_key = None;

    while let Some(arg) = args.next() {
        if arg.starts_with('-') {
            let key = arg.trim_start_matches('-').to_string();
            pending_key = Some(key);
        } else if let Some(key) = pending_key.take() {
            if arg.starts_with('"') {
                let mut value = arg.trim_matches('"').to_string();
                while !value.ends_with('"') {
                    if let Some(next_arg) = args.next() {
                        value.push(' ');
                        value.push_str(next_arg);
                    } else {
                        break;
                    }
                }
                if key != "cmd" {
                    map.insert(key, value);
                }
            } else {
                if key != "cmd" {
                    map.insert(key, arg.to_string());
                }
            }
        } else {
            map.insert("cmd".to_string(), arg.to_string());
        }
    }
    map
}
