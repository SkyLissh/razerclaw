import { invoke } from "@tauri-apps/api/core";

import { PowerSchema, type Power } from "./schemas";

export interface PowerService {
  getPowerInfo(serial: string): Promise<Power>;
  updateBatteryThreshold(serial: string, threshold: number): Promise<void>;
  updateIdleTime(serial: string, idleTime: number): Promise<void>;
}

export const powerService: PowerService = {
  async getPowerInfo(serial: string): Promise<Power> {
    try {
      const power = await invoke<Power>("get_power", { serial });
      return PowerSchema.parse(power);
    } catch (error) {
      console.error(`Error fetching power info for device with serial ${serial}:`, error);
      throw error;
    }
  },
  async updateBatteryThreshold(serial: string, threshold: number): Promise<void> {
    try {
      const parsedThreshold = PowerSchema.shape.low_battery_threshold.parse(threshold);
      await invoke("set_low_battery_threshold", { serial, threshold: parsedThreshold });
    } catch (error) {
      console.error(`Error setting low battery threshold for device with serial ${serial}:`, error);
      throw error;
    }
  },
  async updateIdleTime(serial: string, idleTime: number): Promise<void> {
    try {
      const parsedIdleTime = PowerSchema.shape.idle_time.parse(idleTime);
      await invoke("set_idle_time", { serial, idle_time: parsedIdleTime });
    } catch (error) {
      console.error(`Error setting idle time for device with serial ${serial}:`, error);
      throw error;
    }
  },
};
