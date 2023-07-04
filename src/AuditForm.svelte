<script lang="ts">
  import type { AuditData } from "./shared";

  export let data: AuditData = {};

  // TODO The unknown option should delete the entry entirely
</script>

<fieldset>
  <div>
    <label>
      Funding round:
      <select bind:value={data.funding_round}>
        <option>unknown</option>
        <option value="ATF2">ATF2</option>
        <option value="ATF3">ATF3</option>
      </select>
    </label>
  </div>

  <div>
    <label>
      LTN 120 compliant:
      <select bind:value={data.ltn120_compliant}>
        <option>unknown</option>
        <option value="yes">yes</option>
        <option value="no">no</option>
      </select>
    </label>
  </div>
</fieldset>
