<script lang="ts">
	let { svg = '', error = '' }: { svg: string; error: string } = $props();

	let zoom = $state(1);
	let panX = $state(0);
	let panY = $state(0);
	let isPanning = $state(false);
	let lastPointer = $state({ x: 0, y: 0 });
	let container: HTMLDivElement;

	const MIN_ZOOM = 0.1;
	const MAX_ZOOM = 5;

	function handleWheel(e: WheelEvent) {
		e.preventDefault();
		const rect = container.getBoundingClientRect();
		const mouseX = e.clientX - rect.left;
		const mouseY = e.clientY - rect.top;

		const factor = e.deltaY > 0 ? 0.9 : 1.1;
		const newZoom = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, zoom * factor));
		const scale = newZoom / zoom;

		panX = mouseX - scale * (mouseX - panX);
		panY = mouseY - scale * (mouseY - panY);
		zoom = newZoom;
	}

	function handlePointerDown(e: PointerEvent) {
		if (e.button === 1 || (e.button === 0 && e.shiftKey)) {
			e.preventDefault();
			isPanning = true;
			lastPointer = { x: e.clientX, y: e.clientY };
			(e.target as HTMLElement).setPointerCapture(e.pointerId);
		}
	}

	function handlePointerMove(e: PointerEvent) {
		if (!isPanning) return;
		panX += e.clientX - lastPointer.x;
		panY += e.clientY - lastPointer.y;
		lastPointer = { x: e.clientX, y: e.clientY };
	}

	function handlePointerUp() {
		isPanning = false;
	}

	function zoomIn() {
		zoom = Math.min(MAX_ZOOM, zoom * 1.2);
	}

	function zoomOut() {
		zoom = Math.max(MIN_ZOOM, zoom / 1.2);
	}

	function zoomFit() {
		zoom = 1;
		panX = 0;
		panY = 0;
	}

	function zoomReset() {
		zoom = 1;
		panX = 0;
		panY = 0;
	}

	let zoomPercent = $derived(Math.round(zoom * 100));
	let transformStyle = $derived(`transform: translate(${panX}px, ${panY}px) scale(${zoom})`);
</script>

<div
	class="preview"
	class:panning={isPanning}
	bind:this={container}
	onwheel={handleWheel}
	onpointerdown={handlePointerDown}
	onpointermove={handlePointerMove}
	onpointerup={handlePointerUp}
	role="img"
	aria-label="SVG 预览"
>
	{#if error}
		<div class="error">{error}</div>
	{:else if svg}
		<div class="svg-wrapper" style={transformStyle}>
			{@html svg}
		</div>
	{:else}
		<div class="placeholder">输入 DSL 代码以预览图表</div>
	{/if}

	{#if svg && !error}
		<div class="zoom-controls">
			<button onclick={zoomOut} title="缩小" aria-label="缩小">−</button>
			<span class="zoom-label">{zoomPercent}%</span>
			<button onclick={zoomIn} title="放大" aria-label="放大">+</button>
			<button onclick={zoomFit} title="适应窗口" aria-label="适应窗口">⊡</button>
			<button onclick={zoomReset} title="100%" aria-label="重置缩放">1:1</button>
		</div>
	{/if}
</div>

<style>
	.preview {
		height: 100%;
		overflow: hidden;
		position: relative;
		background: var(--c-preview-bg);
		cursor: default;
	}

	.preview.panning {
		cursor: grabbing;
	}

	.svg-wrapper {
		transform-origin: 0 0;
		display: inline-block;
		padding: 2rem;
	}

	.svg-wrapper :global(svg) {
		display: block;
	}

	.error {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		color: var(--c-error);
		font-family: monospace;
		padding: 1rem;
		white-space: pre-wrap;
		max-width: 80%;
	}

	.placeholder {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		color: var(--c-text-muted);
		font-style: italic;
	}

	.zoom-controls {
		position: absolute;
		bottom: 12px;
		right: 12px;
		display: flex;
		align-items: center;
		gap: 2px;
		background: var(--c-bg);
		border: 1px solid var(--c-border);
		border-radius: 6px;
		padding: 2px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
	}

	.zoom-controls button {
		border: none;
		background: transparent;
		color: var(--c-text);
		font-size: 14px;
		padding: 4px 8px;
		cursor: pointer;
		border-radius: 4px;
		line-height: 1;
	}

	.zoom-controls button:hover {
		background: var(--c-menu-hover);
	}

	.zoom-label {
		font-size: 12px;
		color: var(--c-text-muted);
		min-width: 40px;
		text-align: center;
	}
</style>
