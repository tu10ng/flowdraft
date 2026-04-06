<script lang="ts">
	import { onMount } from 'svelte';

	let {
		direction = 'horizontal',
		initialRatio = 0.5,
		minRatio = 0.15,
		maxRatio = 0.85,
		storageKey = 'flowdraft-split-ratio',
		children
	}: {
		direction?: 'horizontal' | 'vertical';
		initialRatio?: number;
		minRatio?: number;
		maxRatio?: number;
		storageKey?: string;
		children: import('svelte').Snippet<[{ first: boolean }]>;
	} = $props();

	let ratio = $state(0.5);
	let dragging = $state(false);
	let container: HTMLDivElement;

	onMount(() => {
		const stored = localStorage.getItem(storageKey);
		if (stored) {
			const val = parseFloat(stored);
			if (!isNaN(val) && val >= minRatio && val <= maxRatio) {
				ratio = val;
				return;
			}
		}
		ratio = initialRatio;
	});

	function onPointerDown(e: PointerEvent) {
		e.preventDefault();
		dragging = true;
		(e.target as HTMLElement).setPointerCapture(e.pointerId);
	}

	function onPointerMove(e: PointerEvent) {
		if (!dragging || !container) return;
		const rect = container.getBoundingClientRect();
		let newRatio: number;
		if (direction === 'horizontal') {
			newRatio = (e.clientX - rect.left) / rect.width;
		} else {
			newRatio = (e.clientY - rect.top) / rect.height;
		}
		ratio = Math.max(minRatio, Math.min(maxRatio, newRatio));
	}

	function onPointerUp() {
		if (dragging) {
			dragging = false;
			localStorage.setItem(storageKey, ratio.toString());
		}
	}

	function onDoubleClick() {
		ratio = 0.5;
		localStorage.setItem(storageKey, '0.5');
	}

	let firstStyle = $derived(
		direction === 'horizontal'
			? `width: ${ratio * 100}%`
			: `height: ${ratio * 100}%`
	);

	let secondStyle = $derived(
		direction === 'horizontal'
			? `width: ${(1 - ratio) * 100}%`
			: `height: ${(1 - ratio) * 100}%`
	);
</script>

<div
	class="split-pane"
	class:horizontal={direction === 'horizontal'}
	class:vertical={direction === 'vertical'}
	class:dragging
	bind:this={container}
>
	<div class="pane first" style={firstStyle}>
		{@render children({ first: true })}
	</div>
	<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
	<div
		class="handle"
		role="separator"
		aria-orientation={direction}
		tabindex="0"
		onpointerdown={onPointerDown}
		onpointermove={onPointerMove}
		onpointerup={onPointerUp}
		ondblclick={onDoubleClick}
	></div>
	<div class="pane second" style={secondStyle}>
		{@render children({ first: false })}
	</div>
</div>

<style>
	.split-pane {
		display: flex;
		flex: 1;
		min-height: 0;
		min-width: 0;
		overflow: hidden;
	}

	.split-pane.horizontal {
		flex-direction: row;
	}

	.split-pane.vertical {
		flex-direction: column;
	}

	.pane {
		overflow: hidden;
		min-width: 0;
		min-height: 0;
	}

	.handle {
		flex-shrink: 0;
		background: var(--c-border);
		transition: background 0.15s;
		z-index: 10;
	}

	.handle:hover,
	.dragging .handle {
		background: var(--c-split-handle-hover);
	}

	.horizontal > .handle {
		width: 4px;
		cursor: col-resize;
	}

	.vertical > .handle {
		height: 4px;
		cursor: row-resize;
	}

	.dragging {
		cursor: col-resize;
		user-select: none;
	}

	.vertical.dragging {
		cursor: row-resize;
	}
</style>
