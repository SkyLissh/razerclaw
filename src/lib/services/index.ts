import { createContext } from "svelte";
import type { RazerService } from "./razer";

export type ServicesContext = {
  razer: RazerService;
};

export const [getServicesContext, setServicesContext] = createContext<ServicesContext>();
