import { writable, type Writable } from "svelte/store";

export interface AuditData {
  funding_round: undefined | "ATF2" | "ATF3";
  ltn120_compliant: undefined | "yes" | "no";
  // Also timestamp and person who audited it
}

let localStorageData = window.localStorage.getItem("geodiffr");
export const auditData: Writable<{ [id: number]: AuditData }> = writable(
  localStorageData ? JSON.parse(localStorageData) : {}
);
auditData.subscribe((value) =>
  window.localStorage.setItem("geodiffr", JSON.stringify(value))
);
