<script lang="ts">
	import { onMount } from 'svelte';
	import { EditorView, basicSetup } from 'codemirror';
	import { EditorState, Compartment } from '@codemirror/state';
	import { oneDark } from '@codemirror/theme-one-dark';
	import { linter, type Diagnostic } from '@codemirror/lint';
	import { flowdraftLanguage } from '$lib/editor/flowdraft-lang';
	import { flowdraftAutocomplete } from '$lib/editor/completions';

	let {
		value = $bindable(''),
		onchange,
		oncursor,
		isDark = true,
		errors = ''
	}: {
		value: string;
		onchange?: (val: string) => void;
		oncursor?: (line: number, col: number) => void;
		isDark?: boolean;
		errors?: string;
	} = $props();

	let container: HTMLDivElement;
	let view: EditorView;
	let themeCompartment = new Compartment();
	let lintCompartment = new Compartment();

	function buildLinter(errorText: string) {
		return linter(() => {
			if (!errorText) return [];
			const diagnostics: Diagnostic[] = [];
			// Try to extract line info from error message
			const lineMatch = errorText.match(/line\s+(\d+)/i);
			const from = lineMatch && view
				? view.state.doc.line(Math.min(parseInt(lineMatch[1]), view.state.doc.lines)).from
				: 0;
			const to = lineMatch && view
				? view.state.doc.line(Math.min(parseInt(lineMatch[1]), view.state.doc.lines)).to
				: Math.min(view?.state.doc.length ?? 0, 1);
			diagnostics.push({
				from,
				to: Math.max(to, from + 1),
				severity: 'error',
				message: errorText
			});
			return diagnostics;
		});
	}

	onMount(() => {
		const updateListener = EditorView.updateListener.of((update) => {
			if (update.docChanged) {
				const newVal = update.state.doc.toString();
				value = newVal;
				onchange?.(newVal);
			}
			if (update.selectionSet || update.docChanged) {
				const pos = update.state.selection.main.head;
				const line = update.state.doc.lineAt(pos);
				oncursor?.(line.number, pos - line.from + 1);
			}
		});

		view = new EditorView({
			state: EditorState.create({
				doc: value,
				extensions: [
					basicSetup,
					themeCompartment.of(isDark ? oneDark : []),
					flowdraftLanguage,
					flowdraftAutocomplete,
					lintCompartment.of(buildLinter(errors)),
					updateListener,
					EditorView.lineWrapping
				]
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

	$effect(() => {
		if (view) {
			view.dispatch({
				effects: themeCompartment.reconfigure(isDark ? oneDark : [])
			});
		}
	});

	$effect(() => {
		if (view) {
			view.dispatch({
				effects: lintCompartment.reconfigure(buildLinter(errors))
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
