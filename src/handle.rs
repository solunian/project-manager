use std::{fs, env, path::PathBuf};

const DOT_DIR: &str = ".tart";

pub type Message = String;


pub fn init(dir: Option<&PathBuf>) -> Message {
  init_dot_dir(dir)
}

pub fn destroy(dir: Option<&PathBuf>) -> Message {
  destroy_dot_dir(dir)
}

fn init_dot_dir(dir: Option<&PathBuf>) -> Message {
  let mut actual_path = env::current_dir().unwrap();
  if let Some(given_dir) = dir {
    actual_path = given_dir.to_path_buf();
  }
  actual_path.push(DOT_DIR);

  if actual_path.exists() {
    return format!("Path `{}` already exists", &actual_path.to_str().unwrap());
  }

  match fs::create_dir(&actual_path) { // attempts to init dir
    Ok(()) => format!("Initialized tart in {}", get_absolute_path(&actual_path).unwrap()), // path should exist after create_dir
    Err(_) => format!("Failed to initialize tart in {}", &actual_path.to_str().unwrap()) // path does not exist
  }
}

fn destroy_dot_dir(dir: Option<&PathBuf>) -> Message {
  let mut actual_path = env::current_dir().unwrap();
  if let Some(given_dir) = dir {
    actual_path = given_dir.to_path_buf();
  }
  actual_path.push(DOT_DIR);

  match get_absolute_path(&actual_path) {
    Ok(path) => { // path exists 
      match fs::remove_dir_all(&path) { // attempts to remove dir
        Ok(()) => format!("Destroyed tart in {}", &path),
        Err(_) => format!("Failed to destroy tart in {}", &path)
      }
    },
    Err(err_message) => err_message // path does not exist
  }
}

fn get_absolute_path(path: &PathBuf) -> Result<Message, Message> {  
  match fs::canonicalize(&path) {
    Ok(pathbuf) => {
      let mut path_str = pathbuf.to_str().unwrap();

      if cfg!(windows) {
        path_str = path_str.trim_start_matches(r"\\?\");
      }

      Ok(path_str.to_string())
    },
    Err(_) => Err(format!("Path `{}` not found", &path.to_str().unwrap()))
  }
}
