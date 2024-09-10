

export const index = 1;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/error.svelte.js')).default;
export const imports = ["_app/immutable/nodes/1.DuYfzEm1.js","_app/immutable/chunks/scheduler.CfYsyzRB.js","_app/immutable/chunks/index.DIj3ZpPr.js","_app/immutable/chunks/entry.DSPY-2my.js"];
export const stylesheets = [];
export const fonts = [];
