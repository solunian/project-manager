use std::{fs, env};

const DOT_DIR: &str = "./.tart";

pub fn init(args: &[String]) -> String {
  match args.len() {
    1 => init_dot_dir(),
    _ => format!("unknown option")
  }
}

pub fn destroy(args: &[String]) -> String {
  match args.len() {
    1 => {
      match fs::remove_dir_all(DOT_DIR) {
        Ok(()) => format!("Destroyed .tart in {}", get_current_dir()),
        Err(_) => format!("Failed to destroy .tart in {}", get_current_dir())
      }
    },
    _ => format!("unknown option")
  }
  
}

fn init_dot_dir() -> String {
  match fs::create_dir(DOT_DIR) {
    Ok(()) => format!("Initialized tart in {}/.tart", get_current_dir()),
    Err(_) => "Failed to create directory".to_string()
  }
}

fn get_current_dir() -> String {
  env::current_dir().unwrap().into_os_string().into_string().unwrap()
}
