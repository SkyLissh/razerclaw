import { mutationOptions, QueryClient, queryOptions } from "@tanstack/svelte-query";
import { brightnessService, type BrightnessService } from "./service";

const brightnessKeys = {
  bySerial: (serial: string) => ["brightness", serial] as const,
};

export function brightnessQuery(serial: string, service: BrightnessService = brightnessService) {
  return queryOptions({
    queryKey: brightnessKeys.bySerial(serial),
    queryFn: () => service.getBrightness(serial),
  });
}

export function updateBrightnessMutation(
  client: QueryClient,
  serial: string,
  service: BrightnessService = brightnessService
) {
  return mutationOptions({
    mutationFn: (value: number) => service.updateBrightness(serial, value),

    onSuccess: () => {
      // client.invalidateQueries({ queryKey: brightnessKeys.bySerial(serial) });
    },
  });
}
