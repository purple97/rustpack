mod build;

/*  */
use std::env::current_dir;
// use std::fs::OpenOptions;
// use std::io::Read;
// use regex::Regex::replace_all;

use std::collections::HashMap;
// use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
  let mut config: HashMap<&str, String> = HashMap::new();
  let mut modules = HashMap::new();
  let cwd_dir: PathBuf = current_dir().unwrap();
  //
  config.insert("root", String::from("./"));
  config.insert("entry", String::from("./index.js"));
  config.insert("entryPath", String::from("./src/"));
  config.insert("outputPath", String::from("./dist/"));
  //
  modules.insert("name", "dezhao".to_string());
  //
  println!("当前目录：{:?}", cwd_dir);
  println!("config: {:?}", config);

  build::create_module(
    modules,
    cwd_dir.join("./example/src/"),
    // String::from("./example/src/"),
    String::from("index.js"),
  )
}
