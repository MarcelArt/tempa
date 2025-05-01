use std::env;

mod new_cmd;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <name>", args[0]);
        std::process::exit(1);
    }

    let cmd = &args[1];

    match cmd.as_str() {
        "new" => {
            if args.len() < 3 {
                eprintln!("Usage: {} new <name>", args[0]);
                std::process::exit(1);
            }
            let project_name = &args[2];
            new_cmd::execute(project_name.clone());
        }
        _ => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
    }
}
