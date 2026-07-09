<script lang="ts">
  import DynoChart from "../charts/DynoChart.svelte";
  import { views } from "../charts/series";
  import type { CropRange } from "../charts/series";
  import { t } from "../i18n";
  import * as U from "../units";
  import type { CurrentRun } from "../types";

  let {
    current,
    crop = $bindable({ start: 0, end: 1 }),
  }: { current: CurrentRun; crop?: CropRange } = $props();

  const v = $derived(views(current.channels, current.results.kDin, crop));
  const r = $derived(current.results);
  const s = $derived(v.scalars);

  const pw = (val: number | null | undefined) => (val == null ? "—" : U.power(val).toFixed(1));

  function setStart(e: Event) {
    const val = +(e.currentTarget as HTMLInputElement).value / 100;
    crop = { start: Math.min(val, crop.end - 0.02), end: crop.end };
  }
  function setEnd(e: Event) {
    const val = +(e.currentTarget as HTMLInputElement).value / 100;
    crop = { start: crop.start, end: Math.max(val, crop.start + 0.02) };
  }
</script>

<section class="infobox">
  <div class="info-title">{current.title} · {current.date ?? t("detail.noDate")}</div>
  <div class="info-grid">
    <div><span>{t("abbr.pmax")}</span>{pw(s.pmaxKw)} {U.unitPower()}</div>
    <div><span>{t("detail.atN")}</span>{s.rpmAtPmax ?? "—"} {U.unitRpm()}</div>
    <div><span>{t("abbr.ppyora")}</span>{pw(s.ppyoraKw)} {U.unitPower()}</div>
    <div><span>{t("abbr.phavio")}</span>{pw(s.phavioKw)} {U.unitPower()}</div>
    <div><span>{t("abbr.mmax")}</span>{s.mmaxNm == null ? "—" : U.torque(s.mmaxNm).toFixed(1)} {U.unitTorque()}</div>
    <div><span>{t("detail.atN")}</span>{s.rpmAtMmax ?? "—"} {U.unitRpm()}</div>
    <div><span>{t("abbr.k")}</span>{r.kDin == null ? "—" : r.kDin.toFixed(3)}</div>
    <div><span>{t("abbr.pnim")}</span>{pw(r.pnimKw)} {U.unitPower()}</div>
    <div><span>{t("abbr.paine")}</span>{r.pressureHpa ?? "—"} {U.unitPressure()}</div>
    <div><span>{t("abbr.lamp")}</span>{r.tempC == null ? "—" : U.temp(r.tempC).toFixed(0)} {U.unitTemp()}</div>
  </div>
</section>

{#if v.series.length}
  <div class="crop-bar">
    <span class="crop-label">{t("crop.trim")}</span>
    <input type="range" min="0" max="98" value={Math.round(crop.start * 100)} oninput={setStart} aria-label="{t('crop.trim')} start" />
    <input type="range" min="2" max="100" value={Math.round(crop.end * 100)} oninput={setEnd} aria-label="{t('crop.trim')} end" />
    <span class="crop-pct">{Math.round(crop.start * 100)}–{Math.round(crop.end * 100)}%</span>
    <button onclick={() => (crop = { start: 0, end: 1 })}>{t("crop.reset")}</button>
  </div>
{/if}

<section class="charts">
  <div class="chart-card">
    <h3>{t("term.engine")} / {t("term.torque")}</h3>
    {#if v.series.length}
      <DynoChart
        series={v.series}
        rpm={v.rpm}
        leftLabel={U.unitPower()}
        rightLabel={U.unitTorque()}
        xLabel={U.unitRpm()}
      />
    {:else}
      <p class="muted">{t("detail.noPower")}</p>
    {/if}
  </div>
</section>
