export function downloadSvg(svg: string, filename = 'flowdraft.svg') {
	const blob = new Blob([svg], { type: 'image/svg+xml' });
	downloadBlob(blob, filename);
}

export async function downloadPng(svg: string, scale = 2, filename = 'flowdraft.png') {
	const blob = await svgToPngBlob(svg, scale);
	downloadBlob(blob, filename);
}

export async function copySvgToClipboard(svg: string): Promise<void> {
	await navigator.clipboard.writeText(svg);
}

export async function copyPngToClipboard(svg: string, scale = 2): Promise<void> {
	const blob = await svgToPngBlob(svg, scale);
	await navigator.clipboard.write([new ClipboardItem({ 'image/png': blob })]);
}

export function svgToPngBlob(svg: string, scale = 2): Promise<Blob> {
	return new Promise((resolve, reject) => {
		const parser = new DOMParser();
		const doc = parser.parseFromString(svg, 'image/svg+xml');
		const svgEl = doc.documentElement;

		let width = parseFloat(svgEl.getAttribute('width') || '400');
		let height = parseFloat(svgEl.getAttribute('height') || '300');

		// If no explicit width/height, try viewBox
		if (!svgEl.getAttribute('width') && svgEl.getAttribute('viewBox')) {
			const vb = svgEl.getAttribute('viewBox')!.split(/\s+/).map(Number);
			width = vb[2];
			height = vb[3];
		}

		const canvas = document.createElement('canvas');
		canvas.width = width * scale;
		canvas.height = height * scale;
		const ctx = canvas.getContext('2d')!;
		ctx.scale(scale, scale);

		const img = new Image();
		const blob = new Blob([svg], { type: 'image/svg+xml;charset=utf-8' });
		const url = URL.createObjectURL(blob);

		img.onload = () => {
			ctx.drawImage(img, 0, 0, width, height);
			URL.revokeObjectURL(url);
			canvas.toBlob((b) => {
				if (b) resolve(b);
				else reject(new Error('Canvas toBlob failed'));
			}, 'image/png');
		};
		img.onerror = () => {
			URL.revokeObjectURL(url);
			reject(new Error('Failed to load SVG as image'));
		};
		img.src = url;
	});
}

function downloadBlob(blob: Blob, filename: string) {
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = filename;
	a.click();
	URL.revokeObjectURL(url);
}
