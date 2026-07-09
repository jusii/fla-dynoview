<script lang="ts">
  import DynoChart from "../charts/DynoChart.svelte";
  import { powerSeries, torqueSeries } from "../charts/series";
  import type { CurrentRun } from "../types";

  let { current }: { current: CurrentRun } = $props();

  const power = $derived(powerSeries(current.channels, current.results.kDin));
  const torque = $derived(torqueSeries(current.channels, current.results.kDin));
  const r = $derived(current.results);

  const f1 = (v: number | null | undefined) => (v == null ? "—" : v.toFixed(1));
</script>

<section class="infobox">
  <div class="info-title">{current.title} · {current.date ?? "no date"}</div>
  <div class="info-grid">
    <div><span>Pmax</span>{f1(r.pmaxKw)} kW</div>
    <div><span>@ n</span>{r.rpmAtPmax ?? "—"} rpm</div>
    <div><span>Pyörä</span>{f1(r.ppyoraKw)} kW</div>
    <div><span>Häviö</span>{f1(r.phavioKw)} kW</div>
    <div><span>Mmax</span>{f1(r.mmaxNm)} Nm</div>
    <div><span>@ n</span>{r.rpmAtMmax ?? "—"} rpm</div>
    <div><span>k (DIN)</span>{r.kDin == null ? "—" : r.kDin.toFixed(3)}</div>
    <div><span>Pnim</span>{r.pnimKw ?? "—"} kW</div>
    <div><span>Paine</span>{r.pressureHpa ?? "—"} hPa</div>
    <div><span>Lämp.</span>{r.tempC ?? "—"} °C</div>
  </div>
</section>

<section class="charts">
  <div class="chart-card">
    <h3>Power — Moottoriteho</h3>
    {#if power.length}
      <DynoChart series={power} yLabel="kW" xLabel="sweep →" />
    {:else}
      <p class="muted">No power curve in this run.</p>
    {/if}
  </div>
  <div class="chart-card">
    <h3>Torque — Vääntömomentti</h3>
    {#if torque.length}
      <DynoChart series={torque} yLabel="Nm" xLabel="sweep →" />
    {:else}
      <p class="muted">No torque curve in this run.</p>
    {/if}
  </div>
</section>
