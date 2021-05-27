mod build;
mod utils;
/*  */
use build::build::{
  create_module, generate_file, macthing_code_by_modules, modules_to_contents, query_file_template,
  IModuleInfo,
};
use std::collections::HashMap;
use std::env::current_dir;
use std::path::PathBuf;
use utils::utils::path_join;

fn main() {
  let mut config: HashMap<&str, &str> = HashMap::new();
  let modules: Vec<IModuleInfo>;
  let cwd_dir: PathBuf = current_dir().unwrap();
  //
  config.insert("root", "./");
  config.insert("entry", "./index.js");
  config.insert("entryPath", "./src/");
  config.insert("outputPath", "./example/dist/main.js");
  println!("config.entry:{}", config.get_mut("entry").unwrap());
  //
  println!("当前目录：{:?}", cwd_dir);
  //
  let new_entry_path = path_join(cwd_dir, "./example/src/");
  // 导出所有模块
  modules = create_module(new_entry_path, PathBuf::from("./index.js"));
  // 包裹所有代码，引入(module,exports,require)三个对象;
  let modules = macthing_code_by_modules(modules);
  //
  let mut entry = PathBuf::new();
  entry.push(config.get_mut("entry").unwrap());
  // 通过模板组装文件内容;
  let file_contents = query_file_template(
    modules_to_contents(modules),
    entry.to_str().unwrap().to_string(),
  );
  // 生成文件
  let cwd_dir: PathBuf = current_dir().unwrap();
  let output_file_path = path_join(cwd_dir, config.get_mut("outputPath").unwrap());
  generate_file(file_contents, output_file_path.to_str().unwrap());
}
