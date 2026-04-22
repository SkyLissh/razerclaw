<script lang="ts">
  import { createPower } from "../stores.svelte";
  import BatteryBadge from "./battery-badge.svelte";

  const { serial }: { serial: string } = $props();

  const { power } = createPower(() => serial);
</script>

<BatteryBadge isCharging={power.data?.is_charging ?? false} percentage={power.data?.battery ?? 0} />
