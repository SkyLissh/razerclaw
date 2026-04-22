import { mutationOptions, QueryClient, queryOptions } from "@tanstack/svelte-query";
import { powerService, type PowerService } from "./service";

const powerKeys = {
  bySerial: (serial: string) => ["power", serial] as const,
};

export function getPowerInfoQuery(serial: string, service: PowerService = powerService) {
  return queryOptions({
    queryKey: powerKeys.bySerial(serial),
    queryFn: () => service.getPowerInfo(serial),
  });
}

export function updateBatteryThresholdMutation(
  queryClient: QueryClient,
  serial: string,
  service: PowerService = powerService
) {
  return mutationOptions({
    mutationFn: (threshold: number) => service.updateBatteryThreshold(serial, threshold),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: powerKeys.bySerial(serial) });
    },
  });
}

export function updateIdleTimeMutation(
  queryClient: QueryClient,
  serial: string,
  service: PowerService = powerService
) {
  return mutationOptions({
    mutationFn: (idleTime: number) => service.updateIdleTime(serial, idleTime),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: powerKeys.bySerial(serial) });
    },
  });
}
