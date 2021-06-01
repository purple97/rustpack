mod build;
mod utils;
/*  */
use build::build::{
  create_module, generate_file, matching_code_by_modules, modules_to_contents, query_file_template,
  IModuleInfo,
};
use std::collections::HashMap;
use std::path::PathBuf;
use utils::cli::get_args;
use utils::utils::path_join;

fn main() {
  let app_version = "v1.0.0"; //
  let mut config: HashMap<&str, String> = get_args();
  let modules: Vec<IModuleInfo>;
  let cwd_dir: String = config.get("root").unwrap().to_string(); //当前目录
  let entry_file = config.get("entry").unwrap().to_string();

  println!("{}, \nconfig: {:#?}", app_version, config);
  println!("当前目录：{:?}", cwd_dir);
  //
  let entry_dir = path_join(PathBuf::from(&cwd_dir), config.get("entryPath").unwrap());
  println!("entry_dir:{:?} \nentry_file:{}", entry_dir, entry_file);
  // 导出所有模块
  modules = create_module(entry_dir, PathBuf::from(entry_file));
  // 包裹所有代码，引入(module,exports,require)三个对象;
  let modules = matching_code_by_modules(modules);
  // 通过模板组装文件内容;
  let file_contents = query_file_template(
    modules_to_contents(modules),
    config.get("entry").unwrap().to_string(),
  );
  // 文件输出路径
  let output_file_path = path_join(
    PathBuf::from(cwd_dir),
    config.get_mut("outputPath").unwrap(),
  );
  // 生成文件
  generate_file(file_contents, output_file_path.to_str().unwrap());
}
