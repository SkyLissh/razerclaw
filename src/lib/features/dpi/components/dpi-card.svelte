<script lang="ts">
  import { m } from "$lib/paraglide/messages";

  import { ToggleGroup, ToggleGroupItem } from "$lib/components/ui/toggle-group";

  type Props = {
    dpi: number;
    stages: number[];
    onUpdateDpi: (dpi: number) => void;
  };

  const { dpi, stages, onUpdateDpi }: Props = $props();

  let current = $derived(dpi.toString());
</script>

<div class="col-span-1 flex min-h-40 flex-col gap-6 rounded bg-card p-4">
  <div class="flex items-center justify-between">
    <p class="text-lg font-semibold">{m.dpi()}</p>
    <p class="text-2xl font-bold text-primary">{dpi}</p>
  </div>

  <ToggleGroup
    bind:value={current}
    type="single"
    variant="outline"
    spacing={2}
    size="lg"
    class="w-full"
  >
    {#each stages as stage}
      <ToggleGroupItem
        value={stage.toString()}
        onclick={() => onUpdateDpi(stage)}
        class="flex-1 rounded text-sm font-semibold data-[state=on]:bg-primary data-[state=on]:text-primary-foreground"
      >
        {stage}
      </ToggleGroupItem>
    {/each}
  </ToggleGroup>
</div>
