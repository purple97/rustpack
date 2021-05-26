use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;

/*  */
fn path_end_slash(_path: PathBuf) -> PathBuf {
  if _path.ends_with("/") {
    println!("_path:{}", _path.display())
  }
  _path
}
/*  */
pub fn path_join(path_one: PathBuf, path_two: &str) -> PathBuf {
  let mut new_path_tow = path_two.to_string();
  if path_two.starts_with("./") {
    new_path_tow = path_two.to_string().replace("./", "");
  }
  let new_path: PathBuf = path_end_slash(path_one).join(&new_path_tow);

  new_path
}

pub fn format_path(path: &str) -> String {
  let path = path.replace("\"", "");
  return path;
}

/*
* 合并两个 HashMap
* 例子：
* let mut first_context = HashMap::new();
* first_context.insert("Hello", "World");
* let mut second_context = HashMap::new();
* second_context.insert("Hey", "There");
* println!("Generic:\t{}", merge(&first_context, &second_context));
*/
pub fn merge<K: Hash + Eq + Copy, V: Copy>(
  first_context: &HashMap<K, V>,
  second_context: &HashMap<K, V>,
) -> HashMap<K, V> {
  let mut new_context = HashMap::new();
  for (key, value) in first_context.iter() {
    new_context.insert(*key, *value);
  }
  for (key, value) in second_context.iter() {
    new_context.insert(*key, *value);
  }
  new_context
}
