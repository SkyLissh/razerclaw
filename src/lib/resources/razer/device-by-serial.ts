import { mutationOptions, QueryClient, queryOptions } from "@tanstack/svelte-query";

import type { RazerService } from "$lib/services/razer";
import { razerQueryKeys } from "./keys";

type UpdatePollRateVariables = {
  serial: string;
  pollRate: number;
};

export function deviceBySerialResource(service: RazerService, serial: () => string) {
  return {
    getDeviceBySerial: () =>
      queryOptions({
        queryKey: razerQueryKeys.device(serial()),
        queryFn: () => service.getDeviceBySerial(serial()),
      }),
    updatePollRate: (queryClient: QueryClient) =>
      mutationOptions({
        mutationFn: async ({ serial, pollRate }: UpdatePollRateVariables) => {
          await service.updatePollRate(serial, pollRate);
        },
        onSuccess: async (_, variables) => {
          await queryClient.invalidateQueries({
            queryKey: razerQueryKeys.device(variables.serial),
          });
        },
      }),
  };
}
