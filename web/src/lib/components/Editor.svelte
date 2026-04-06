<script lang="ts">
	import { onMount } from 'svelte';
	import { EditorView, basicSetup } from 'codemirror';
	import { EditorState } from '@codemirror/state';
	import { oneDark } from '@codemirror/theme-one-dark';

	let { value = $bindable(''), onchange }: { value: string; onchange?: (val: string) => void } =
		$props();

	let container: HTMLDivElement;
	let view: EditorView;

	onMount(() => {
		const updateListener = EditorView.updateListener.of((update) => {
			if (update.docChanged) {
				const newVal = update.state.doc.toString();
				value = newVal;
				onchange?.(newVal);
			}
		});

		view = new EditorView({
			state: EditorState.create({
				doc: value,
				extensions: [basicSetup, oneDark, updateListener, EditorView.lineWrapping]
			}),
			parent: container
		});

		return () => view.destroy();
	});

	$effect(() => {
		if (view && value !== view.state.doc.toString()) {
			view.dispatch({
				changes: { from: 0, to: view.state.doc.length, insert: value }
			});
		}
	});
</script>

<div class="editor" bind:this={container}></div>

<style>
	.editor {
		height: 100%;
		overflow: auto;
	}
	.editor :global(.cm-editor) {
		height: 100%;
	}
</style>
