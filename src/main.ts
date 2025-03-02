import { createApp } from "vue";
import App from "./App.vue";

import "./assets/styles/main.css";

import { registerPlugins } from "./plugins";

const app = createApp(App);
registerPlugins(app);
app.mount("#app");
