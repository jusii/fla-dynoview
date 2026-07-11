<script lang="ts">
  import DynoChart from "../charts/DynoChart.svelte";
  import { views } from "../charts/series";
  import type { CropRange } from "../charts/series";
  import { t, label } from "../i18n";
  import * as U from "../units";
  import type { CurrentRun } from "../types";
  import { curveVisibility, setCurveVisibility } from "../settings.svelte";

  let {
    current,
    crop = $bindable({ start: 0, end: 1 }),
  }: { current: CurrentRun; crop?: CropRange } = $props();

  const vis = $derived(curveVisibility());
  const v = $derived(views(current.channels, current.results.kDin, crop, vis));
  const r = $derived(current.results);
  const s = $derived(v.scalars);

  // Editable readings (display-only overrides) fall back to the decoded value.
  const tempShown = $derived(current.overrides.tempC ?? r.tempC);
  const pressShown = $derived(current.overrides.pressureHpa ?? r.pressureHpa);
  const tempEdited = $derived(current.overrides.tempC != null);
  const pressEdited = $derived(current.overrides.pressureHpa != null);

  const pw = (val: number | null | undefined) => (val == null ? "—" : U.power(val).toFixed(1));

  const CURVES = [
    { key: "engine", term: "term.engine" },
    { key: "wheel", term: "term.wheel" },
    { key: "loss", term: "term.loss" },
    { key: "torque", term: "term.torque" },
  ] as const;
  function toggle(key: (typeof CURVES)[number]["key"], on: boolean) {
    setCurveVisibility({ ...vis, [key]: on });
  }

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
    <div><span>{label("abbr.pmax")}</span>{pw(s.pmaxKw)} {U.unitPower()}</div>
    <div><span>{t("detail.atN")}</span>{s.rpmAtPmax ?? "—"} {U.unitRpm()}</div>
    <div><span>{label("abbr.ppyora")}</span>{pw(s.ppyoraKw)} {U.unitPower()}</div>
    <div><span>{label("abbr.phavio")}</span>{pw(s.phavioKw)} {U.unitPower()}</div>
    <div><span>{label("abbr.mmax")}</span>{s.mmaxNm == null ? "—" : U.torque(s.mmaxNm).toFixed(1)} {U.unitTorque()}</div>
    <div><span>{t("detail.atN")}</span>{s.rpmAtMmax ?? "—"} {U.unitRpm()}</div>
    <div><span>{label("abbr.pnim")}</span>{pw(r.pnimKw)} {U.unitPower()}</div>
    <div><span>{label("abbr.k")}</span>{r.kDin == null ? "—" : r.kDin.toFixed(3)}</div>
    <div class:edited={pressEdited}><span>{label("abbr.paine")}</span>{pressShown ?? "—"} {U.unitPressure()}</div>
    <div class:edited={tempEdited}><span>{label("abbr.lamp")}</span>{tempShown == null ? "—" : U.temp(tempShown).toFixed(0)} {U.unitTemp()}</div>
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
    <div class="chart-head">
      <h3>{label("term.engine")} / {label("term.torque")}</h3>
      <div class="curve-toggle">
        {#each CURVES as c (c.key)}
          <label><input type="checkbox" checked={vis[c.key]} onchange={(e) => toggle(c.key, e.currentTarget.checked)} />{label(c.term)}</label>
        {/each}
      </div>
    </div>
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

<style>
  .info-grid div.edited span::after {
    content: "•";
    color: #e6a94a;
    margin-left: 3px;
  }
  .chart-head {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    justify-content: space-between;
    gap: 0.5rem 1rem;
  }
  .curve-toggle {
    display: flex;
    flex-wrap: wrap;
    gap: 0.2rem 0.9rem;
    font-size: 0.8rem;
    color: var(--muted);
  }
  .curve-toggle label {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    cursor: pointer;
  }
  .curve-toggle input {
    margin: 0;
  }
</style>
