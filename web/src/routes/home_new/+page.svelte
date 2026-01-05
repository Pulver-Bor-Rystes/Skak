<script>
	import { username, active_players, friends } from "$lib/state";
	
	
	
</script>




<div class="m-5">
	<h1> Hej { $username } </h1>
	

	<div class="mt-5">
		<div class="border w-1/4 ml">
			<h3 class="-mt-4 ml-2 pl-2 pr-2 bg-[#fff] w-20"> Online </h3>
			{#each $active_players as user}
				<p class="ml-2 cursor-pointer hover:text-[#3a85d0]"> {user} </p>
			{/each}
		</div>
	
	
		<br>
	
		<div class="border w-1/4">
			<h3 class="-mt-4 ml-2 pl-2 pr-2 bg-[#fff] w-20"> Friends </h3>
			{#each $friends as user}
				<p class="ml-2"> {user} </p>
			{/each}
		</div>

	</div>
	
</div>





