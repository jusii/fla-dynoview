<script lang="ts">
  import { onMount } from "svelte";
  import { ask, open } from "@tauri-apps/plugin-dialog";
  import * as api from "./lib/api";
  import type { CurrentRun, DecodedRun, ImageSummary, RunEntry, RunIndexEntry, RunRecord } from "./lib/types";
  import RunDetail from "./lib/components/RunDetail.svelte";
  import PrintReport from "./lib/components/PrintReport.svelte";

  type Mode = "browse" | "library";

  let mode = $state<Mode>("browse");
  let image = $state<ImageSummary | null>(null);
  let library = $state<RunIndexEntry[]>([]);
  let libQuery = $state("");
  let current = $state<CurrentRun | null>(null);
  let selectedKey = $state<string | null>(null);
  let busy = $state(false);
  let err = $state<string | null>(null);
  let toast = $state<string | null>(null);

  function baseName(p: string): string {
    return p.split(/[\\/]/).pop() ?? p;
  }
  function fromDecoded(run: DecodedRun, title: string): CurrentRun {
    return { title, date: run.date, description: "", results: run.results, channels: run.channels, libId: null };
  }
  function fromRecord(rec: RunRecord): CurrentRun {
    return {
      title: rec.sourceEntry ?? rec.id.slice(0, 8),
      date: rec.runDate,
      description: rec.description,
      results: rec.results,
      channels: rec.channels,
      libId: rec.id,
    };
  }

  async function pickImage() {
    err = null;
    const path = await open({ filters: [{ name: "Floppy image", extensions: ["img", "IMG"] }] });
    if (typeof path !== "string") return;
    busy = true;
    try {
      image = await api.openImage(path);
      mode = "browse";
      current = null;
      selectedKey = null;
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function pickErg() {
    err = null;
    const path = await open({ filters: [{ name: "ERG run file", extensions: ["erg", "ERG"] }] });
    if (typeof path !== "string") return;
    busy = true;
    try {
      const run = await api.openErgFile(path);
      current = fromDecoded(run, baseName(path));
      selectedKey = path;
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function selectImageRun(r: RunEntry) {
    if (!image) return;
    selectedKey = r.path;
    busy = true; err = null;
    try {
      const run = await api.readErgFromImage(image.imagePath, r.path);
      current = fromDecoded(run, r.name);
    } catch (e) { err = String(e); current = null; } finally { busy = false; }
  }

  async function selectLibRun(entry: RunIndexEntry) {
    selectedKey = entry.id;
    busy = true; err = null;
    try {
      current = fromRecord(await api.getDbRun(entry.id));
    } catch (e) { err = String(e); current = null; } finally { busy = false; }
  }

  async function refreshLibrary() {
    try { library = await api.listDbRuns(libQuery); } catch (e) { err = String(e); }
  }

  async function switchMode(m: Mode) {
    mode = m;
    current = null;
    selectedKey = null;
    if (m === "library") await refreshLibrary();
  }

  async function importNew() {
    if (!image) return;
    const paths = image.runs.filter((r) => !r.deleted && !r.inLibrary).map((r) => r.path);
    if (!paths.length) { toast = "Nothing new to import — all runs are already in the library."; return; }
    busy = true; err = null;
    try {
      const rep = await api.importRuns(image.imagePath, paths, false);
      toast = `Imported ${rep.added.length} new run(s); skipped ${rep.skipped.length}${rep.failed.length ? `, ${rep.failed.length} failed` : ""}.`;
      image = await api.openImage(image.imagePath);
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function importAllOverwrite() {
    if (!image) return;
    const ok = await ask(
      `Re-import ALL ${image.runs.filter((r) => !r.deleted).length} runs and OVERWRITE any copies already in the library?`,
      { title: "Import all / overwrite", kind: "warning" },
    );
    if (!ok) return;
    busy = true; err = null;
    try {
      const rep = await api.importAll(image.imagePath, true, false);
      toast = `Imported ${rep.added.length}, overwrote ${rep.overwritten.length}.`;
      image = await api.openImage(image.imagePath);
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function doReset() {
    if (!image) return;
    const ok = await ask(
      `Delete ALL dyno runs from this disk image?\n\nSettings & calibration (FLA.CFG), language tables and fonts are kept. A timestamped backup of the image is saved first.\n\n${image.imagePath}`,
      { title: "Reset disk", kind: "warning" },
    );
    if (!ok) return;
    busy = true; err = null;
    try {
      const rep = await api.resetImage(image.imagePath, true);
      toast = `Wiped ${rep.deleted.length} run(s). Backup saved to ${rep.backupPath}`;
      image = await api.openImage(image.imagePath);
      current = null; selectedKey = null;
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function saveDescription() {
    if (!current?.libId) return;
    busy = true; err = null;
    try {
      await api.updateRunDescription(current.libId, current.description);
      toast = "Description saved.";
      await refreshLibrary();
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function deleteCurrent() {
    if (!current?.libId) return;
    const ok = await ask("Remove this run from your library? (The source disk is not touched.)", {
      title: "Delete run", kind: "warning",
    });
    if (!ok) return;
    busy = true; err = null;
    try {
      await api.deleteDbRun(current.libId);
      current = null; selectedKey = null;
      await refreshLibrary();
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  function printReport() {
    window.print();
  }

  const newCount = $derived(image ? image.runs.filter((r) => !r.deleted && !r.inLibrary).length : 0);

  onMount(async () => {
    try {
      const p = await api.initialPath();
      if (!p) return;
      busy = true;
      if (p.toLowerCase().endsWith(".erg")) {
        current = fromDecoded(await api.openErgFile(p), baseName(p));
      } else {
        image = await api.openImage(p);
        const first = image.runs.find((r) => !r.deleted) ?? image.runs[0];
        if (first) await selectImageRun(first);
      }
    } catch (e) { err = String(e); } finally { busy = false; }
  });
</script>

<div class="app">
  <header class="toolbar">
    <div class="brand">FLA&nbsp;Dynoview</div>
    <button onclick={pickImage} disabled={busy}>Open .img…</button>
    <button onclick={pickErg} disabled={busy}>Open .ERG…</button>
    <div class="tabs">
      <button class:tabsel={mode === "browse"} onclick={() => switchMode("browse")}>Disk</button>
      <button class:tabsel={mode === "library"} onclick={() => switchMode("library")}>Library</button>
    </div>
    <div class="spacer"></div>
    {#if current}
      <button class="primary" onclick={printReport}>Print / PDF…</button>
    {/if}
    {#if image && mode === "browse"}
      <div class="shop">🏁 {image.shopName || "(no shop name)"}</div>
    {/if}
  </header>

  {#if err}
    <div class="bar error">⚠ {err}<button class="x" onclick={() => (err = null)}>✕</button></div>
  {/if}
  {#if toast}
    <div class="bar toast">{toast}<button class="x" onclick={() => (toast = null)}>✕</button></div>
  {/if}

  <div class="body">
    <aside class="sidebar">
      {#if mode === "browse"}
        {#if image}
          <div class="side-actions">
            <button onclick={importNew} disabled={busy || newCount === 0}>
              Import new ({newCount})
            </button>
            <button onclick={importAllOverwrite} disabled={busy}>Import all…</button>
            <button class="danger" onclick={doReset} disabled={busy}>Reset disk…</button>
          </div>
          <div class="side-head">{image.runs.length} run{image.runs.length === 1 ? "" : "s"}</div>
          <ul class="runlist">
            {#each image.runs as r, i (r.path + "#" + i)}
              <li>
                <button
                  class="runitem"
                  class:active={selectedKey === r.path}
                  class:deleted={r.deleted}
                  onclick={() => selectImageRun(r)}
                >
                  <span class="rname">{r.name}</span>
                  <span class="rdate">{r.date ?? "—"}</span>
                  {#if r.inLibrary}<span class="chk" title="In library">✓</span>{/if}
                  {#if r.deleted}<span class="tag">deleted</span>{/if}
                </button>
              </li>
            {/each}
          </ul>
        {:else}
          <div class="empty">
            <p>Open a Bosch FLA&nbsp;203 floppy image (<code>.img</code>) to browse its dyno
            runs, or open a single <code>.ERG</code> file.</p>
          </div>
        {/if}
      {:else}
        <div class="side-actions">
          <input class="search" placeholder="Search description / date…"
                 bind:value={libQuery} oninput={refreshLibrary} />
        </div>
        <div class="side-head">{library.length} in library</div>
        <ul class="runlist">
          {#each library as e (e.id)}
            <li>
              <button class="runitem libitem" class:active={selectedKey === e.id} onclick={() => selectLibRun(e)}>
                <span class="rdate strong">{e.runDate ?? "—"}</span>
                <span class="rdesc">{e.description || e.sourceImage || "(no description)"}</span>
              </button>
            </li>
          {:else}
            <li class="empty"><p>No runs imported yet. Open a disk and “Import new”.</p></li>
          {/each}
        </ul>
      {/if}
    </aside>

    <main class="content">
      {#if current}
        <RunDetail {current} />
        {#if current.libId}
          <section class="desc-editor">
            <label for="desc">Description</label>
            <textarea id="desc" rows="2" bind:value={current.description}
                      placeholder="Customer, vehicle, notes…"></textarea>
            <div class="desc-actions">
              <button class="primary" onclick={saveDescription} disabled={busy}>Save</button>
              <button class="danger" onclick={deleteCurrent} disabled={busy}>Delete from library</button>
            </div>
          </section>
        {/if}
      {:else}
        <div class="placeholder">
          {#if busy}
            Loading…
          {:else}
            <div class="ph-inner"><div class="ph-logo">📈</div><p>Select a run to view its curves.</p></div>
          {/if}
        </div>
      {/if}
    </main>
  </div>
</div>

{#if current}
  <PrintReport shopName={image?.shopName ?? ""} {current} />
{/if}
