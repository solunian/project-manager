use std::env;
use colored::Colorize;

mod handle;

const VERSION: &str = "0.0.1";
const DEV_MODE: bool = true;

fn main() {
    // vector includes executable as first, then args
    let exec_args: Vec<String> = env::args().collect();


    // DEV STUFF
    if DEV_MODE {
        println!("{}", "\n=======================================================".blue().bold());
        println!(        "Command Line `exec_args` (each value surrounded by < >)");
        for i in exec_args.iter() {
            print!("<{}> ", i.purple().bold());
        }
        println!("{}", "\n=======================================================".blue().bold());
    }
    // ===========================


    let args: &[String] = &exec_args[1..];
    
    if args.len() == 0 {
        println!("\ntart - a project manager\n");
        return;
    }
    
    let command: &str = &args[0];

    let message = match command {
        "init" => {
            handle::init(args)
        },
        "destroy" => {
            handle::destroy(args)
        }
        "create" => {
            "create".to_string()
        }
        "status" => {
            "status".to_string()
        },
        "stats" => {
            "stats".to_string()
        },
        "help" => {
            "help".to_string()
        }
        _ => {
            match command {
                "-v" | "--version" => {
                    format!("project-manager v{}", VERSION)
                },
                _ => {
                    "'{}' is not a tart command. See 'tart help'.".to_string()
                }
            }
        }
    };

    println!("\n{}\n", message); // \n for extra space

}
