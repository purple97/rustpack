/*
 */
fn main() {
  let filename = "./test.txt";
  let mut file: File;

  let args: Vec<String> = args().collect();
  println!("运行参数：{:?}", args);

  //
  if Path::new(filename).is_file() {
    println!("{} 文件已存在", filename);
    file = OpenOptions::new().append(true).open(filename).unwrap();
  } else {
    println!("未找到文件{},创建新文件{}", filename, filename);
    file = File::create(filename).unwrap();
  }

  //
  let mut contents = String::new();
  contents.push_str("\n新来一段文字");

  //
  file.write_all("Hello world!".as_bytes()).unwrap();

  file.write_all(contents.as_bytes()).unwrap();

  let file_content = read_to_string(filename).unwrap();
  println!("文件内容：\n{}", file_content)
}
