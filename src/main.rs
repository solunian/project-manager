use std::env;
use colored::Colorize;

use crate::handle::Message;

mod handle;

const DEV_MODE: bool = true;

fn main() {
  // Vec includes executable as first, then args
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
  let mut message: Message = handle::help(args);
  
  if args.len() > 0 {
    let command: &str = &args[0];

    message = match command {
      "init" => {
        handle::init(args)
      },
      "destroy" => {
        handle::destroy(args)
      }
      "create" => {
        handle::create(args)
      }
      "status" => {
        "status not implemented yet".to_string()
      },
      "stats" => {
        "stats not implemented yet".to_string()
      },
      "help" => {
        handle::help(args)
      }
      _ => {
        handle::tart(args)
      }
    };

  }
  
  println!("\n{}\n", message); // \n for extra space

}
