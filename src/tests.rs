#[cfg(test)]
mod tests {
  use std::path;
  use std::fs;
  
  #[test]
  fn filesystem() {
    println!("{}", fs::canonicalize(&path::PathBuf::from("./").join(".tart")).unwrap().as_os_str().to_str().unwrap());
  }
}
