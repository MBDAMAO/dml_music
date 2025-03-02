import type { App } from "vue";
import { install as piniaInstall } from "./pinia";
import { install as routerInstall } from "./router";
export function registerPlugins(app: App) {
    piniaInstall(app);
    routerInstall(app);
}
