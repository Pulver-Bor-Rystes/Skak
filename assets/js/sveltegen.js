window.onload = () => {
	const components = document.querySelectorAll('.svelte-component')

	components.forEach(elem => {
		const name = elem.getAttribute('data-name')
		const parsed_props = JSON.parse(elem.getAttribute('data-props'))

		const svelte_app = require(`./svelte/${ name }`)
		if (!svelte_app) return

		const app = new svelte_app.default({
			target: elem.parentElement,
			props: parsed_props,
		})

		elem.remove()
	})
}

export default null