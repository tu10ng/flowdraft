# Flowdraft Web Playground

基于 SvelteKit 的交互式 Web 编辑器，实时预览 Flowdraft DSL 渲染结果。

## 功能特性

- 🎨 **实时预览** - 编辑器与 SVG 预览同步更新
- 🌓 **主题切换** - 支持亮色/暗色主题
- 📝 **代码编辑** - CodeMirror 6 编辑器，语法高亮
- 📁 **多文件管理** - 标签页支持，本地存储
- 🎯 **模板库** - 内置示例模板快速开始
- 📤 **导出功能** - 导出 SVG/PNG 格式
- ⌨️ **快捷键** - 完整的键盘快捷键支持
- 📖 **语法参考** - 内置 DSL 语法文档

## 开发

```bash
# 安装依赖
pnpm install

# 构建 WASM + 启动开发服务器
pnpm run dev

# 仅构建 WASM
pnpm run wasm

# 构建静态站点
pnpm run build

# 预览构建结果
pnpm run preview
```

## 项目结构

```
src/
├── lib/
│   ├── components/
│   │   ├── Editor.svelte           # CodeMirror 编辑器
│   │   ├── Preview.svelte          # SVG 预览面板
│   │   ├── MenuBar.svelte          # 菜单栏
│   │   ├── TabBar.svelte           # 文件标签栏
│   │   ├── StatusBar.svelte        # 状态栏
│   │   ├── SplitPane.svelte        # 可调整分栏
│   │   ├── TemplateGallery.svelte  # 模板对话框
│   │   ├── ExportDialog.svelte     # 导出对话框
│   │   ├── SyntaxReference.svelte  # 语法参考
│   │   └── WelcomeOverlay.svelte   # 欢迎页
│   ├── stores/
│   │   ├── files.ts                # 文件状态管理
│   │   ├── theme.ts                # 主题状态管理
│   │   └── shortcuts.ts            # 快捷键管理
│   ├── styles/
│   │   └── themes.ts               # 主题定义（亮色/暗色）
│   ├── templates.ts                # 内置模板
│   ├── wasm.ts                     # WASM 加载器
│   └── pkg/                        # WASM 构建输出（构建时生成）
└── routes/
    └── +page.svelte                # 主页面

build/                              # 静态站点输出
```

## 技术栈

- **SvelteKit** - Web 框架，使用 Svelte 5 runes
- **CodeMirror 6** - 代码编辑器
- **Vite** - 构建工具
- **wasm-pack** - Rust → WASM 编译
- **TypeScript** - 类型安全

## 主题系统

主题 CSS 变量通过 `<svelte:head>` 注入到 `:root`，确保所有组件（包括对话框）都能访问：

```typescript
// src/lib/stores/theme.ts
export const themeMode = writable<'light' | 'dark'>('dark');
export function getThemeCSS(mode: 'light' | 'dark'): string {
  const theme = themes[mode];
  return Object.entries(theme)
    .map(([key, value]) => `${key}: ${value};`)
    .join(' ');
}
```

```svelte
<!-- src/routes/+page.svelte -->
<svelte:head>
  {@html `<style>:root { ${themeCSS} }</style>`}
</svelte:head>
```

这样对话框组件即使渲染在主应用容器外，也能正确继承主题变量。

## 快捷键

- `Ctrl+N` - 新建文件
- `Ctrl+O` - 打开文件
- `Ctrl+S` - 保存
- `Ctrl+Shift+S` - 另存为
- `Ctrl+E` - 导出
- `Ctrl+Z` - 撤销
- `Ctrl+Y` - 重做

## 部署

构建静态站点：

```bash
pnpm run build
```

输出到 `build/` 目录，可部署到任何静态托管服务（Vercel、Netlify、GitHub Pages 等）。
