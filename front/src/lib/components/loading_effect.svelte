<script lang="ts">
    let { start = "", character = '.', max = 4 } = $props();
    
    let timer = 500;
    let progress: string[] = start.split("");
    let progress_text: string = $state("");
    let erase = false;

    setInterval(() => {
        if (erase) {
            progress.pop();
            erase = progress.length != 0;
        }
        else {
            if (progress.length < max)
                progress.push(character);
            erase = progress.length >= max;
        }

        progress_text = progress.join("");
    }, timer);
</script>



{ progress_text || start }