import { invoke } from "@tauri-apps/api/core";
import { DeviceSummarySchema, type DeviceSummary } from "./schemas/device-summary";

export interface MiscService {
  getDevices(): Promise<DeviceSummary[]>;
  getDeviceBySerial(serial: string): Promise<DeviceSummary>;
  updatePollRate(serial: string, pollRate: number): Promise<void>;
  suspend(serial: string): Promise<void>;
  resume(serial: string): Promise<void>;
}

export const miscService: MiscService = {
  async getDevices(): Promise<DeviceSummary[]> {
    try {
      const device = await invoke("get_devices");

      return DeviceSummarySchema.array().parse(device);
    } catch (error) {
      console.error("Error fetching devices:", error);
      throw error;
    }
  },

  async getDeviceBySerial(serial: string): Promise<DeviceSummary> {
    try {
      const device = await invoke("get_device_by_serial", { serial });

      return DeviceSummarySchema.parse(device);
    } catch (error) {
      console.error(`Error fetching device with serial ${serial}:`, error);
      throw error;
    }
  },

  async updatePollRate(serial: string, pollRate: number): Promise<void> {
    try {
      await invoke("set_poll_rate", { serial, pollRate });
    } catch (error) {
      console.error(`Error updating poll rate for device with serial ${serial}:`, error);
      throw error;
    }
  },

  async suspend(serial: string): Promise<void> {
    try {
      await invoke("suspend_device", { serial });
    } catch (error) {
      console.error(`Error suspending the device with serial ${serial}:`, error);
      throw error;
    }
  },

  async resume(serial: string): Promise<void> {
    try {
      await invoke("resume_device", { serial });
    } catch (error) {
      console.error(`Error resuming the device with serial ${serial}:`, error);
      throw error;
    }
  },
};
