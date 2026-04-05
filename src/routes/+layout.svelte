<script lang="ts">
  import "./layout.css";

  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";

  import { Titlebar } from "$lib/components/blocks/titlebar";
  import { setServicesContext } from "$lib/services";
  import { RazerService } from "$lib/services/razer";

  const { children } = $props();

  const queryClient = new QueryClient();

  setServicesContext({
    razer: new RazerService(),
  });
</script>

<QueryClientProvider client={queryClient}>
  <div class="flex h-full flex-col">
    <Titlebar />

    {@render children()}
  </div>
  <SvelteQueryDevtools />
</QueryClientProvider>
