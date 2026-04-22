import { invoke } from "@tauri-apps/api/core";
import { BrightnessSchema, type Brightness } from "./schemas";

export interface BrightnessService {
  getBrightness(serial: string): Promise<number>;
  updateBrightness(serial: string, value: number): Promise<void>;
}

export const brightnessService: BrightnessService = {
  async getBrightness(serial: string): Promise<number> {
    try {
      const brightness = await invoke<Brightness>("get_brightness", { serial });

      return BrightnessSchema.parse(brightness).brightness;
    } catch (error) {
      console.error(`Error fetching brightness for device with serial ${serial}:`, error);
      throw error;
    }
  },

  async updateBrightness(serial: string, value: number): Promise<void> {
    try {
      const brightness = BrightnessSchema.parse({ brightness: value });
      await invoke("set_brightness", { serial, brightness });
    } catch (error) {
      console.error(`Error setting brightness for device with serial ${serial}:`, error);
      throw error;
    }
  },
};
