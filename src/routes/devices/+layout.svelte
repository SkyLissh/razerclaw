<script lang="ts">
  import { page } from "$app/state";
  import type { LayoutProps } from "./$types";

  import { LineSquiggle } from "@lucide/svelte";

  import { Sidebar } from "$lib/components/blocks/sidebar";
  import { createDevicesList } from "$lib/features/misc/stores";

  let { children }: LayoutProps = $props();

  const { devices } = createDevicesList();
  const selected = $derived(page.params.serial ?? null);
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
  <div class="flex h-full overflow-auto">
    <Sidebar
      {selected}
      devices={devices.data}
      refreshing={devices.isFetching}
      onRefresh={() => devices.refetch()}
    />
    {@render children()}
  </div>
{/if}
