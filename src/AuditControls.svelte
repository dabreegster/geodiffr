<script lang="ts">
  import { auditData } from "./shared";

  let dialog;
</script>

<button on:click={() => dialog.showModal()}>Debug audit data</button>
<button on:click={() => ($auditData = {})}>Reset all data</button>

<p>{Object.keys($auditData).length} objects have been audited</p>

<dialog bind:this={dialog}>
  <button on:click={() => dialog.close()}>Close</button>
  <pre>{JSON.stringify($auditData, null, "  ")}</pre>
</dialog>
