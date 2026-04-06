export interface ThemeColors {
	bg: string;
	bgSecondary: string;
	bgTertiary: string;
	text: string;
	textMuted: string;
	border: string;
	borderLight: string;
	accent: string;
	accentHover: string;
	error: string;
	success: string;
	previewBg: string;
	headerBg: string;
	menuBg: string;
	menuHover: string;
	statusBg: string;
	tabBg: string;
	tabActiveBg: string;
	splitHandle: string;
	splitHandleHover: string;
}

export const darkTheme: ThemeColors = {
	bg: '#1a1a2e',
	bgSecondary: '#16213e',
	bgTertiary: '#0f3460',
	text: '#e0e0e0',
	textMuted: '#888',
	border: '#333',
	borderLight: '#444',
	accent: '#4a90d9',
	accentHover: '#5ba0e9',
	error: '#ef5350',
	success: '#66bb6a',
	previewBg: '#1e1e1e',
	headerBg: '#16213e',
	menuBg: '#1a1a2e',
	menuHover: '#2a2a4e',
	statusBg: '#16213e',
	tabBg: '#1a1a2e',
	tabActiveBg: '#16213e',
	splitHandle: '#444',
	splitHandleHover: '#4a90d9'
};

export const lightTheme: ThemeColors = {
	bg: '#ffffff',
	bgSecondary: '#f5f5f5',
	bgTertiary: '#e8e8e8',
	text: '#1a1a1a',
	textMuted: '#666',
	border: '#ddd',
	borderLight: '#ccc',
	accent: '#1976d2',
	accentHover: '#1565c0',
	error: '#d32f2f',
	success: '#388e3c',
	previewBg: '#fafafa',
	headerBg: '#f5f5f5',
	menuBg: '#ffffff',
	menuHover: '#e8e8e8',
	statusBg: '#f5f5f5',
	tabBg: '#e8e8e8',
	tabActiveBg: '#ffffff',
	splitHandle: '#ccc',
	splitHandleHover: '#1976d2'
};

export function themeToCSS(theme: ThemeColors): string {
	return Object.entries(theme)
		.map(([key, value]) => {
			const cssVar = key.replace(/([A-Z])/g, '-$1').toLowerCase();
			return `--c-${cssVar}: ${value};`;
		})
		.join('\n');
}
