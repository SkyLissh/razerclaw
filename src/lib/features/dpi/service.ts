import { invoke } from "@tauri-apps/api/core";

import { DeviceDpiSchema, type DeviceDpi, type DpiStage } from "./schemas";

export interface DpiService {
  getDpi(serial: string): Promise<DeviceDpi>;
  updateDpi(serial: string, dpi: number): Promise<void>;
  updateDpiStages(serial: string, active: number, stages: DpiStage[]): Promise<void>;
}

export const dpiService: DpiService = {
  async getDpi(serial: string): Promise<DeviceDpi> {
    try {
      const result = await invoke<DeviceDpi>("get_dpi", { serial });
      return DeviceDpiSchema.parse(result);
    } catch (error) {
      console.error("Error getting DPI:", error);
      throw error;
    }
  },
  async updateDpi(serial: string, dpi: number): Promise<void> {
    try {
      await invoke("set_dpi", { serial, dpi });
    } catch (error) {
      console.error(`Error setting DPI for device with serial ${serial}:`, error);
      throw error;
    }
  },
  async updateDpiStages(serial: string, active: number, stages: DpiStage[]): Promise<void> {
    try {
      await invoke("set_dpi_stages", {
        serial,
        active_stage: active,
        stages,
      });
    } catch (error) {
      console.error(`Error setting DPI stages for device with serial ${serial}:`, error);
      throw error;
    }
  },
};
