import { invoke } from "@tauri-apps/api/core";

import { DeviceDetail, DeviceSummary } from "$lib/schemas/device";

export class RazerService {
  async getDevices(): Promise<DeviceSummary[]> {
    const devices = await invoke("get_devices");

    return DeviceSummary.array().parse(devices);
  }

  async getDeviceBySerial(serial: string): Promise<DeviceDetail> {
    const device = await invoke("get_device_by_serial", { serial });

    return DeviceDetail.parse(device);
  }

  async updatePollRate(serial: string, pollRate: number): Promise<void> {
    await invoke("update_poll_rate", { serial, pollRate });
  }
}
