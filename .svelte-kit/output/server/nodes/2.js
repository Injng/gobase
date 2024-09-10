

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/2.i1aMhA4h.js","_app/immutable/chunks/scheduler.CfYsyzRB.js","_app/immutable/chunks/index.DIj3ZpPr.js"];
export const stylesheets = [];
export const fonts = [];
