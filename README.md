# Flowdraft

Lisp 风格 DSL 到 SVG 图表的渲染工具，提供命令行工具和 Web Playground。

## 安装与使用

### Web Playground（推荐）

在线体验：访问 Web Playground 即可在浏览器中编辑和预览图表。

本地运行：

```bash
# 一键构建并启动开发服务器
./build.sh dev

# 或手动构建
./build.sh web
cd web && pnpm run dev
```

### 命令行工具

安装：

```bash
cargo install --path .
```

使用：

```bash
# 文件 → 文件
flowdraft input.fd -o output.svg

# stdin → stdout
echo '(tree :down (a b c))' | flowdraft > out.svg

# 监听模式：文件变化时自动重新渲染
flowdraft watch input.fd -o output.svg
```

省略 `-o` 时 watch 模式默认输出到 `input.svg`。

## DSL 语法

### 树 (tree)

```lisp
(tree :down        ;; :down 向下布局 | :right 向右布局
  (root
    (child1 leaf1 leaf2)
    (child2 leaf3)))
```

括号表示子树，裸符号表示叶子节点。节点可用 `:label` 指定显示文本：

```lisp
(tree :down
  (ceo :label "CEO"
    (dev :label "研发部")
    (pm :label "产品部")))
```

### 连线 (line)

```lisp
(line :straight a -> b)                          ;; 基本连线
(line :straight a -> b :desc "描述" :color "#f00") ;; 带标签和颜色
```

线型：`:straight` | `:curved`

箭头：`->` 正向 | `<-` 反向 | `<->` 双向 | `--` 无箭头

### 样式 (style)

```lisp
(style node-name :fill "#e8f4fd" :stroke "#2196f3")
```

## 完整示例

```lisp
(tree :down
  (ceo :label "CEO"
    (dev :label "研发部"
      (fe :label "前端")
      (be :label "后端")
      (qa :label "测试"))
    (pm :label "产品部")
    (hr :label "人力资源")))

(line :straight fe -> be :desc "协作" :color "#4a90d9")

(style ceo :fill "#e8f4fd" :stroke "#2196f3")
(style dev :fill "#e8f5e9" :stroke "#4caf50")
(style pm  :fill "#fff3e0" :stroke "#ff9800")
(style hr  :fill "#fce4ec" :stroke "#e91e63")
```

```bash
flowdraft examples/org.fd -o examples/org.svg
```

## 架构

```
src/
├── parse/          # Lisp DSL 解析 (lexpr) → AST
├── ir/             # AST → 中间表示 (节点/边/树信息)
├── layout/         # Reingold-Tilford 树布局算法
├── render/         # IR → SVG 输出
├── style/          # 默认主题常量
├── lib.rs          # 公共 API: process(input) → SVG string
├── main.rs         # CLI 入口 (clap)
└── wasm.rs         # WASM 绑定 (wasm-bindgen)

web/
├── src/
│   ├── lib/
│   │   ├── components/   # Svelte 组件 (Editor, Preview, 对话框等)
│   │   ├── stores/       # 状态管理 (文件、主题、快捷键)
│   │   ├── styles/       # 主题定义
│   │   ├── wasm.ts       # WASM 加载器
│   │   └── pkg/          # WASM 构建输出 (构建时生成)
│   └── routes/
│       └── +page.svelte  # 主页面
└── build/                # 静态站点输出
```

## 构建

```bash
./build.sh         # 测试 + CLI + WASM + Web 静态站点
./build.sh cli     # 仅 CLI
./build.sh wasm    # 仅 WASM
./build.sh web     # WASM + Web 静态站点
./build.sh dev     # WASM + 启动开发服务器
./build.sh test    # 仅测试
```

## 技术栈

### Rust
- **lexpr** - S-expression 解析
- **reingold-tilford** - 树布局算法
- **svg** - SVG 生成
- **clap** - CLI 参数解析
- **wasm-bindgen** - WASM 绑定

### Web
- **SvelteKit** - Web 框架
- **CodeMirror 6** - 代码编辑器
- **Vite** - 构建工具

## License

MIT
