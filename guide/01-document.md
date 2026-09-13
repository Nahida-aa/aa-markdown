# 01：定义 Markdown 文档

现在开始设计第一个公开概念：`Document`。

这一阶段只要求库能够接收 Markdown 源文本并保存它。暂时不要解析标题、段落、链接或 HTML。

## 行为要求

库需要提供：

```rust
pub struct Document {
    // 由你决定字段是否公开以及内部如何保存
}

pub fn parse(source: &str) -> Document;
```

`parse` 的行为要求：

- 输入空字符串时，返回一个有效的空文档。
- 输入普通文本时，返回一个有效文档。
- 输入内容不应被修改。
- 当前阶段不需要返回 `Result`，因为还没有定义解析失败的情况。

你需要自己决定如何让测试观察到“内容没有被修改”。可以设计一个合适的公开方法，也可以先设计一个合适的可比较表示。不要为了满足示例而暴露不必要的内部字段。

## 先做设计猜想

在写 Rust 代码前，先回答：

1. TypeScript 中的 `Document` 会是什么类型？
2. `source: &str` 和 `String` 分别适合什么场景？
3. `parse` 返回拥有数据的 `Document`，还是借用调用者的数据？为什么？
4. 测试应该放在 `src/lib.rs` 内，还是 `crates/markdown/tests/` 中？

## 实现任务

1. 删除 Cargo 自动生成的 `add` 函数和 `it_works` 测试。
2. 定义公开的 `Document` 类型。
3. 定义公开的 `parse` 函数。
4. 添加至少两个测试：空文档和普通文本。
5. 为公开 API 添加简短的 Rust 文档注释，使 `cargo doc` 能说明它们的用途。

建议优先使用标准库，不要添加依赖。

## TS/JS 对照

一种可能的 TypeScript 设计是：

```ts
type Document = {
  source: string;
};

function parse(source: string): Document;
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

- 你对上面四个设计问题的回答。
- `src/lib.rs` 的实现。
- 验收命令的结果。

下一阶段会基于你的 API 设计第一个 AST 节点，因此这一阶段不要提前实现解析逻辑。