<script lang="ts">
	let { onClose, onOpenTemplates }: { onClose: () => void; onOpenTemplates: () => void } = $props();

	let step = $state(0);

	const steps = [
		{
			title: '欢迎使用 Flowdraft',
			text: '用简洁的 Lisp 风格 DSL 创建流程图、树形图等 SVG 图表。'
		},
		{
			title: '编写 DSL',
			text: '在左侧编辑器中输入代码，右侧实时预览。支持语法高亮和自动补全。'
		},
		{
			title: '导出与分享',
			text: '支持导出 SVG 和 PNG 格式，也可以直接复制到剪贴板。'
		},
		{
			title: '开始创作',
			text: '从模板开始，或直接编写你的第一个图表。'
		}
	];

	function next() {
		if (step < steps.length - 1) {
			step++;
		} else {
			finish();
		}
	}

	function finish() {
		localStorage.setItem('flowdraft-welcome-done', '1');
		onClose();
	}

	function startFromTemplates() {
		finish();
		onOpenTemplates();
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onkeydown={() => {}}>
	<div class="welcome">
		<div class="step-content">
			<h2>{steps[step].title}</h2>
			<p>{steps[step].text}</p>
		</div>

		<div class="step-dots">
			{#each steps as _, i}
				<span class="dot" class:active={i === step}></span>
			{/each}
		</div>

		<div class="actions">
			<button class="btn secondary" onclick={finish}>跳过</button>
			{#if step === steps.length - 1}
				<button class="btn secondary" onclick={startFromTemplates}>浏览模板</button>
				<button class="btn primary" onclick={finish}>开始使用</button>
			{:else}
				<button class="btn primary" onclick={next}>下一步</button>
			{/if}
		</div>
	</div>
</div>

<style>
	.overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 3000;
	}

	.welcome {
		background: var(--c-bg);
		border: 1px solid var(--c-border);
		border-radius: 12px;
		padding: 32px;
		width: 400px;
		max-width: 90vw;
		text-align: center;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
	}

	.step-content {
		margin-bottom: 24px;
	}

	h2 {
		font-size: 20px;
		font-weight: 600;
		color: var(--c-text);
		margin: 0 0 8px 0;
	}

	p {
		font-size: 14px;
		color: var(--c-text-muted);
		margin: 0;
		line-height: 1.5;
	}

	.step-dots {
		display: flex;
		justify-content: center;
		gap: 6px;
		margin-bottom: 24px;
	}

	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--c-border);
	}

	.dot.active {
		background: var(--c-accent);
	}

	.actions {
		display: flex;
		justify-content: center;
		gap: 8px;
	}

	.btn {
		padding: 8px 20px;
		border-radius: 6px;
		border: 1px solid var(--c-border);
		font-size: 14px;
		cursor: pointer;
	}

	.btn.secondary {
		background: transparent;
		color: var(--c-text-muted);
	}

	.btn.secondary:hover {
		background: var(--c-menu-hover);
		color: var(--c-text);
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
