<script lang="ts">
	let { svg = '', error = '' }: { svg: string; error: string } = $props();
</script>

<div class="preview">
	{#if error}
		<div class="error">{error}</div>
	{:else if svg}
		<div class="svg-container">
			{@html svg}
		</div>
	{:else}
		<div class="placeholder">Enter some DSL to see the preview</div>
	{/if}
</div>

<style>
	.preview {
		height: 100%;
		overflow: auto;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #1e1e1e;
	}
	.svg-container {
		padding: 2rem;
	}
	.svg-container :global(svg) {
		max-width: 100%;
		height: auto;
	}
	.error {
		color: #ef5350;
		font-family: monospace;
		padding: 1rem;
		white-space: pre-wrap;
		max-width: 80%;
	}
	.placeholder {
		color: #666;
		font-style: italic;
	}
</style>
