use std::{fs, env, path::PathBuf};

const VERSION: &str = "0.0.1";
const DOT_DIR: &str = "./.tart";

pub type Message = String;


// HELP IS NOT WRITTEN!!!

// tart [-v | --version] [-h | --help]
pub fn tart(args: &[String]) -> Message {
  let grouped_args = group_args(args);

  for group in grouped_args {
     // should match group[0] with options or main commands
     // then loop through remaining values in group for related commands|opts
    match group[0] {
      "-v" | "--version" => {
         return about_tart();
      },
      "-h" | "--help" => {
        return help(args);
      }
      _ => {
        return unknown();
      }
    }
  }
  
  help(args) // base function of command without args/opts
}

// tart init [directory]  [-h | --help]
pub fn init(args: &[String]) -> Message {
  let grouped_args = group_args(&args[1..]);

  if grouped_args.len() == 0 {
    return init_dot_dir(None);
  }
  
  for group in grouped_args {
     // should match group[0] with options or main commands
     // then loop through remaining values in group for related commands|opts
    match group[0] {
      "-h" | "--help" => {
        return help(args);
      },
      _ => {
        return init_dot_dir(Some(group[0]));
      }
    }
  }
  
  help(args) // base function of command without args/opts
}

pub fn destroy(args: &[String]) -> Message {
  let grouped_args = group_args(&args[1..]);

  if grouped_args.len() == 0 {
    return destroy_dot_dir(None);
  }
  
  for group in grouped_args {
     // should match group[0] with options or main commands
     // then loop through remaining values in group for related commands|opts
    match group[0] {
      "-h" | "--help" => {
        return help(args);
      },
      _ => {
        return destroy_dot_dir(Some(group[0]));
      }
    }
  }
  
  help(args) // base function of command without args/opts
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
  
You called for help... but help is a work in progress.".to_string()
}

fn about_tart() -> Message {
  format!("tart v{}", VERSION)
}

// TODO: check if dir already exists
fn init_dot_dir(dir: Option<&str>) -> Message {
  let mut actual_path = PathBuf::from(DOT_DIR);
  if let Some(given_dir) = dir {
    actual_path = PathBuf::from(given_dir).join(".tart");
  }

  match fs::create_dir(&actual_path) {
    Ok(()) => format!("Initialized tart in {}", fs::canonicalize(&actual_path).unwrap().to_str().unwrap()),
    Err(_) => "Failed to create directory".to_string()
  }
}


fn destroy_dot_dir(dir: Option<&str>) -> Message {
  let mut actual_path = PathBuf::from(DOT_DIR);
  if let Some(given_dir) = dir {
    actual_path = PathBuf::from(given_dir).join(".tart");
  }

  match fs::remove_dir_all(&actual_path) {
    Ok(()) => format!("Destroyed .tart in {}", fs::canonicalize(&actual_path).unwrap().to_str().unwrap()),
    Err(_) => format!("Failed to destroy .tart in {}", &actual_path.as_os_str().to_str().unwrap())
  }
}

fn _get_current_dir() -> String {
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