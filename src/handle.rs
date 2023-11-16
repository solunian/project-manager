use std::{fs, env, path::PathBuf};

const DOT_DIR: &str = ".tart";

pub type Message = String;


pub fn init(dir: Option<&PathBuf>) -> Message {
  init_dot_dir(dir)
}

pub fn destroy(dir: Option<&PathBuf>) -> Message {
  destroy_dot_dir(dir)
}

// TODO: check if dir already exists
fn init_dot_dir(dir: Option<&PathBuf>) -> Message {
  let mut actual_path = env::current_dir().unwrap().join(DOT_DIR);
  if let Some(given_dir) = dir {
    actual_path = given_dir.join(".tart");
  }

  // match fs::canonicalize(&actual_path) { // check if path exists
  //   Ok(path) => {
      
  //   },
  //   Err(_) => format!("Path `{}` not found", &actual_path.to_str().unwrap())
  // }

  match fs::create_dir(&actual_path) { // attempts to init dir
    Ok(()) => format!("Initialized tart in {}", fs::canonicalize(&actual_path).unwrap().to_str().unwrap()),
    Err(_) => format!("Failed to initialize tart in {}", &actual_path.to_str().unwrap())
  }
}


fn destroy_dot_dir(dir: Option<&PathBuf>) -> Message {
  let mut actual_path = env::current_dir().unwrap().join(DOT_DIR);
  if let Some(given_dir) = dir {
    actual_path = given_dir.join(".tart");
  }

  match fs::canonicalize(&actual_path) { // check if path exists
    Ok(path) => {
      match fs::remove_dir_all(&path) { // attempts to remove dir
        Ok(()) => format!("Destroyed tart in {}", &path.to_str().unwrap()),
        Err(_) => format!("Failed to destroy tart in {}", &path.to_str().unwrap())
      }
    },
    Err(_) => format!("Path `{}` not found!", &actual_path.to_str().unwrap())
  }
}

fn _get_current_dir() -> String {
  env::current_dir().unwrap().into_os_string().into_string().unwrap()
}
