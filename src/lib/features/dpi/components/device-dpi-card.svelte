<script lang="ts">
  import { createDeviceDpi } from "../stores";
  import DpiCard from "./dpi-card.svelte";

  const { serial }: { serial: string } = $props();

  const { dpi, updateDpi } = createDeviceDpi(() => serial);

  const stages = $derived.by(() => {
    const stages = dpi.data?.stages.stages.map((stage) => stage.x) ?? [];
    return stages;
  });
</script>

<DpiCard
  dpi={dpi.data?.dpi.x ?? 0}
  {stages}
  onUpdateDpi={(dpi) => updateDpi.mutate({ serial, dpi })}
/>
