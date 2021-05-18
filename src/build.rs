// use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// struct IModules = HashMap<&str, String>

fn resource_parse(resource: String) -> (String, Vec<String>) {
  let mut deps: Vec<String> = Vec::new();
  let rx = Regex::new(r"require\((.+)\)").unwrap();
  for cap in rx.captures_iter(&resource) {
    deps.push(cap[1].to_string())
  }
  let mut _str = rx.replace_all(&resource, "__rustpack_require__($1)");

  println!("{}", _str);
  return (_str.to_string(), deps);
}

pub fn create_module(_modules: HashMap<&str, String>, entry_path: String, entry: String) {
  let mut file: File;
  let mut contents = String::new();
  let new_entry_path = entry_path + &entry;
  let file_path = Path::new(&new_entry_path);
  println!("file_path:{:?}", file_path.display());
  //
  if Path::new(&file_path).is_file() {
    // println!("{} 文件已存在", entry_path);
    file = File::open(file_path).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let (code, deps) = resource_parse(contents);
    println!("code:\n{}\ndesp:\n{:?}", code, deps);

    for dep in deps {
      println!("子模块：{}", dep);
      // create_module(_modules, entry_path, dep);
    }
  } else {
    println!(
      "未找到文件{},创建新文件{}",
      file_path.display(),
      file_path.display()
    );
    // file = File::create(entry_path).unwrap();
    std::process::exit(0x0100);
  }
}
