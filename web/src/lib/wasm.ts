import init, { render as wasmRender } from './pkg/flowdraft.js';

let initialized = false;

export async function initWasm() {
	if (initialized) return;
	await init();
	initialized = true;
}

export function render(input: string): string {
	return wasmRender(input);
}
