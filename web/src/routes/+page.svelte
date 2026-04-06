<script lang="ts">
	import { onMount } from 'svelte';
	import { initWasm, render } from '$lib/wasm';
	import { themeMode, getThemeCSS } from '$lib/stores/theme';
	import { fileStore } from '$lib/stores/files';
	import { registerShortcuts, handleKeydown, getShortcuts } from '$lib/stores/shortcuts';
	import type { Template } from '$lib/templates';
	import Editor from '$lib/components/Editor.svelte';
	import Preview from '$lib/components/Preview.svelte';
	import MenuBar from '$lib/components/MenuBar.svelte';
	import TabBar from '$lib/components/TabBar.svelte';
	import SplitPane from '$lib/components/SplitPane.svelte';
	import StatusBar from '$lib/components/StatusBar.svelte';
	import ExportDialog from '$lib/components/ExportDialog.svelte';
	import TemplateGallery from '$lib/components/TemplateGallery.svelte';
	import SyntaxReference from '$lib/components/SyntaxReference.svelte';
	import WelcomeOverlay from '$lib/components/WelcomeOverlay.svelte';

	let svg = $state('');
	let error = $state('');
	let ready = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout>;
	let cursorLine = $state(1);
	let cursorCol = $state(1);
	let layoutMode = $state<'horizontal' | 'vertical'>('horizontal');
	let showExportDialog = $state(false);
	let showTemplateGallery = $state(false);
	let showSyntaxReference = $state(false);
	let showShortcutsHelp = $state(false);
	let showWelcome = $state(false);

	let themeCSS = $derived(getThemeCSS($themeMode));
	let isDark = $derived($themeMode === 'dark');

	const { files, activeId } = fileStore;
	let activeFile = $derived($files.find((f) => f.id === $activeId));
	let code = $derived(activeFile?.code ?? '');

	onMount(() => {
		initWasm().then(() => {
			ready = true;
			doRender(code);
		});

		// Show welcome on first visit
		if (!localStorage.getItem('flowdraft-welcome-done')) {
			showWelcome = true;
		}

		registerShortcuts([
			{ key: 'n', ctrl: true, label: 'Ctrl+N', description: '新建文件', action: handleNewFile },
			{ key: 'o', ctrl: true, label: 'Ctrl+O', description: '打开文件', action: handleOpenFile },
			{ key: 's', ctrl: true, label: 'Ctrl+S', description: '保存', action: handleSave },
			{ key: 's', ctrl: true, shift: true, label: 'Ctrl+Shift+S', description: '另存为', action: handleSaveAs },
			{ key: 'e', ctrl: true, label: 'Ctrl+E', description: '导出', action: handleExport }
		]);

		const onKey = (e: KeyboardEvent) => handleKeydown(e);
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	});

	$effect(() => {
		if (ready) {
			doRender(code);
		}
	});

	function doRender(input: string) {
		try {
			svg = render(input);
			error = '';
		} catch (e: any) {
			error = e.message || String(e);
			svg = '';
		}
	}

	function onEditorChange(val: string) {
		const id = $activeId;
		if (!id) return;
		fileStore.updateCode(id, val);
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => doRender(val), 300);
	}

	function onCursor(line: number, col: number) {
		cursorLine = line;
		cursorCol = col;
	}

	function handleNewFile() {
		fileStore.newFile();
	}

	function handleOpenFile() {
		const input = document.createElement('input');
		input.type = 'file';
		input.accept = '.fd';
		input.onchange = async () => {
			const file = input.files?.[0];
			if (!file) return;
			const content = await file.text();
			fileStore.openFile(file.name, content);
		};
		input.click();
	}

	function handleSave() {
		if (!activeFile) return;
		downloadFile(activeFile.code, activeFile.name, 'text/plain');
		fileStore.markSaved(activeFile.id);
	}

	function handleSaveAs() {
		if (!activeFile) return;
		const name = prompt('文件名:', activeFile.name) || activeFile.name;
		downloadFile(activeFile.code, name, 'text/plain');
		fileStore.markSaved(activeFile.id, name);
	}

	function handleExport() {
		if (!svg) return;
		showExportDialog = true;
	}

	function handleTemplateSelect(template: Template) {
		fileStore.newFile(template.code, `${template.name}.fd`);
	}

	function downloadFile(content: string, filename: string, type: string) {
		const blob = new Blob([content], { type });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = filename;
		a.click();
		URL.revokeObjectURL(url);
	}

	function handleSelectTab(id: string) {
		fileStore.setActive(id);
	}

	function handleCloseTab(id: string) {
		fileStore.closeFile(id);
	}

	function handleCloseOthers(id: string) {
		fileStore.closeOthers(id);
	}

	function handleCloseAll() {
		fileStore.closeAll();
	}
</script>

<svelte:head>
	<title>Flowdraft</title>
	{@html `<style>:root { ${themeCSS} }</style>`}
</svelte:head>

{#if !ready}
	<div class="loading">Loading WASM...</div>
{:else}
	<div class="app">
		<MenuBar
			onNewFile={handleNewFile}
			onOpenFile={handleOpenFile}
			onSave={handleSave}
			onSaveAs={handleSaveAs}
			onExport={handleExport}
			onTemplateGallery={() => (showTemplateGallery = true)}
			onSyntaxReference={() => (showSyntaxReference = true)}
			onShortcutsHelp={() => (showShortcutsHelp = true)}
			bind:layoutMode
		/>
		<TabBar
			files={$files}
			activeId={$activeId}
			onSelect={handleSelectTab}
			onClose={handleCloseTab}
			onNew={handleNewFile}
			onCloseOthers={handleCloseOthers}
			onCloseAll={handleCloseAll}
		/>
		<SplitPane direction={layoutMode} storageKey="flowdraft-split-{layoutMode}">
			{#snippet children({ first })}
				{#if first}
					{#key $activeId}
						<Editor value={code} onchange={onEditorChange} oncursor={onCursor} {isDark} errors={error} />
					{/key}
				{:else}
					<Preview {svg} {error} />
				{/if}
			{/snippet}
		</SplitPane>
		<StatusBar {error} {cursorLine} {cursorCol} charCount={code.length} />
	</div>

	{#if showExportDialog && svg}
		<ExportDialog
			{svg}
			filename={activeFile?.name.replace(/\.fd$/, '') || 'flowdraft'}
			onClose={() => (showExportDialog = false)}
		/>
	{/if}

	{#if showTemplateGallery}
		<TemplateGallery
			onSelect={handleTemplateSelect}
			onClose={() => (showTemplateGallery = false)}
		/>
	{/if}

	{#if showSyntaxReference}
		<SyntaxReference onClose={() => (showSyntaxReference = false)} />
	{/if}

	{#if showShortcutsHelp}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="overlay" onclick={() => (showShortcutsHelp = false)} onkeydown={() => {}}>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="shortcuts-panel" onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
				<div class="shortcuts-header">
					<h2>快捷键</h2>
					<button class="close-btn" onclick={() => (showShortcutsHelp = false)} aria-label="关闭">×</button>
				</div>
				<div class="shortcuts-body">
					{#each getShortcuts() as shortcut}
						<div class="shortcut-row">
							<span class="shortcut-desc">{shortcut.description}</span>
							<kbd>{shortcut.label}</kbd>
						</div>
					{/each}
					<div class="shortcut-row">
						<span class="shortcut-desc">撤销</span>
						<kbd>Ctrl+Z</kbd>
					</div>
					<div class="shortcut-row">
						<span class="shortcut-desc">重做</span>
						<kbd>Ctrl+Y</kbd>
					</div>
				</div>
			</div>
		</div>
	{/if}

	{#if showWelcome}
		<WelcomeOverlay
			onClose={() => (showWelcome = false)}
			onOpenTemplates={() => (showTemplateGallery = true)}
		/>
	{/if}
{/if}

<style>
	:global(body) {
		margin: 0;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		background: var(--c-bg);
		color: var(--c-text);
	}
	.loading {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100vh;
		font-size: 1.2rem;
		color: var(--c-text-muted);
		background: var(--c-bg);
	}
	.app {
		display: flex;
		flex-direction: column;
		height: 100vh;
	}
	.overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 2000;
	}
	.shortcuts-panel {
		background: var(--c-bg);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		width: 360px;
		max-width: 90vw;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
	}
	.shortcuts-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px;
		border-bottom: 1px solid var(--c-border);
	}
	.shortcuts-header h2 {
		margin: 0;
		font-size: 16px;
		font-weight: 600;
		color: var(--c-text);
	}
	.close-btn {
		border: none;
		background: transparent;
		color: var(--c-text-muted);
		font-size: 20px;
		cursor: pointer;
		padding: 0 4px;
		line-height: 1;
	}
	.close-btn:hover {
		color: var(--c-text);
	}
	.shortcuts-body {
		padding: 12px 16px;
	}
	.shortcut-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 6px 0;
	}
	.shortcut-desc {
		font-size: 13px;
		color: var(--c-text);
	}
	kbd {
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 12px;
		padding: 2px 6px;
		border: 1px solid var(--c-border);
		border-radius: 4px;
		background: var(--c-bg-secondary);
		color: var(--c-text-muted);
	}
</style>
