const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const appWindow = window.__TAURI__.window.getCurrentWindow();

const THEMES = [
  { id: 'shimo', name: '石墨', nameEn: 'Graphite', bg: '#e9eaee', accent: '#5b6472' },
  { id: 'yuebai', name: '月白', nameEn: 'Moon White', bg: '#f2f0ea', accent: '#8a6f4d' },
  { id: 'wulan', name: '雾蓝', nameEn: 'Mist Blue', bg: '#e4eaf2', accent: '#4a6fa5' },
  { id: 'mocha', name: '抹茶', nameEn: 'Matcha', bg: '#e8eddf', accent: '#6b8f4e' },
  { id: 'naihuang', name: '奶黄', nameEn: 'Cream', bg: '#f5eedd', accent: '#c08a2d' },
  { id: 'yingfen', name: '樱粉', nameEn: 'Sakura', bg: '#f5e7e8', accent: '#b56a75' },
  { id: 'bohe', name: '薄荷', nameEn: 'Mint', bg: '#e1eeea', accent: '#3f907f' },
  { id: 'muye', name: '暮紫', nameEn: 'Dusk Purple', bg: '#eae6f0', accent: '#7a63a8' },
  { id: 'shuini', name: '水泥', nameEn: 'Cement', bg: '#e3e5e7', accent: '#667080' },
  { id: 'tanhei', name: '碳黑', nameEn: 'Carbon', bg: '#2b2e33', accent: '#8ab4f8' },
];

const STR = {
  zh: {
    selectInstruments: '选择品种',
    pinWindow: '窗口置顶',
    settings: '设置',
    panelInstruments: '选择品种',
    maxHint: '最多显示 4 张',
    panelSettings: '设置',
    theme: '主题配色',
    updown: '涨跌颜色',
    redUp: '红涨绿跌',
    greenUp: '绿涨红跌',
    interval: '刷新间隔',
    sec5: '5 秒',
    sec10: '10 秒',
    sec30: '30 秒',
    language: '语言',
    hideToTray: '隐藏到托盘',
    quit: '退出',
    disclaimer: '行情数据来自腾讯 / 新浪 / 东方财富公开接口，仅供参考，不构成投资建议。',
    shownGroup: '已显示（点击移除）',
    toastMax: '最多显示 4 张，先关一张',
    addInstrument: '添加自选品种',
    nameLabel: '名称',
    codeLabel: '接口代码',
    sourceLabel: '数据源',
    srcTencent: '腾讯',
    srcSina: '新浪',
    srcEastmoney: '东方财富',
    dialogAdd: '添加',
    dialogCancel: '取消',
    addMissing: '请填写名称和代码',
    addInvalidCode: '代码格式不符合所选数据源',
    addDup: '该代码已在列表中存在',
    addNoQuote: '未获取到行情，请检查代码与来源',
    addOk: '已添加自选品种',
    removeInstrument: '删除自选',
    swapFx: '交换货币对',
    quoteStale: '行情超过 3 分钟未更新',
    quoteSourceUnavailable: '数据源暂时不可用',
    quoteInvalidResponse: '数据源未返回有效行情',
    quoteNoData: '暂无可用数据',
    quoteLastUpdated: '上次更新',
    reorderInstrument: '调整品种顺序',
    codeHintTencent: '示例：sh000001 / hf_GC / whEURUSD',
    codeHintSina: '仅支持 nf_ 期货或 int_ 国际指数，如 nf_AU0',
    codeHintEastmoney: '使用 market.symbol 格式，如 100.KS11',
  },
  en: {
    selectInstruments: 'Select instruments',
    pinWindow: 'Always on top',
    settings: 'Settings',
    panelInstruments: 'Instruments',
    maxHint: 'Up to 4 shown',
    panelSettings: 'Settings',
    theme: 'Theme',
    updown: 'Up / down colors',
    redUp: 'Red up, green down',
    greenUp: 'Green up, red down',
    interval: 'Refresh interval',
    sec5: '5 s',
    sec10: '10 s',
    sec30: '30 s',
    language: 'Language',
    hideToTray: 'Hide to tray',
    quit: 'Quit',
    disclaimer: 'Quotes from Tencent / Sina / Eastmoney public APIs. For reference only, not investment advice.',
    shownGroup: 'Shown (click to remove)',
    toastMax: 'Up to 4 cards — remove one first',
    addInstrument: 'Add custom instrument',
    nameLabel: 'Name',
    codeLabel: 'Code',
    sourceLabel: 'Source',
    srcTencent: 'Tencent',
    srcSina: 'Sina',
    srcEastmoney: 'Eastmoney',
    dialogAdd: 'Add',
    dialogCancel: 'Cancel',
    addMissing: 'Enter a name and code',
    addInvalidCode: 'Code format does not match the selected source',
    addDup: 'Code already exists',
    addNoQuote: 'No quote returned — check code and source',
    addOk: 'Custom instrument added',
    removeInstrument: 'Remove',
    swapFx: 'Swap base/quote currencies',
    quoteStale: 'Quote has not updated for over 3 minutes',
    quoteSourceUnavailable: 'Quote source is temporarily unavailable',
    quoteInvalidResponse: 'Quote source returned no valid data',
    quoteNoData: 'No quote is available',
    quoteLastUpdated: 'Last updated',
    reorderInstrument: 'Reorder instrument',
    codeHintTencent: 'Examples: sh000001 / hf_GC / whEURUSD',
    codeHintSina: 'Only nf_ futures or int_ global indices, e.g. nf_AU0',
    codeHintEastmoney: 'Use market.symbol format, e.g. 100.KS11',
  },
};

const CURRENCY_ZH = {
  USD: '美元',
  CNY: '人民币',
  EUR: '欧元',
  JPY: '日元',
  HKD: '港币',
  GBP: '英镑',
  AUD: '澳元',
  NZD: '新西兰元',
  CAD: '加元',
  CHF: '瑞士法郎',
  SGD: '新加坡元',
  KRW: '韩元',
  THB: '泰铢',
  TWD: '新台币',
  MYR: '马来西亚林吉特',
};

const SETTINGS_SCHEMA = 2;
const DEFAULTS = { schemaVersion: SETTINGS_SCHEMA, selected: ['sse', 'gold', 'dji'], theme: 'shimo', updown: 'cn', interval: 10, lang: 'zh', pinned: true, custom: [], swapped: {} };
const STORE_KEY = 'mp-settings-v1';
const VALID_INTERVALS = new Set([5, 10, 30]);
const VALID_SOURCES = new Set(['tencent', 'sina', 'eastmoney']);
const MAX_FUTURE_SKEW_MS = 5 * 60 * 1000;
const STALE_AFTER_MS = 180 * 1000;

let poolInstruments = [];
let instruments = [];
let byId = new Map();
let state = loadState();
let quotes = new Map();
let quoteFailures = new Map();
let paused = false;
let timer = null;
let suppressRowClickUntil = 0;
let refreshRevision = 0;
let completedRefreshRevision = 0;
let refreshLoopPromise = null;

function loadState() {
  try {
    const raw = localStorage.getItem(STORE_KEY);
    if (raw) return normalizeState(JSON.parse(raw));
  } catch (e) {
    console.warn('settings load failed', e);
  }
  return normalizeState(DEFAULTS);
}

function normalizeState(value) {
  const input = value && typeof value === 'object' ? value : {};
  const custom = Array.isArray(input.custom)
    ? input.custom.map(normalizeCustomInstrument).filter(Boolean)
    : [];
  const uniqueCustom = [];
  const customCodes = new Set();
  for (const item of custom) {
    if (customCodes.has(item.code)) continue;
    customCodes.add(item.code);
    uniqueCustom.push(item);
  }
  const selected = Array.isArray(input.selected)
    ? [...new Set(input.selected.filter((id) => typeof id === 'string'))].slice(0, 4)
    : [...DEFAULTS.selected];
  const swapped = {};
  if (input.swapped && typeof input.swapped === 'object' && !Array.isArray(input.swapped)) {
    for (const [id, value] of Object.entries(input.swapped)) {
      if (typeof id === 'string' && typeof value === 'boolean') swapped[id] = value;
    }
  }
  return {
    schemaVersion: SETTINGS_SCHEMA,
    selected,
    theme: THEMES.some((theme) => theme.id === input.theme) ? input.theme : DEFAULTS.theme,
    updown: input.updown === 'intl' ? 'intl' : 'cn',
    interval: VALID_INTERVALS.has(Number(input.interval)) ? Number(input.interval) : DEFAULTS.interval,
    lang: input.lang === 'en' ? 'en' : 'zh',
    pinned: typeof input.pinned === 'boolean' ? input.pinned : DEFAULTS.pinned,
    custom: uniqueCustom,
    swapped,
  };
}

function normalizeCustomInstrument(value) {
  if (!value || typeof value !== 'object') return null;
  const name = typeof value.name === 'string' ? value.name.trim().slice(0, 24) : '';
  const code = typeof value.code === 'string' ? value.code.trim().slice(0, 32) : '';
  const source = VALID_SOURCES.has(value.source) ? value.source : '';
  if (!name || !isValidCustomCode(code, source)) return null;
  return {
    id: `c_${code}`,
    name,
    code,
    source,
    decimals: code.toLowerCase().startsWith('wh') ? 4 : 2,
  };
}

function isValidCustomCode(code, source) {
  if (!VALID_SOURCES.has(source) || !/^[A-Za-z0-9_.]{1,32}$/.test(code)) return false;
  if (source === 'sina') return code.startsWith('nf_') || code.startsWith('int_');
  if (source === 'eastmoney') return /^[A-Za-z0-9]+\.[A-Za-z0-9]+$/.test(code);
  return true;
}

function saveState() {
  try {
    localStorage.setItem(STORE_KEY, JSON.stringify(state));
  } catch (e) {
    console.warn('settings save failed', e);
  }
}

function t(key) {
  const dict = STR[state.lang] || STR.zh;
  return dict[key] || STR.zh[key] || key;
}

function instName(inst) {
  return state.lang === 'en' ? inst.nameEn : inst.name;
}

function instCategory(inst) {
  return state.lang === 'en' ? inst.categoryEn : inst.category;
}

async function init() {
  poolInstruments = await invoke('get_instruments');
  rebuildInstruments();
  reconcileState();

  bindEvents();
  renderAll();
  fillVersion();

  try {
    await appWindow.setAlwaysOnTop(state.pinned);
  } catch (e) {
    console.warn('pin restore failed', e);
  }

  invoke('set_language', { lang: state.lang }).catch((e) => console.warn('language sync failed', e));

  await listen('mp-hidden', () => {
    paused = true;
    clearTimeout(timer);
  });
  await listen('mp-shown', () => {
    paused = false;
    tick();
  });

  await refresh();
  if (!paused) schedule(state.interval * 1000);
}

function reconcileState() {
  const validIds = new Set(instruments.map((instrument) => instrument.id));
  state.selected = state.selected.filter((id) => validIds.has(id)).slice(0, 4);
  state.swapped = Object.fromEntries(
    Object.entries(state.swapped || {}).filter(([id, value]) => validIds.has(id) && typeof value === 'boolean'),
  );
  saveState();
}

function rebuildInstruments() {
  const customs = (state.custom || []).map((c) => ({
    id: c.id,
    name: c.name,
    nameEn: c.name,
    short: c.name,
    shortEn: c.name,
    category: '自选',
    categoryEn: 'Custom',
    decimals: c.decimals,
    code: c.code,
    source: c.source,
    custom: true,
  }));
  instruments = [...customs, ...poolInstruments];
  byId = new Map(instruments.map((i) => [i.id, i]));
}

function applyState() {
  document.documentElement.dataset.theme = state.theme;
  document.documentElement.lang = state.lang === 'en' ? 'en' : 'zh-CN';
  document.body.className = 'updown-' + state.updown;
  document.getElementById('btn-pin').classList.toggle('active', state.pinned);
  document.getElementById('btn-pin').setAttribute('aria-pressed', String(state.pinned));
}

function applyI18n() {
  for (const el of document.querySelectorAll('[data-i18n]')) {
    el.textContent = t(el.dataset.i18n);
  }
  for (const el of document.querySelectorAll('[data-i18n-label]')) {
    el.setAttribute('aria-label', t(el.dataset.i18nLabel));
  }
}

function renderAll() {
  applyState();
  applyI18n();
  renderAddSourceState();
  renderThemeGrid();
  renderSettingsState();
  renderCards();
  renderInstrumentList();
}

function fmtPrice(value, decimals) {
  return value.toLocaleString('en-US', {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  });
}

function fmtPct(pct) {
  const sign = pct > 0 ? '+' : pct < 0 ? '-' : '';
  return sign + Math.abs(pct).toFixed(2) + '%';
}

function fmtChange(value, decimals) {
  const sign = value > 0 ? '+' : value < 0 ? '-' : '';
  return sign + Math.abs(value).toLocaleString('en-US', {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  });
}

function isSwappable(inst) {
  return /^wh[A-Za-z]{6}$/.test(inst.code || '');
}

function defaultSwapRequired(inst) {
  const m = /^wh([A-Za-z]{3})([A-Za-z]{3})$/.exec(inst.code || '');
  if (!m) return false;
  const base = m[1].toUpperCase();
  const quote = m[2].toUpperCase();
  if (quote === 'USD') return true;
  if (base === 'CNY') return true;
  return false;
}

function isSwapped(inst) {
  const explicit = state.swapped ? state.swapped[inst.id] : undefined;
  if (typeof explicit === 'boolean') return explicit;
  return defaultSwapRequired(inst);
}

function swapDisplayName(inst) {
  const m = /^wh([A-Za-z]{3})([A-Za-z]{3})$/.exec(inst.code || '');
  if (!m) return instName(inst);
  const a = m[2].toUpperCase();
  const b = m[1].toUpperCase();
  const zhA = CURRENCY_ZH[a];
  const zhB = CURRENCY_ZH[b];
  if (state.lang === 'en' || !zhA || !zhB) return `${a}/${b}`;
  return `${zhA}${zhB}`;
}

function displayInstName(inst) {
  return isSwappable(inst) && isSwapped(inst) ? swapDisplayName(inst) : instName(inst);
}

function invertQuote(q) {
  if (!q || q.price <= 0) return q;
  const prev = q.price - q.change;
  const price = 1 / q.price;
  const change = prev > 0 ? price - 1 / prev : 0;
  const pct = prev > 0 ? (change / (1 / prev)) * 100 : 0;
  return { ...q, price, change, pct };
}

function toggleFxSwap(id) {
  const inst = byId.get(id);
  if (!inst) return;
  state.swapped = state.swapped || {};
  state.swapped[id] = !isSwapped(inst);
  saveState();
  renderCards();
}

function renderCards() {
  const grid = document.getElementById('card-grid');
  const focusedStatusId = document.activeElement?.matches?.('.card-status')
    ? document.activeElement.dataset.quoteId
    : null;
  hideAppTooltip();
  grid.innerHTML = '';
  for (const id of state.selected) {
    const inst = byId.get(id);
    if (!inst) continue;
    const swapped = isSwapped(inst);
    const raw = quotes.get(id);
    const q = swapped ? invertQuote(raw) : raw;
    const status = quoteStatus(id, raw);
    const card = document.createElement('div');
    card.className = 'card';
    if (status.kind === 'stale') card.classList.add('data-stale');
    if (status.kind === 'error') card.classList.add('data-error');
    const name = document.createElement('div');
    name.className = 'name';
    const nameText = document.createElement('span');
    nameText.className = 'name-text';
    nameText.textContent = displayInstName(inst);
    name.append(nameText);
    if (isSwappable(inst)) {
      const swapBtn = document.createElement('button');
      swapBtn.type = 'button';
      swapBtn.className = 'fx-swap';
      swapBtn.textContent = '⇄';
      swapBtn.setAttribute('aria-label', t('swapFx'));
      bindTooltip(swapBtn, () => ({ title: t('swapFx') }));
      swapBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleFxSwap(inst.id);
      });
      name.append(swapBtn);
    }
    if (status.kind !== 'fresh') {
      const statusDot = document.createElement('button');
      statusDot.type = 'button';
      statusDot.className = `card-status ${status.kind}`;
      statusDot.dataset.quoteId = id;
      statusDot.setAttribute('aria-label', quoteStatusTitle(status, raw));
      bindTooltip(statusDot, () => {
        const details = quoteStatusDetails(status, raw);
        return { title: details.reason, detail: details.updated, kind: status.kind };
      });
      name.append(statusDot);
    }

    const price = document.createElement('div');
    price.className = 'price';
    price.textContent = q ? fmtPrice(q.price, inst.decimals) : '--';

    const chg = document.createElement('div');
    chg.className = 'chg';
    if (q) {
      const amt = document.createElement('span');
      amt.className = 'amt';
      amt.textContent = fmtChange(q.change, inst.decimals);

      const pct = document.createElement('span');
      pct.className = 'pct';
      pct.textContent = fmtPct(q.pct);

      if (q.pct > 0) chg.classList.add('up');
      if (q.pct < 0) chg.classList.add('down');
      chg.append(amt, pct);
    } else {
      chg.textContent = '--';
    }

    card.append(name, price, chg);
    grid.append(card);
  }

  for (let i = state.selected.length; i < 4; i++) {
    const placeholder = document.createElement('button');
    placeholder.className = 'card empty';
    placeholder.textContent = '+';
    placeholder.setAttribute('aria-label', t('selectInstruments'));
    placeholder.addEventListener('click', () => showView('instruments'));
    grid.append(placeholder);
  }

  if (focusedStatusId) {
    const nextStatus = [...grid.querySelectorAll('.card-status')]
      .find((element) => element.dataset.quoteId === focusedStatusId);
    nextStatus?.focus({ preventScroll: true });
  }
}

function quoteStatus(id, quote, now = Date.now(), failures = quoteFailures) {
  const failure = failures.get(id);
  if (failure) return { kind: 'error', failure };
  if (quote && !isQuoteFresh(quote, now)) return { kind: 'stale', failure: null };
  return { kind: 'fresh', failure: null };
}

function quoteStatusDetails(status, quote) {
  let reason = '';
  if (status.kind === 'error') {
    reason = isSourceUnavailable(status.failure) ? t('quoteSourceUnavailable') : t('quoteInvalidResponse');
  } else if (status.kind === 'stale') {
    reason = t('quoteStale');
  }
  let updated = '';
  const timestamp = quoteTimestamp(quote);
  if (timestamp) {
    const locale = state.lang === 'en' ? 'en-US' : 'zh-CN';
    const formatted = new Date(timestamp).toLocaleString(locale, {
      month: 'numeric',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
    updated = `${t('quoteLastUpdated')} ${formatted}`;
  } else if (status.kind === 'error') {
    updated = t('quoteNoData');
  }
  return { reason, updated };
}

function quoteStatusTitle(status, quote) {
  const details = quoteStatusDetails(status, quote);
  return [details.reason, details.updated].filter(Boolean).join(' · ');
}

function tooltipPosition(
  anchorRect,
  tooltipRect,
  viewportWidth,
  viewportHeight,
  preferred = 'bottom',
  margin = 10,
  gap = 7,
) {
  if (preferred === 'right') {
    const right = anchorRect.right + gap;
    const leftSide = anchorRect.left - tooltipRect.width - gap;
    const useLeft = right + tooltipRect.width > viewportWidth - margin && leftSide >= margin;
    const candidateLeft = useLeft ? leftSide : right;
    const maxLeft = Math.max(margin, viewportWidth - tooltipRect.width - margin);
    const left = Math.min(Math.max(candidateLeft, margin), maxLeft);
    const centeredTop = anchorRect.top + (anchorRect.height - tooltipRect.height) / 2;
    const maxTop = Math.max(margin, viewportHeight - tooltipRect.height - margin);
    const top = Math.min(Math.max(centeredTop, margin), maxTop);
    return { left: Math.round(left), top: Math.round(top), placement: useLeft ? 'left' : 'right' };
  }
  const maxLeft = Math.max(margin, viewportWidth - tooltipRect.width - margin);
  const left = Math.min(Math.max(anchorRect.right - tooltipRect.width, margin), maxLeft);
  const below = anchorRect.bottom + gap;
  const above = anchorRect.top - tooltipRect.height - gap;
  const useAbove = below + tooltipRect.height > viewportHeight - margin && above >= margin;
  const candidateTop = useAbove ? above : below;
  const maxTop = Math.max(margin, viewportHeight - tooltipRect.height - margin);
  const top = Math.min(Math.max(candidateTop, margin), maxTop);
  return { left: Math.round(left), top: Math.round(top), placement: useAbove ? 'top' : 'bottom' };
}

let activeTooltipAnchor = null;

function showAppTooltip(anchor, content, preferred = 'bottom') {
  const tooltip = document.getElementById('app-tooltip');
  if (!tooltip || !anchor.isConnected) return;
  tooltip.querySelector('.app-tooltip-title').textContent = content.title;
  const detail = tooltip.querySelector('.app-tooltip-detail');
  detail.textContent = content.detail || '';
  detail.hidden = !content.detail;
  tooltip.dataset.kind = content.kind || '';
  tooltip.classList.remove('show');
  tooltip.hidden = false;
  tooltip.style.left = '0px';
  tooltip.style.top = '0px';
  const position = tooltipPosition(
    anchor.getBoundingClientRect(),
    tooltip.getBoundingClientRect(),
    window.innerWidth,
    window.innerHeight,
    preferred,
  );
  tooltip.style.left = `${position.left}px`;
  tooltip.style.top = `${position.top}px`;
  tooltip.dataset.placement = position.placement;
  activeTooltipAnchor = anchor;
  requestAnimationFrame(() => {
    if (activeTooltipAnchor === anchor) tooltip.classList.add('show');
  });
}

function hideAppTooltip() {
  const tooltip = document.getElementById('app-tooltip');
  activeTooltipAnchor = null;
  if (!tooltip) return;
  tooltip.classList.remove('show');
  tooltip.hidden = true;
}

function bindTooltip(anchor, content, preferred = 'bottom') {
  const show = () => showAppTooltip(anchor, content(), preferred);
  anchor.addEventListener('mouseenter', show);
  anchor.addEventListener('mouseleave', hideAppTooltip);
  anchor.addEventListener('focus', show);
  anchor.addEventListener('blur', hideAppTooltip);
  anchor.addEventListener('keydown', (event) => {
    if (event.key !== 'Escape') return;
    hideAppTooltip();
    anchor.blur();
  });
}

function bindStaticTooltips() {
  for (const anchor of document.querySelectorAll('[data-tooltip-key]')) {
    bindTooltip(
      anchor,
      () => ({ title: t(anchor.dataset.tooltipKey) }),
      anchor.dataset.tooltipPlacement || 'bottom',
    );
  }
}

function isSourceUnavailable(failure) {
  const message = String(failure?.message || '').toLowerCase();
  return message.startsWith('request failed:') || message.startsWith('http ');
}

function quoteTimestamp(quote, now = Date.now()) {
  if (!quote) return 0;
  const sourceTs = Number(quote.ts);
  if (Number.isFinite(sourceTs) && sourceTs > 0 && sourceTs <= now + MAX_FUTURE_SKEW_MS) {
    return sourceTs;
  }
  const receivedAt = Number(quote.receivedAt);
  return Number.isFinite(receivedAt) && receivedAt > 0 ? receivedAt : 0;
}

function renderInstrumentList() {
  const list = document.getElementById('instrument-list');
  list.innerHTML = '';

  const selected = state.selected.map((id) => byId.get(id)).filter(Boolean);
  if (selected.length) {
    list.append(groupLabel(t('shownGroup')));
    for (const inst of selected) list.append(panelRow(inst, true));
  }

  const rest = instruments.filter((i) => !state.selected.includes(i.id));
  let lastCategory = null;
  for (const inst of rest) {
    const category = instCategory(inst);
    if (category !== lastCategory) {
      lastCategory = category;
      list.append(groupLabel(category));
    }
    list.append(panelRow(inst, false));
  }
}

function groupLabel(text) {
  const el = document.createElement('div');
  el.className = 'group-label';
  el.textContent = text;
  return el;
}

function panelRow(inst, selected) {
  const row = document.createElement('div');
  row.className = 'panel-row' + (selected ? ' selected' : '');
  row.dataset.id = inst.id;
  row.tabIndex = 0;
  row.setAttribute('role', 'checkbox');
  row.setAttribute('aria-checked', String(selected));
  row.setAttribute('aria-label', displayInstName(inst));

  const name = document.createElement('span');
  name.className = 'row-name';
  name.textContent = displayInstName(inst);

  const mark = document.createElement('span');
  mark.className = 'row-state';
  mark.textContent = selected ? '✓' : '';

  if (selected) {
    const handle = document.createElement('span');
    handle.className = 'drag-handle';
    handle.tabIndex = 0;
    handle.setAttribute('role', 'button');
    handle.setAttribute('aria-label', `${t('reorderInstrument')}: ${displayInstName(inst)}`);
    handle.addEventListener('mousedown', (e) => startRowReorder(e, row));
    handle.addEventListener('click', (e) => e.stopPropagation());
    handle.addEventListener('keydown', (e) => {
      if (e.key !== 'ArrowUp' && e.key !== 'ArrowDown') return;
      e.preventDefault();
      e.stopPropagation();
      moveSelectedInstrument(inst.id, e.key === 'ArrowUp' ? -1 : 1);
    });
    row.append(handle, name, mark);
  } else {
    row.append(name, mark);
  }
  if (inst.custom) {
    const remove = document.createElement('span');
    remove.className = 'row-remove';
    remove.textContent = '×';
    remove.tabIndex = 0;
    remove.setAttribute('role', 'button');
    remove.setAttribute('aria-label', `${t('removeInstrument')}: ${displayInstName(inst)}`);
    remove.addEventListener('mousedown', (e) => e.stopPropagation());
    remove.addEventListener('click', (e) => {
      e.stopPropagation();
      removeCustomInstrument(inst.id);
    });
    remove.addEventListener('keydown', (e) => {
      if (e.key !== 'Enter' && e.key !== ' ') return;
      e.preventDefault();
      e.stopPropagation();
      removeCustomInstrument(inst.id);
    });
    row.append(remove);
  }
  row.addEventListener('click', () => {
    if (Date.now() < suppressRowClickUntil) return;
    toggleInstrument(inst.id);
  });
  row.addEventListener('keydown', (e) => {
    if (e.target !== row || (e.key !== 'Enter' && e.key !== ' ')) return;
    e.preventDefault();
    toggleInstrument(inst.id);
  });
  return row;
}

function moveSelectedInstrument(id, offset) {
  const from = state.selected.indexOf(id);
  const to = Math.min(state.selected.length - 1, Math.max(0, from + offset));
  if (from < 0 || from === to) return;
  state.selected.splice(from, 1);
  state.selected.splice(to, 0, id);
  saveState();
  renderCards();
  renderInstrumentList();
  const row = [...document.querySelectorAll('.panel-row.selected')].find((item) => item.dataset.id === id);
  row?.querySelector('.drag-handle')?.focus();
}

let addSource = 'tencent';

function openAddDialog() {
  document.getElementById('add-name').value = '';
  document.getElementById('add-code').value = '';
  clearAddError();
  addSource = 'tencent';
  renderAddSourceState();
  const dialog = document.getElementById('add-dialog');
  dialog.classList.remove('hidden');
  dialog.removeAttribute('inert');
  dialog.setAttribute('aria-hidden', 'false');
  document.getElementById('add-name').focus();
}

function renderAddSourceState() {
  for (const btn of document.querySelectorAll('#seg-source button')) {
    const on = btn.dataset.v === addSource;
    btn.classList.toggle('on', on);
    btn.setAttribute('aria-pressed', String(on));
  }
  const hintKey = {
    tencent: 'codeHintTencent',
    sina: 'codeHintSina',
    eastmoney: 'codeHintEastmoney',
  }[addSource];
  document.getElementById('add-code-hint').textContent = t(hintKey);
}

function closeAddDialog() {
  const dialog = document.getElementById('add-dialog');
  dialog.classList.add('hidden');
  dialog.setAttribute('aria-hidden', 'true');
  dialog.setAttribute('inert', '');
  document.getElementById('btn-add-instrument').focus();
}

function showAddError(message) {
  const el = document.getElementById('add-error');
  el.textContent = message;
  el.classList.remove('hidden');
}

function clearAddError() {
  const el = document.getElementById('add-error');
  el.textContent = '';
  el.classList.add('hidden');
}

async function submitAdd() {
  if (document.getElementById('add-dialog').classList.contains('hidden')) return;
  const name = document.getElementById('add-name').value.trim();
  const code = document.getElementById('add-code').value.trim();
  if (!name || !code) {
    showAddError(t('addMissing'));
    return;
  }
  if (!isValidCustomCode(code, addSource)) {
    showAddError(t('addInvalidCode'));
    return;
  }
  const dup =
    poolInstruments.some((i) => i.code === code) ||
    (state.custom || []).some((c) => c.code === code);
  if (dup) {
    showAddError(t('addDup'));
    return;
  }
  const id = 'c_' + code;
  const confirmBtn = document.getElementById('btn-add-confirm');
  confirmBtn.disabled = true;
  try {
    const response = await invoke('fetch_custom_quotes', {
      items: [{ id, code, source: addSource }],
    });
    if (!response.quotes.length) {
      showAddError(t('addNoQuote'));
      return;
    }
    const decimals = code.toLowerCase().startsWith('wh') ? 4 : 2;
    state.custom = state.custom || [];
    state.custom.push({ id, name, code, source: addSource, decimals });
    saveState();
    rebuildInstruments();
    renderInstrumentList();
    closeAddDialog();
    toast(t('addOk'));
  } catch (e) {
    console.warn('custom fetch failed', e);
    showAddError(t('addNoQuote'));
  } finally {
    confirmBtn.disabled = false;
  }
}

function removeCustomInstrument(id) {
  state.custom = (state.custom || []).filter((c) => c.id !== id);
  state.selected = state.selected.filter((s) => s !== id);
  if (state.swapped) delete state.swapped[id];
  quotes.delete(id);
  saveState();
  rebuildInstruments();
  renderAll();
}

function toggleInstrument(id) {
  const index = state.selected.indexOf(id);
  if (index >= 0) {
    state.selected.splice(index, 1);
  } else {
    if (state.selected.length >= 4) {
      toast(t('toastMax'));
      shake(document.getElementById('btn-instruments'));
      return;
    }
    state.selected.push(id);
  }
  saveState();
  pruneQuoteState();
  renderCards();
  renderInstrumentList();
  refresh();
}

function pruneQuoteState() {
  const selected = new Set(state.selected);
  for (const id of quotes.keys()) {
    if (!selected.has(id)) quotes.delete(id);
  }
  for (const id of quoteFailures.keys()) {
    if (!selected.has(id)) quoteFailures.delete(id);
  }
}

function startRowReorder(e, row) {
  if (e.button !== 0) return;
  const id = row.dataset.id;
  if (state.selected.indexOf(id) < 0) return;
  e.preventDefault();
  e.stopPropagation();

  const list = document.getElementById('instrument-list');
  const rows = [...list.querySelectorAll('.panel-row.selected')];
  const rowRect = row.getBoundingClientRect();
  const origIndex = rows.indexOf(row);
  const groupTop = rows[0].getBoundingClientRect().top;
  const groupBottom = rows[rows.length - 1].getBoundingClientRect().bottom;
  const others = rows.filter((r) => r !== row);
  const midpoints = others.map((r) => {
    const box = r.getBoundingClientRect();
    return box.top + box.height / 2;
  });
  const pitch =
    rows.length > 1
      ? rows[1].getBoundingClientRect().top - rows[0].getBoundingClientRect().top
      : rowRect.height + 7;
  const startY = e.clientY;
  const minDy = groupTop - rowRect.top;
  const maxDy = groupBottom - rowRect.height - rowRect.top;

  let dragged = false;
  let lastTarget = origIndex;
  row.classList.add('dragging');
  document.body.classList.add('row-dragging');

  const targetFor = (rawDy) => {
    if (rawDy <= minDy) return 0;
    if (rawDy >= maxDy) return others.length;
    const center = rowRect.top + rawDy + rowRect.height / 2;
    const up = rawDy < 0;
    for (let i = 0; i < others.length; i++) {
      if (up ? center <= midpoints[i] : center < midpoints[i]) return i;
    }
    return others.length;
  };

  const applyPreview = (target) => {
    rows.forEach((r, i) => {
      if (r === row) return;
      let shift = 0;
      if (target < origIndex && i >= target && i < origIndex) shift = pitch;
      else if (target > origIndex && i > origIndex && i <= target) shift = -pitch;
      r.style.transform = shift ? `translateY(${shift}px)` : '';
    });
  };

  const onMove = (ev) => {
    const rawDy = ev.clientY - startY;
    if (Math.abs(rawDy) > 3) dragged = true;
    const dy = Math.min(maxDy, Math.max(minDy, rawDy));
    row.style.transform = `translateY(${Math.round(dy)}px)`;
    const target = targetFor(rawDy);
    if (target !== lastTarget) {
      lastTarget = target;
      applyPreview(target);
    }
  };

  const onUp = (ev) => {
    document.removeEventListener('mousemove', onMove, true);
    document.removeEventListener('mouseup', onUp, true);
    window.removeEventListener('blur', onUp);
    row.style.transform = '';
    row.classList.remove('dragging');
    document.body.classList.remove('row-dragging');
    if (typeof ev.clientY === 'number') {
      lastTarget = targetFor(ev.clientY - startY);
    }

    const ordered = others.map((r) => r.dataset.id);
    ordered.splice(lastTarget, 0, id);
    if (dragged && ordered.join() !== state.selected.join()) {
      state.selected = ordered;
      saveState();
      renderCards();
    }
    if (dragged) suppressRowClickUntil = Date.now() + 400;
    const scrollTop = list.scrollTop;
    renderInstrumentList();
    list.scrollTop = scrollTop;
  };

  document.addEventListener('mousemove', onMove, true);
  document.addEventListener('mouseup', onUp, true);
  window.addEventListener('blur', onUp);
}

function renderThemeGrid() {
  const grid = document.getElementById('theme-grid');
  grid.innerHTML = '';
  for (const theme of THEMES) {
    const label = state.lang === 'en' ? theme.nameEn : theme.name;
    const btn = document.createElement('button');
    btn.className = 'theme-swatch';
    btn.dataset.themeId = theme.id;
    btn.style.background = theme.bg;
    btn.style.color = textColorFor(theme.bg);

    const dot = document.createElement('span');
    dot.className = 'dot';
    dot.style.background = theme.accent;

    const text = document.createElement('span');
    text.textContent = label;

    btn.append(dot, text);
    btn.addEventListener('click', () => {
      state.theme = theme.id;
      saveState();
      applyState();
      renderSettingsState();
    });
    grid.append(btn);
  }
}

function textColorFor(hex) {
  const n = parseInt(hex.slice(1), 16);
  const r = (n >> 16) & 255;
  const g = (n >> 8) & 255;
  const b = n & 255;
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
  return luminance > 0.62 ? '#3a3a3a' : '#f2f2f2';
}

function renderSettingsState() {
  for (const btn of document.querySelectorAll('.theme-swatch')) {
    const on = btn.dataset.themeId === state.theme;
    btn.classList.toggle('on', on);
    btn.setAttribute('aria-pressed', String(on));
  }
  for (const btn of document.querySelectorAll('#seg-updown button')) {
    const on = btn.dataset.v === state.updown;
    btn.classList.toggle('on', on);
    btn.setAttribute('aria-pressed', String(on));
  }
  for (const btn of document.querySelectorAll('#seg-interval button')) {
    const on = Number(btn.dataset.v) === state.interval;
    btn.classList.toggle('on', on);
    btn.setAttribute('aria-pressed', String(on));
  }
  for (const btn of document.querySelectorAll('#seg-lang button')) {
    const on = btn.dataset.v === state.lang;
    btn.classList.toggle('on', on);
    btn.setAttribute('aria-pressed', String(on));
  }
}

function isOverScrollbar(e, el) {
  return e.clientX >= el.getBoundingClientRect().right - 8;
}

function bindEvents() {
  bindStaticTooltips();
  document.getElementById('btn-add-instrument').addEventListener('click', openAddDialog);
  document.getElementById('btn-add-cancel').addEventListener('click', closeAddDialog);
  document.getElementById('btn-add-confirm').addEventListener('click', submitAdd);
  document.getElementById('add-name').addEventListener('input', clearAddError);
  document.getElementById('add-code').addEventListener('input', clearAddError);
  const addDialog = document.getElementById('add-dialog');
  addDialog.addEventListener('mousedown', (e) => {
    if (e.target === addDialog) closeAddDialog();
  });
  addDialog.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') closeAddDialog();
    if (e.key === 'Enter' && e.target.matches('input')) submitAdd();
    if (e.key === 'Tab') {
      const focusable = [...addDialog.querySelectorAll('button:not(:disabled), input:not(:disabled)')];
      if (!focusable.length) return;
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    }
  });
  for (const btn of document.querySelectorAll('#seg-source button')) {
    btn.addEventListener('click', () => {
      addSource = btn.dataset.v;
      clearAddError();
      renderAddSourceState();
    });
  }

  const instrumentList = document.getElementById('instrument-list');
  instrumentList.addEventListener('mousedown', (e) => {
    if (e.button !== 0 || e.target.closest('.panel-row') || isOverScrollbar(e, instrumentList)) return;
    e.preventDefault();
    appWindow.startDragging().catch(() => {});
  });

  const settingsBody = document.querySelector('.settings-body');
  settingsBody.addEventListener('mousedown', (e) => {
    if (e.button !== 0 || e.target.closest('button') || isOverScrollbar(e, settingsBody)) return;
    e.preventDefault();
    appWindow.startDragging().catch(() => {});
  });

  document.getElementById('btn-instruments').addEventListener('click', () => {
    showView(currentView() === 'instruments' ? 'cards' : 'instruments');
  });
  document.getElementById('btn-settings').addEventListener('click', () => {
    showView(currentView() === 'settings' ? 'cards' : 'settings');
  });
  document.getElementById('btn-pin').addEventListener('click', async () => {
    const nextPinned = !state.pinned;
    try {
      await appWindow.setAlwaysOnTop(nextPinned);
      state.pinned = nextPinned;
      saveState();
      applyState();
    } catch (e) {
      console.warn('pin failed', e);
    }
  });
  document.getElementById('btn-hide').addEventListener('click', () => invoke('hide_window'));
  document.getElementById('btn-quit').addEventListener('click', () => invoke('quit_app'));

  for (const btn of document.querySelectorAll('#seg-updown button')) {
    btn.addEventListener('click', () => {
      state.updown = btn.dataset.v;
      saveState();
      applyState();
      renderSettingsState();
    });
  }
  for (const btn of document.querySelectorAll('#seg-interval button')) {
    btn.addEventListener('click', () => {
      state.interval = Number(btn.dataset.v);
      saveState();
      renderSettingsState();
      schedule(state.interval * 1000);
    });
  }
  for (const btn of document.querySelectorAll('#seg-lang button')) {
    btn.addEventListener('click', () => {
      state.lang = btn.dataset.v;
      saveState();
      renderAll();
      invoke('set_language', { lang: state.lang }).catch((e) => console.warn('language sync failed', e));
    });
  }
}

function currentView() {
  if (!document.getElementById('view-instruments').classList.contains('hidden')) return 'instruments';
  if (!document.getElementById('view-settings').classList.contains('hidden')) return 'settings';
  return 'cards';
}

function showView(view) {
  hideAppTooltip();
  document.getElementById('view-cards').classList.toggle('hidden', view !== 'cards');
  document.getElementById('view-instruments').classList.toggle('hidden', view !== 'instruments');
  document.getElementById('view-settings').classList.toggle('hidden', view !== 'settings');
  document.getElementById('btn-instruments').classList.toggle('active', view === 'instruments');
  document.getElementById('btn-settings').classList.toggle('active', view === 'settings');
  document.getElementById('btn-instruments').setAttribute('aria-expanded', String(view === 'instruments'));
  document.getElementById('btn-settings').setAttribute('aria-expanded', String(view === 'settings'));
}

function refresh() {
  refreshRevision += 1;
  if (!refreshLoopPromise) refreshLoopPromise = runRefreshLoop();
  return refreshLoopPromise;
}

async function runRefreshLoop() {
  try {
    while (completedRefreshRevision < refreshRevision) {
      const revision = refreshRevision;
      await refreshOnce(revision);
      completedRefreshRevision = revision;
    }
  } finally {
    refreshLoopPromise = null;
    if (completedRefreshRevision < refreshRevision) return refresh();
  }
}

async function refreshOnce(revision) {
  if (!state.selected.length) {
    quotes = new Map();
    quoteFailures = new Map();
    renderCards();
    return;
  }
  const poolIds = [];
  const customs = [];
  for (const id of state.selected) {
    const inst = byId.get(id);
    if (!inst) continue;
    if (inst.custom) customs.push({ id: inst.id, code: inst.code, source: inst.source });
    else poolIds.push(id);
  }
  const requests = [];
  if (poolIds.length) {
    requests.push({ ids: poolIds, promise: invoke('fetch_quotes', { ids: poolIds }) });
  }
  if (customs.length) {
    requests.push({ ids: customs.map((item) => item.id), promise: invoke('fetch_custom_quotes', { items: customs }) });
  }
  const results = await Promise.allSettled(requests.map((request) => request.promise));
  if (revision !== refreshRevision) return;

  const receivedAt = Date.now();
  results.forEach((result, index) => {
    const requestedIds = requests[index].ids;
    if (result.status === 'rejected') {
      console.warn('fetch failed', result.reason);
      for (const id of requestedIds) quoteFailures.set(id, { id, source: '', message: String(result.reason) });
      return;
    }
    const response = result.value;
    for (const quote of response.quotes || []) {
      if (!requestedIds.includes(quote.id)) continue;
      quotes.set(quote.id, { ...quote, receivedAt });
      quoteFailures.delete(quote.id);
    }
    for (const failure of response.failures || []) {
      if (requestedIds.includes(failure.id)) quoteFailures.set(failure.id, failure);
    }
  });
  pruneQuoteState();
  renderCards();
}

async function tick() {
  if (paused) return;
  await refresh();
  if (paused) return;
  schedule(nextRefreshDelayMs(state.selected, quotes, state.interval));
}

function isQuoteFresh(quote, now = Date.now()) {
  const basis = quoteTimestamp(quote, now);
  return Number.isFinite(basis) && basis > 0 && now - basis >= 0 && now - basis < STALE_AFTER_MS;
}

function nextRefreshDelayMs(selected, quoteMap, intervalSeconds, now = Date.now()) {
  const hasFreshQuote = selected.some((id) => isQuoteFresh(quoteMap.get(id), now));
  return (hasFreshQuote ? intervalSeconds : 60) * 1000;
}

function schedule(ms) {
  clearTimeout(timer);
  timer = setTimeout(tick, ms);
}

let toastTimer = null;
function toast(message) {
  const el = document.getElementById('toast');
  el.textContent = message;
  el.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => el.classList.remove('show'), 1800);
}

function shake(el) {
  el.classList.remove('shake');
  void el.offsetWidth;
  el.classList.add('shake');
}

async function fillVersion() {
  try {
    const version = await window.__TAURI__.app.getVersion();
    document.getElementById('about-title').textContent = `Henry's MarketPeek v${version}`;
  } catch (e) {
    console.warn('version failed', e);
  }
}

if (window.__MARKETPEEK_TEST__) {
  window.__MARKETPEEK_TEST_API = {
    STR,
    THEMES,
    invertQuote,
    isQuoteFresh,
    isValidCustomCode,
    nextRefreshDelayMs,
    normalizeCustomInstrument,
    normalizeState,
    quoteStatus,
    quoteStatusDetails,
    quoteStatusTitle,
    quoteTimestamp,
    tooltipPosition,
  };
}

if (!window.__MARKETPEEK_SKIP_INIT__) {
  init().catch((e) => console.error('init failed', e));
}
