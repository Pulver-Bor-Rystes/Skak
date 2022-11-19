export const manifest = {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png"]),
	mimeTypes: {".png":"image/png"},
	_: {
		entry: {"file":"_app/immutable/start-285ac975.js","imports":["_app/immutable/start-285ac975.js","_app/immutable/chunks/index-7f932505.js","_app/immutable/chunks/singletons-7c861e1e.js"],"stylesheets":[]},
		nodes: [
			() => import('./nodes/0.js'),
			() => import('./nodes/1.js'),
			() => import('./nodes/2.js')
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				names: [],
				types: [],
				optional: [],
				page: { layouts: [0], errors: [1], leaf: 2 },
				endpoint: () => import('./entries/endpoints/_server.ts.js')
			}
		],
		matchers: async () => {
			
			return {  };
		}
	}
};
