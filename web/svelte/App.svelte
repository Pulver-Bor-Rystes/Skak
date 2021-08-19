<script lang="ts">
	let c: number = 0

	function on_click(): void {
		c += 1
	}
</script>


<main>
	<h1> Eksempel 😉 </h1>
	<button on:click={ on_click }>Klik på mig: {c}</button>
	<p id="kage"> Jeg kan godt lide kage! 🎂 </p>
</main>


<style lang="sass">
	main
		margin: 3rem

	h1
		color: white

	p
		color: lightgreen
		padding: 1rem
		border: 2px solid lightgreen
		transition: border .2s
		&:hover
			border-width: 2px 1rem 1rem 2px

	button
		color: white
		padding: 1rem
		border: 2px solid yellow
		font-size: 2rem
		width: 300px
</style>