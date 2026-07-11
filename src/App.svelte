<script lang="ts">
  import { onMount } from "svelte";
  import { ask, open } from "@tauri-apps/plugin-dialog";
  import * as api from "./lib/api";
  import type { CompareRun, CurrentRun, DecodedRun, ImageSummary, RunEntry, RunIndexEntry, RunRecord } from "./lib/types";
  import RunDetail from "./lib/components/RunDetail.svelte";
  import PrintReport from "./lib/components/PrintReport.svelte";
  import Comparison from "./lib/components/Comparison.svelte";
  import { autoCrop } from "./lib/charts/series";
  import { t, LOCALES, LABEL_KEYS } from "./lib/i18n";
  import type { Locale } from "./lib/i18n";
  import {
    lang,
    unitSys,
    setLanguage,
    setUnitSystem,
    labelOverrides,
    setLabelOverride,
    resetLabelOverrides,
    logoDataUri,
    setLogo,
    printHeaderText,
    setPrintHeaderText,
  } from "./lib/settings.svelte";
  import type { UnitSystem } from "./lib/units";

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
  let showSettings = $state(false);
  let crop = $state({ start: 0, end: 1 });
  let dateInput = $state("");
  let tempOverride = $state("");
  let pressOverride = $state("");
  // Comparison: library ids selected for overlay, and the loaded runs to show.
  let compareSel = $state<string[]>([]);
  let compareRuns = $state<CompareRun[] | null>(null);

  function baseName(p: string): string {
    return p.split(/[\\/]/).pop() ?? p;
  }
  // Show a run: default the trim to the detected pull, seed the date/reading editors.
  function showRun(c: CurrentRun) {
    current = c;
    crop = autoCrop(c.channels, c.results.kDin);
    dateInput = c.date ?? "";
    tempOverride = c.overrides.tempC != null ? String(c.overrides.tempC) : "";
    pressOverride = c.overrides.pressureHpa != null ? String(c.overrides.pressureHpa) : "";
  }
  function setDateNow() {
    dateInput = new Date().toISOString().slice(0, 10);
  }
  function fromDecoded(run: DecodedRun, title: string, shopName: string | null): CurrentRun {
    return {
      title,
      date: run.date,
      description: "",
      results: run.results,
      channels: run.channels,
      shopName,
      overrides: { tempC: null, pressureHpa: null },
      libId: null,
    };
  }
  function fromRecord(rec: RunRecord): CurrentRun {
    return {
      title: rec.sourceEntry ?? rec.id.slice(0, 8),
      date: rec.runDate,
      description: rec.description,
      results: rec.results,
      channels: rec.channels,
      shopName: rec.shopName,
      overrides: rec.valueOverrides ?? { tempC: null, pressureHpa: null },
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
      showRun(fromDecoded(await api.openErgFile(path), baseName(path), null));
      selectedKey = path;
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function selectImageRun(r: RunEntry) {
    if (!image) return;
    selectedKey = r.path;
    busy = true; err = null;
    try {
      showRun(fromDecoded(await api.readErgFromImage(image.imagePath, r.path), r.name, image.shopName));
    } catch (e) { err = String(e); current = null; } finally { busy = false; }
  }

  async function selectLibRun(entry: RunIndexEntry) {
    selectedKey = entry.id;
    compareRuns = null;
    busy = true; err = null;
    try {
      showRun(fromRecord(await api.getDbRun(entry.id)));
    } catch (e) { err = String(e); current = null; } finally { busy = false; }
  }

  async function refreshLibrary() {
    try { library = await api.listDbRuns(libQuery); } catch (e) { err = String(e); }
  }

  async function switchMode(m: Mode) {
    mode = m;
    current = null;
    selectedKey = null;
    compareRuns = null;
    compareSel = [];
    if (m === "library") await refreshLibrary();
  }

  async function importNew() {
    if (!image) return;
    const paths = image.runs.filter((r) => !r.deleted && !r.inLibrary).map((r) => r.path);
    if (!paths.length) { toast = t("toast.nothingNew"); return; }
    busy = true; err = null;
    try {
      const rep = await api.importRuns(image.imagePath, paths, false);
      toast = t("toast.imported", { added: rep.added.length, skipped: rep.skipped.length });
      image = await api.openImage(image.imagePath);
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function importAllOverwrite() {
    if (!image) return;
    const ok = await ask(t("dialog.importAllMsg", { count: image.runs.filter((r) => !r.deleted).length }), {
      title: t("dialog.importAllTitle"), kind: "warning",
    });
    if (!ok) return;
    busy = true; err = null;
    try {
      const rep = await api.importAll(image.imagePath, true, false);
      toast = t("toast.importedOver", { added: rep.added.length, overwritten: rep.overwritten.length });
      image = await api.openImage(image.imagePath);
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function doReset() {
    if (!image) return;
    const ok = await ask(t("dialog.resetMsg", { path: image.imagePath }), {
      title: t("dialog.resetTitle"), kind: "warning",
    });
    if (!ok) return;
    busy = true; err = null;
    try {
      const rep = await api.resetImage(image.imagePath, true);
      toast = t("toast.wiped", { deleted: rep.deleted.length, path: rep.backupPath });
      image = await api.openImage(image.imagePath);
      current = null; selectedKey = null;
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  async function saveDescription() {
    if (!current?.libId) return;
    busy = true; err = null;
    try {
      await api.updateRunDescription(current.libId, current.description);
      if (dateInput && dateInput !== (current.date ?? "")) {
        await api.updateRunDate(current.libId, dateInput);
        current.date = dateInput;
      }
      toast = t("toast.descSaved");
      await refreshLibrary();
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  // Persist the display-only reading overrides (temperature / pressure, °C / hPa).
  async function saveReadings() {
    if (!current?.libId) return;
    const parse = (s: string) => (s.trim() === "" ? null : Math.round(Number(s)));
    const ov = { tempC: parse(tempOverride), pressureHpa: parse(pressOverride) };
    if ((ov.tempC != null && Number.isNaN(ov.tempC)) || (ov.pressureHpa != null && Number.isNaN(ov.pressureHpa))) return;
    busy = true; err = null;
    try {
      await api.updateRunOverrides(current.libId, ov);
      current.overrides = ov;
      toast = t("toast.readingsSaved");
    } catch (e) { err = String(e); } finally { busy = false; }
  }

  // --- comparison -----------------------------------------------------------
  function toggleCompare(id: string) {
    compareSel = compareSel.includes(id) ? compareSel.filter((x) => x !== id) : [...compareSel, id];
  }
  async function startCompare() {
    if (compareSel.length < 2) return;
    busy = true; err = null;
    try {
      const recs = await Promise.all(compareSel.map((id) => api.getDbRun(id)));
      compareRuns = recs.map((rec) => ({
        id: rec.id,
        title: rec.description || rec.sourceEntry || rec.id.slice(0, 8),
        date: rec.runDate,
        channels: rec.channels,
        kDin: rec.results.kDin,
      }));
      current = null;
      selectedKey = null;
    } catch (e) { err = String(e); } finally { busy = false; }
  }
  function closeCompare() {
    compareRuns = null;
  }

  async function chooseLogo() {
    const path = await open({
      filters: [{ name: "Image", extensions: ["png", "jpg", "jpeg", "gif", "webp", "svg", "bmp"] }],
    });
    if (typeof path !== "string") return;
    try {
      await setLogo(await api.readImageDataUri(path));
    } catch (e) { err = String(e); }
  }

  async function deleteCurrent() {
    if (!current?.libId) return;
    const ok = await ask(t("dialog.deleteMsg"), { title: t("dialog.deleteTitle"), kind: "warning" });
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

  async function revealFolder() {
    try {
      await api.openDataFolder();
    } catch (e) { err = String(e); }
  }

  const newCount = $derived(image ? image.runs.filter((r) => !r.deleted && !r.inLibrary).length : 0);

  onMount(async () => {
    try {
      const p = await api.initialPath();
      if (!p) return;
      busy = true;
      if (p.toLowerCase().endsWith(".erg")) {
        showRun(fromDecoded(await api.openErgFile(p), baseName(p), null));
      } else {
        image = await api.openImage(p);
        const first = image.runs.find((r) => !r.deleted) ?? image.runs[0];
        if (first) await selectImageRun(first);
      }
    } catch (e) { err = String(e); } finally { busy = false; }
  });
</script>

<svelte:window onkeydown={(e) => { if (e.key === "Escape") showSettings = false; }} />

<div class="app">
  <header class="toolbar">
    <div class="brand">FLA&nbsp;Dynoview</div>
    <button onclick={pickImage} disabled={busy}>{t("app.openImg")}</button>
    <button onclick={pickErg} disabled={busy}>{t("app.openErg")}</button>
    <div class="tabs">
      <button class:tabsel={mode === "browse"} onclick={() => switchMode("browse")}>{t("app.tabDisk")}</button>
      <button class:tabsel={mode === "library"} onclick={() => switchMode("library")}>{t("app.tabLibrary")}</button>
    </div>
    <div class="spacer"></div>
    {#if current}
      <button class="primary" onclick={printReport}>{t("app.printPdf")}</button>
    {/if}
    <button class="icon" title={t("app.settings")} onclick={() => (showSettings = true)}>⚙</button>
    {#if current?.shopName}
      <div class="shop">🏁 {current.shopName}</div>
    {:else if image && mode === "browse"}
      <div class="shop">🏁 {image.shopName || t("app.shopFallback")}</div>
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
            <button onclick={importNew} disabled={busy || newCount === 0}>{t("app.importNew", { count: newCount })}</button>
            <button onclick={importAllOverwrite} disabled={busy}>{t("app.importAll")}</button>
            <button class="danger" onclick={doReset} disabled={busy}>{t("app.resetDisk")}</button>
          </div>
          <div class="side-head">{t("app.runsHeader", { count: image.runs.length })}</div>
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
                  {#if r.inLibrary}<span class="chk" title={t("app.inLibrary")}>✓</span>{/if}
                  {#if r.deleted}<span class="tag">{t("app.deleted")}</span>{/if}
                </button>
              </li>
            {/each}
          </ul>
        {:else}
          <div class="empty"><p>{t("app.browseEmpty")}</p></div>
        {/if}
      {:else}
        <div class="side-actions">
          <input class="search" placeholder={t("app.searchPlaceholder")} bind:value={libQuery} oninput={refreshLibrary} />
          {#if compareSel.length >= 2}
            <button class="primary" onclick={startCompare} disabled={busy}>{t("app.compare", { count: compareSel.length })}</button>
          {:else if compareSel.length === 1}
            <div class="cmp-hint">{t("compare.selectHint")}</div>
          {/if}
        </div>
        <div class="side-head">{t("app.libraryHeader", { count: library.length })}</div>
        <ul class="runlist">
          {#each library as e (e.id)}
            <li class="librow">
              <input
                class="cmp-check"
                type="checkbox"
                checked={compareSel.includes(e.id)}
                onchange={() => toggleCompare(e.id)}
                title={t("compare.checkTitle")}
              />
              <button class="runitem libitem" class:active={selectedKey === e.id} onclick={() => selectLibRun(e)}>
                <span class="rdate strong">{e.runDate ?? "—"}</span>
                <span class="rdesc">{e.description || e.sourceImage || t("app.noDescription")}</span>
              </button>
            </li>
          {:else}
            <li class="empty"><p>{t("app.libraryEmpty")}</p></li>
          {/each}
        </ul>
      {/if}
    </aside>

    <main class="content">
      {#if compareRuns}
        <Comparison runs={compareRuns} onClose={closeCompare} />
      {:else if current}
        <RunDetail {current} bind:crop />
        {#if current.libId}
          <section class="desc-editor">
            <div class="date-row">
              <label for="rundate">{t("app.runDate")}</label>
              <input id="rundate" type="date" bind:value={dateInput} />
              <button onclick={setDateNow} disabled={busy}>{t("app.dateNow")}</button>
              <span class="readings">
                <label for="ovtemp">{t("abbr.lamp")}</label>
                <input id="ovtemp" class="ovnum" type="number" value={tempOverride} oninput={(e) => (tempOverride = e.currentTarget.value)} placeholder={`${current.results.tempC ?? ""}`} />
                <span class="ovunit">°C</span>
                <label for="ovpress">{t("abbr.paine")}</label>
                <input id="ovpress" class="ovnum" type="number" value={pressOverride} oninput={(e) => (pressOverride = e.currentTarget.value)} placeholder={`${current.results.pressureHpa ?? ""}`} />
                <span class="ovunit">hPa</span>
                <button onclick={saveReadings} disabled={busy}>{t("app.saveReadings")}</button>
              </span>
            </div>
            <label for="desc">{t("app.description")}</label>
            <textarea id="desc" rows="2" bind:value={current.description} placeholder={t("app.descPlaceholder")} onblur={saveDescription}></textarea>
            <div class="desc-actions">
              <button class="primary" onclick={saveDescription} disabled={busy}>{t("app.save")}</button>
              <button class="danger" onclick={deleteCurrent} disabled={busy}>{t("app.deleteFromLibrary")}</button>
            </div>
          </section>
        {/if}
      {:else}
        <div class="placeholder">
          {#if busy}
            {t("app.loading")}
          {:else}
            <div class="ph-inner"><div class="ph-logo">📈</div><p>{t("app.selectRun")}</p></div>
          {/if}
        </div>
      {/if}
    </main>
  </div>
</div>

{#if showSettings}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" role="presentation" onclick={() => (showSettings = false)}>
    <div class="modal" role="dialog" aria-modal="true" tabindex="-1" onclick={(e) => e.stopPropagation()}>
      <h2>{t("settings.title")}</h2>
      <label class="field">
        {t("settings.language")}
        <select value={lang()} onchange={(e) => setLanguage(e.currentTarget.value as Locale)}>
          {#each LOCALES as l (l.code)}<option value={l.code}>{l.name}</option>{/each}
        </select>
      </label>
      <label class="field">
        {t("settings.units")}
        <select value={unitSys()} onchange={(e) => setUnitSystem(e.currentTarget.value as UnitSystem)}>
          <option value="metric">{t("settings.unitMetric")}</option>
          <option value="imperial">{t("settings.unitImperial")}</option>
        </select>
      </label>

      <div class="field-group">
        <div class="fg-head">
          <span>{t("settings.fieldLabels")}</span>
          <button class="link" onclick={resetLabelOverrides}>{t("settings.resetLabels")}</button>
        </div>
        <p class="fg-hint">{t("settings.fieldLabelsHint")}</p>
        <div class="label-grid">
          {#each LABEL_KEYS as k (k)}
            <span class="lbl-default">{t(k)}</span>
            <input
              class="lbl-input"
              value={labelOverrides()[k] ?? ""}
              placeholder={t(k)}
              onchange={(e) => setLabelOverride(k, e.currentTarget.value)}
            />
          {/each}
        </div>
      </div>

      <div class="field-group">
        <div class="fg-head"><span>{t("settings.logo")}</span></div>
        <div class="logo-row">
          {#if logoDataUri()}<img class="logo-prev" src={logoDataUri()} alt="" />{/if}
          <button onclick={chooseLogo}>{t("settings.chooseLogo")}</button>
          {#if logoDataUri()}<button class="danger" onclick={() => setLogo(null)}>{t("settings.removeLogo")}</button>{/if}
        </div>
      </div>

      <label class="field">
        {t("settings.headerText")}
        <input
          value={printHeaderText() ?? ""}
          placeholder={t("settings.headerHint")}
          onchange={(e) => setPrintHeaderText(e.currentTarget.value)}
        />
      </label>

      <div class="modal-actions">
        <button onclick={revealFolder}>{t("settings.revealFolder")}</button>
        <button class="primary" onclick={() => (showSettings = false)}>{t("settings.close")}</button>
      </div>
    </div>
  </div>
{/if}

{#if current}
  <PrintReport {current} {crop} />
{/if}
