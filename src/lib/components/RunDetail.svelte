<script lang="ts">
  import DynoChart from "../charts/DynoChart.svelte";
  import { powerSeries, torqueSeries } from "../charts/series";
  import { t } from "../i18n";
  import * as U from "../units";
  import type { CurrentRun } from "../types";

  let { current }: { current: CurrentRun } = $props();

  const power = $derived(powerSeries(current.channels, current.results.kDin));
  const torque = $derived(torqueSeries(current.channels, current.results.kDin));
  const r = $derived(current.results);

  const f1 = (v: number | null | undefined) => (v == null ? "—" : v.toFixed(1));
  const pw = (v: number | null | undefined) => (v == null ? "—" : U.power(v).toFixed(1));
</script>

<section class="infobox">
  <div class="info-title">{current.title} · {current.date ?? t("detail.noDate")}</div>
  <div class="info-grid">
    <div><span>{t("abbr.pmax")}</span>{pw(r.pmaxKw)} {U.unitPower()}</div>
    <div><span>{t("detail.atN")}</span>{r.rpmAtPmax ?? "—"} {U.unitRpm()}</div>
    <div><span>{t("abbr.ppyora")}</span>{pw(r.ppyoraKw)} {U.unitPower()}</div>
    <div><span>{t("abbr.phavio")}</span>{pw(r.phavioKw)} {U.unitPower()}</div>
    <div><span>{t("abbr.mmax")}</span>{r.mmaxNm == null ? "—" : U.torque(r.mmaxNm).toFixed(1)} {U.unitTorque()}</div>
    <div><span>{t("detail.atN")}</span>{r.rpmAtMmax ?? "—"} {U.unitRpm()}</div>
    <div><span>{t("abbr.k")}</span>{r.kDin == null ? "—" : r.kDin.toFixed(3)}</div>
    <div><span>{t("abbr.pnim")}</span>{pw(r.pnimKw)} {U.unitPower()}</div>
    <div><span>{t("abbr.paine")}</span>{r.pressureHpa ?? "—"} {U.unitPressure()}</div>
    <div><span>{t("abbr.lamp")}</span>{r.tempC == null ? "—" : U.temp(r.tempC).toFixed(0)} {U.unitTemp()}</div>
  </div>
</section>

<section class="charts">
  <div class="chart-card">
    <h3>{t("term.engine")}</h3>
    {#if power.length}
      <DynoChart series={power} yLabel={U.unitPower()} xLabel={t("chart.sweep")} />
    {:else}
      <p class="muted">{t("detail.noPower")}</p>
    {/if}
  </div>
  <div class="chart-card">
    <h3>{t("term.torque")}</h3>
    {#if torque.length}
      <DynoChart series={torque} yLabel={U.unitTorque()} xLabel={t("chart.sweep")} />
    {:else}
      <p class="muted">{t("detail.noTorque")}</p>
    {/if}
  </div>
</section>
