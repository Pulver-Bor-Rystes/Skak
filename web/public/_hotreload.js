/*
	Bør ikke ændres!

    Scriptet sørger for at opdatere scripts uden at browseren behøver at reloade
*/

function _reload() {
	const socket = io()
	const latest_update = []
	const exceptions = ['hotreload.js']

	socket.on('__hotreload', reload)

	function reload(file) {
		if (exceptions.includes(file)) return
		if (file.includes('.map')) return
		if (latest_update.includes(file)) return

		if (latest_update.length >= 2) latest_update.shift()
		setTimeout(() => {
			latest_update.shift()
		}, 150)

		latest_update.push(file)

		const format = file.split('.')[1]

		const tag =
			format == 'js' ? 'script' : format == 'css' ? 'link' : undefined
		const src_type =
			format == 'js' ? 'src' : format == 'css' ? 'href' : undefined

		// Find det rigtige element
		let elem
		const elements = document.querySelectorAll(tag)
		for (const E of elements) if (E[src_type].includes(file)) elem = E

		if (!elem) return

		const new_source = new_version(elem[src_type])
		elem.remove()

		let new_elem = document.createElement(tag)

		if (format == 'js') {
			document.body.querySelectorAll('*').forEach(child => child.remove())
			new_elem.defer = true
		} else if (format == 'css') {
			new_elem.setAttribute('rel', 'stylesheet')
		}

		new_elem[src_type] = new_source

		document.head.appendChild(new_elem)
	}

	function new_version(str) {
		return str.split('?')[0] + '?version=' + new Date().getMilliseconds()
	}
}

_reload()
