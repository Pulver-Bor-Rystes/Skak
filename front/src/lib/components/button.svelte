<script lang="ts">
    let { color = '#ffffff', text = 'black', fn = () => {} } = $props();
</script>


<button onclick={() => { fn() }} style="background: {color}6a; color: {text};" class="m-1 hover:shadow-xl hover:ps-5 transition-all cursor-pointer shadow rounded-xl p-3 ps-4 text-center">
    <slot />
</button>