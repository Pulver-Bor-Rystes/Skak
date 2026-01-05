<script lang="ts">
    import { fly } from "svelte/transition";

    interface Position {
        x: number;
        y: number;
    }

    let arr: Position[] = [];

    let start = 3000;
    let after = 1000;

    setInterval(() => {
        arr.push({ x: 0, y: 0 });
        arr = arr;

        setTimeout(() => {
            arr.pop();
            arr = arr;
        }, after);
    }, start);
</script>

{#each arr as _}
    <div
        in:fly={{ duration: 1500, y: 100 }}
        out:fly={{ duration: 1000, y: -100 }}
        class="w-[50px] h-[50px] bg-primary-300"
    ></div>
{/each}
