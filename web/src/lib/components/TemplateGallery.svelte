<script lang="ts">
	import { templates, categories, type Template } from '$lib/templates';

	let {
		onSelect,
		onClose
	}: {
		onSelect: (template: Template) => void;
		onClose: () => void;
	} = $props();

	let activeCategory = $state<string>('all');

	let filtered = $derived(
		activeCategory === 'all'
			? templates
			: templates.filter((t) => t.category === activeCategory)
	);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onclick={onClose} onkeydown={() => {}}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="gallery" onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
		<div class="gallery-header">
			<h2>从模板新建</h2>
			<button class="close-btn" onclick={onClose} aria-label="关闭">×</button>
		</div>

		<div class="categories">
			{#each categories as cat}
				<button
					class="cat-btn"
					class:active={activeCategory === cat.id}
					onclick={() => (activeCategory = cat.id)}
				>
					{cat.label}
				</button>
			{/each}
		</div>

		<div class="grid">
			{#each filtered as template (template.id)}
				<button
					class="card"
					onclick={() => { onSelect(template); onClose(); }}
				>
					<div class="card-preview">
						<code class="card-code">{template.code.slice(0, 80)}...</code>
					</div>
					<div class="card-info">
						<span class="card-name">{template.name}</span>
						<span class="card-desc">{template.description}</span>
					</div>
				</button>
			{/each}
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

	.gallery {
		background: var(--c-bg);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		width: 640px;
		max-width: 90vw;
		max-height: 80vh;
		display: flex;
		flex-direction: column;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
	}

	.gallery-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px;
		border-bottom: 1px solid var(--c-border);
	}

	.gallery-header h2 {
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

	.categories {
		display: flex;
		gap: 4px;
		padding: 8px 16px;
		border-bottom: 1px solid var(--c-border);
	}

	.cat-btn {
		padding: 4px 12px;
		border: 1px solid var(--c-border);
		border-radius: 16px;
		background: transparent;
		color: var(--c-text-muted);
		font-size: 12px;
		cursor: pointer;
	}

	.cat-btn:hover {
		background: var(--c-menu-hover);
	}

	.cat-btn.active {
		background: var(--c-accent);
		color: #fff;
		border-color: var(--c-accent);
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
		gap: 12px;
		padding: 16px;
		overflow-y: auto;
	}

	.card {
		border: 1px solid var(--c-border);
		border-radius: 6px;
		background: var(--c-bg-secondary);
		cursor: pointer;
		overflow: hidden;
		text-align: left;
		padding: 0;
		transition: border-color 0.15s;
	}

	.card:hover {
		border-color: var(--c-accent);
	}

	.card-preview {
		padding: 8px;
		background: var(--c-preview-bg);
		height: 80px;
		overflow: hidden;
	}

	.card-code {
		font-size: 9px;
		color: var(--c-text-muted);
		white-space: pre-wrap;
		word-break: break-all;
		line-height: 1.3;
	}

	.card-info {
		padding: 8px;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.card-name {
		font-size: 13px;
		font-weight: 500;
		color: var(--c-text);
	}

	.card-desc {
		font-size: 11px;
		color: var(--c-text-muted);
	}
</style>
