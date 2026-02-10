

export const index = 3;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/account/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/3.DHTUYljG.js","_app/immutable/chunks/Dp-e6d5r.js","_app/immutable/chunks/BTTlp1_X.js","_app/immutable/chunks/Ch6oBkdf.js"];
export const stylesheets = [];
export const fonts = [];
