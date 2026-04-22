<script lang="ts">
  import "./layout.css";

  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";

  import { Titlebar } from "$lib/components/blocks/titlebar";
  import * as Tooltip from "$lib/components/ui/tooltip";

  const { children } = $props();

  const queryClient = new QueryClient();
</script>

<QueryClientProvider client={queryClient}>
  <div class="flex h-full flex-col">
    <Titlebar />

    <Tooltip.Provider>
      {@render children()}
    </Tooltip.Provider>
  </div>
  <SvelteQueryDevtools />
</QueryClientProvider>
