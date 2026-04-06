<script lang="ts">
	import logo from '$lib/assets/logo.svg';
	import { themeMode } from '$lib/stores/theme';

	let {
		onNewFile,
		onOpenFile,
		onSave,
		onSaveAs,
		onExport,
		onTemplateGallery,
		onSyntaxReference,
		onShortcutsHelp,
		layoutMode = $bindable('horizontal' as 'horizontal' | 'vertical')
	}: {
		onNewFile?: () => void;
		onOpenFile?: () => void;
		onSave?: () => void;
		onSaveAs?: () => void;
		onExport?: () => void;
		onTemplateGallery?: () => void;
		onSyntaxReference?: () => void;
		onShortcutsHelp?: () => void;
		layoutMode?: 'horizontal' | 'vertical';
	} = $props();

	let openMenu = $state<string | null>(null);

	const menus = [
		{
			id: 'file',
			label: '文件',
			items: [
				{ label: '新建', shortcut: 'Ctrl+N', action: () => onNewFile?.() },
				{ label: '打开...', shortcut: 'Ctrl+O', action: () => onOpenFile?.() },
				{ label: '---' },
				{ label: '保存', shortcut: 'Ctrl+S', action: () => onSave?.() },
				{ label: '另存为...', shortcut: 'Ctrl+Shift+S', action: () => onSaveAs?.() },
				{ label: '---' },
				{ label: '导出...', shortcut: 'Ctrl+E', action: () => onExport?.() },
				{ label: '---' },
				{ label: '从模板新建...', action: () => onTemplateGallery?.() }
			]
		},
		{
			id: 'view',
			label: '视图',
			items: [
				{
					label: '切换主题',
					action: () => themeMode.toggle()
				},
				{
					label: '水平布局',
					action: () => (layoutMode = 'horizontal')
				},
				{
					label: '垂直布局',
					action: () => (layoutMode = 'vertical')
				}
			]
		},
		{
			id: 'help',
			label: '帮助',
			items: [
				{ label: '快捷键', action: () => onShortcutsHelp?.() },
				{ label: '语法参考', action: () => onSyntaxReference?.() }
			]
		}
	];

	function toggleMenu(id: string) {
		openMenu = openMenu === id ? null : id;
	}

	function enterMenu(id: string) {
		if (openMenu !== null) {
			openMenu = id;
		}
	}

	function handleItemClick(item: { action?: () => void }) {
		item.action?.();
		openMenu = null;
	}

	function handleBackdropClick() {
		openMenu = null;
	}
</script>

<nav class="menubar">
	<div class="brand">
		<img src={logo} alt="Flowdraft" class="logo" />
		<span class="brand-name">Flowdraft</span>
	</div>

	<div class="menus">
		{#each menus as menu}
			<div class="menu-wrapper">
				<button
					class="menu-trigger"
					class:active={openMenu === menu.id}
					role="menuitem"
					aria-haspopup="true"
					aria-expanded={openMenu === menu.id}
					onclick={() => toggleMenu(menu.id)}
					onmouseenter={() => enterMenu(menu.id)}
				>
					{menu.label}
				</button>
				{#if openMenu === menu.id}
					<div class="menu-dropdown" role="menu">
						{#each menu.items as item}
							{#if item.label === '---'}
								<div class="menu-separator"></div>
							{:else}
								<button
									class="menu-item"
									role="menuitem"
									onclick={() => handleItemClick(item)}
								>
									<span class="menu-item-label">{item.label}</span>
									{#if 'shortcut' in item && item.shortcut}
										<span class="menu-item-shortcut">{item.shortcut}</span>
									{/if}
								</button>
							{/if}
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<div class="spacer"></div>

	<button
		class="theme-toggle"
		onclick={() => themeMode.toggle()}
		title="切换主题"
		aria-label="切换主题"
	>
		{$themeMode === 'dark' ? '☀️' : '🌙'}
	</button>
</nav>

{#if openMenu !== null}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="menu-backdrop" onclick={handleBackdropClick} onkeydown={() => {}}></div>
{/if}

<style>
	.menubar {
		display: flex;
		align-items: center;
		height: 36px;
		padding: 0 8px;
		background: var(--c-header-bg);
		border-bottom: 1px solid var(--c-border);
		font-size: 13px;
		user-select: none;
		gap: 0;
	}

	.brand {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-right: 12px;
	}

	.logo {
		width: 20px;
		height: 20px;
	}

	.brand-name {
		font-weight: 600;
		font-size: 14px;
		color: var(--c-text);
	}

	.menus {
		display: flex;
		align-items: center;
	}

	.menu-wrapper {
		position: relative;
	}

	.menu-trigger {
		padding: 4px 8px;
		border: none;
		background: transparent;
		color: var(--c-text-muted);
		cursor: pointer;
		font-size: 13px;
		border-radius: 4px;
	}

	.menu-trigger:hover,
	.menu-trigger.active {
		background: var(--c-menu-hover);
		color: var(--c-text);
	}

	.menu-dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		min-width: 200px;
		background: var(--c-menu-bg);
		border: 1px solid var(--c-border);
		border-radius: 6px;
		padding: 4px 0;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
		z-index: 1000;
	}

	.menu-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 6px 12px;
		border: none;
		background: transparent;
		color: var(--c-text);
		cursor: pointer;
		font-size: 13px;
		text-align: left;
	}

	.menu-item:hover {
		background: var(--c-menu-hover);
	}

	.menu-item-shortcut {
		color: var(--c-text-muted);
		font-size: 12px;
		margin-left: 24px;
	}

	.menu-separator {
		height: 1px;
		background: var(--c-border);
		margin: 4px 0;
	}

	.spacer {
		flex: 1;
	}

	.theme-toggle {
		padding: 4px 8px;
		border: none;
		background: transparent;
		cursor: pointer;
		font-size: 16px;
		border-radius: 4px;
		line-height: 1;
	}

	.theme-toggle:hover {
		background: var(--c-menu-hover);
	}

	.menu-backdrop {
		position: fixed;
		inset: 0;
		z-index: 999;
	}
</style>
