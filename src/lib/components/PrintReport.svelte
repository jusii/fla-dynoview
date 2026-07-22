<script lang="ts">
  import DynoChart from "../charts/DynoChart.svelte";
  import { views } from "../charts/series";
  import type { CropRange } from "../charts/series";
  import { t, label } from "../i18n";
  import * as U from "../units";
  import type { CurrentRun } from "../types";
  import { curveVisibility, logoDataUri, printHeaderText } from "../settings.svelte";

  let {
    current,
    crop,
  }: { current: CurrentRun; crop?: CropRange } = $props();

  const vis = $derived(curveVisibility());
  const v = $derived(views(current.channels, current.results.kDin, crop, vis));
  const r = $derived(current.results);
  const s = $derived(v.scalars);
  const pw = (val: number | null | undefined) => (val == null ? "—" : U.power(val).toFixed(1));
  // "@ rpm" for rpm runs, "@ km/h" for runs recorded without an rpm pickup.
  const atPmax = $derived(
    v.hasRpm
      ? `${s.rpmAtPmax ?? "—"} ${U.unitRpm()}`
      : s.vAtPmax == null
        ? "—"
        : `${U.speed(s.vAtPmax).toFixed(0)} ${U.unitSpeed()}`,
  );

  const tempShown = $derived(current.overrides.tempC ?? r.tempC);
  const pressShown = $derived(current.overrides.pressureHpa ?? r.pressureHpa);

  const logo = $derived(logoDataUri());
  const headerText = $derived(printHeaderText());
</script>

<!-- Hidden on screen; shown by the @media print rules in app.css. -->
<div id="print-report">
  <header class="pr-head">
    {#if logo}<img class="pr-logo" src={logo} alt="" />{/if}
    <div class="pr-shop">{headerText || current.shopName || t("print.reportTitle")}</div>
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
        <th>{label("abbr.pmax")} ({label("term.engine")})</th>
        <td>{pw(s.pmaxKw)} {U.unitPower()} @ {atPmax}</td>
        <th>{label("abbr.mmax")} ({label("term.torque")})</th>
        <td>{s.mmaxNm == null ? "—" : U.torque(s.mmaxNm).toFixed(1)} {U.unitTorque()} @ {s.rpmAtMmax ?? "—"} {U.unitRpm()}</td>
      </tr>
      <tr>
        <th>{label("abbr.ppyora")} ({label("term.wheel")})</th>
        <td>{pw(s.ppyoraKw)} {U.unitPower()}</td>
        <th>{label("abbr.phavio")} ({label("term.loss")})</th>
        <td>{pw(s.phavioKw)} {U.unitPower()}</td>
      </tr>
      <tr>
        <th>{label("abbr.pnim")}</th>
        <td>{pw(r.pnimKw)} {U.unitPower()}</td>
        <th>{label("abbr.k")}</th>
        <td>{r.kDin == null ? "—" : r.kDin.toFixed(3)}</td>
      </tr>
      <tr>
        <th>{label("abbr.paine")} / {label("abbr.lamp")}</th>
        <td>{pressShown ?? "—"} {U.unitPressure()} / {tempShown == null ? "—" : U.temp(tempShown).toFixed(0)} {U.unitTemp()}</td>
        <th>{t("print.runDate")}</th>
        <td>{current.date ?? "—"}</td>
      </tr>
    </tbody>
  </table>

  <div class="chart-block">
    <h4>{label("term.engine")} [{U.unitPower()}] / {label("term.torque")} [{U.unitTorque()}]</h4>
    {#if v.series.length}<DynoChart series={v.series} rpm={v.x} leftLabel={U.unitPower()} rightLabel={U.unitTorque()} xLabel={v.hasRpm ? U.unitRpm() : U.unitSpeed()} width={1000} height={440} interactive={false} />{/if}
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
    align-items: center;
    gap: 12px;
    border-bottom: 2px solid #000;
    padding-bottom: 6px;
    margin-bottom: 10px;
  }
  .pr-logo {
    max-height: 46px;
    max-width: 190px;
    object-fit: contain;
  }
  .pr-shop {
    font-size: 18px;
    font-weight: 700;
    margin-right: auto;
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
