import * as clipboard from "@tauri-apps/plugin-clipboard-manager";

export type CopyState = "idle" | "copied" | "error";

export function createCopySerial() {
  let state = $state<CopyState>("idle");

  async function copy(serial: string) {
    try {
      await clipboard.writeText(serial);
      state = "copied";

      setTimeout(() => {
        state = "idle";
      }, 2000);
    } catch (error) {
      console.error("Failed to copy serial to clipboard:", error);
      state = "error";
    }
  }

  return {
    get state() {
      return state;
    },
    copy,
  };
}
