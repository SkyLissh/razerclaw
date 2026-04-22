<script lang="ts">
  import type { PageProps } from "./$types";

  import { m } from "$lib/paraglide/messages";

  import { LineSquiggle } from "@lucide/svelte";

  import { DeviceBrightnessCard } from "$lib/features/brightness/components";
  import { DeviceDpiCard } from "$lib/features/dpi/components";
  import { HeaderInfo, PollRateCard } from "$lib/features/misc/components";
  import { DeviceBattery } from "$lib/features/power/components";

  import { createDeviceDetail } from "$lib/features/misc/stores";

  const { params }: PageProps = $props();

  const { device, updatePollRate, supportsPollRate, hasDpi, hasBrightness, hasPower } =
    createDeviceDetail(() => params.serial);
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
  <main class="container mx-auto space-y-4 p-8 xl:max-w-300">
    <HeaderInfo name={device.data.name} serial={device.data.serial} image={device.data.image}>
      {#if hasPower()}
        <DeviceBattery serial={params.serial} />
      {/if}
    </HeaderInfo>

    <div class="relative z-10 grid grid-cols-2 gap-4">
      {#if supportsPollRate()}
        <PollRateCard
          pollRate={device.data.poll_rate!}
          supportedPollRates={device.data.supported_poll_rates!}
          onUpdatePollRate={(newPollRate) =>
            updatePollRate.mutate({ serial: params.serial, pollRate: newPollRate })}
        />
      {/if}
      {#if hasDpi()}
        <DeviceDpiCard serial={params.serial} />
      {/if}
      {#if hasBrightness()}
        <DeviceBrightnessCard serial={params.serial} />
      {/if}
    </div>
  </main>
{/if}
