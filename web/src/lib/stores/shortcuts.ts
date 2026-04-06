export interface Shortcut {
	key: string;
	ctrl?: boolean;
	shift?: boolean;
	alt?: boolean;
	label: string;
	description: string;
	action: () => void;
}

let shortcuts: Shortcut[] = [];

export function registerShortcuts(newShortcuts: Shortcut[]) {
	shortcuts = newShortcuts;
}

export function handleKeydown(e: KeyboardEvent) {
	for (const s of shortcuts) {
		const ctrlMatch = (s.ctrl ?? false) === (e.ctrlKey || e.metaKey);
		const shiftMatch = (s.shift ?? false) === e.shiftKey;
		const altMatch = (s.alt ?? false) === e.altKey;
		if (ctrlMatch && shiftMatch && altMatch && e.key.toLowerCase() === s.key.toLowerCase()) {
			e.preventDefault();
			s.action();
			return;
		}
	}
}

export function getShortcuts(): Shortcut[] {
	return shortcuts;
}
