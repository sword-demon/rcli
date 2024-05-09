# Rust 语言训练营

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

在我们的课程中，新的项目会使用 `tyr-rust-bootcamp/template` 模版生成基本的代码：

```bash
cargo generate tyr-rust-bootcamp/template
```

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install` 即可。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny
```

检测依赖

```bash
cargo deny check -d
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

生成`changelog.md`

```bash
git cliff -o CHANGELOG.md
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```

## 处理返回结果

```bash
cargo add anyhow
```

>经常有`anyhow:Ok;` 引用的问题

## 处理 csv

```bash
cargo add csv
```

## 使用 serde

```bash
cargo add serde --features derive
cargo add serde-json
cargo add serde-yaml
```

## 处理 toml

```bash
cargo add toml
```

## 处理 base64

```bash
cargo add base64
```

## base64 解密解密

```bash
cargo run -- base64 encode -i cargo.toml > fixtures/standard_b64.txt
cargo run -- base64 decode --format urlsafe -i fixtures/b64.txt
```

## 运行所有的测试代码

```bash
cargo nextest run --all-features
```

## Hash 库 blake3

现在认为`sha-1`和`md5`都是都要比`blake`的差，以及处于淘汰的`hash`算法，是不安全的。

[https://github.com/BLAKE3-team/BLAKE3](https://github.com/BLAKE3-team/BLAKE3)

```bash
cargo add blake3
```

## 非对称加密库`ed25519_dalek`
```bash
cargo add ed25519_dalek
```

### 验证

```bash
cargo run -- genpass -l 32 > fixtures/blake3.txt

cargo run -- text sign -k fixtures/blake3.txt
>输入: hello
LJeaB_4zDSzWw_weWkVCvg9OMyWVEOKBD-yz_gpguiQ

cargo run -- text verify -k fixtures/blake3.txt --sig LJeaB_4zDSzWw_weWkVCvg9OMyWVEOKBD-yz_gpguiQ
>输入: hello
true
```

## 文本前面

```bash
cargo run -- text generate -o fixtures
   Compiling rcli v0.1.0 (/Users/xxx/RustProjects/rcli)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `/Users/xxx/.target/debug/rcli text generate -o fixtures`
TextKeyGenerateOpts { format: Blake3, output: "fixtures" }
cargo run -- text generate -o fixtures --format ed25519
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `/Users/xxx/.target/debug/rcli text generate -o fixtures --format ed25519`
TextKeyGenerateOpts { format: Ed25519, output: "fixtures" }
cargo run -- text sign -k fixtures/edd25519.sk --format ed25519
   Compiling rcli v0.1.0 (/Users/xxx/RustProjects/rcli)
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
     Running `/Users/xxx/.target/debug/rcli text sign -k fixtures/edd25519.sk --format ed25519`
hello!
sign: VRT9gZx6jPIk14KtOBT9OwIrJvWS8ST__OgK6oPPH7nHzflH1SCHpSFPPjgEEhOXhPjPndK1k6ePIE4kDimGCA
cargo run -- text verify -k fixtures/edd25519.sk --format ed25519 --sig VRT9gZx6jPIk14KtOBT9OwIrJvWS8ST__OgK6oPPH7nHzflH1SCHpSFPPjgEEhOXhPjPndK1k6ePIE4kDimGCA
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `/Users/xxx/.target/debug/rcli text verify -k fixtures/edd25519.sk --format ed25519 --sig VRT9gZx6jPIk14KtOBT9OwIrJvWS8ST__OgK6oPPH7nHzflH1SCHpSFPPjgEEhOXhPjPndK1k6ePIE4kDimGCA`
hello!
false
cargo run -- text verify -k fixtures/edd25519.pk --format ed25519 --sig VRT9gZx6jPIk14KtOBT9OwIrJvWS8ST__OgK6oPPH7nHzflH1SCHpSFPPjgEEhOXhPjPndK1k6ePIE4kDimGCA
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `/Users/xxx/.target/debug/rcli text verify -k fixtures/edd25519.pk --format ed25519 --sig VRT9gZx6jPIk14KtOBT9OwIrJvWS8ST__OgK6oPPH7nHzflH1SCHpSFPPjgEEhOXhPjPndK1k6ePIE4kDimGCA`
hello!
true
```

## tokio

- rt
- rt-multi-thread
- marcos

这 3 个基本是必加的

```bash
cargo add tokio --features rt --features rt-multi-thread --features macros --features net --features fs
```

## tracing

```bash
cargo add tracing
cargo add tracing-subscriber --features env-filter
```

这样命令就可以使用`RUST_LOG=debug cargo run`的方式执行

```bash
RUST_LOG=info cargo run -- http serve
```

## axum

```bash
cargo add axum --features http2 --features query --features tracing
```

## test

```text
### test index page

GET http://127.0.0.1:8080
```

```text
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 11
date: Sun, 28 Apr 2024 15:26:49 GMT

hello world

```

## 本地安装使用本项目

```bash
cargo install --path .
```

```bash
rcli
Simple program to deal with csv

Usage: rcli <COMMAND>

Commands:
  csv      Show csv, or convert CSV to other formats
  genpass  generate a random password
  base64
  text
  http
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## enum dispatch

```bash
cargo add enum_dispatch
```

## 重构后测试历史命令

```bash
RUST_LOG=info rcli http serve
rcli text sign -k fixtures/ed25519.sk --format ed25519
rcli text generate -o fixtures --format ed25519
rcli text sign -k fixtures/ed25519.sk --format ed25519
rcli text verify -k fixtures/ed25519.sk --format ed25519 -sig fRrgj334WL-Z9M30ThWTRZldgUVH_uqjclGzSb3ReQAXvnzkmoXHoFyZ6zEY8bR0mT_mNPHmjVIprnC5-8W3CA
rcli text verify -k fixtures/ed25519.sk --format ed25519 --sig fRrgj334WL-Z9M30ThWTRZldgUVH_uqjclGzSb3ReQAXvnzkmoXHoFyZ6zEY8bR0mT_mNPHmjVIprnC5-8W3CA
rcli text verify -k fixtures/ed25519.pk --format ed25519 --sig fRrgj334WL-Z9M30ThWTRZldgUVH_uqjclGzSb3ReQAXvnzkmoXHoFyZ6zEY8bR0mT_mNPHmjVIprnC5-8W3C
```
