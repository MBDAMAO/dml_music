import router from "@/router";
import type { App } from "vue";
export function install(app: App) {
    app.use(router);
}
