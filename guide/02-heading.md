# 02：解析 ATX 标题

这一阶段在 `01` 的最小 AST 上增加 Markdown 的 ATX 标题，也就是以 `#` 开头的标题：

```markdown
# 一级标题
## 二级标题
### 三级标题
```

目标不是一次实现所有 Markdown 标题规则，而是让解析器增加一种清晰、可测试的块级节点。

## 本阶段范围

支持 1 到 6 级标题。连续的 `#` 后面必须是行尾、空格或 Tab；多个空格和 Tab 可以连续出现：

```markdown
# Hello       <!-- 标题，level = H1 -->
## Hello      <!-- 标题，level = H2 -->
###### Hello  <!-- 标题，level = H6 -->
##    Hello   <!-- 多个空格，标题，level = H2 -->
##	Hello     <!-- Tab，标题，level = H2 -->
##            <!-- 行尾，空标题，level = H2 -->
```

暂时不支持：

- 七个或更多 `#`，例如 `####### Hello`
- 没有空格的写法，例如 `#Hello`
- Setext 标题，例如 `Title` 下一行的 `=====`
- 标题内的强调、链接或行内代码
- 自动生成 `id` 或 slug
- HTML 渲染

为了保持本阶段简单，标题内容可以直接作为一个 `InlineNode::Text`。后续实现行内语法时，再改变标题 children 的生成方式。

## 目标 AST

在 [types.rs](../crates/markdown/src/types.rs) 的 `BlockNode` 中增加一个变体：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum BlockNode {
	Heading {
		level: HeadingLevel,
		children: Vec<InlineNode>,
	},
	Paragraph {
		children: Vec<InlineNode>,
	},
}
```

例如：

```markdown
# Hello
```

应得到：

```text
Document
└── Heading { level: H1 }
	└── Text("Hello")
```

这里的 `level` 使用 `HeadingLevel`，因为标题级别只能是 `H1` 到 `H6`。解析时需要把连续的 `#` 数量转换成对应的枚举变体，不要让调用者传入任意数字。

## 解析顺序

当前 `parser.rs` 已经按行遍历输入。对每一行，推荐按这个顺序思考：

1. 先判断这一行是否是标题。
2. 如果是标题，先结束之前积累的段落，再添加 `Heading`。
3. 如果不是标题，再按原来的逻辑积累到段落缓冲区。
4. 输入结束时，别忘记刷新最后一个段落。

标题是独立的块级节点，所以：

```markdown
before
# Title
after
```

应当得到三个节点：

```text
Paragraph("before")
Heading(1, "Title")
Paragraph("after")
```

## 建议的实现思路

可以先为解析一行标题写一个小函数。Rust 的 `strip_prefix` 很适合判断开头是否有 `#`：

```rust
let without_hash = line.strip_prefix('#');
```

然后逐个消费开头的 `#`，统计标题级别；统计结束后检查下一个字符是否是空格或 Tab，或者已经到达行尾。你也可以选择其他实现方式，但需要让“最多六个 `#`”和“分隔符必须是空格、Tab 或行尾”这两个规则清楚可见。

不要通过 `line.trim()` 判断标题，因为它会同时删除内容两侧的空白，使解析规则变得不明确。当前阶段只需要决定标题内容是否去除分隔用的空格，并用测试固定这个行为。

## 测试任务

在 `parser.rs` 的 `#[cfg(test)] mod tests` 中添加测试。下面的表格同时给出测试输入、预期行为和建议的测试名称：

| 测试名称 | 输入 | 预期结果 |
| --- | --- | --- |
| `empty_input` | `""` | `Document { children: vec![] }` |
| `parses_h1` | `"# Hello"` | 一个 `Heading { level: HeadingLevel::H1, ... }` |
| `parses_h2` | `"## Hello"` | 一个 `Heading { level: HeadingLevel::H2, ... }` |
| `parses_h6` | `"###### Hello"` | 一个 `Heading { level: HeadingLevel::H6, ... }` |
| `parses_empty_h2` | `"##"` | 一个内容为空的 `Heading { level: HeadingLevel::H2, ... }` |
| `accepts_multiple_spaces` | `"##    Hello"` | 一个 `Heading { level: HeadingLevel::H2, ... }` |
| `accepts_tab_separator` | `"##\tHello"` | 一个 `Heading { level: HeadingLevel::H2, ... }` |
| `seven_hashes_are_paragraph` | `"####### Hello"` | 一个 `Paragraph`，不是 `Heading` |
| `missing_space_is_paragraph` | `"#Hello"` | 一个 `Paragraph`，不是 `Heading` |
| `keeps_paragraph_behavior` | `"普通文本"` | 一个 `Paragraph`，且 `01` 的测试继续通过 |
| `keeps_block_order` | `"before\n# Title\nafter"` | `Paragraph`、`Heading(H1)`、`Paragraph`，顺序不变 |

例如，单个标题测试可以写成：

```rust
use crate::types::{HeadingLevel, InlineNode};

#[test]
fn parses_h1() {
	assert_eq!(
		parse_markdown("# Hello"),
		Document {
			children: vec![BlockNode::Heading {
				level: HeadingLevel::H1,
				children: vec![InlineNode::Text {
					value: "Hello".into(),
				}],
			}],
		},
	);
}
```

这个示例不是要求你逐字复制，而是展示“输入”和“完整 AST 期望值”之间的对应关系。其他测试可以先按照表格写成失败测试，再逐个实现解析逻辑。

建议继续使用 `assert_eq!` 比较完整 AST。测试中的辅助函数可以帮助减少重复，但名称要能表达它构造的是标题还是段落。

还要保留 [public_api.rs](../crates/markdown/tests/public_api.rs) 的集成测试，确认新增 `Heading` 没有破坏 crate 根部的 `parse_markdown` 导出。

## 验收

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps
```

完成后，把以下内容发给我进行代码审查：

- `types.rs` 和 `parser.rs` 的实现。
- 新增测试及其运行结果。
- 验收命令的结果。

完成实现并提交到 `main` 后，再创建 `tutorial/02-heading` 作为本阶段参考分支。不要在教程尚未验证前创建阶段快照。
