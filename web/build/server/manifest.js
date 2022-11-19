const manifest = {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png"]),
	mimeTypes: {".png":"image/png"},
	_: {
		entry: {"file":"_app/immutable/start-285ac975.js","imports":["_app/immutable/start-285ac975.js","_app/immutable/chunks/index-7f932505.js","_app/immutable/chunks/singletons-7c861e1e.js"],"stylesheets":[]},
		nodes: [
			() => import('./chunks/0-59a9c84c.js'),
			() => import('./chunks/1-be89a04e.js'),
			() => import('./chunks/2-0281f98e.js')
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				names: [],
				types: [],
				optional: [],
				page: { layouts: [0], errors: [1], leaf: 2 },
				endpoint: () => import('./chunks/_server.ts-af777ee9.js')
			}
		],
		matchers: async () => {
			
			return {  };
		}
	}
};

export { manifest };
//# sourceMappingURL=manifest.js.map
