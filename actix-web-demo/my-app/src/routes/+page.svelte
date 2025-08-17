<script lang="ts">
  import { onMount } from "svelte";

  let events: string[] = [];

  onMount(() => {
    new EventSource("/api/v1/sse/ping").addEventListener("ping", (event) => {
      console.log(event);
      events = [...events, event.data];
    });
  });
</script>

<h1>Welcome to SvelteKit</h1>
<p>
  Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the
  documentation
</p>

<h2>Events</h2>
{#each events as event, index}
  <p>{event}</p>
{/each}
