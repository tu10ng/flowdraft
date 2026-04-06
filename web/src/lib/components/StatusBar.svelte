<script lang="ts">
	let {
		error = '',
		cursorLine = 1,
		cursorCol = 1,
		charCount = 0
	}: {
		error?: string;
		cursorLine?: number;
		cursorCol?: number;
		charCount?: number;
	} = $props();
</script>

<div class="statusbar">
	<div class="left">
		{#if error}
			<span class="status-error">✕ 错误</span>
		{:else}
			<span class="status-ok">✓ 就绪</span>
		{/if}
	</div>
	<div class="right">
		<span class="status-item">行 {cursorLine}, 列 {cursorCol}</span>
		<span class="status-item">{charCount} 字符</span>
	</div>
</div>

<style>
	.statusbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 24px;
		padding: 0 12px;
		background: var(--c-status-bg);
		border-top: 1px solid var(--c-border);
		font-size: 12px;
		color: var(--c-text-muted);
		user-select: none;
	}

	.left, .right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.status-error {
		color: var(--c-error);
	}

	.status-ok {
		color: var(--c-success);
	}

	.status-item {
		color: var(--c-text-muted);
	}
</style>
