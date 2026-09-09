<script lang="ts">
  import { goto } from "$app/navigation";
  import { getDb } from "$lib/db";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  let { name, kind, error, currentId }: { name:string; kind:"product"|"supplier"; error:string; currentId?:number } = $props();
  let existing = $state<{id:number;archived:number}|null>(null);
  $effect(() => {
    const key=name.trim().split(/\s+/u).join(" ").normalize("NFC").toLowerCase().normalize("NFC");
    const message=error; const id=currentId; const table=kind==="product"?"products":"suppliers";
    let cancelled=false; existing=null;
    if (message.includes("Ya existe")) {
      getDb().then(db=>db.select<{id:number;archived:number}[]>(`SELECT id,archived FROM ${table} WHERE name_key=? AND id<>?`,[key,id??-1]))
        .then(rows=>{if(!cancelled) existing=rows[0]??null;}).catch(()=>{});
    }
    return ()=>{cancelled=true;};
  });
  function open() {
    if(!existing) return;
    modalService.closeAll();
    goto(`/${kind==="product"?"products":"suppliers"}?id=${existing.id}&archived=${existing.archived}`);
  }
</script>
{#if existing}<Button variant="secondary" onclick={open}>Ver registro existente{existing.archived ? " (archivado)" : ""}</Button>{/if}
