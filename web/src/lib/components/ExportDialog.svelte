<script lang="ts">
	import { downloadSvg, downloadPng, copySvgToClipboard, copyPngToClipboard } from '$lib/utils/export';

	let {
		svg,
		filename = 'flowdraft',
		onClose
	}: {
		svg: string;
		filename?: string;
		onClose: () => void;
	} = $props();

	let format = $state<'svg' | 'png'>('svg');
	let pngScale = $state(2);
	let copyStatus = $state('');

	async function handleDownload() {
		if (format === 'svg') {
			downloadSvg(svg, `${filename}.svg`);
		} else {
			await downloadPng(svg, pngScale, `${filename}.png`);
		}
		onClose();
	}

	async function handleCopy() {
		try {
			if (format === 'svg') {
				await copySvgToClipboard(svg);
			} else {
				await copyPngToClipboard(svg, pngScale);
			}
			copyStatus = '已复制';
			setTimeout(() => (copyStatus = ''), 2000);
		} catch {
			copyStatus = '复制失败';
			setTimeout(() => (copyStatus = ''), 2000);
		}
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onclick={onClose} onkeydown={() => {}}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="dialog" onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
		<div class="dialog-header">
			<h2>导出</h2>
			<button class="close-btn" onclick={onClose} aria-label="关闭">×</button>
		</div>

		<div class="dialog-body">
			<div class="field">
				<span class="field-label">格式</span>
				<div class="radio-group">
					<label class="radio-label">
						<input type="radio" bind:group={format} value="svg" /> SVG
					</label>
					<label class="radio-label">
						<input type="radio" bind:group={format} value="png" /> PNG
					</label>
				</div>
			</div>

			{#if format === 'png'}
				<div class="field">
					<label for="png-scale">缩放倍率</label>
					<select id="png-scale" bind:value={pngScale}>
						<option value={1}>1x</option>
						<option value={2}>2x</option>
						<option value={3}>3x</option>
					</select>
				</div>
			{/if}

			<div class="preview-box">
				{@html svg}
			</div>
		</div>

		<div class="dialog-footer">
			{#if copyStatus}
				<span class="copy-status">{copyStatus}</span>
			{/if}
			<button class="btn secondary" onclick={handleCopy}>复制到剪贴板</button>
			<button class="btn primary" onclick={handleDownload}>下载</button>
		</div>
	</div>
</div>

<style>
	.overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 2000;
	}

	.dialog {
		background: var(--c-bg);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		width: 420px;
		max-width: 90vw;
		max-height: 80vh;
		display: flex;
		flex-direction: column;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
	}

	.dialog-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px;
		border-bottom: 1px solid var(--c-border);
	}

	.dialog-header h2 {
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

	.dialog-body {
		padding: 16px;
		overflow-y: auto;
	}

	.field {
		margin-bottom: 12px;
	}

	.field > label,
	.field-label {
		display: block;
		font-size: 13px;
		color: var(--c-text-muted);
		margin-bottom: 4px;
	}

	.radio-group {
		display: flex;
		gap: 16px;
	}

	.radio-label {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 13px;
		color: var(--c-text);
		cursor: pointer;
	}

	select {
		padding: 4px 8px;
		border: 1px solid var(--c-border);
		border-radius: 4px;
		background: var(--c-bg-secondary);
		color: var(--c-text);
		font-size: 13px;
	}

	.preview-box {
		border: 1px solid var(--c-border);
		border-radius: 4px;
		padding: 12px;
		background: var(--c-preview-bg);
		max-height: 200px;
		overflow: auto;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.preview-box :global(svg) {
		max-width: 100%;
		max-height: 180px;
	}

	.dialog-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 8px;
		padding: 12px 16px;
		border-top: 1px solid var(--c-border);
	}

	.copy-status {
		font-size: 12px;
		color: var(--c-success);
		margin-right: auto;
	}

	.btn {
		padding: 6px 16px;
		border-radius: 4px;
		border: 1px solid var(--c-border);
		font-size: 13px;
		cursor: pointer;
	}

	.btn.secondary {
		background: var(--c-bg-secondary);
		color: var(--c-text);
	}

	.btn.secondary:hover {
		background: var(--c-menu-hover);
	}

	.btn.primary {
		background: var(--c-accent);
		color: #fff;
		border-color: var(--c-accent);
	}

	.btn.primary:hover {
		background: var(--c-accent-hover);
	}
</style>
