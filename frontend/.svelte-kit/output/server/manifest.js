export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["robots.txt"]),
	mimeTypes: {".txt":"text/plain"},
	_: {
		client: {start:"_app/immutable/entry/start.BuX6n3Dt.js",app:"_app/immutable/entry/app.B4FOizO9.js",imports:["_app/immutable/entry/start.BuX6n3Dt.js","_app/immutable/chunks/Ooel2MN5.js","_app/immutable/chunks/BTTlp1_X.js","_app/immutable/chunks/BF8ulL8l.js","_app/immutable/entry/app.B4FOizO9.js","_app/immutable/chunks/BTTlp1_X.js","_app/immutable/chunks/C25z8XKX.js","_app/immutable/chunks/Dd817DGY.js","_app/immutable/chunks/Dp-e6d5r.js","_app/immutable/chunks/BF8ulL8l.js","_app/immutable/chunks/D2hWl5aQ.js","_app/immutable/chunks/V-LdXSTq.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js')),
			__memo(() => import('./nodes/3.js')),
			__memo(() => import('./nodes/4.js')),
			__memo(() => import('./nodes/5.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			},
			{
				id: "/account",
				pattern: /^\/account\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			},
			{
				id: "/login",
				pattern: /^\/login\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 4 },
				endpoint: null
			},
			{
				id: "/login/register",
				pattern: /^\/login\/register\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 5 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
