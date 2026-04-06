<script lang="ts">
	let { onClose }: { onClose: () => void } = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onclick={onClose} onkeydown={() => {}}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="panel" onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
		<div class="panel-header">
			<h2>语法参考</h2>
			<button class="close-btn" onclick={onClose} aria-label="关闭">×</button>
		</div>
		<div class="panel-body">
			<section>
				<h3>树形图 (tree)</h3>
				<pre><code>{`(tree :down
  (root :label "根"
    (child1 :label "子1")
    (child2 :label "子2")))`}</code></pre>
				<p>方向: <code>:down</code> <code>:right</code></p>
			</section>

			<section>
				<h3>流程图 (flow)</h3>
				<pre><code>{`(flow :right
  (a -> b -> c)
  (b -> d))`}</code></pre>
			</section>

			<section>
				<h3>连线 (line)</h3>
				<pre><code>{`(line a -> b :desc "描述" :color "#4a90d9")
(line :straight a -> b)`}</code></pre>
				<p>选项: <code>:straight</code> <code>:desc</code> <code>:color</code></p>
			</section>

			<section>
				<h3>样式 (style)</h3>
				<pre><code>{`(style node :fill "#e8f4fd" :stroke "#2196f3")
(style node :color "#333")`}</code></pre>
				<p>属性: <code>:fill</code> <code>:stroke</code> <code>:color</code></p>
			</section>

			<section>
				<h3>节点属性</h3>
				<p><code>:label "文本"</code> — 节点显示文本</p>
				<p>节点名为标识符，如 <code>myNode</code></p>
			</section>

			<section>
				<h3>模板定义 (define)</h3>
				<pre><code>{`(define server (params name)
  (cpu :label "\${name} CPU")
  (eth0 :label "ETH0"))`}</code></pre>
				<p>定义可复用的组件模板，支持 <code>{'${param}'}</code> 参数替换</p>
			</section>

			<section>
				<h3>模板实例化</h3>
				<pre><code>{`(tree :down
  (rack
    (server s1 "S1")
    (server s2 "S2")))`}</code></pre>
				<p>实例化后子节点 ID 自动加前缀：<code>s1.cpu</code>、<code>s1.eth0</code></p>
				<p>可用点号引用子节点：<code>(line s1.eth0 -> s2.eth0)</code></p>
				<p>可对子节点设样式：<code>(style s1.cpu :fill "#e8f4fd")</code></p>
			</section>
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

	.panel {
		background: var(--c-bg);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		width: 480px;
		max-width: 90vw;
		max-height: 80vh;
		display: flex;
		flex-direction: column;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px;
		border-bottom: 1px solid var(--c-border);
	}

	.panel-header h2 {
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

	.panel-body {
		padding: 16px;
		overflow-y: auto;
	}

	section {
		margin-bottom: 16px;
	}

	h3 {
		font-size: 14px;
		font-weight: 600;
		color: var(--c-text);
		margin: 0 0 8px 0;
	}

	pre {
		background: var(--c-bg-secondary);
		border: 1px solid var(--c-border);
		border-radius: 4px;
		padding: 8px;
		margin: 4px 0;
		overflow-x: auto;
	}

	code {
		font-size: 12px;
		color: var(--c-text);
		font-family: 'SF Mono', 'Fira Code', monospace;
	}

	p {
		font-size: 13px;
		color: var(--c-text-muted);
		margin: 4px 0;
		line-height: 1.5;
	}

	p code {
		background: var(--c-bg-secondary);
		padding: 1px 4px;
		border-radius: 3px;
	}
</style>
