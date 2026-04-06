import { writable } from 'svelte/store';
import { darkTheme, lightTheme, themeToCSS, type ThemeColors } from '$lib/styles/themes';

export type ThemeMode = 'dark' | 'light';

function createThemeStore() {
	const stored = typeof localStorage !== 'undefined' ? localStorage.getItem('flowdraft-theme') : null;
	const initial: ThemeMode = stored === 'light' ? 'light' : 'dark';

	const { subscribe, set, update } = writable<ThemeMode>(initial);

	return {
		subscribe,
		toggle() {
			update((mode) => {
				const next = mode === 'dark' ? 'light' : 'dark';
				localStorage.setItem('flowdraft-theme', next);
				return next;
			});
		},
		set(mode: ThemeMode) {
			localStorage.setItem('flowdraft-theme', mode);
			set(mode);
		}
	};
}

export const themeMode = createThemeStore();

export function getThemeColors(mode: ThemeMode): ThemeColors {
	return mode === 'dark' ? darkTheme : lightTheme;
}

export function getThemeCSS(mode: ThemeMode): string {
	return themeToCSS(mode === 'dark' ? darkTheme : lightTheme);
}
