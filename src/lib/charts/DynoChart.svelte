<script lang="ts">
  import { scaleLinear } from "d3-scale";
  import { line } from "d3-shape";
  import type { Series } from "./chart-types";

  let {
    series = [],
    xLabel = "",
    yLabel = "",
    width = 680,
    height = 440,
  }: {
    series?: Series[];
    xLabel?: string;
    yLabel?: string;
    width?: number;
    height?: number;
  } = $props();

  const margin = { top: 18, right: 18, bottom: 44, left: 56 };

  const innerW = $derived(width - margin.left - margin.right);
  const innerH = $derived(height - margin.top - margin.bottom);

  const allValues = $derived(series.flatMap((s) => s.values));
  const maxLen = $derived(Math.max(1, ...series.map((s) => s.values.length)));
  const yMin = $derived(allValues.length ? Math.min(0, ...allValues) : 0);
  const yMax = $derived(allValues.length ? Math.max(1, ...allValues) : 1);

  const x = $derived(
    scaleLinear().domain([0, Math.max(1, maxLen - 1)]).range([0, innerW]),
  );
  const y = $derived(
    scaleLinear().domain([yMin, yMax]).nice().range([innerH, 0]),
  );

  const gen = $derived(
    line<number>()
      .x((_d, i) => x(i))
      .y((d) => y(d)),
  );

  const yTicks = $derived(y.ticks(6));
  const xTicks = $derived(x.ticks(8));
</script>

<figure class="dyno-chart">
  <svg
    viewBox="0 0 {width} {height}"
    preserveAspectRatio="xMidYMid meet"
    role="img"
    aria-label="{yLabel} versus {xLabel}"
  >
    <rect class="plot-bg" x="0" y="0" width={width} height={height} />
    <g transform="translate({margin.left},{margin.top})">
      {#each yTicks as t (t)}
        <line class="grid" x1="0" x2={innerW} y1={y(t)} y2={y(t)} />
        <text class="tick" x="-9" y={y(t)} dy="0.32em" text-anchor="end">{t}</text>
      {/each}
      {#each xTicks as t (t)}
        <line class="grid grid-x" x1={x(t)} x2={x(t)} y1="0" y2={innerH} />
        <text class="tick" x={x(t)} y={innerH + 20} text-anchor="middle">{t}</text>
      {/each}

      {#if yMin < 0 && yMax > 0}
        <line class="axis-zero" x1="0" x2={innerW} y1={y(0)} y2={y(0)} />
      {/if}

      {#each series as s (s.label)}
        <path d={gen(s.values) ?? ""} fill="none" stroke={s.color} stroke-width="1.7" />
      {/each}

      <text class="axis-label" x={-margin.left + 4} y="-6">{yLabel}</text>
      <text class="axis-label" x={innerW} y={innerH + 38} text-anchor="end">{xLabel}</text>
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
    gap: 1.2rem;
    padding: 0.35rem 0 0 0.4rem;
    font-size: 0.82rem;
  }
  .legend-item {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
  }
  .swatch {
    width: 14px;
    height: 3px;
    border-radius: 2px;
  }

  /* Print: keep the plot readable on white paper without wasting ink. */
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
