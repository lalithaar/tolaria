# Code Health Report — Laputa App

**Date:** 2026-02-17
**Branch:** `experiment/shadcn-migration`
**Overall Project Score:** 8.34 / 10.0 (Yellow — approaching Green)
**Tool:** CodeScene Code Health Analysis (project ID: 76865)

---

## Summary

The Laputa App codebase scores **8.34** overall — solid but with four files in the "Yellow" zone (< 8.0) that are dragging down maintainability. One file (`vault.rs`) is in the **Red zone** at 4.8, representing the most critical technical debt.

| Zone | Score Range | File Count | Description |
|------|------------|------------|-------------|
| Optimal | 10.0 | 4 | Perfect — optimized for human and AI comprehension |
| Green | 9.0 – 9.9 | 2 | High quality, minor issues only |
| Yellow | 4.0 – 8.9 | 4 | Problematic technical debt |
| Red | 1.0 – 3.9 | 0 | — |
| N/A | — | 2 | CSS files (unsupported by CodeScene) |

---

## File-by-File Scores

| File | LoC | Score | Zone | Key Issues |
|------|-----|-------|------|------------|
| `src-tauri/src/main.rs` | 6 | **10.0** | Optimal | None |
| `src-tauri/src/git.rs` | 423 | **10.0** | Optimal | None |
| `src/mock-tauri.ts` | 707 | **10.0** | Optimal | None |
| `src/components/StatusBar.tsx` | 62 | **10.0** | Optimal | None |
| `src-tauri/src/lib.rs` | 78 | **9.68** | Green | String-heavy function arguments (93%) |
| `src/components/QuickOpenPalette.tsx` | 145 | **9.55** | Green | Complex Method (cc=16) |
| `src/components/Sidebar.tsx` | 233 | **9.02** | Green | Complex Method (cc=25), Large Method (181 LoC) |
| `src/components/NoteList.tsx` | 339 | **8.11** | Yellow | Bumpy Road, Complex Method (cc=30), Large Method (166 LoC) |
| `src/components/Inspector.tsx` | 912 | **7.49** | Yellow | Bumpy Road (x2), Complex Methods (x4), Complex Conditionals |
| `src/App.tsx` | 593 | **7.13** | Yellow | Brain Method: App() at cc=56 / 381 LoC, Bumpy Road, Deep Nesting |
| `src/components/Editor.tsx` | 540 | **6.94** | Yellow | Deep Nesting, Complex Methods (cc=29), Large Method (265 LoC) |
| `src-tauri/src/vault.rs` | 815 | **4.80** | Yellow (near Red) | Bumpy Road (x2), Deep Nesting, Complex Methods (x3), Code Duplication, Large Method, String-heavy args |
| `src/App.css` | 48 | N/A | — | CSS not supported |
| `src/components/Editor.css` | 106 | N/A | — | CSS not supported |

---

## Technical Debt Hotspots

CodeScene reports **0 hotspots** from the project-level analysis (the project may need more commit history to identify change-frequency hotspots). However, based on the code health scores and file sizes, the **de facto hotspots** are:

| Priority | File | Score | LoC | Risk Factor |
|----------|------|-------|-----|-------------|
| 1 | `src-tauri/src/vault.rs` | 4.80 | 815 | Largest Rust file, near-Red health, 8 distinct code smells |
| 2 | `src/components/Editor.tsx` | 6.94 | 540 | Core editor component, deep nesting, high complexity |
| 3 | `src/App.tsx` | 7.13 | 593 | Root component, Brain Method (cc=56, 381 LoC) |
| 4 | `src/components/Inspector.tsx` | 7.49 | 912 | Largest file in codebase, 4 complex methods |

---

## Detailed Analysis — Files Scoring Below 8.0

### 1. `src-tauri/src/vault.rs` — Score: 4.80 (CRITICAL)

This is the **worst-scoring file** in the codebase and the highest priority for refactoring.

**Code Smells Found:**

| Smell | Location | Details | Severity |
|-------|----------|---------|----------|
| Bumpy Road | `update_frontmatter_content` (L452–536) | 4 bumps of nested logic | High |
| Bumpy Road | `days_from_epoch` (L253–285) | 3 bumps | High |
| Deep Nesting | `update_frontmatter_content` (L452–536) | 4 levels of conditionals | High |
| Complex Method | `FrontmatterValue::to_yaml_value` (L371–412) | cc = 17 | Medium |
| Complex Method | `update_frontmatter_content` (L452–536) | cc = 16 | Medium |
| Complex Method | `days_from_epoch` (L253–285) | cc = 14 | Medium |
| Complex Conditional | `to_yaml_value` (L375–377) | 8 complex expressions | Medium |
| Complex Conditional | `format_yaml_key` (L565–566) | 5 complex expressions | Medium |
| Complex Conditional | `days_from_epoch` (L255) | 3 complex expressions | Medium |
| Code Duplication | `update_frontmatter` / `delete_frontmatter_property` (L416–449) | Near-identical functions | Medium |
| Large Method | `parse_md_file` (L101–190) | 78 LoC (limit: 70) | Medium |
| String-Heavy Args | Module-wide | 63% of args are strings | Medium |

**Business Case:** Improving to industry average (5.15) yields 2–7% defect reduction and 1–3% speed improvement.

---

### 2. `src/components/Editor.tsx` — Score: 6.94

**Code Smells Found:**

| Smell | Location | Details | Severity |
|-------|----------|---------|----------|
| Deep Nesting | `expandWikilinksInContent` (L119–146) | 4 levels of conditionals | High |
| Complex Method | `Editor` (L263–540) | cc = 29 | Medium |
| Complex Method | `DiffView` (L148–186) | cc = 13 | Medium |
| Complex Conditional | `DiffView:169` | 4 complex expressions | Medium |
| Complex Conditional | `expandWikilinksInContent:122` | 2 complex expressions | Medium |
| Large Method | `Editor` (L263–540) | 265 LoC (limit: 120) | Medium |
| Overall Code Complexity | File-wide | High mean cyclomatic complexity | Medium |

**Business Case:** Improving to 9.1 yields 27–40% defect reduction and 21–32% speed improvement. **This is the highest-ROI refactoring target.**

---

### 3. `src/App.tsx` — Score: 7.13

**Code Smells Found:**

| Smell | Location | Details | Severity |
|-------|----------|---------|----------|
| Bumpy Road | `updateMockFrontmatter` (L21–88) | 2 bumps | High |
| Deep Nesting | `updateMockFrontmatter` (L21–88) | 4 levels deep | High |
| Complex Method | `App` (L131–591) | cc = 56 (**Brain Method**) | Medium |
| Complex Method | `updateMockFrontmatter` (L21–88) | cc = 17 | Medium |
| Large Method | `App` (L131–591) | 381 LoC (limit: 120, **3x over**) | Medium |

**Business Case:** Improving to 9.1 yields 27–39% defect reduction and 20–31% speed improvement.

---

### 4. `src/components/Inspector.tsx` — Score: 7.49

**Code Smells Found:**

| Smell | Location | Details | Severity |
|-------|----------|---------|----------|
| Bumpy Road | `parseFrontmatter` (L78–150) | 2 bumps | High |
| Bumpy Road | `DynamicPropertiesPanel` (L441–516) | 2 bumps | High |
| Complex Method | `parseFrontmatter` (L78–150) | cc = 21 | Medium |
| Complex Method | `DynamicPropertiesPanel` (L441–516) | cc = 18 | Medium |
| Complex Method | `renderEditableValue` (L527–624) | cc = 16 | Medium |
| Complex Method | `Inspector` (L806–912) | cc = 14 | Medium |
| Complex Conditional | `renderEditableValue:590` | 3 complex expressions | Medium |
| Complex Conditional | `parseFrontmatter:115` | 2 complex expressions | Medium |
| Overall Code Complexity | File-wide | High mean cyclomatic complexity | Medium |

**Business Case:** Improving to 9.1 yields 26–36% defect reduction and 18–28% speed improvement.

---

## Quick Wins (Low Effort, High Impact)

These are targeted refactorings that can improve scores immediately:

### 1. Extract `App` component state into custom hooks
**File:** `src/App.tsx` | **Impact:** cc 56 → ~10 per hook
- Extract `useVaultData()` — vault loading, entry management
- Extract `useEditorState()` — selected entry, editor content, undo/redo
- Extract `useKeyboardShortcuts()` — all keyboard event handlers
- Extract `useSearch()` — search/filter state and logic
- The `App` component becomes a thin composition layer

### 2. Extract `updateMockFrontmatter` helper functions
**File:** `src/App.tsx` (L21–88) | **Impact:** Eliminates deep nesting + bumpy road
- Extract date-comparison logic into a helper
- Extract frontmatter field-update logic per field type
- Flatten conditionals with early returns

### 3. Deduplicate `update_frontmatter` / `delete_frontmatter_property`
**File:** `src-tauri/src/vault.rs` (L416–449) | **Impact:** Eliminates code duplication smell
- Both functions share the same file-read + frontmatter-parse + write-back pattern
- Extract shared logic into a `with_frontmatter<F>(path, f: F)` helper

### 4. Flatten `expandWikilinksInContent`
**File:** `src/components/Editor.tsx` (L119–146) | **Impact:** Eliminates deep nesting
- Use early `continue` in the loop
- Extract inner matching logic into a `resolveWikilink()` helper

### 5. Split `DynamicPropertiesPanel` into sub-components
**File:** `src/components/Inspector.tsx` | **Impact:** Reduces 4 complex methods to focused units
- Extract `EditableValue` component for `renderEditableValue`
- Extract `FrontmatterParser` utility for `parseFrontmatter`
- Extract `PropertyRow` component for individual property rendering

---

## Larger Refactoring Plans

### Plan A: Decompose `vault.rs` (Priority 1 — Score 4.80 → 8+)

**Goal:** Raise from near-Red (4.80) to Green (9+) in 4–5 steps.

| Step | Action | Target Smell |
|------|--------|--------------|
| 1 | Extract `frontmatter.rs` module — move `FrontmatterValue`, `to_yaml_value`, `update_frontmatter_content`, `format_yaml_key` | Reduces file size, isolates complexity |
| 2 | Introduce `with_frontmatter()` helper to deduplicate `update_frontmatter` / `delete_frontmatter_property` | Code Duplication |
| 3 | Refactor `update_frontmatter_content` — flatten nesting with early returns, extract `apply_field_update()` helper | Deep Nesting, Bumpy Road |
| 4 | Refactor `days_from_epoch` — use a lookup table or `chrono` crate helpers instead of manual conditionals | Bumpy Road, Complex Conditional |
| 5 | Replace string args with newtype wrappers (`VaultPath`, `FilePath`, `PropertyKey`) | String-Heavy Args |

### Plan B: Decompose `Editor.tsx` (Priority 2 — Score 6.94 → 9+)

**Goal:** Raise from Yellow (6.94) to Green (9+) in 3–4 steps.

| Step | Action | Target Smell |
|------|--------|--------------|
| 1 | Extract `useEditorExtensions()` hook — all CodeMirror extension setup | Reduces Editor cc from 29 |
| 2 | Extract `useWikilinks()` hook — wikilink expansion, resolution | Deep Nesting, Complex Conditional |
| 3 | Extract `DiffView` into its own file `DiffView.tsx` | Complex Method (cc=13) |
| 4 | Extract `useEditorContent()` hook — content loading, saving, dirty state | Further reduces Editor complexity |

### Plan C: Decompose `App.tsx` (Priority 3 — Score 7.13 → 9+)

**Goal:** Break the Brain Method pattern. The `App` function at cc=56 and 381 LoC is the single worst function in the frontend.

| Step | Action | Target Smell |
|------|--------|--------------|
| 1 | Extract `useVaultLoader()` — vault path selection, entry loading, refresh | Reduces App cc by ~15 |
| 2 | Extract `useNoteActions()` — create, delete, rename, move note | Reduces App cc by ~10 |
| 3 | Extract `useAppKeyboard()` — all keyboard shortcut handlers | Reduces App cc by ~8 |
| 4 | Extract `useFrontmatterSync()` — mock frontmatter update logic | Eliminates Bumpy Road + Deep Nesting |
| 5 | Move layout JSX into `AppLayout.tsx` presentational component | Reduces App to pure orchestration |

### Plan D: Decompose `Inspector.tsx` (Priority 4 — Score 7.49 → 9+)

**Goal:** The largest file (912 LoC) with 4 complex methods needs structural decomposition.

| Step | Action | Target Smell |
|------|--------|--------------|
| 1 | Extract `parseFrontmatter()` to `src/utils/frontmatter.ts` shared utility | Complex Method (cc=21), Bumpy Road |
| 2 | Extract `EditableValue.tsx` component from `renderEditableValue` | Complex Method (cc=16) |
| 3 | Extract `DynamicPropertiesPanel.tsx` as standalone component | Bumpy Road, Complex Method (cc=18) |
| 4 | Simplify `Inspector` to compose sub-components | Reduces cc from 14 to ~5 |

---

## Refactoring ROI Summary

| File | Current | Target | Defect Reduction | Speed Improvement |
|------|---------|--------|------------------|-------------------|
| `Editor.tsx` | 6.94 | 9.1 | 27–40% | 21–32% |
| `App.tsx` | 7.13 | 9.1 | 27–39% | 20–31% |
| `Inspector.tsx` | 7.49 | 9.1 | 26–36% | 18–28% |
| `vault.rs` | 4.80 | 5.15* | 2–7% | 1–3% |

*\*vault.rs target is conservative (industry average). A full refactoring to 9+ would yield dramatically higher ROI but requires more structural work.*

---

## Recommended Execution Order

1. **`vault.rs`** — Lowest score, near-Red zone. Foundational backend code that all frontmatter operations depend on. Fix this first to prevent defect accumulation.
2. **`Editor.tsx`** — Highest ROI. Core user-facing component where bugs are most visible. Decomposition into hooks is straightforward.
3. **`App.tsx`** — Brain Method must be broken up. The cc=56 function is a maintenance trap — any new feature added here increases risk nonlinearly.
4. **`Inspector.tsx`** — Largest file but lower priority because it's more self-contained. Decompose after the above three.
5. **`NoteList.tsx`** (8.11) and **`Sidebar.tsx`** (9.02) — minor improvements, address opportunistically.

---

## Files in Good Shape

These files need no immediate attention:

- `src-tauri/src/main.rs` — 10.0 (6 LoC, clean entry point)
- `src-tauri/src/git.rs` — 10.0 (423 LoC, well-structured)
- `src/mock-tauri.ts` — 10.0 (707 LoC, clean mock layer)
- `src/components/StatusBar.tsx` — 10.0 (62 LoC, minimal component)
- `src-tauri/src/lib.rs` — 9.68 (minor: string-heavy args)
- `src/components/QuickOpenPalette.tsx` — 9.55 (minor: cc=16)

---

*Report generated by CodeScene MCP analysis. For interactive exploration, visit: https://codescene.io/projects/76865*
