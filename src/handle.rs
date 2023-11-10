use std::{fs, env};

const VERSION: &str = "0.0.1";
const DOT_DIR: &str = "./.tart";

pub type Message = String;


pub fn tart(args: &[String]) -> Message {
  let grouped_args = group_args(args);

  for group in grouped_args {
     // should match group[0] with options or main commands
     // then loop through remaining values in group for certain
    match group[0] {
      "-v" | "--version" => {
         return about_tart();
      },
      _ => {
        return unknown();
      }
    }
  }
  
  help(args) // base function of command without args/opts
}

pub fn init(args: &[String]) -> Message {
  match args.len() {
    1 => init_dot_dir(),
    _ => unknown()
  }
}

pub fn destroy(args: &[String]) -> Message {
  match args.len() {
    1 => {
      match fs::remove_dir_all(DOT_DIR) {
        Ok(()) => format!("Destroyed .tart in {}", get_current_dir()),
        Err(_) => format!("Failed to destroy .tart in {}", get_current_dir())
      }
    },
    _ => unknown()
  }
}

pub fn create(args: &[String]) -> Message {
  match args.len() {
    2 => {
      format!("Created project {}", args[1])
    },
    _ => unknown()
  }
}

pub fn help(_args: &[String]) -> Message {
"tart: a simple command-line project manager
  
work in progress!".to_string()
}

fn about_tart() -> Message {
  format!("tart v{}", VERSION)
}

fn init_dot_dir() -> Message {
  match fs::create_dir(DOT_DIR) {
    Ok(()) => format!("Initialized tart in {}/.tart", get_current_dir()),
    Err(_) => "Failed to create directory".to_string()
  }
}

fn get_current_dir() -> String {
  env::current_dir().unwrap().into_os_string().into_string().unwrap()
}

fn unknown() -> Message {
  "unknown command or option".to_string()
}

// separates args into `-` | `--` and the following non-opt args on one Vec each
fn group_args(args: &[String]) -> Vec<Vec<&str>> {
  let mut new_args: Vec<Vec<&str>> = Vec::new();
  let mut subvec_idx: usize = 0;

  for arg in args {
    if arg.starts_with("--") {
      subvec_idx += 1;
      new_args.push(vec![arg]);
    } else if arg.starts_with("-") {
      subvec_idx += 1;
      if arg.len() == 2 { // normal opt
        new_args.push(vec![&arg[0..2]]);
      } else { // for one-letter opt with the body tacked onto it
        new_args.push(vec![&arg[0..2], &arg[2..]]);
      }
    } else {
      // for when there are no current Vec in new_args
      if new_args.len() == subvec_idx {
        new_args.push(Vec::new());
      }

      // all non-opt get added to the newest Vec
      new_args[subvec_idx].push(arg);
    }
  }

  new_args
}