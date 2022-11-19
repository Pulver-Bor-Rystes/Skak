<script lang="ts">
  import { browser } from '$app/environment'
  import { socket } from "../stores/state"

  import Auth from "./auth.svelte"

  if (browser) {
    /* @ts-ignore */
    $socket = io()
  }
</script>


<Auth/>