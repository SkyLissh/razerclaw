<script lang="ts">
  import { createBrightness } from "../stores";
  import BrightnessCard from "./brightness-card.svelte";

  type Props = {
    serial: string;
  };

  const { serial }: Props = $props();

  const { brightness, updateBrightness } = createBrightness(() => serial);
</script>

<BrightnessCard
  brightness={brightness.data ?? 0}
  onUpdateBrightness={(value) => updateBrightness(value)}
/>
