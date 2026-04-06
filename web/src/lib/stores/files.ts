import { writable, get } from 'svelte/store';

export interface FileEntry {
	id: string;
	name: string;
	code: string;
	modified: boolean;
	lastSaved: number;
}

function generateId(): string {
	return Date.now().toString(36) + Math.random().toString(36).slice(2, 7);
}

function createFilesStore() {
	const STORAGE_KEY = 'flowdraft-files';
	const ACTIVE_KEY = 'flowdraft-active-file';

	let stored: FileEntry[] = [];
	let storedActive = '';

	if (typeof localStorage !== 'undefined') {
		try {
			const raw = localStorage.getItem(STORAGE_KEY);
			if (raw) stored = JSON.parse(raw);
			storedActive = localStorage.getItem(ACTIVE_KEY) || '';
		} catch {}
	}

	if (stored.length === 0) {
		const defaultFile: FileEntry = {
			id: generateId(),
			name: 'untitled.fd',
			code: '',
			modified: false,
			lastSaved: Date.now()
		};
		stored = [defaultFile];
		storedActive = defaultFile.id;
	}

	if (!storedActive || !stored.find((f) => f.id === storedActive)) {
		storedActive = stored[0].id;
	}

	const files = writable<FileEntry[]>(stored);
	const activeId = writable<string>(storedActive);

	let saveTimer: ReturnType<typeof setTimeout>;

	function persist() {
		clearTimeout(saveTimer);
		saveTimer = setTimeout(() => {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(get(files)));
			localStorage.setItem(ACTIVE_KEY, get(activeId));
		}, 500);
	}

	files.subscribe(() => persist());
	activeId.subscribe(() => persist());

	function getActive(): FileEntry | undefined {
		const all = get(files);
		const id = get(activeId);
		return all.find((f) => f.id === id);
	}

	return {
		files,
		activeId,

		newFile(code = '', name = 'untitled.fd'): string {
			const file: FileEntry = {
				id: generateId(),
				name,
				code,
				modified: false,
				lastSaved: Date.now()
			};
			files.update((f) => [...f, file]);
			activeId.set(file.id);
			return file.id;
		},

		openFile(name: string, code: string): string {
			const file: FileEntry = {
				id: generateId(),
				name,
				code,
				modified: false,
				lastSaved: Date.now()
			};
			files.update((f) => [...f, file]);
			activeId.set(file.id);
			return file.id;
		},

		setActive(id: string) {
			activeId.set(id);
		},

		updateCode(id: string, code: string) {
			files.update((all) =>
				all.map((f) => (f.id === id ? { ...f, code, modified: true } : f))
			);
		},

		markSaved(id: string, name?: string) {
			files.update((all) =>
				all.map((f) =>
					f.id === id
						? { ...f, modified: false, lastSaved: Date.now(), name: name ?? f.name }
						: f
				)
			);
		},

		closeFile(id: string) {
			const all = get(files);
			if (all.length <= 1) {
				// Last tab — replace with empty
				const newId = generateId();
				files.set([
					{ id: newId, name: 'untitled.fd', code: '', modified: false, lastSaved: Date.now() }
				]);
				activeId.set(newId);
				return;
			}
			const idx = all.findIndex((f) => f.id === id);
			const remaining = all.filter((f) => f.id !== id);
			files.set(remaining);
			if (get(activeId) === id) {
				const newIdx = Math.min(idx, remaining.length - 1);
				activeId.set(remaining[newIdx].id);
			}
		},

		closeOthers(id: string) {
			const all = get(files);
			const keep = all.filter((f) => f.id === id);
			files.set(keep);
			activeId.set(id);
		},

		closeAll() {
			const newId = generateId();
			files.set([
				{ id: newId, name: 'untitled.fd', code: '', modified: false, lastSaved: Date.now() }
			]);
			activeId.set(newId);
		},

		getActive
	};
}

export const fileStore = createFilesStore();
