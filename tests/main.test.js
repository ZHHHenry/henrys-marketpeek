const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const test = require('node:test');

global.localStorage = {
  getItem() {
    return null;
  },
  setItem() {},
};

global.window = {
  __MARKETPEEK_SKIP_INIT__: true,
  __MARKETPEEK_TEST__: true,
  __TAURI__: {
    core: { invoke() {} },
    event: { listen() {} },
    window: { getCurrentWindow() { return {}; } },
  },
};

require('../src/main.js');

const {
  STR,
  THEMES,
  invertQuote,
  isQuoteFresh,
  isValidCustomCode,
  nextRefreshDelayMs,
  normalizeCustomInstrument,
  normalizeState,
  quoteStatus,
  quoteStatusTitle,
  quoteTimestamp,
} = window.__MARKETPEEK_TEST_API;

const projectRoot = path.resolve(__dirname, '..');

test('normalizeState migrates malformed persisted settings to safe values', () => {
  const state = normalizeState({
    selected: ['sse', 'sse', 7, 'gold', 'dji', 'fx', 'extra'],
    theme: 'missing',
    updown: 'unexpected',
    interval: -1,
    lang: 'fr',
    pinned: 'yes',
    custom: [
      { name: 'Euro', code: 'whEURUSD', source: 'tencent', decimals: 99 },
      { name: 'Injected', code: 'sh000001&x=1', source: 'tencent' },
    ],
    swapped: { fx: true, bad: 'yes' },
  });

  assert.deepEqual(state.selected, ['sse', 'gold', 'dji', 'fx']);
  assert.equal(state.theme, 'shimo');
  assert.equal(state.updown, 'cn');
  assert.equal(state.interval, 10);
  assert.equal(state.lang, 'zh');
  assert.equal(state.pinned, true);
  assert.equal(normalizeState({ pinned: false }).pinned, false);
  assert.deepEqual(state.custom, [{
    id: 'c_whEURUSD',
    name: 'Euro',
    code: 'whEURUSD',
    source: 'tencent',
    decimals: 4,
  }]);
  assert.deepEqual(state.swapped, { fx: true });
});

test('custom code validation follows the parsers supported by each source', () => {
  assert.equal(isValidCustomCode('whEURUSD', 'tencent'), true);
  assert.equal(isValidCustomCode('nf_AU0', 'sina'), true);
  assert.equal(isValidCustomCode('sh600000', 'sina'), false);
  assert.equal(isValidCustomCode('100.KS11', 'eastmoney'), true);
  assert.equal(isValidCustomCode('KS11', 'eastmoney'), false);
  assert.equal(normalizeCustomInstrument({ name: '', code: 'whEURUSD', source: 'tencent' }), null);
});

test('freshness uses source time when present and receipt time as a safe fallback', () => {
  const now = 2_000_000;
  assert.equal(isQuoteFresh({ ts: now - 1_000, receivedAt: now - 100_000 }, now), true);
  assert.equal(isQuoteFresh({ ts: now - 181_000, receivedAt: now }, now), false);
  assert.equal(isQuoteFresh({ ts: 0, receivedAt: now - 1_000 }, now), true);
  assert.equal(isQuoteFresh({ ts: now + 600_000, receivedAt: now - 1_000 }, now), true);
  assert.equal(isQuoteFresh({ ts: 0, receivedAt: now - 181_000 }, now), false);
  assert.equal(quoteTimestamp({ ts: now + 600_000, receivedAt: now - 1_000 }, now), now - 1_000);
});

test('polling slows only when every selected quote is stale', () => {
  const now = 2_000_000;
  const quoteMap = new Map([
    ['selected-stale', { ts: now - 181_000 }],
    ['selected-fresh', { ts: now - 1_000 }],
    ['deselected-fresh', { ts: now - 1_000 }],
  ]);
  assert.equal(nextRefreshDelayMs(['selected-stale'], quoteMap, 10, now), 60_000);
  assert.equal(nextRefreshDelayMs(['selected-stale', 'selected-fresh'], quoteMap, 10, now), 10_000);
  assert.equal(nextRefreshDelayMs(['selected-stale'], quoteMap, 5, now), 60_000);
  assert.equal(nextRefreshDelayMs([], quoteMap, 30, now), 60_000);
});

test('card status prioritizes failures and keeps technical details out of the tooltip', () => {
  const now = 2_000_000;
  const quote = { ts: now - 181_000, receivedAt: now };
  assert.equal(quoteStatus('sse', quote, now, new Map()).kind, 'stale');

  const failure = { id: 'sse', source: 'tencent', message: 'request failed: secret detail' };
  const status = quoteStatus('sse', quote, now, new Map([['sse', failure]]));
  assert.equal(status.kind, 'error');
  const title = quoteStatusTitle(status, quote);
  assert.match(title, /数据源暂时不可用/);
  assert.doesNotMatch(title, /secret detail/);
});

test('FX inversion preserves the mathematically inverted change direction', () => {
  const inverted = invertQuote({ price: 2, change: 0.5, pct: 33.33, ts: 1 });
  assert.equal(inverted.price, 0.5);
  assert.ok(Math.abs(inverted.change + 1 / 6) < 1e-12);
  assert.ok(Math.abs(inverted.pct + 25) < 1e-12);
});

test('theme definitions and translation keys stay synchronized', () => {
  const css = fs.readFileSync(path.join(projectRoot, 'src', 'styles.css'), 'utf8');
  const cssThemeIds = new Set(
    [...css.matchAll(/html\[data-theme="([^"]+)"\]\s*\{/g)].map((match) => match[1]),
  );
  assert.deepEqual([...cssThemeIds].sort(), THEMES.map((theme) => theme.id).sort());
  assert.deepEqual(Object.keys(STR.zh).sort(), Object.keys(STR.en).sort());
});

test('the custom instrument dialog is excluded from accessibility until opened', () => {
  const html = fs.readFileSync(path.join(projectRoot, 'src', 'index.html'), 'utf8');
  const main = fs.readFileSync(path.join(projectRoot, 'src', 'main.js'), 'utf8');
  assert.match(
    html,
    /id="add-dialog"[^>]*class="[^"]*hidden[^"]*"[^>]*aria-hidden="true"[^>]*\binert\b/,
  );
  assert.match(main, /dialog\.removeAttribute\('inert'\)/);
  assert.match(main, /dialog\.setAttribute\('aria-hidden', 'false'\)/);
  assert.match(main, /dialog\.setAttribute\('aria-hidden', 'true'\)/);
  assert.match(main, /dialog\.setAttribute\('inert', ''\)/);
});

test('release version remains synchronized across all three manifests', () => {
  const packageVersion = JSON.parse(fs.readFileSync(path.join(projectRoot, 'package.json'), 'utf8')).version;
  const tauriVersion = JSON.parse(
    fs.readFileSync(path.join(projectRoot, 'src-tauri', 'tauri.conf.json'), 'utf8'),
  ).version;
  const cargo = fs.readFileSync(path.join(projectRoot, 'src-tauri', 'Cargo.toml'), 'utf8');
  const cargoVersion = cargo.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
  assert.equal(packageVersion, tauriVersion);
  assert.equal(packageVersion, cargoVersion);
});

test('vendored winres patch continues to preserve apostrophes in RC paths', () => {
  const helper = fs.readFileSync(
    path.join(projectRoot, 'src-tauri', 'vendor', 'tauri-winres', 'src', 'helpers.rs'),
    'utf8',
  );
  assert.ok(helper.includes(String.raw`'\'' => escaped.push('\''),`));
});

test('the DAX instrument uses the current official name', () => {
  const quotes = fs.readFileSync(path.join(projectRoot, 'src-tauri', 'src', 'quotes.rs'), 'utf8');
  const dax = quotes.match(/Instrument \{\s*id: "dax",[\s\S]*?\n\s*\},/)?.[0] || '';
  assert.match(dax, /name: "德国DAX"/);
  assert.match(dax, /name_en: "DAX"/);
  assert.doesNotMatch(dax, /DAX\s*30/);
});
