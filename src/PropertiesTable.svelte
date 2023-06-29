<script lang="ts">
  export let properties;
</script>

<table>
  <tbody>
    {#each Object.entries(properties) as [key, value]}
      <tr><td>{key}</td><td>{value}</td></tr>
    {/each}
  </tbody>
</table>
