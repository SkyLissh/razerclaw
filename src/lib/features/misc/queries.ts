import { mutationOptions, QueryClient, queryOptions } from "@tanstack/svelte-query";
import { miscService, type MiscService } from "./service";

type UpdatePollRateVariables = {
  serial: string;
  pollRate: number;
};

export const miscKeys = {
  devices: ["devices"] as const,
  deviceDetail: (serial: string) => ["device", serial] as const,
};

export function devicesQuery(service: MiscService = miscService) {
  return queryOptions({
    queryKey: miscKeys.devices,
    queryFn: async () => {
      const devices = await service.getDevices();

      return devices.map((device) => ({
        name: device.name,
        serial: device.serial,
        type: device.type,
      }));
    },
  });
}

export function deviceDetailQuery(serial: string, service: MiscService = miscService) {
  return queryOptions({
    queryKey: miscKeys.deviceDetail(serial),
    queryFn: () => service.getDeviceBySerial(serial),
  });
}

export function updatePollRateMutation(client: QueryClient, service: MiscService = miscService) {
  return mutationOptions({
    mutationFn: ({ serial, pollRate }: UpdatePollRateVariables) =>
      service.updatePollRate(serial, pollRate),
    onSuccess: (_, { serial }) => {
      client.invalidateQueries({ queryKey: miscKeys.deviceDetail(serial) });
    },
  });
}

export function suspendMutation(client: QueryClient, service: MiscService = miscService) {
  return mutationOptions({
    mutationFn: ({ serial }: { serial: string }) => service.suspend(serial),
    onSuccess: (_, { serial }) => {
      client.invalidateQueries({ queryKey: miscKeys.deviceDetail(serial) });
    },
  });
}

export function resumeMutation(client: QueryClient, service: MiscService = miscService) {
  return mutationOptions({
    mutationFn: ({ serial }: { serial: string }) => service.resume(serial),
    onSuccess: (_, { serial }) => {
      client.invalidateQueries({ queryKey: miscKeys.deviceDetail(serial) });
    },
  });
}
