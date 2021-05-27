# rustpack

- Rust 语言练习

## 功能描述

- 使用 Rust 语音编写一个类似 webpack 的 js 文件打包工具；
  - 只实现打包功能，无 loader、plugin；

## 功能

- 获取命令行和参数
- 命令行人机交互
- 读取 js 文件内容
- 修改和替换 js 文件内容
- 生成输出目录
- 生成打包后的文件 main.js
- 全局安装 rustpack，可以实现`rustpack build`
- 各平台安装

---

## 一些感受和遇到的问题

### 文档

- 相关的文章很少，而且不少人自己还没写明白，所以讲的也不太明白；
- 英文文档也只是个工具书，不能指望深入讲解；

### rust 语音的各种类型，以及类型转换;

- 绝对可以劝退新手，基础类型就有 N 种（n>10），u8、u32、i32 这类都很好理解，usize、String、&str、就开始懵； 然后枚举类型的 Cow、Vec、直接疯掉，再来一壶 result::Option、HashMap、Path、PathBuf，还有他们之间的转换就更加懵逼

### 错误处理

- 也是完全不一样的思路，Result 类型是什么？还能这样写？每次后面跟着.unwarp()好奇怪;
  - 读取 hashBuf、路径、操作文件都会要求处理失败情况，其实理解后发现还行，而且还可以让错误冒泡。
- 搞懂了还是不错的，例子：

```rust
let f = File::open("./updated_time.txt");
     let f = match f {
         Ok(file) => file,
         Err(error) => {
             panic!("Problem in opening file: {:?}", error);
         }
     };
```

### 变量传递

- 第二劝退的地方
- 因为 Rust 对内存管理的安全，设计出来的一套使用规则；借用、引用的概念要该清楚， 可变借用和不可变借用也要缕清楚；
- 虽然写完了这个项目， 但是我还是搞不明白这个东西；
- 每次遇到“error: use of moved value”就懵逼，“移动了的值”啥意思？
- 例子：

```rs
struct Info {
    s: String,
}

fn fn_a(info: Info) {
    println!("in fn_a");
}

fn main() {
    let foo = Info {s : "abc".to_string() };
    fn_a(foo);
    fn_a(foo); // 这里多使用了一次foo
}
```

- foo 已经给第一个 fn_a()用过了，到了第二个 fn_a()的时候就是 moved value 了。然后就不让用了。
- 遇到这种情况，气不打一处来， 没地方讲理去。
- 解决办法：https://www.cnblogs.com/dhcn/p/12152116.html

### 模块引入

- 用过 node.js、java 过来的人，觉得这个模块移入和方法暴露也是有些奇怪的，但是还是能理解和处理它，不算太劝退吧；
