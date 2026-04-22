<script lang="ts">
  import { m } from "$lib/paraglide/messages";

  import { BatteryCharging, BatteryFull, BatteryLow, BatteryMedium } from "@lucide/svelte";
  import { createBatteryStatus } from "../stores.svelte";

  type Props = {
    percentage: number;
    isCharging?: boolean;
  };

  const { percentage, isCharging = false }: Props = $props();

  const { batteryStatus } = createBatteryStatus(() => ({
    percentage,
    isCharging,
  }));
</script>

<div
  data-status={batteryStatus}
  class="flex items-center gap-2 data-[status=charging]:text-accent data-[status=discharging]:text-secondary data-[status=full]:text-primary data-[status=low]:text-destructive [&_svg]:size-5"
>
  {#if batteryStatus === "full"}
    <BatteryFull />
  {:else if batteryStatus === "charging"}
    <BatteryCharging />
  {:else if batteryStatus === "discharging"}
    <BatteryMedium />
  {:else if batteryStatus === "low"}
    <BatteryLow />
  {/if}
  <span class="text-sm">{m.battery_charge({ charge: percentage })}</span>
</div>
