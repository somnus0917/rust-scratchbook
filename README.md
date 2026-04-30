# Rust Scratchbook

这个目录是 Rust 学习草稿本。日常写小实验时，不需要反复 `cargo new`。

## 用法

在 `src/bin/` 下面新建一个 `.rs` 文件：

```rust
fn main() {
    println!("hello rust");
}
```

假设文件名是 `hello.rs`，运行：

```sh
cargo rb hello
```

检查但不运行：

```sh
cargo cb hello
```

## 加依赖

把依赖统一加到 `Cargo.toml` 的 `[dependencies]` 下面，然后所有 `src/bin/*.rs` 都能使用。

例如：

```toml
[dependencies]
rand = "0.8"
```

## 建议

- 一个知识点一个文件，比如 `ownership.rs`、`match_enum.rs`、`iterator.rs`。
- 如果某个实验开始变大，再单独 `cargo new` 成正式项目。
- 没有外部依赖、只是测试语法时，也可以直接用 `rustc some_file.rs`，但长期学习更推荐这个 Cargo 草稿本。

