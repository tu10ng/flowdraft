<script lang="ts">
	import { onMount } from 'svelte';
	import { initWasm, render } from '$lib/wasm';
	import { examples } from '$lib/examples';
	import Editor from '$lib/components/Editor.svelte';
	import Preview from '$lib/components/Preview.svelte';

	let code = $state(examples[0].code);
	let svg = $state('');
	let error = $state('');
	let ready = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout>;

	onMount(async () => {
		await initWasm();
		ready = true;
		doRender(code);
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
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => doRender(val), 300);
	}

	function selectExample(e: Event) {
		const idx = (e.target as HTMLSelectElement).selectedIndex;
		code = examples[idx].code;
		doRender(code);
	}

	function exportSvg() {
		if (!svg) return;
		const blob = new Blob([svg], { type: 'image/svg+xml' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = 'flowdraft.svg';
		a.click();
		URL.revokeObjectURL(url);
	}
</script>

<svelte:head>
	<title>Flowdraft Playground</title>
</svelte:head>

{#if !ready}
	<div class="loading">Loading WASM...</div>
{:else}
	<div class="app">
		<header>
			<h1>Flowdraft Playground</h1>
			<div class="toolbar">
				<select onchange={selectExample} aria-label="Select example">
					{#each examples as ex}
						<option>{ex.name}</option>
					{/each}
				</select>
				<button onclick={exportSvg} disabled={!svg}>Export SVG</button>
			</div>
		</header>
		<main>
			<div class="pane editor-pane">
				<Editor bind:value={code} onchange={onEditorChange} />
			</div>
			<div class="pane preview-pane">
				<Preview {svg} {error} />
			</div>
		</main>
	</div>
{/if}

<style>
	:global(body) {
		margin: 0;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		background: #1a1a2e;
		color: #e0e0e0;
	}
	.loading {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100vh;
		font-size: 1.2rem;
		color: #888;
	}
	.app {
		display: flex;
		flex-direction: column;
		height: 100vh;
	}
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 1rem;
		background: #16213e;
		border-bottom: 1px solid #333;
	}
	h1 {
		font-size: 1rem;
		margin: 0;
		font-weight: 600;
	}
	.toolbar {
		display: flex;
		gap: 0.5rem;
	}
	select,
	button {
		padding: 0.3rem 0.6rem;
		border-radius: 4px;
		border: 1px solid #444;
		background: #1a1a2e;
		color: #e0e0e0;
		font-size: 0.85rem;
		cursor: pointer;
	}
	button:hover:not(:disabled) {
		background: #2a2a4e;
	}
	button:disabled {
		opacity: 0.5;
		cursor: default;
	}
	main {
		display: flex;
		flex: 1;
		min-height: 0;
	}
	.pane {
		flex: 1;
		min-width: 0;
	}
	.editor-pane {
		border-right: 1px solid #333;
	}
</style>
