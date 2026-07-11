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
  }: {
    series?: Series[];
    rpm?: number[];
    xLabel?: string;
    leftLabel?: string;
    rightLabel?: string;
    width?: number;
    height?: number;
  } = $props();

  const margin = { top: 18, right: 54, bottom: 46, left: 54 };
  const innerW = $derived(width - margin.left - margin.right);
  const innerH = $derived(height - margin.top - margin.bottom);

  const leftSeries = $derived(series.filter((s) => s.axis !== "right"));
  const rightSeries = $derived(series.filter((s) => s.axis === "right"));

  function extent(list: Series[]): [number, number] {
    const vals = list.flatMap((s) => s.values);
    if (!vals.length) return [0, 1];
    return [Math.min(0, ...vals), Math.max(1, ...vals)];
  }
  const leftDom = $derived(extent(leftSeries));
  const rightDom = $derived(extent(rightSeries));

  // X is the engine-rpm domain (a proper dyno rpm axis).
  const rpmDom = $derived<[number, number]>(
    rpm.length ? [Math.min(...rpm), Math.max(...rpm)] : [0, 1],
  );
  const x = $derived(scaleLinear().domain(rpmDom).nice().range([0, innerW]));
  const yL = $derived(scaleLinear().domain(leftDom).nice().range([innerH, 0]));
  const yR = $derived(scaleLinear().domain(rightDom).nice().range([innerH, 0]));

  const genL = $derived(line<number>().x((_d, i) => x(rpm[i])).y((d) => yL(d)));
  const genR = $derived(line<number>().x((_d, i) => x(rpm[i])).y((d) => yR(d)));

  const yLTicks = $derived(yL.ticks(6));
  const yRTicks = $derived(yR.ticks(6));
  const xTicks = $derived(x.ticks(8));
</script>

<figure class="dyno-chart">
  <svg
    viewBox="0 0 {width} {height}"
    preserveAspectRatio="xMidYMid meet"
    role="img"
    aria-label="{leftLabel} / {rightLabel} {t('chart.versus')} {xLabel}"
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
        <path d={genL(s.values) ?? ""} fill="none" stroke={s.color} stroke-width="1.8" />
      {/each}
      {#each rightSeries as s (s.label)}
        <path d={genR(s.values) ?? ""} fill="none" stroke={s.color} stroke-width="1.8" />
      {/each}

      <text class="axis-label" x={-margin.left + 4} y="-6">{leftLabel}</text>
      <text class="axis-label" x={innerW + margin.right - 4} y="-6" text-anchor="end">{rightLabel}</text>
      <text class="axis-label" x={innerW / 2} y={innerH + 40} text-anchor="middle">{xLabel}</text>
    </g>
  </svg>

  <div class="legend">
    {#each series as s (s.label)}
      <span class="legend-item">
        <span class="swatch" style="background:{s.color}"></span>{s.label}
      </span>
    {/each}
  </div>
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
  }
</style>
