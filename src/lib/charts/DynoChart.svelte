<script lang="ts">
  import { scaleLinear } from "d3-scale";
  import { line } from "d3-shape";
  import type { Series } from "./chart-types";
  import { t } from "../i18n";

  let {
    series = [],
    rpm = [],
    xLabel = "",
    leftLabel = "",
    rightLabel = "",
    width = 760,
    height = 460,
    interactive = true,
    showLegend = true,
  }: {
    series?: Series[];
    rpm?: number[];
    xLabel?: string;
    leftLabel?: string;
    rightLabel?: string;
    width?: number;
    height?: number;
    interactive?: boolean;
    showLegend?: boolean;
  } = $props();

  const margin = { top: 18, right: 54, bottom: 46, left: 54 };
  const innerW = $derived(width - margin.left - margin.right);
  const innerH = $derived(height - margin.top - margin.bottom);

  const leftSeries = $derived(series.filter((s) => s.axis !== "right"));
  const rightSeries = $derived(series.filter((s) => s.axis === "right"));

  // X values for a series: its own rpm sweep when present (comparison overlay),
  // else the chart's shared rpm array.
  const rpmOf = (s: Series): number[] => s.rpm ?? rpm;

  function extent(list: Series[]): [number, number] {
    const vals = list.flatMap((s) => s.values);
    if (!vals.length) return [0, 1];
    return [Math.min(0, ...vals), Math.max(1, ...vals)];
  }
  const leftDom = $derived(extent(leftSeries));
  const rightDom = $derived(extent(rightSeries));

  // X is the engine-rpm domain — spanning every series' own rpm sweep.
  const rpmDom = $derived.by<[number, number]>(() => {
    let lo = Infinity;
    let hi = -Infinity;
    for (const s of series) {
      for (const v of rpmOf(s)) {
        if (v < lo) lo = v;
        if (v > hi) hi = v;
      }
    }
    if (!isFinite(lo)) return rpm.length ? [Math.min(...rpm), Math.max(...rpm)] : [0, 1];
    return [lo, hi];
  });
  const x = $derived(scaleLinear().domain(rpmDom).nice().range([0, innerW]));
  const yL = $derived(scaleLinear().domain(leftDom).nice().range([innerH, 0]));
  const yR = $derived(scaleLinear().domain(rightDom).nice().range([innerH, 0]));

  function pathFor(s: Series, yScale: (v: number) => number): string {
    const rx = rpmOf(s);
    const gen = line<number>()
      .x((_d, i) => x(rx[i]))
      .y((d) => yScale(d));
    return gen(s.values) ?? "";
  }

  const yLTicks = $derived(yL.ticks(6));
  const yRTicks = $derived(yR.ticks(6));
  const xTicks = $derived(x.ticks(8));

  // --- hover / scrub readout ------------------------------------------------
  let svgEl: SVGSVGElement | undefined = $state();
  let hoverRpm = $state<number | null>(null);

  function unitOf(s: Series): string {
    return s.axis === "right" ? rightLabel : leftLabel;
  }
  function nearestIndex(s: Series, target: number): number {
    const rx = rpmOf(s);
    let bi = -1;
    let bd = Infinity;
    for (let i = 0; i < rx.length && i < s.values.length; i++) {
      const d = Math.abs(rx[i] - target);
      if (d < bd) {
        bd = d;
        bi = i;
      }
    }
    return bi;
  }

  interface Readout {
    color: string;
    value: number;
    unit: string;
    px: number;
    py: number;
  }
  const readouts = $derived.by<Readout[]>(() => {
    if (hoverRpm == null) return [];
    const out: Readout[] = [];
    for (const s of series) {
      const i = nearestIndex(s, hoverRpm);
      if (i < 0) continue;
      const yScale = s.axis === "right" ? yR : yL;
      out.push({
        color: s.color,
        value: s.values[i],
        unit: unitOf(s),
        px: x(rpmOf(s)[i]),
        py: yScale(s.values[i]),
      });
    }
    return out;
  });
  const cursorX = $derived(hoverRpm == null ? 0 : x(hoverRpm));
  // Keep the readout box inside the plot (flip left when near the right edge).
  const boxW = 116;
  const boxH = $derived(18 + readouts.length * 16);
  const boxX = $derived(cursorX + 12 + boxW > innerW ? cursorX - 12 - boxW : cursorX + 12);

  function onMove(ev: PointerEvent) {
    if (!interactive || !svgEl) return;
    const ctm = svgEl.getScreenCTM();
    if (!ctm) return;
    const pt = svgEl.createSVGPoint();
    pt.x = ev.clientX;
    pt.y = ev.clientY;
    const loc = pt.matrixTransform(ctm.inverse());
    const px = loc.x - margin.left;
    if (px < 0 || px > innerW) {
      hoverRpm = null;
      return;
    }
    hoverRpm = x.invert(px);
  }
  function onLeave() {
    hoverRpm = null;
  }
</script>

<figure class="dyno-chart">
  <svg
    bind:this={svgEl}
    viewBox="0 0 {width} {height}"
    preserveAspectRatio="xMidYMid meet"
    role="img"
    aria-label="{leftLabel} / {rightLabel} {t('chart.versus')} {xLabel}"
    onpointermove={onMove}
    onpointerleave={onLeave}
  >
    <rect class="plot-bg" x="0" y="0" width={width} height={height} />
    <g transform="translate({margin.left},{margin.top})">
      {#each yLTicks as tk (tk)}
        <line class="grid" x1="0" x2={innerW} y1={yL(tk)} y2={yL(tk)} />
        <text class="tick" x="-9" y={yL(tk)} dy="0.32em" text-anchor="end">{tk}</text>
      {/each}
      {#each yRTicks as tk (tk)}
        <text class="tick tick-r" x={innerW + 9} y={yR(tk)} dy="0.32em" text-anchor="start">{tk}</text>
      {/each}
      {#each xTicks as tk (tk)}
        <line class="grid grid-x" x1={x(tk)} x2={x(tk)} y1="0" y2={innerH} />
        <text class="tick" x={x(tk)} y={innerH + 20} text-anchor="middle">{tk}</text>
      {/each}

      {#if leftDom[0] < 0 && leftDom[1] > 0}
        <line class="axis-zero" x1="0" x2={innerW} y1={yL(0)} y2={yL(0)} />
      {/if}

      {#each leftSeries as s (s.label)}
        <path
          d={pathFor(s, yL)}
          fill="none"
          stroke={s.color}
          stroke-width="1.8"
          stroke-dasharray={s.dash ? "6 4" : undefined}
        />
      {/each}
      {#each rightSeries as s (s.label)}
        <path
          d={pathFor(s, yR)}
          fill="none"
          stroke={s.color}
          stroke-width="1.8"
          stroke-dasharray={s.dash ? "6 4" : undefined}
        />
      {/each}

      <text class="axis-label" x={-margin.left + 4} y="-6">{leftLabel}</text>
      <text class="axis-label" x={innerW + margin.right - 4} y="-6" text-anchor="end">{rightLabel}</text>
      <text class="axis-label" x={innerW / 2} y={innerH + 40} text-anchor="middle">{xLabel}</text>

      {#if hoverRpm != null && readouts.length}
        <line class="cursor" x1={cursorX} x2={cursorX} y1="0" y2={innerH} />
        {#each readouts as r (r.color)}
          <circle class="cursor-dot" cx={r.px} cy={r.py} r="3.2" fill={r.color} />
        {/each}
        <g transform="translate({boxX},6)" class="readout" pointer-events="none">
          <rect class="ro-box" x="0" y="0" width={boxW} height={boxH} rx="4" />
          <text class="ro-rpm" x="8" y="14">{Math.round(hoverRpm)} {xLabel}</text>
          {#each readouts as r, i (r.color)}
            <rect class="ro-sw" x="8" y={22 + i * 16} width="9" height="9" fill={r.color} />
            <text class="ro-val" x="22" y={30 + i * 16}>{r.value.toFixed(1)} {r.unit}</text>
          {/each}
        </g>
      {/if}
    </g>
  </svg>

  {#if showLegend}
    <div class="legend">
      {#each series as s (s.label)}
        <span class="legend-item">
          <span class="swatch" class:dashed={s.dash} style="background:{s.color}"></span>{s.label}
        </span>
      {/each}
    </div>
  {/if}
</figure>

<style>
  .dyno-chart {
    margin: 0;
  }
  svg {
    width: 100%;
    height: auto;
    display: block;
  }
  .plot-bg {
    fill: #0d1b2a;
  }
  .grid {
    stroke: #24405c;
    stroke-width: 1;
  }
  .grid-x {
    stroke: #1a2f45;
  }
  .axis-zero {
    stroke: #d9c04a;
    stroke-dasharray: 2 3;
    stroke-width: 1;
  }
  .tick {
    fill: #d9c04a;
    font-size: 12px;
    font-family: ui-monospace, "Cascadia Mono", monospace;
  }
  .axis-label {
    fill: #d9c04a;
    font-size: 12px;
    font-weight: 600;
  }
  .cursor {
    stroke: #9fb4c9;
    stroke-width: 1;
    stroke-dasharray: 3 3;
  }
  .cursor-dot {
    stroke: #0d1b2a;
    stroke-width: 1;
  }
  .readout .ro-box {
    fill: rgba(9, 18, 30, 0.92);
    stroke: #3a5a7c;
  }
  .ro-rpm {
    fill: #d9c04a;
    font-size: 11px;
    font-weight: 600;
    font-family: ui-monospace, monospace;
  }
  .ro-val {
    fill: #e8eef4;
    font-size: 11px;
    font-family: ui-monospace, monospace;
  }
  .legend {
    display: flex;
    flex-wrap: wrap;
    gap: 1.1rem;
    padding: 0.4rem 0 0 0.4rem;
    font-size: 0.82rem;
  }
  .legend-item {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
  }
  .swatch {
    width: 16px;
    height: 3px;
    border-radius: 2px;
  }
  .swatch.dashed {
    background-image: none;
    -webkit-mask-image: repeating-linear-gradient(90deg, #000 0 4px, transparent 4px 7px);
    mask-image: repeating-linear-gradient(90deg, #000 0 4px, transparent 4px 7px);
  }

  @media print {
    .plot-bg {
      fill: #ffffff;
    }
    .grid {
      stroke: #cccccc;
    }
    .grid-x {
      stroke: #e5e5e5;
    }
    .tick,
    .axis-label {
      fill: #333333;
    }
    .axis-zero {
      stroke: #999999;
    }
    .cursor,
    .cursor-dot,
    .readout {
      display: none;
    }
  }
</style>
