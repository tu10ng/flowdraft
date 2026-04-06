# Flowdraft

Lisp 风格 DSL 到 SVG 图表的命令行渲染工具。

## 安装

```bash
cargo install --path .
```

## 使用

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
└── main.rs         # CLI 入口 (clap)
```

## License

MIT
