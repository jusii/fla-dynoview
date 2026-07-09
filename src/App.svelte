<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import * as api from "./lib/api";
  import type { DecodedRun, ImageSummary, RunEntry } from "./lib/types";
  import DynoChart from "./lib/charts/DynoChart.svelte";
  import type { Series } from "./lib/charts/chart-types";

  const MAGENTA = "#d94fd9";
  const GREEN = "#35d43a";
  const ORANGE = "#ff7a4a";
  const CYAN = "#49c7ff";

  let image = $state<ImageSummary | null>(null);
  let selectedPath = $state<string | null>(null);
  let run = $state<DecodedRun | null>(null);
  let busy = $state(false);
  let err = $state<string | null>(null);

  async function pickImage() {
    err = null;
    const path = await open({
      filters: [{ name: "Floppy image", extensions: ["img", "IMG"] }],
    });
    if (typeof path !== "string") return;
    busy = true;
    try {
      image = await api.openImage(path);
      selectedPath = null;
      run = null;
    } catch (e) {
      err = String(e);
    } finally {
      busy = false;
    }
  }

  async function pickErg() {
    err = null;
    const path = await open({
      filters: [{ name: "ERG run file", extensions: ["erg", "ERG"] }],
    });
    if (typeof path !== "string") return;
    busy = true;
    try {
      run = await api.openErgFile(path);
      image = null;
      selectedPath = null;
    } catch (e) {
      err = String(e);
    } finally {
      busy = false;
    }
  }

  async function selectRun(r: RunEntry) {
    if (!image) return;
    selectedPath = r.path;
    busy = true;
    err = null;
    try {
      run = await api.readErgFromImage(image.imagePath, r.path);
    } catch (e) {
      err = String(e);
      run = null;
    } finally {
      busy = false;
    }
  }

  // Auto-open a file passed on the command line ("open with" / CLI).
  onMount(async () => {
    try {
      const p = await api.initialPath();
      if (!p) return;
      busy = true;
      if (p.toLowerCase().endsWith(".erg")) {
        run = await api.openErgFile(p);
      } else {
        image = await api.openImage(p);
        const first = image.runs.find((r) => !r.deleted) ?? image.runs[0];
        if (first) await selectRun(first);
      }
    } catch (e) {
      err = String(e);
    } finally {
      busy = false;
    }
  });

  const powerSeries = $derived<Series[]>(
    run
      ? [
          { values: run.channels.ch0, color: MAGENTA, label: "Engine power (ch0)" },
          { values: run.channels.ch1, color: GREEN, label: "Wheel-loss (ch1)" },
        ].filter((s) => s.values.length)
      : [],
  );

  const torqueSeries = $derived<Series[]>(
    run
      ? [
          { values: run.channels.ch2, color: ORANGE, label: "Torque (ch2)" },
          { values: run.channels.ch3, color: CYAN, label: "RPM (ch3)" },
        ].filter((s) => s.values.length)
      : [],
  );
</script>

<div class="app">
  <header class="toolbar">
    <div class="brand">FLA&nbsp;Dynoview</div>
    <button onclick={pickImage} disabled={busy}>Open .img…</button>
    <button onclick={pickErg} disabled={busy}>Open .ERG…</button>
    <div class="spacer"></div>
    {#if image}
      <div class="shop">🏁 {image.shopName || "(no shop name)"}</div>
    {/if}
  </header>

  {#if err}
    <div class="error">⚠ {err}</div>
  {/if}

  <div class="body">
    <aside class="sidebar">
      {#if image}
        <div class="side-head">
          {image.runs.length} run{image.runs.length === 1 ? "" : "s"}
        </div>
        <ul class="runlist">
          {#each image.runs as r (r.path)}
            <li>
              <button
                class="runitem"
                class:active={selectedPath === r.path}
                class:deleted={r.deleted}
                onclick={() => selectRun(r)}
              >
                <span class="rname">{r.name}</span>
                <span class="rdate">{r.date ?? "—"}</span>
                {#if r.deleted}<span class="tag">deleted</span>{/if}
              </button>
            </li>
          {/each}
        </ul>
      {:else}
        <div class="empty">
          <p>Open a Bosch FLA&nbsp;203 floppy image (<code>.img</code>) to browse its
          dyno runs, or open a single <code>.ERG</code> file.</p>
        </div>
      {/if}
    </aside>

    <main class="content">
      {#if run}
        <section class="infobox">
          <div class="info-title">
            {selectedPath ?? "ERG file"} · {run.date ?? "no date"}
          </div>
          <div class="info-grid">
            <div><span>Pnim</span>{run.results.pnimKw ?? "—"} kW</div>
            <div><span>Paine</span>{run.results.pressureHpa ?? "—"} hPa</div>
            <div><span>Lämp.</span>{run.results.tempC ?? "—"} °C</div>
            <div><span>Channels</span>{run.numChannels}</div>
            <div><span>Size</span>{run.size} B</div>
            <div><span>SHA-256</span><code>{run.sha256.slice(0, 12)}…</code></div>
          </div>
          <p class="note">
            Curves show raw <code>i16</code> samples pending axis calibration
            (the emulator-oracle pass maps sample→km/h and raw→kW/Nm).
          </p>
        </section>

        <section class="charts">
          <div class="chart-card">
            <h3>Power — Moottoriteho</h3>
            {#if powerSeries.length}
              <DynoChart series={powerSeries} yLabel="raw i16" xLabel="sample →" />
            {:else}
              <p class="muted">No channel data in this run.</p>
            {/if}
          </div>
          <div class="chart-card">
            <h3>Torque — Vääntömomentti</h3>
            {#if torqueSeries.length}
              <DynoChart series={torqueSeries} yLabel="raw i16" xLabel="sample →" />
            {:else}
              <p class="muted">No channel data in this run.</p>
            {/if}
          </div>
        </section>
      {:else}
        <div class="placeholder">
          {#if busy}
            Loading…
          {:else}
            <div class="ph-inner">
              <div class="ph-logo">📈</div>
              <p>Select a run to view its power and torque curves.</p>
            </div>
          {/if}
        </div>
      {/if}
    </main>
  </div>
</div>
