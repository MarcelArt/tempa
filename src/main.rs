use std::env;

mod new_cmd;
mod add_cmd;
mod utils;

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
        "add" => {
            if args.len() < 4 {
                eprintln!("Usage: {} add <node name> <base node>", args[0]);
                std::process::exit(1);
            }
            let node_name = &args[2];
            let base_node = &args[3];
            if let Err(e) = add_cmd::execute(node_name.clone(), base_node.clone()) {
                eprintln!("Error adding node: {}", e);
                std::process::exit(1);
            }
            println!("Node {} added successfully!", node_name);
        }
        project_name => new_cmd::execute(project_name.to_string())
    }
}
