use std::env;

/// Parse command-line arguments and return the pattern and optional load file.
pub fn parse_arguments() -> (String, Option<String>, bool) {
    let mut pattern = "line".to_string();
    let mut load_file = None;
    let mut history = false;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--pattern" if i + 1 < args.len() => {
                pattern = args[i + 1].clone();
                i += 2;
            }
            "--load" if i + 1 < args.len() => {
                load_file = Some(args[i + 1].clone());
                i += 2;
            }
            "--history" => {
                history = true;
                i += 1;
            }
            _ => {
                i += 1; // Ignore unknown arguments
            }
        }
    }
    (pattern, load_file, history)
}
