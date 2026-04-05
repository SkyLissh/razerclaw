<script lang="ts">
  import type { LayoutProps } from "./$types";

  import { LineSquiggle } from "@lucide/svelte";

  import { Sidebar } from "$lib/components/blocks/sidebar";

  import { createDevices } from "$lib/composables/devices";

  let { children }: LayoutProps = $props();

  const { devices } = createDevices();
  let selected = $derived(devices?.data?.at(0) ?? null);
</script>

{#if devices.isPending}
  <div class="flex h-full items-center justify-center">
    <span class="sr-only">Loading devices...</span>
    <LineSquiggle class="size-24 animate-pulse text-primary" />
  </div>
{:else if devices.isError}
  <div class="flex h-full items-center justify-center">
    <span class="text-sm text-destructive">Failed to load devices.</span>
  </div>
{:else if devices.data?.length === 0}
  <div class="flex h-full items-center justify-center">
    <span class="text-sm text-muted">No devices found.</span>
  </div>
{:else}
  <div class="flex h-full gap-12 overflow-auto">
    <Sidebar
      selected={selected?.serial ?? null}
      devices={devices.data.map((d) => ({
        name: d.name,
        serial: d.serial,
        type: d.type,
      }))}
      onSelect={(serial) => (selected = devices.data.find((d) => d.serial === serial) ?? null)}
    />
    {@render children()}
  </div>
{/if}
