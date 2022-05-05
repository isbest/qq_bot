qq_sign_assistant
==
+ 本项目基于[rust_proc_qq](https://github.com/niuhuan/rust_proc_qq)
+ 封装了讯飞OCR，主要用于QQ群的截图统计工作
### 如何使用
```rust
# 设置rust默认环境为 nightly
rustup default nightly
# 设置当前项目rust环境设置为 nightly
rustup override set nightly
# 设置xunfei_ocr
替换 qq_bot/sign_in.rs:63 中的参数为你自己的
# 启动qq_bot
cargo run --bin=qq_bot
```

