export const razerQueryKeys = {
  devices: ["razer", "devices"] as const,
  device: (serial: string) => ["razer", "device", serial] as const,
};
