# Flowdraft — Claude Code 项目指南

## 项目概述

Rust CLI 工具 + Web Playground，将 Lisp 风格 DSL (`.fd` 文件) 渲染为 SVG 图表。

## 构建与测试

```bash
cargo build          # 编译 CLI
cargo test           # 运行全部测试
cargo run -- FILE.fd -o OUT.svg  # 运行 CLI

# WASM 构建
wasm-pack build --target web --features wasm --no-default-features

# Web Playground
cd web && pnpm install && pnpm run build   # 静态站点输出到 web/build/
cd web && pnpm run dev                      # 本地开发服务器

# 一键构建全部
./build.sh         # test + CLI + WASM + Web
./build.sh cli     # 仅 CLI
./build.sh wasm    # 仅 WASM
./build.sh web     # WASM + Web 静态站点
./build.sh dev     # WASM + 启动 vite dev server
./build.sh test    # 仅测试
./build.sh package # 构建 Web + serve 二进制，打包到 dist/
```

## 代码结构

### Rust (src/)

- `src/parse/` — DSL 解析。`ast.rs` 定义 AST 类型（含 `DefineTemplate`），`transform.rs` 用 lexpr 解析 s-expression 并转换为 AST，`expand.rs` 展开 define/instantiate 模板
- `src/ir/` — 中间表示。`types.rs` 定义 Node/Edge/TreeInfo/Group，`build.rs` 从 AST 构建 IR
- `src/layout/` — 布局算法。`tree.rs` 使用 reingold-tilford crate 做树布局，`freeform.rs` 为无操作布局
- `src/render/svg.rs` — 用 svg crate 将 IR 渲染为 SVG
- `src/style/defaults.rs` — 默认颜色、字体、间距等主题常量
- `src/lib.rs` — 公共入口 `process(input) -> Result<String>`
- `src/main.rs` — CLI (clap)，支持文件/stdin 输入、文件/stdout 输出、watch 模式
- `src/wasm.rs` — wasm-bindgen 包装层，导出 `render(input) -> Result<String, JsError>`

### Web (web/)

- `web/src/lib/wasm.ts` — WASM 加载器，import `./pkg/flowdraft.js`（Vite 打包）
- `web/src/lib/templates.ts` — 预置模板 DSL 片段
- `web/src/lib/components/` — Svelte 组件
  - `Editor.svelte` — CodeMirror 6 编辑器（one-dark 主题）
  - `Preview.svelte` — SVG 预览面板
  - `MenuBar.svelte` — 菜单栏（文件、视图、帮助）
  - `TabBar.svelte` — 文件标签栏
  - `StatusBar.svelte` — 状态栏（错误、光标位置、字符数）
  - `SplitPane.svelte` — 可调整分栏
  - `TemplateGallery.svelte` — 模板对话框
  - `ExportDialog.svelte` — 导出对话框
  - `SyntaxReference.svelte` — 语法参考
  - `WelcomeOverlay.svelte` — 欢迎页
- `web/src/lib/stores/` — 状态管理
  - `files.ts` — 文件状态（多文件、标签页、本地存储）
  - `theme.ts` — 主题状态（亮色/暗色切换）
  - `shortcuts.ts` — 快捷键注册与处理
- `web/src/lib/styles/themes.ts` — 主题定义（CSS 变量）
- `web/src/routes/+page.svelte` — 主页面：分栏布局、对话框管理
- `web/src/lib/pkg/` — wasm-pack 构建输出（gitignore，构建时生成，Vite 作为模块打包）

## Feature Flags

- `default = ["cli"]` — 默认构建 CLI
- `cli` — clap/notify/notify-debouncer-mini（仅 CLI）
- `wasm` — wasm-bindgen（仅 WASM 目标）

## 关键依赖

### Rust
- `lexpr` — S-expression 解析，使用 `ColonPrefix` keyword 语法
- `reingold-tilford` — 树布局算法，通过 `NodeInfo` trait 适配
- `svg` — SVG 生成，`element::Text::new(content)` 需要传入文本内容
- `clap` — CLI 参数解析（optional, cli feature）
- `notify` / `notify-debouncer-mini` — 文件监听（optional, cli feature）
- `wasm-bindgen` — WASM 绑定（optional, wasm feature）

### Web
- SvelteKit + `@sveltejs/adapter-static` — 静态站点生成
- CodeMirror 6 — 代码编辑器
- Vite — 构建工具

## DSL 形式

四种顶层 form：`tree`、`line`、`style`、`define`。关键字用冒号前缀 (`:down`, `:label`, `:fill`)。

### define / 模板实例化

`define` 定义可复用组件模板，在 `tree` 和 `flow` 中通过模板名实例化。展开在 AST 层完成（`src/parse/expand.rs`），IR/layout/render 无需感知模板概念。

```lisp
;; 定义模板，带参数
(define server (params name)
  (cpu :label "${name} CPU")
  (eth0 :label "ETH0")
  (eth1 :label "ETH1"))

;; 在 tree 中实例化 — 位置传参
(tree :down
  (rack
    (server s1 "S1")
    (server s2 "S2")))

;; 在 flow 中内联实例化
(flow :right
  (rack -> (server s3 "S3"))
  (rack -> (server s4 "S4")))

;; 用点号引用子节点
(line :straight s1.eth0 -> s2.eth0 :desc "网络连接")
(style s1.cpu :fill "#e8f4fd")
```

展开流程：`parse → Document → expand_defines() → Document → build_ir → layout → render`

展开规则：
- 实例化节点的第一个 child 作为 instance_id，后续 leaf children 作为位置参数
- 模板 body 中的节点 ID 加前缀 `{instance_id}.`（如 `s1.cpu`、`s1.eth0`）
- label 中的 `${param}` 被替换为参数值
- 展开后生成 `Group`（分组容器），在 SVG 中渲染为带标题栏的圆角矩形
- 支持嵌套模板（模板 body 中使用其他模板），不支持递归自引用

## 注意事项

- `HashMap` 迭代顺序不确定，子节点顺序通过 `TreeInfo.children_order` 保持 DSL 中的声明顺序
- 节点坐标 (x, y) 是中心点，宽高用于矩形绘制
- CJK 字符宽度通过 `unicode-width` 计算，每个 CJK 字符算 2 列宽
- 测试中含嵌套引号的字符串用 `r##"..."##` 原始字符串
- WASM 构建输出到项目根 `pkg/`，由 `build.sh` 复制到 `web/src/lib/pkg/` 供 Vite 打包（wasm-pack 的 `--out-dir` 在新版 cargo 不可用）

### Web 主题系统

主题 CSS 变量通过 `<svelte:head>` 注入到 `:root`，确保所有组件（包括对话框）都能访问：

```svelte
<!-- web/src/routes/+page.svelte -->
<svelte:head>
  {@html `<style>:root { ${themeCSS} }</style>`}
</svelte:head>
```

**重要**：对话框组件（TemplateGallery、ExportDialog、SyntaxReference、WelcomeOverlay）渲染在主应用容器外，必须通过全局 CSS 变量继承主题。不要在 `.app` 元素上使用 `style={themeCSS}`，否则对话框无法访问主题变量。

## 踩坑记录

记录已遇到的 bug 及根因分析，避免重复犯错。

### 1. fix-paths.js 正则替换丢失 `/`

**现象**：打包后 `"/_app/foo"` 变成了 `"._app/foo"` 而非 `"./_app/foo"`，浏览器 404。

**原因**：替换字符串写成 `'$1.$2$3'`，`$2` 捕获的是 `_app/...`（不含前导 `/`），拼接后 `"."`+`"_app/"` = `"._app/"`，少了 `/`。

**修复**：`'$1./$2$3'`，手动补上 `/`。

**教训**：正则替换中移除字符再拼接时，容易丢失分隔符。写完替换后应该用实际输入验证一遍输出，尤其是路径拼接场景——`"." + "_app"` ≠ `"./" + "_app"`。

### 2. serve.rs 用相对路径读文件，依赖 cwd

**现象**：从非可执行文件所在目录运行 `flowdraft-serve` 时，所有静态文件 404。

**原因**：`Path::new(&path)` 构造的是相对路径，相对于进程的 cwd 而非 exe 所在目录。打包后用户可能从任意目录运行。

**修复**：用 `std::env::current_exe().parent()` 获取 exe 所在目录，再 `root.join(&path)` 拼接。

**教训**：嵌入式/自包含的静态文件服务器必须基于 exe 路径解析资源，不能假设 cwd。这是打包分发场景的常见陷阱——开发时 cwd 恰好正确所以不会暴露问题，部署后才出错。编写文件服务器时应默认问自己："用户会从哪个目录运行这个程序？"

### 3. 树边没有连接到 group 容器顶部

**现象**：instance 节点（如 `s1`）展开为 group 后，父节点到 group 的树边终点悬空，没接到 group 容器的视觉顶部，差了约 14px。

**原因**：group bounding box 计算时 `group.y` 因 title bar 上移了 `title_h/2`，但树边终点用的是 instance 节点原始的 `(x, y, height)`，两者的"顶部"差了 `title_h/2`。

**修复**：在 `lib.rs` group bounding box 计算完成后，同步更新 instance 节点的 x/y/width/height 使其与 group 容器一致。

**教训**：当同一个逻辑实体有两套坐标（instance 节点 vs group 容器），修改一方后必须同步另一方，否则依赖其中一方的下游计算（如树边渲染）会出现偏移。引入"影子节点"模式时，应在计算完成后立即同步，而非期望下游各自处理差异。

### 4. AST 结构体新增字段的适配范围

**背景**：为支持 flow 内联模板，给 `FlowSegment` 新增了 `inline_node: Option<TreeNode>` 字段。

**影响范围排查**：修改 AST 结构体时，必须检查所有消费该结构体的位置：
- `src/parse/transform.rs` — 构造 `FlowSegment` 的地方（`parse_flow_chain`）
- `src/parse/expand.rs` — 遍历/修改 segments 的地方（`expand_defines` 的 `Form::Flow` 分支）
- `src/ir/build.rs` — 读取 `seg.node` 构建 IR 的地方（`Form::Flow` 分支）

**本次结论**：`inline_node` 是 `Option`，在 expand 阶段被消费并清除为 `None`。IR 构建只读 `seg.node`，不需要感知 `inline_node`，因此 `build.rs` 无需修改。

**教训**：给 AST 结构体加字段时，按数据流方向逐层检查：`parse → expand → build_ir → layout → render`。对于 `Option` 字段，确认它在哪一层被消费、下游是否需要感知。如果新字段在某层之前就被"归一化"掉了（如 expand 阶段将 inline_node 展开为普通 node），则下游无需适配——但这个结论需要显式验证，不能假设。
