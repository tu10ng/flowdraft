<script lang="ts">
	import type { FileEntry } from '$lib/stores/files';

	let {
		files,
		activeId,
		onSelect,
		onClose,
		onNew,
		onCloseOthers,
		onCloseAll
	}: {
		files: FileEntry[];
		activeId: string;
		onSelect: (id: string) => void;
		onClose: (id: string) => void;
		onNew: () => void;
		onCloseOthers?: (id: string) => void;
		onCloseAll?: () => void;
	} = $props();

	let contextMenu = $state<{ x: number; y: number; fileId: string } | null>(null);

	function handleContext(e: MouseEvent, fileId: string) {
		e.preventDefault();
		contextMenu = { x: e.clientX, y: e.clientY, fileId };
	}

	function closeContext() {
		contextMenu = null;
	}

	function handleMiddleClick(e: MouseEvent, fileId: string) {
		if (e.button === 1) {
			e.preventDefault();
			onClose(fileId);
		}
	}
</script>

<div class="tabbar">
	<div class="tabs">
		{#each files as file (file.id)}
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="tab"
				class:active={file.id === activeId}
				onclick={() => onSelect(file.id)}
				onkeydown={(e) => { if (e.key === 'Enter') onSelect(file.id); }}
				onauxclick={(e) => handleMiddleClick(e, file.id)}
				oncontextmenu={(e) => handleContext(e, file.id)}
				title={file.name}
				role="tab"
				tabindex="0"
				aria-selected={file.id === activeId}
			>
				<span class="tab-name">{file.name}</span>
				{#if file.modified}
					<span class="modified-dot" title="未保存">●</span>
				{/if}
				<button
					class="tab-close"
					onclick={(e) => { e.stopPropagation(); onClose(file.id); }}
					title="关闭"
					aria-label="关闭 {file.name}"
				>×</button>
			</div>
		{/each}
	</div>
	<button class="tab-new" onclick={onNew} title="新建文件" aria-label="新建文件">+</button>
</div>

{#if contextMenu}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="context-backdrop" onclick={closeContext} onkeydown={() => {}}></div>
	<div class="context-menu" style="left: {contextMenu.x}px; top: {contextMenu.y}px;">
		<button onclick={() => { onClose(contextMenu!.fileId); closeContext(); }}>关闭</button>
		{#if onCloseOthers}
			<button onclick={() => { onCloseOthers!(contextMenu!.fileId); closeContext(); }}>关闭其他</button>
		{/if}
		{#if onCloseAll}
			<button onclick={() => { onCloseAll!(); closeContext(); }}>关闭全部</button>
		{/if}
	</div>
{/if}

<style>
	.tabbar {
		display: flex;
		align-items: center;
		height: 32px;
		background: var(--c-bg);
		border-bottom: 1px solid var(--c-border);
		overflow: hidden;
	}

	.tabs {
		display: flex;
		flex: 1;
		overflow-x: auto;
		scrollbar-width: none;
	}

	.tabs::-webkit-scrollbar {
		display: none;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 0 8px;
		height: 32px;
		border: none;
		border-right: 1px solid var(--c-border);
		background: var(--c-tab-bg);
		color: var(--c-text-muted);
		font-size: 12px;
		cursor: pointer;
		white-space: nowrap;
		flex-shrink: 0;
	}

	.tab:hover {
		background: var(--c-menu-hover);
	}

	.tab.active {
		background: var(--c-tab-active-bg);
		color: var(--c-text);
	}

	.tab-name {
		max-width: 120px;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.modified-dot {
		color: var(--c-accent);
		font-size: 10px;
		line-height: 1;
	}

	.tab-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 16px;
		height: 16px;
		border: none;
		background: transparent;
		color: var(--c-text-muted);
		font-size: 14px;
		cursor: pointer;
		border-radius: 3px;
		padding: 0;
		line-height: 1;
	}

	.tab-close:hover {
		background: var(--c-menu-hover);
		color: var(--c-text);
	}

	.tab-new {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		margin: 0 4px;
		border: none;
		background: transparent;
		color: var(--c-text-muted);
		font-size: 18px;
		cursor: pointer;
		border-radius: 4px;
		flex-shrink: 0;
	}

	.tab-new:hover {
		background: var(--c-menu-hover);
		color: var(--c-text);
	}

	.context-backdrop {
		position: fixed;
		inset: 0;
		z-index: 999;
	}

	.context-menu {
		position: fixed;
		z-index: 1000;
		background: var(--c-menu-bg);
		border: 1px solid var(--c-border);
		border-radius: 6px;
		padding: 4px 0;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
		min-width: 120px;
	}

	.context-menu button {
		display: block;
		width: 100%;
		padding: 6px 12px;
		border: none;
		background: transparent;
		color: var(--c-text);
		font-size: 13px;
		cursor: pointer;
		text-align: left;
	}

	.context-menu button:hover {
		background: var(--c-menu-hover);
	}
</style>
