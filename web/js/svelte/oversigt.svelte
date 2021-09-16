<script type="ts">
	import { post_request } from "./public";

	const files_promise = post_request("/testing/svelte_files");
</script>

<div class="example">
	<h1>Velkommen</h1>

	<p>
		Du kan teste dine svelte komponenter via addressen 
		<span>
			/test/komponent_navn
		</span>
		eller trykke på en af de nedenstående links :)
		{#await files_promise}
			<p>...venter på filerne</p>
		{:then files}
			{#each files as file}
				<br>
				<br>
				<span class="link">		
					<a href="/test/{String(file).replace(/\\/g, "/").replace(/\//g, '>')}"> { String(file).replace(/\\/g, "/") } </a>
				</span>
			{/each}
		{:catch error}
			<p style="color: red">{error.message}</p>
		{/await}
	</p>
</div>

<style type="scss">
	.example {
		background-color: antiquewhite;
		padding: 1rem;
		margin: 1rem;

		border-radius: 5px;

		box-shadow: rgba(0, 0, 0, 0.25) 0px 14px 28px,
			rgba(0, 0, 0, 0.22) 0px 10px 10px;

		p {
			span {
				background: slategray;
				box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px,
					rgba(60, 64, 67, 0.15) 0px 2px 6px 2px;
				padding: 0.3rem 0.5rem;
				border-radius: 5px;
			}

			.link {
				&:hover {
					box-shadow: rgba(0, 0, 0, 0.25) 0px 14px 28px, rgba(0, 0, 0, 0.22) 0px 10px 10px;

					cursor: pointer;
				}

				transition: box-shadow 0.3s;
			}
		}
	}
</style>
