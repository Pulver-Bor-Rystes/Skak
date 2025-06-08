<script lang="ts">
    let { 
        color = '#ffffff', 
        text = 'black',
        class_extra = '',
        fn = () => {},
    } = $props();

    let hover_class = class_extra.includes("hover") ? "" : "hover:shadow-lg transition-all cursor-pointer";
</script>


<!-- svelte-ignore slot_element_deprecated -->
<button onclick={() => { fn() }} style="background: {color}6a; color: {text};" class="shadow my-rounded p-3 ps-4 text-center {hover_class} {class_extra}">
    <slot />
</button>