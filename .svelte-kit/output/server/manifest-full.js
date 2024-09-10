export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png"]),
	mimeTypes: {".png":"image/png"},
	_: {
		client: {"start":"_app/immutable/entry/start.Cvc1xlk8.js","app":"_app/immutable/entry/app.CYhmag6W.js","imports":["_app/immutable/entry/start.Cvc1xlk8.js","_app/immutable/chunks/entry.DSPY-2my.js","_app/immutable/chunks/scheduler.CfYsyzRB.js","_app/immutable/entry/app.CYhmag6W.js","_app/immutable/chunks/scheduler.CfYsyzRB.js","_app/immutable/chunks/index.DIj3ZpPr.js"],"stylesheets":[],"fonts":[],"uses_env_dynamic_public":false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
