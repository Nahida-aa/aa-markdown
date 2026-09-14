# 00：建立学习契约

这一阶段不实现 Markdown 语法。先从一个空 Git 仓库建立 Cargo workspace，再创建可以持续演进的 Rust library crate，并确认学习流程。

## 你的角色

你是这个教程的实际使用者，也是库的实现者。每个阶段请按以下顺序进行：

1. 阅读本阶段的行为要求。
2. 亲自执行命令或实现代码。
3. 运行格式化、测试和 Clippy。
4. 遇到编译错误时，先阅读错误信息，再向教练提问。

我会提供任务拆解、概念解释、测试设计和代码审查，但默认不会直接替你完成核心实现。

## 本章路线

这一章的主线是：

```text
执行初始化命令 → 检查生成文件 → 创建 library crate → 验收 workspace
```

本章会遇到 workspace、package、crate、`.gitignore` 和 Git 状态等概念。它们是辅助理解，不需要在开始前全部掌握；先执行命令，再根据实际生成的文件理解它们。

## 第一个任务：初始化 workspace

如果你从空仓库开始，先在仓库根目录执行：

```bash
cargo workspaces init
```

这个命令只会创建基础的 workspace manifest，不会创建 `.gitignore`，也不会创建任何 package。检查根目录的 `Cargo.toml`，初始内容类似：

```toml
[workspace]
members = []
```

手动创建根目录的 `.gitignore`：

```gitignore
/target/
```

`target/` 是 Cargo 的构建输出目录，不应提交到 Git。这个项目保留 `Cargo.lock`，因为它是 workspace 的依赖锁定文件。

接着创建 library crate：

```bash
cargo new crates/markdown --lib --name aa-markdown
```

如果不写 `--name aa-markdown`，Cargo 会使用路径最后一段作为 package 名：

```bash
cargo new crates/markdown --lib
```

上面的命令会默认生成：

```toml
[package]
name = "markdown"
```

因此这里显式使用 `--name aa-markdown`，把仓库内部的目录名和 Cargo package 名区分开。

这里特意让文件夹名和 package 名不同：

```text
仓库目录：      aa-markdown/
crate 目录：    crates/markdown/
package 名：    aa-markdown
Rust 库名称：   aa_markdown
```

它们分别解决不同问题：

- `crates/markdown/` 是仓库内部的文件组织方式，便于按领域命名目录。
- `name = "aa-markdown"` 是 Cargo package 的公开身份，出现在依赖声明和发布信息中；workspace 的 `members` 则填写 crate 目录路径。
- Rust 代码中的库名称会把连字符转换成下划线，因此使用 `aa_markdown`，不能使用 `aa-markdown`。

创建后检查 `crates/markdown/Cargo.toml`，应看到：

```toml
[package]
name = "aa-markdown"
```

这和 TypeScript/JavaScript 中“源码目录名”和 `package.json` 的 `name` 字段可以不同是类似的。目录名是仓库内部约定，package 名是工具链和使用者看到的包身份。

然后把根目录 `Cargo.toml` 的 workspace 成员改为：

```toml
[workspace]
resolver = "3"
members = ["crates/markdown"]
```

不要先添加解析器、AST 或 HTML renderer。当前阶段只关心 workspace 是否能发现并构建 library crate，并确认构建产物不会出现在 Git 变更中。

## 验收标准

以下命令都应成功：

```bash
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

## TS/JS 对照

这个阶段的概念对应关系是：

| Rust | TypeScript/JavaScript |
| --- | --- |
| workspace | npm/pnpm workspace |
| package | package.json 对应的包 |
| library crate | 可被 import 的 npm package |
| `cargo check` | 类型检查和构建检查 |
| `cargo test` | Vitest/Jest 测试 |

Rust 的 workspace 不是运行时模块系统，而是 Cargo 管理多个 package 的工程边界。后面我们会在同一个 workspace 中逐步发展库、示例和测试。

## 完成后检查

完成后运行：

```bash
git status --short
git status --short --ignored
git check-ignore -v target/
```

你应该能看到：

- `Cargo.toml`、`.gitignore` 和 `crates/markdown/` 是待添加的项目文件。
- `target/` 被 `.gitignore` 忽略，不会作为待添加文件出现。
- `git check-ignore -v target/` 能显示具体命中的忽略规则。

注意：这些文件尚未被 Git 跟踪时，`git diff -- Cargo.toml` 不会显示它们的内容；这是 Git 对未跟踪文件的正常行为。当前阶段不要求提交 Git commit。把 `git status`、忽略检查和所有验证命令的结果发给我，下一阶段再一起定义第一个公开 API。