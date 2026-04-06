import { autocompletion, type CompletionContext, type CompletionResult } from '@codemirror/autocomplete';

const topLevelForms = [
	{ label: 'tree', type: 'keyword', info: '树形图' },
	{ label: 'flow', type: 'keyword', info: '流程图' },
	{ label: 'line', type: 'keyword', info: '连线' },
	{ label: 'style', type: 'keyword', info: '样式' }
];

const directionKeywords = [
	{ label: ':down', type: 'keyword', info: '向下布局' },
	{ label: ':right', type: 'keyword', info: '向右布局' }
];

const styleKeywords = [
	{ label: ':label', type: 'keyword', info: '节点标签' },
	{ label: ':fill', type: 'keyword', info: '填充颜色' },
	{ label: ':stroke', type: 'keyword', info: '边框颜色' },
	{ label: ':color', type: 'keyword', info: '文字颜色' },
	{ label: ':desc', type: 'keyword', info: '连线描述' }
];

const lineKeywords = [
	{ label: ':straight', type: 'keyword', info: '直线连接' },
	{ label: ':desc', type: 'keyword', info: '连线描述' },
	{ label: ':color', type: 'keyword', info: '连线颜色' }
];

function flowdraftCompletions(context: CompletionContext): CompletionResult | null {
	// Match colon-prefixed keywords
	const colonMatch = context.matchBefore(/:[a-zA-Z_-]*/);
	if (colonMatch) {
		// Determine context by looking back for the form type
		const lineText = context.state.doc.lineAt(context.pos).text;
		const beforeCursor = lineText.slice(0, colonMatch.from - context.state.doc.lineAt(context.pos).from);

		let options = [...styleKeywords, ...directionKeywords, ...lineKeywords];

		if (/\(\s*(tree|flow)\b/.test(beforeCursor)) {
			options = [...directionKeywords, ...styleKeywords];
		} else if (/\(\s*line\b/.test(beforeCursor)) {
			options = lineKeywords;
		} else if (/\(\s*style\b/.test(beforeCursor)) {
			options = styleKeywords;
		}

		return {
			from: colonMatch.from,
			options,
			validFor: /^:[a-zA-Z_-]*$/
		};
	}

	// Match word at start of form (after open paren)
	const wordMatch = context.matchBefore(/\(\s*[a-zA-Z]*/);
	if (wordMatch) {
		const textAfterParen = wordMatch.text.replace(/^\(\s*/, '');
		return {
			from: wordMatch.from + wordMatch.text.indexOf(textAfterParen),
			options: topLevelForms,
			validFor: /^[a-zA-Z]*$/
		};
	}

	return null;
}

export const flowdraftAutocomplete = autocompletion({
	override: [flowdraftCompletions]
});
