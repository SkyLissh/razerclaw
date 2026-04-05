<script lang="ts">
  import type { PageProps } from "./$types";

  import { writeText } from "@tauri-apps/plugin-clipboard-manager";

  import { m } from "$lib/paraglide/messages";

  import { BatteryFull, BatteryLow, BatteryMedium, Files, LineSquiggle } from "@lucide/svelte";

  import { Button } from "$lib/components/ui/button";
  import { Slider } from "$lib/components/ui/slider";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";

  import { createDeviceBySerial } from "$lib/composables/device-by-serial";
  import { formatNumber } from "$lib/utils";

  const { params }: PageProps = $props();

  const { device, updatePollRate } = createDeviceBySerial(() => params.serial);

  let pollRateIndex = $derived.by(() => {
    let pollRate = device.data?.poll_rate;
    if (!pollRate) return 0;

    const index = device.data?.supported_poll_rates?.indexOf(pollRate) ?? 0;
    return index >= 0 ? index : 0;
  });

  const pollRate = $derived.by(() => {
    if (!device.data?.supported_poll_rates) return 0;
    return device.data?.supported_poll_rates[pollRateIndex];
  });

  const power = $derived.by(() => {
    return device.data?.power ?? null;
  });
  const battery_status = $derived.by(() => {
    if (!power) return null;

    if (power.is_charging) return "charging";
    if (power.battery >= 70) return "full";
    if (power.battery >= 30) return "draining";
    return "low";
  });

  const dpi = $derived.by(() => {
    return device.data?.dpi ?? null;
  });

  async function onUpdatePollRate() {
    if (!device.data?.supported_poll_rates) return;

    const newPollRate = device.data?.supported_poll_rates[pollRateIndex];
    await updatePollRate.mutateAsync({ serial: device.data.serial, pollRate: newPollRate });
  }
</script>

{#if device.isPending}
  <main class="container flex h-full items-center justify-center p-4">
    <span class="sr-only">{m.loading()}</span>
    <LineSquiggle class="size-24 animate-pulse text-primary" />
  </main>
{:else if device.isError}
  <main class="container flex h-full items-center justify-center p-4">
    <span class="text-sm text-destructive">Failed to load device.</span>
  </main>
{:else}
  <main class="container mx-auto space-y-4 p-4 xl:max-w-300">
    <div class="mt-6 flex items-center justify-between gap-4">
      <div>
        <h2 class="text-2xl font-bold">{device.data.name}</h2>
        {#if power && battery_status}
          <div
            data-battery={battery_status}
            class="flex items-center gap-2 text-sm font-medium data-[battery=draining]:text-yellow-500 data-[battery=full]:text-primary data-[battery=low]:text-destructive"
          >
            {#if battery_status === "full"}
              <BatteryFull />
            {:else if battery_status === "draining"}
              <BatteryMedium />
            {:else}
              <BatteryLow />
            {/if}

            <p>{m.battery_charge({ charge: power.battery.toFixed(0) })}</p>
          </div>
        {/if}
        <div class="flex items-center gap-1">
          <p class="text-sm text-muted-foreground">{device.data.serial}</p>
          <Button variant="ghost" size="icon" onclick={() => writeText(device.data.serial)}>
            <span class="sr-only">{m.copy_to_clipboard()}</span>
            <Files class="text-muted-foreground" />
          </Button>
        </div>
      </div>
      <div class="relative mr-6 flex items-center justify-center">
        <div class="absolute size-40 rounded-full bg-primary/15 blur-2xl"></div>
        <div class="absolute size-16 rounded-full bg-primary/70 blur-xl"></div>
        <img
          src={device.data.image}
          alt={device.data.name}
          class="relative size-32 object-contain"
        />
      </div>
    </div>

    <div class="relative z-10 grid grid-cols-2 gap-4">
      {#if device.data.supported_poll_rates && device.data.poll_rate}
        <div class="col-span-1 flex min-h-40 flex-col gap-6 rounded bg-card p-4">
          <div class="flex items-center justify-between">
            <p class="text-lg font-semibold">Poll Rate:</p>
            <p class="text-2xl font-bold text-primary">{pollRate} Hz</p>
          </div>

          <div class="flex flex-col gap-2">
            <Slider
              type="single"
              bind:value={pollRateIndex}
              min={0}
              max={device.data.supported_poll_rates.length - 1}
              tickLabels={device.data.supported_poll_rates.map(formatNumber)}
              onValueCommit={onUpdatePollRate}
            />
          </div>
        </div>
      {/if}

      {#if dpi}
        <div class="col-span-1 flex min-h-40 flex-col gap-6 rounded bg-card p-4">
          <p class="text-lg font-semibold">DPI:</p>

          <ToggleGroup.Root type="single" spacing={2} variant="outline" size="lg" class="w-full">
            {#each dpi.stages.stages as stage}
              <ToggleGroup.Item
                value={stage.x.toString()}
                class="flex-1 rounded-sm data-[state=on]:bg-primary data-[state=on]:text-primary-foreground"
              >
                <p class="text-sm">{stage.x.toString()}</p>
              </ToggleGroup.Item>
            {/each}
          </ToggleGroup.Root>
        </div>
      {/if}
    </div>
  </main>
{/if}
