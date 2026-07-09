<script lang="ts">
  import DynoChart from "../charts/DynoChart.svelte";
  import { powerSeries, torqueSeries } from "../charts/series";
  import type { CurrentRun } from "../types";

  let { current }: { current: CurrentRun } = $props();

  const power = $derived(powerSeries(current.channels));
  const torque = $derived(torqueSeries(current.channels));
  const r = $derived(current.results);
</script>

<section class="infobox">
  <div class="info-title">{current.title} · {current.date ?? "no date"}</div>
  <div class="info-grid">
    <div><span>Pnim</span>{r.pnimKw ?? "—"} kW</div>
    <div><span>Paine</span>{r.pressureHpa ?? "—"} hPa</div>
    <div><span>Lämp.</span>{r.tempC ?? "—"} °C</div>
    {#if r.rpmRaw != null}<div><span>rpm*</span>{r.rpmRaw}</div>{/if}
  </div>
  <p class="note">
    Curves show raw <code>i16</code> samples pending axis calibration (the
    emulator-oracle pass maps sample→km/h and raw→kW/Nm).
  </p>
</section>

<section class="charts">
  <div class="chart-card">
    <h3>Power — Moottoriteho</h3>
    {#if power.length}
      <DynoChart series={power} yLabel="raw i16" xLabel="sample →" />
    {:else}
      <p class="muted">No channel data in this run.</p>
    {/if}
  </div>
  <div class="chart-card">
    <h3>Torque — Vääntömomentti</h3>
    {#if torque.length}
      <DynoChart series={torque} yLabel="raw i16" xLabel="sample →" />
    {:else}
      <p class="muted">No channel data in this run.</p>
    {/if}
  </div>
</section>
