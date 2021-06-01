use std::collections::HashMap;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
  /// verbosity level
  #[structopt(short = "v", parse(from_occurrences))]
  verbose: u32,
  /// Set speed
  #[structopt(short = "s", long = "speed", default_value = "42")]
  speed: f64,
  /// Input file
  #[structopt(parse(from_str))]
  input: String,
}
pub fn get_args<'a>() -> HashMap<&'a str, String> {
  let opt = Opt::from_args();
  let Opt { input, .. } = opt;
  let cwd_dir: PathBuf = current_dir().unwrap(); //当前目录
  let mut config: HashMap<&str, String> = HashMap::new();
  let entry_path: String = PathBuf::from(input.as_str())
    .parent()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();
  let entry_file = Path::new(input.as_str())
    .file_name()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();
  config.insert("entry", entry_file);
  config.insert("root", cwd_dir.to_str().unwrap().to_string());
  config.entry("entry").or_insert("./index.js".to_string());
  config.insert("entryPath", entry_path);
  config.insert("outputPath", "./dist/main.js".to_string());
  config
}
