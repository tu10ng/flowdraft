import { StreamLanguage, type StreamParser } from '@codemirror/language';

const flowdraftParser: StreamParser<{ depth: number }> = {
	startState() {
		return { depth: 0 };
	},
	token(stream, state) {
		// Skip whitespace
		if (stream.eatSpace()) return null;

		// Comments: lines starting with ;
		if (stream.match(/^;.*/)) {
			return 'comment';
		}

		// Strings: "..."
		if (stream.match(/^"(?:[^"\\]|\\.)*"/)) {
			return 'string';
		}

		// Arrow operator
		if (stream.match('->')) {
			return 'operator';
		}

		// Parentheses
		if (stream.eat('(')) {
			state.depth++;
			return 'paren';
		}
		if (stream.eat(')')) {
			state.depth--;
			return 'paren';
		}

		// Keywords with colon prefix
		if (stream.match(/^:[a-zA-Z_-]+/)) {
			return 'keyword';
		}

		// Top-level forms
		if (stream.match(/^(tree|flow|line|style)\b/)) {
			return 'typeName';
		}

		// Node names / identifiers
		if (stream.match(/^[a-zA-Z_][a-zA-Z0-9_-]*/)) {
			return 'variableName';
		}

		// Numbers
		if (stream.match(/^#[0-9a-fA-F]{3,8}/)) {
			return 'color';
		}

		// Skip unknown character
		stream.next();
		return null;
	}
};

export const flowdraftLanguage = StreamLanguage.define(flowdraftParser);
