import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
import {
  getPowerInfoQuery,
  updateBatteryThresholdMutation,
  updateIdleTimeMutation,
} from "./queries";

export type BatteryStatus = "charging" | "discharging" | "full" | "low";

export function createPower(serial: () => string) {
  const queryClient = useQueryClient();

  const power = createQuery(() => getPowerInfoQuery(serial()));
  const updateBatteryThreshold = createMutation(() =>
    updateBatteryThresholdMutation(queryClient, serial())
  );
  const updateIdleTime = createMutation(() => updateIdleTimeMutation(queryClient, serial()));

  return {
    power,
    updateBatteryThreshold,
    updateIdleTime,
  };
}

export function createBatteryStatus(info: () => { percentage: number; isCharging: boolean }) {
  const batteryStatus = $derived.by(() => {
    if (info().isCharging) {
      return "charging";
    } else if (info().percentage >= 80) {
      return "full";
    } else if (info().percentage >= 20) {
      return "discharging";
    } else if (info().percentage > 0) {
      return "low";
    } else {
      return "charging";
    }
  });

  return {
    get batteryStatus() {
      return batteryStatus;
    },
  };
}
