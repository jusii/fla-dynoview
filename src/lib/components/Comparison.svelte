<script lang="ts">
  import DynoChart from "../charts/DynoChart.svelte";
  import { physical, autoCrop } from "../charts/series";
  import type { Series } from "../charts/chart-types";
  import { t, label } from "../i18n";
  import * as U from "../units";
  import type { CompareRun } from "../types";

  let { runs, onClose }: { runs: CompareRun[]; onClose: () => void } = $props();

  // Distinct, readable-on-dark hues; one per run (used for both its curves).
  const PALETTE = ["#35d43a", "#4db8ff", "#ff9d3a", "#c07dff", "#ff5d8f", "#ffe14d"];

  interface Built {
    color: string;
    title: string;
    date: string | null;
    pmax: number;
    rpmAtPmax: number;
    tmax: number;
    rpmAtTmax: number;
  }

  function argmax(a: number[]): number {
    let bi = 0;
    let bv = -Infinity;
    for (let i = 0; i < a.length; i++) if (a[i] > bv) { bv = a[i]; bi = i; }
    return bi;
  }

  const built = $derived.by(() => {
    const series: Series[] = [];
    const summary: Built[] = [];
    runs.forEach((run, i) => {
      const color = PALETTE[i % PALETTE.length];
      const p = physical(run.channels, run.kDin);
      const cr = autoCrop(run.channels, run.kDin);
      const n = p.rpm.length;
      const a = Math.max(0, Math.floor(cr.start * n));
      const b = Math.min(n, Math.ceil(cr.end * n));
      const rpm = p.rpm.slice(a, b);
      const eng = p.engineKw.slice(a, b).map(U.power);
      const tq = p.torqueNm.slice(a, b).map(U.torque);
      if (!rpm.length) return;
      series.push({ values: eng, color, label: `${i}·P`, axis: "left", rpm });
      series.push({ values: tq, color, label: `${i}·T`, axis: "right", rpm, dash: true });
      const pi = argmax(eng);
      const ti = argmax(tq);
      summary.push({
        color,
        title: run.title,
        date: run.date,
        pmax: eng[pi],
        rpmAtPmax: rpm[pi],
        tmax: tq[ti],
        rpmAtTmax: rpm[ti],
      });
    });
    return { series, summary };
  });
</script>

<section class="cmp">
  <div class="cmp-head">
    <h3>{t("app.comparison")}</h3>
    <button onclick={onClose}>{t("compare.close")}</button>
  </div>

  <div class="chart-card">
    {#if built.series.length}
      <DynoChart
        series={built.series}
        leftLabel={U.unitPower()}
        rightLabel={U.unitTorque()}
        xLabel={U.unitRpm()}
        showLegend={false}
      />
    {:else}
      <p class="muted">{t("detail.noPower")}</p>
    {/if}
  </div>

  <table class="cmp-table">
    <thead>
      <tr>
        <th></th>
        <th>{t("app.runDate")}</th>
        <th>{label("abbr.pmax")} ({U.unitPower()})</th>
        <th>{label("abbr.mmax")} ({U.unitTorque()})</th>
      </tr>
    </thead>
    <tbody>
      {#each built.summary as b (b.color)}
        <tr>
          <td class="run-cell"><span class="dot" style="background:{b.color}"></span>{b.title}</td>
          <td>{b.date ?? "—"}</td>
          <td>{b.pmax.toFixed(1)} @ {b.rpmAtPmax}</td>
          <td>{b.tmax.toFixed(1)} @ {b.rpmAtTmax}</td>
        </tr>
      {/each}
    </tbody>
  </table>
  <p class="cmp-note">{t("compare.lineNote")}</p>
</section>

<style>
  .cmp-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 1rem;
    margin-bottom: 0.6rem;
  }
  .cmp-head h3 {
    margin: 0;
    font-size: 1rem;
  }
  .chart-card {
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 8px;
    padding: 0.7rem 0.8rem 0.5rem;
  }
  .chart-card :global(svg) {
    max-height: 52vh;
  }
  .cmp-table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 0.8rem;
    font-size: 0.85rem;
  }
  .cmp-table th,
  .cmp-table td {
    text-align: left;
    padding: 0.3rem 0.6rem;
    border-bottom: 1px solid var(--line);
  }
  .cmp-table th {
    color: var(--muted);
    font-weight: 600;
  }
  .run-cell {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .dot {
    width: 12px;
    height: 12px;
    border-radius: 3px;
    flex: none;
  }
  .cmp-note {
    color: var(--muted);
    font-size: 0.8rem;
    margin: 0.5rem 0 0;
  }
</style>
