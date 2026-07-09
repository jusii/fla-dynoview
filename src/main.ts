import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";
import { initSettings } from "./lib/settings.svelte";

// Load the persisted language / unit settings before first paint.
await initSettings();

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
