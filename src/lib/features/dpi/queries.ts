import { mutationOptions, QueryClient, queryOptions } from "@tanstack/svelte-query";

import { dpiService, type DpiService } from "./service";

const dpiKeys = {
  dpi: (serial: string) => ["dpi", serial] as const,
};

export function dpiQuery(serial: string, service: DpiService = dpiService) {
  return queryOptions({
    queryKey: dpiKeys.dpi(serial),
    queryFn: () => service.getDpi(serial),
  });
}

export function updateDpiMutation(client: QueryClient, service: DpiService = dpiService) {
  return mutationOptions({
    mutationFn: ({ serial, dpi }: { serial: string; dpi: number }) =>
      service.updateDpi(serial, dpi),
    onSuccess: (_, { serial }) => {
      client.invalidateQueries({ queryKey: dpiKeys.dpi(serial) });
    },
  });
}
