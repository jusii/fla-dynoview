<script lang="ts">
  import DynoChart from "../charts/DynoChart.svelte";
  import { powerSeries, torqueSeries } from "../charts/series";
  import { t } from "../i18n";
  import * as U from "../units";
  import type { CurrentRun } from "../types";

  let { shopName = "", current }: { shopName?: string; current: CurrentRun } = $props();

  const power = $derived(powerSeries(current.channels, current.results.kDin));
  const torque = $derived(torqueSeries(current.channels, current.results.kDin));
  const r = $derived(current.results);
  const pw = (v: number | null | undefined) => (v == null ? "—" : U.power(v).toFixed(1));
</script>

<!-- Hidden on screen; shown by the @media print rules in app.css. -->
<div id="print-report">
  <header class="pr-head">
    <div class="pr-shop">{shopName || t("print.reportTitle")}</div>
    <div class="pr-meta">
      <span>{current.title}</span>
      <span>{current.date ?? ""}</span>
    </div>
  </header>

  {#if current.description}
    <p class="pr-desc">{current.description}</p>
  {/if}

  <table class="pr-table">
    <tbody>
      <tr>
        <th>{t("abbr.pmax")} ({t("term.engine")})</th>
        <td>{pw(r.pmaxKw)} {U.unitPower()} @ {r.rpmAtPmax ?? "—"} {U.unitRpm()}</td>
        <th>{t("abbr.mmax")} ({t("term.torque")})</th>
        <td>{r.mmaxNm == null ? "—" : U.torque(r.mmaxNm).toFixed(1)} {U.unitTorque()} @ {r.rpmAtMmax ?? "—"} {U.unitRpm()}</td>
      </tr>
      <tr>
        <th>{t("abbr.ppyora")} ({t("term.wheel")})</th>
        <td>{pw(r.ppyoraKw)} {U.unitPower()}</td>
        <th>{t("abbr.phavio")} ({t("term.loss")})</th>
        <td>{pw(r.phavioKw)} {U.unitPower()}</td>
      </tr>
      <tr>
        <th>{t("abbr.k")}</th>
        <td>{r.kDin == null ? "—" : r.kDin.toFixed(3)}</td>
        <th>{t("abbr.pnim")}</th>
        <td>{pw(r.pnimKw)} {U.unitPower()}</td>
      </tr>
      <tr>
        <th>{t("abbr.paine")} / {t("abbr.lamp")}</th>
        <td>{r.pressureHpa ?? "—"} {U.unitPressure()} / {r.tempC == null ? "—" : U.temp(r.tempC).toFixed(0)} {U.unitTemp()}</td>
        <th>{t("print.runDate")}</th>
        <td>{current.date ?? "—"}</td>
      </tr>
    </tbody>
  </table>

  <div class="chart-block">
    <h4>{t("term.engine")} [{U.unitPower()}]</h4>
    {#if power.length}<DynoChart series={power} yLabel={U.unitPower()} xLabel={t("chart.sweep")} width={720} height={340} />{/if}
  </div>
  <div class="chart-block">
    <h4>{t("term.torque")} [{U.unitTorque()}]</h4>
    {#if torque.length}<DynoChart series={torque} yLabel={U.unitTorque()} xLabel={t("chart.sweep")} width={720} height={340} />{/if}
  </div>

  <footer class="pr-foot">{t("print.generatedBy")}</footer>
</div>

<style>
  #print-report {
    color: #000;
    background: #fff;
    font-family: system-ui, sans-serif;
  }
  .pr-head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    border-bottom: 2px solid #000;
    padding-bottom: 6px;
    margin-bottom: 10px;
  }
  .pr-shop {
    font-size: 18px;
    font-weight: 700;
  }
  .pr-meta {
    display: flex;
    gap: 16px;
    font-family: ui-monospace, monospace;
    font-size: 12px;
  }
  .pr-desc {
    font-style: italic;
    margin: 0 0 10px;
  }
  .pr-table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 12px;
    font-size: 12px;
  }
  .pr-table th,
  .pr-table td {
    text-align: left;
    padding: 3px 8px;
    border: 1px solid #bbb;
  }
  .pr-table th {
    background: #f0f0f0;
    font-weight: 600;
    width: 20%;
  }
  .chart-block {
    margin-bottom: 10px;
  }
  .chart-block h4 {
    margin: 0 0 4px;
    font-size: 13px;
  }
  .pr-foot {
    margin-top: 8px;
    font-size: 10px;
    color: #666;
    text-align: right;
  }
</style>
