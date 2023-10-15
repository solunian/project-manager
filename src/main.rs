use std::env;

mod handle;

const VERSION: &str = "0.0.1";
const DEV_MODE: bool = true;

fn main() {
    let args: Vec<String> = env::args().collect();


    // DEV STUFF
    if DEV_MODE {
        println!("\nCommand Line Args");
        for i in args.iter() {
            print!("<{}> ", i);
        }
        println!("\n\n============================\n");
    }
    // ===========================

    if args.len() <= 1 {
        return;
    }

    let command: &str = &args[1];

    match command {
        "-v" | "--version" => {
            println!("project-manager v{}", VERSION);
        },
        "init" => {
            println!("init");
            handle::init_dot_dir();
        },
        "create" => {
            println!("create");
        }
        "status" => {
            println!("status");
        },
        "stats" => {
            println!("stats");
        },
        _ => {
            println!("Command does not match anything. lol!");
        }
    }
}
