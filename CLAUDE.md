# Flowdraft — Claude Code 项目指南

## 项目概述

Rust CLI 工具，将 Lisp 风格 DSL (`.fd` 文件) 渲染为 SVG 图表。

## 构建与测试

```bash
cargo build          # 编译
cargo test           # 运行全部测试
cargo run -- FILE.fd -o OUT.svg  # 运行
```

## 代码结构

- `src/parse/` — DSL 解析。`ast.rs` 定义 AST 类型，`transform.rs` 用 lexpr 解析 s-expression 并转换为 AST
- `src/ir/` — 中间表示。`types.rs` 定义 Node/Edge/TreeInfo，`build.rs` 从 AST 构建 IR
- `src/layout/` — 布局算法。`tree.rs` 使用 reingold-tilford crate 做树布局，`freeform.rs` 为无操作布局
- `src/render/svg.rs` — 用 svg crate 将 IR 渲染为 SVG
- `src/style/defaults.rs` — 默认颜色、字体、间距等主题常量
- `src/lib.rs` — 公共入口 `process(input) -> Result<String>`
- `src/main.rs` — CLI (clap)，支持文件/stdin 输入、文件/stdout 输出、watch 模式

## 关键依赖

- `lexpr` — S-expression 解析，使用 `ColonPrefix` keyword 语法
- `reingold-tilford` — 树布局算法，通过 `NodeInfo` trait 适配
- `svg` — SVG 生成，`element::Text::new(content)` 需要传入文本内容
- `clap` — CLI 参数解析
- `notify` / `notify-debouncer-mini` — 文件监听

## DSL 形式

三种顶层 form：`tree`、`line`、`style`。关键字用冒号前缀 (`:down`, `:label`, `:fill`)。

## 注意事项

- `HashMap` 迭代顺序不确定，子节点顺序通过 `TreeInfo.children_order` 保持 DSL 中的声明顺序
- 节点坐标 (x, y) 是中心点，宽高用于矩形绘制
- CJK 字符宽度通过 `unicode-width` 计算，每个 CJK 字符算 2 列宽
- 测试中含嵌套引号的字符串用 `r##"..."##` 原始字符串
