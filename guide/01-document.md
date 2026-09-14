# 01：定义 Markdown 文档

现在开始设计第一个公开概念：`Document`，并把类型定义和解析入口拆到不同模块。

这一阶段要建立最小但真实的 Markdown AST：普通文本解析为段落，段落中包含文本节点。暂时不要解析标题、链接、行内代码或 HTML。

## 小提示

### 推荐的模块职责

```text
src/
├── lib.rs       # 对外公开 API 和模块组织
├── types.rs     # Document 等公开数据类型
└── parser.rs    # parse_markdown 的实现
```

模块拆分是代码组织方式。现在只实现 `Document`、`Paragraph` 和 `Text` 三种概念，后续再随着 Markdown 支持范围增长扩展 AST。

### 从 `lib.rs` 公开 API

最小的 `lib.rs` 可以这样写：

```rust
mod parser;
mod types;

pub use parser::parse_markdown;
pub use types::{BlockNode, Document, InlineNode};
```

这里有两个不同层次：

- `mod parser;` 和 `mod types;` 告诉 Rust 编译 `parser.rs` 和 `types.rs`。
- `pub use ...;` 把模块中的项目重新导出到 crate 根部，让使用者可以直接写 `aa_markdown::parse_markdown` 和 `aa_markdown::Document`。

如果只写：

```rust
mod parser;
mod types;
```

模块会参与编译，但它们仍然是 crate 内部实现，外部使用者不能访问其中的公开项目。也可以写成 `pub mod parser;`，这样使用者通过 `aa_markdown::parser::parse_markdown` 访问；当前教程选择 `pub use`，是为了让模块目录结构保持内部实现细节，同时提供简洁稳定的顶层 API。

注意：`pub use` 只能重新导出已经在对应模块中声明为 `pub` 的项目。例如 `types.rs` 中的 `Document` 必须是 `pub struct Document`。

### 为什么要写 `#[derive(...)]`

AST 类型通常需要被测试比较、被调试输出，也经常需要复制一份来构造测试数据。Rust 不会自动为自定义类型提供这些能力，需要通过 `derive` 请求编译器生成常用实现：

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
  pub children: Vec<BlockNode>,
}
```

这三个 trait 的作用可以先这样理解：

| 写法 | 用途 | 类似的 TypeScript/JavaScript 场景 |
| --- | --- | --- |
| `Debug` | 允许使用 `{:?}` 打印值，便于查看测试失败时的实际结构 | `console.log(document)` |
| `Clone` | 允许显式复制值：`document.clone()` | 复制一个对象，概念上接近展开对象或深拷贝 |
| `PartialEq` | 允许使用 `==` 和 `assert_eq!` 比较值 | `expect(actual).toEqual(expected)` |

`derive` 不是把字段变成公开，也不是运行时反射。它只是让编译器根据字段类型自动生成 trait 实现。例如 `Document` 的字段是 `Vec<BlockNode>`，因此 `BlockNode` 和它包含的 `InlineNode` 也需要派生相应的 trait：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum BlockNode {
  Paragraph { children: Vec<InlineNode> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineNode {
  Text { value: String },
}
```

如果忘记 `PartialEq`，下面的测试会无法编译：

```rust
assert_eq!(actual, expected);
```

编译器会告诉你类型没有实现 `PartialEq`。能力不是默认附着在类型上的，而是通过 trait 明确声明。

### 绑定、借用表达式与引用类型

```rust
pub fn parse_markdown(markdown: &str) -> Document
```

先区分三个概念：

- **绑定**：用 `let` 把一个名字绑定到一个值，例如 `let owned = String::from("hello");`。
- **借用表达式**：在值前面写 `&`，创建对这个值的借用，并得到一个引用值。例如，`&owned` 这个表达式的类型是 `&String`：

  ```rust
  let owned = String::from("hello");
  let borrowed = &owned; // borrowed 的类型是 &String
  ```

  `&String::from("hello")` 的类型也确实是 `&String`，但它借用的是临时创建的 `String`，通常只适合立即传给函数，不适合保存为长期使用的引用。
- **引用类型**：在类型前面写 `&`，表示这个类型是指向 `T` 的引用类型，例如 `&String` 或 `&str`。

### 测试应该放在哪里

这个阶段有三种合理位置，它们用途不同：

```text
src/lib.rs                 # crate 级别或很小的公开 API 测试
src/parser.rs              # parser 模块的单元测试，当前阶段推荐
tests/parse_markdown.rs    # 集成测试，从 crate 外部验证公开 API
```

在 `parser.rs` 末尾使用 `#[cfg(test)] mod tests`，可以直接访问当前模块的私有辅助函数，例如 `flush_paragraph`。这非常适合测试解析器内部步骤，也是你当前实现所采用的方式。

`crates/markdown/tests/` 下的测试属于集成测试。它们只能像普通使用者一样访问公开 API，适合验证 `aa_markdown::parse_markdown` 和 `aa_markdown::Document` 是否真的被 `lib.rs` 导出。`src/lib.rs` 中也可以放测试，但随着模块变多，通常不如把测试放在被测模块旁边清晰。

这一步不要求把所有测试都迁移到 `tests/`。建议先在 `parser.rs` 写解析行为测试，再额外添加一个很小的集成测试验证顶层导出。

### 不要丢掉 Cargo 自动生成的示例

`cargo new --lib` 会生成一个最小的库和测试。它不是无用代码，而是一个可以回看的 Rust 模板。开始改造前，建议把它记录在本教程中：

```rust
pub fn add(left: u64, right: u64) -> u64 {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
```

阅读它可以看到 Rust 测试的基本形状：`#[test]` 标记测试函数，`assert_eq!` 比较期望值和实际值，`use super::*` 把父模块中的项目带入测试模块。接下来不要假设学习者熟悉 Git 回退；可以保留这段示例作为文档参考，然后把测试改造成 Markdown 行为测试。删除的是当前文件中的样例代码，不是删除学习材料。

## 行为要求

库需要提供：

```rust
pub struct Document {
  pub children: Vec<BlockNode>
}

pub fn parse_markdown(markdown: &str) -> Document;
```

`parse_markdown` 的行为要求：

- 输入空字符串时，返回一个有效的空文档。
- 输入普通文本时，返回一个包含段落和文本节点的文档。
- 两个由空行分隔的文本块，应解析为两个段落。
- 文本内容应保留，不应被静默修改。
- 当前阶段不需要返回 `Result`，因为还没有定义解析失败的情况。

你需要自己决定 AST 的字段是否公开，以及如何让测试观察解析结果。为了让测试能够直接比较结果，可以为 AST 类型派生 `Debug`、`Clone` 和 `PartialEq`；这不是唯一方案，但很适合当前学习阶段。

## 实现任务

1. 先阅读并保留 Cargo 自动生成的 `add` 函数和 `it_works` 测试示例，然后把源码中的占位实现改造成 Markdown 实现。
2. 在 `types.rs` 中定义最小 AST：`Document`、段落节点和文本节点，并为嵌套类型派生 `Debug`、`Clone` 和 `PartialEq`。
3. 在 `parser.rs` 中实现 `parse_markdown`，不要只写带分号的函数声明。
4. 在 `parser.rs` 中按空行切分段落，并把每个文本块包装成段落节点。
5. 在 `lib.rs` 中声明模块，并将需要的类型和函数重新导出。
6. 在 `parser.rs` 中添加空文档、单个段落和两个段落的单元测试。
7. 从 crate 外部调用 `aa_markdown::parse_markdown`，确认顶层 API 可用；可以使用 `crates/markdown/tests/` 中的集成测试。
8. 为公开 API 添加简短的 Rust 文档注释，使 `cargo doc` 能说明它们的用途。

建议优先使用标准库，不要添加依赖。

## TS/JS 对照

一种可能的 TypeScript 设计是：

```ts
type Document = {
  children: BlockNode[];
};

type BlockNode = {
  kind: "paragraph";
  children: InlineNode[];
};

type InlineNode = {
  kind: "text";
  value: string;
};

function parseMarkdown(source: string): Document;
```

Rust 不需要完全照搬这个表示。我们现在关注的是公开行为，而不是让 Rust 代码看起来像 TypeScript。

## 验收

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps
```

完成后把以下内容发给我：

- `src/lib.rs` 的实现。
- 验收命令的结果。

下一阶段会在这个最小 AST 上增加标题。当前阶段明确不做：换行符规范化、BOM 处理、行内代码、链接、标题、HTML renderer 和复杂错误处理。先把一个小而完整的 AST 流程做通，比同时准备未来所有节点更重要。

## 可选复盘

下面的 Markdown 字符串包含两个段落：

```markdown
hello

world
```

请画出或写出它解析后的 AST 结构。预期结构是一个 `Document`，其中包含两个按顺序排列的 `Paragraph`，每个段落分别包含一个 `Text` 节点：

```text
Document
├── Paragraph
│   └── Text("hello")
└── Paragraph
    └── Text("world")
```

这个复盘只检查你是否理解输入文本、空行和 AST 节点之间的对应关系，不是额外的实现要求。
