const assert = require('node:assert/strict');
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
  invertQuote,
  isQuoteFresh,
  isValidCustomCode,
  normalizeCustomInstrument,
  normalizeState,
  quoteStatus,
  quoteStatusTitle,
  quoteTimestamp,
} = window.__MARKETPEEK_TEST_API;

test('normalizeState migrates malformed persisted settings to safe values', () => {
  const state = normalizeState({
    selected: ['sse', 'sse', 7, 'gold', 'dji', 'fx', 'extra'],
    theme: 'missing',
    updown: 'unexpected',
    interval: -1,
    lang: 'fr',
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
