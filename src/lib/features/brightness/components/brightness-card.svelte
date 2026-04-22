<script lang="ts">
  import { m } from "$lib/paraglide/messages";

  import { Slider } from "$lib/components/ui/slider";

  import { Sun } from "@lucide/svelte";

  type Props = {
    brightness: number;
    onUpdateBrightness: (brightness: number) => void;
  };

  const { brightness, onUpdateBrightness }: Props = $props();

  let current = $derived(brightness);
</script>

<div class="col-span-1 flex min-h-40 flex-col gap-12 rounded bg-card p-4">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <Sun class="size-6 text-primary" />
      <p class="text-lg font-semibold">{m.brightness()}</p>
    </div>
    <p class="text-2xl font-bold text-primary">{current} %</p>
  </div>

  <Slider
    type="single"
    bind:value={current}
    min={0}
    max={100}
    onValueChange={(value) => onUpdateBrightness(value)}
  />
</div>
