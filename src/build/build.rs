// mod utils;
use crate::utils::utils::{format_path, path_join};
use regex::Regex;
use std::borrow::Cow;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/* 模块信息的数据结构 */
#[derive(Debug)]
pub struct IModuleInfo {
  name: String,
  code: String,
  deps: Vec<String>,
}

impl IModuleInfo {
  fn set_name(&mut self, name: String) {
    self.name = name;
  }
  fn set_code(&mut self, code: String) {
    self.code = code;
  }
  fn set_deps(&mut self, deps: Vec<String>) {
    self.deps = deps;
  }
}
// 对code加工
fn parse_to_code(resource: &str) -> Cow<'_, str> {
  let rx = Regex::new(r"require\((.+)\)").unwrap();
  let code = rx.replace_all(&resource, "__rustpack_require__($1)");
  code
}

// 提取依赖文件
fn parse_to_deps(resource: &str) -> Vec<String> {
  let mut deps = Vec::new();
  let rx = Regex::new(r"require\((.+)\)").unwrap();
  for cap in rx.captures_iter(&resource) {
    deps.push(cap[1].to_string())
  }
  return deps;
}

/*
* 创建打包文件对应的信息
* @return <ModuleInfo>{name, code, deps}
*/
fn create_module_info(entry_path: &PathBuf, name: String) -> IModuleInfo {
  let mut file: File;
  let file_path = Path::new(&entry_path);
  let mut module_info = IModuleInfo {
    name,
    code: "".to_string(),
    deps: vec!["".to_string()],
  };
  let mut contents = String::new();
  file = File::open(file_path).unwrap();
  file.read_to_string(&mut contents).unwrap();
  let mut contents = contents.as_str();
  let mut code: Cow<'_, str> = parse_to_code(&mut contents);
  let deps: Vec<String> = parse_to_deps(&mut contents);
  // module_info.set_name(entry_path.to_str().unwrap().to_string());
  module_info.set_deps(deps);
  module_info.set_code(code.to_mut().to_string());
  return module_info;
}

/*
* 给代码添加(module, exports, require)三个对象；
*/
fn wrap_code(code: &str) -> String {
  let mut new_code = "function(module, exports, __rustpack_require__){\n".to_string();
  new_code += &code;
  new_code += &"}";
  return new_code.to_string();
}

/*
* 创建modules数组
* @return: Vec[{name,code,deps}]
*/
pub fn create_module(entry_path: PathBuf, entry: PathBuf) -> Vec<IModuleInfo> {
  let mut modules_array: Vec<IModuleInfo> = Vec::new();
  let entry = format_path(entry.to_str().unwrap());
  let new_entry_path = path_join(entry_path, &entry);
  //
  if Path::new(&new_entry_path).is_file() {
    println!("{} 文件已存在", new_entry_path.display());
    let module_info: IModuleInfo = create_module_info(&new_entry_path, entry);
    let IModuleInfo { deps, .. } = &module_info;
    // println!("文件{:?}\n module_info\n{:?}", entry, module_info);
    for dep in deps {
      // println!("子模块：{}", dep);
      let _dep = PathBuf::from(&dep);
      let mut entry_parent_path = PathBuf::new();
      entry_parent_path.push(new_entry_path.parent().unwrap());
      let new_modules_array = create_module(entry_parent_path, _dep);
      for item in new_modules_array {
        modules_array.push(item);
      }
    }
    modules_array.push(module_info);
  } else {
    print!("未找到文件:{:?}", new_entry_path);
    // file = File::create(entry_path).unwrap();
    std::process::exit(0x0100);
  }
  // println!("modules_array:\n{:?}", modules_array);
  return modules_array;
}

/*
* 对文件的上下文二次加工；
*/
pub fn macthing_code_by_modules(modules: Vec<IModuleInfo>) -> Vec<IModuleInfo> {
  let mut new_modules: Vec<IModuleInfo> = vec![];
  for (_, module) in modules.iter().enumerate() {
    // println!("===macthing_code_by_modules:\n{:?}", new_modules[index]);
    let IModuleInfo { code, name, deps } = module;
    let code = wrap_code(&code);
    let module = IModuleInfo {
      code,
      name: name.to_string(),
      deps: deps.to_vec(),
    };
    new_modules.push(module);
  }
  // println!("===macthing_code_by_modules:\n{:?}", new_modules);
  return new_modules;
}

/*
* 所有模块转成js的json对象
* json: {"fileName":"content text"}
*/
pub fn modules_to_contents(modules: Vec<IModuleInfo>) -> String {
  let mut contents = String::new();
  for (_, info) in modules.iter().enumerate() {
    let content = format!("{:?}:{},\n", &info.name, &info.code);
    contents += &content;
  }
  return contents;
}

/*
* 往输出的内容 添加__rustpack_require__ 和全局modules对象;
*/
pub fn query_file_template(modules_code: String, entry: String) -> String {
  let mut template = String::from(
    "!(function (modules) {
  var installedModules = {};
  function __rustpack_require__(moduleId) {
    if (installedModules[moduleId]) {
      return installedModules[moduleId].exports;
    }
    var module = (installedModules[moduleId] = {
      exports: {},
    });
    modules[moduleId].call(
      module.exports,
      module,
      module.exports,
      __denopack_require__
    );
    return module.exports;
  }
  // 入口
  return __denopack_require__(\"__rustpack_entry__\");
})({__modules__code__});",
  );
  template = template.replace("__rustpack_entry__", &entry);
  template = template.replace("__modules__code__", &modules_code);
  return template;
}

/*
* 生成文件
*/
pub fn generate_file(file_content: String, file_path: &str) {
  let output_file_path = Path::new(file_path).parent().unwrap();
  create_dir_all(output_file_path).unwrap();
  let mut file: File = File::create(file_path).expect("create failed");
  file.write_all(file_content.as_bytes()).unwrap();
}
