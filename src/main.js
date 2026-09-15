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
    disclaimer: '行情数据来自腾讯 / 新浪公开接口，仅供参考，不构成投资建议。',
    shownGroup: '已显示（点击移除）',
    toastMax: '最多显示 4 张，先关一张',
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
    disclaimer: 'Quotes from Tencent / Sina public APIs. For reference only, not investment advice.',
    shownGroup: 'Shown (click to remove)',
    toastMax: 'Up to 4 cards — remove one first',
  },
};

const DEFAULTS = { selected: ['sse', 'gold', 'dji'], theme: 'shimo', updown: 'cn', interval: 10, lang: 'zh' };
const STORE_KEY = 'mp-settings-v1';

let instruments = [];
let byId = new Map();
let state = loadState();
let quotes = new Map();
let paused = false;
let timer = null;
let pinned = true;

function loadState() {
  try {
    const raw = localStorage.getItem(STORE_KEY);
    if (raw) return { ...DEFAULTS, ...JSON.parse(raw) };
  } catch (e) {
    console.warn('settings load failed', e);
  }
  return { ...DEFAULTS };
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
  instruments = await invoke('get_instruments');
  byId = new Map(instruments.map((i) => [i.id, i]));

  bindEvents();
  renderAll();
  fillVersion();

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
  schedule(state.interval * 1000);
}

function applyState() {
  document.documentElement.dataset.theme = state.theme;
  document.documentElement.lang = state.lang === 'en' ? 'en' : 'zh-CN';
  document.body.className = 'updown-' + state.updown;
  document.getElementById('btn-pin').classList.toggle('active', pinned);
}

function applyI18n() {
  for (const el of document.querySelectorAll('[data-i18n]')) {
    el.textContent = t(el.dataset.i18n);
  }
  for (const el of document.querySelectorAll('[data-i18n-title]')) {
    el.title = t(el.dataset.i18nTitle);
  }
}

function renderAll() {
  applyState();
  applyI18n();
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

function renderCards() {
  const grid = document.getElementById('card-grid');
  grid.innerHTML = '';
  for (const id of state.selected) {
    const inst = byId.get(id);
    if (!inst) continue;
    const q = quotes.get(id);
    const card = document.createElement('div');
    card.className = 'card';
    card.setAttribute('data-tauri-drag-region', '');

    const name = document.createElement('div');
    name.className = 'name';
    name.textContent = instName(inst);

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
    const placeholder = document.createElement('div');
    placeholder.className = 'card empty';
    placeholder.textContent = '+';
    placeholder.title = t('selectInstruments');
    placeholder.addEventListener('click', () => showView('instruments'));
    grid.append(placeholder);
  }
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

  const name = document.createElement('span');
  name.className = 'row-name';
  name.textContent = instName(inst);

  const mark = document.createElement('span');
  mark.className = 'row-state';
  mark.textContent = selected ? '✓' : '';

  row.append(name, mark);
  row.addEventListener('click', () => toggleInstrument(inst.id));
  return row;
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
  renderCards();
  renderInstrumentList();
  refresh();
}

function renderThemeGrid() {
  const grid = document.getElementById('theme-grid');
  grid.innerHTML = '';
  for (const theme of THEMES) {
    const label = state.lang === 'en' ? theme.nameEn : theme.name;
    const btn = document.createElement('button');
    btn.className = 'theme-swatch';
    btn.dataset.themeId = theme.id;
    btn.title = label;
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
    btn.classList.toggle('on', btn.dataset.themeId === state.theme);
  }
  for (const btn of document.querySelectorAll('#seg-updown button')) {
    btn.classList.toggle('on', btn.dataset.v === state.updown);
  }
  for (const btn of document.querySelectorAll('#seg-interval button')) {
    btn.classList.toggle('on', Number(btn.dataset.v) === state.interval);
  }
  for (const btn of document.querySelectorAll('#seg-lang button')) {
    btn.classList.toggle('on', btn.dataset.v === state.lang);
  }
}

function bindEvents() {
  document.getElementById('btn-instruments').addEventListener('click', () => {
    showView(currentView() === 'instruments' ? 'cards' : 'instruments');
  });
  document.getElementById('btn-settings').addEventListener('click', () => {
    showView(currentView() === 'settings' ? 'cards' : 'settings');
  });
  document.getElementById('btn-pin').addEventListener('click', async () => {
    pinned = !pinned;
    try {
      await appWindow.setAlwaysOnTop(pinned);
    } catch (e) {
      console.warn('pin failed', e);
    }
    document.getElementById('btn-pin').classList.toggle('active', pinned);
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
  document.getElementById('view-cards').classList.toggle('hidden', view !== 'cards');
  document.getElementById('view-instruments').classList.toggle('hidden', view !== 'instruments');
  document.getElementById('view-settings').classList.toggle('hidden', view !== 'settings');
  document.getElementById('btn-instruments').classList.toggle('active', view === 'instruments');
  document.getElementById('btn-settings').classList.toggle('active', view === 'settings');
}

async function refresh() {
  if (!state.selected.length) {
    quotes = new Map();
    renderCards();
    return;
  }
  try {
    const list = await invoke('fetch_quotes', { ids: state.selected });
    for (const quote of list) quotes.set(quote.id, quote);
  } catch (e) {
    console.warn('fetch failed', e);
  }
  renderCards();
}

async function tick() {
  if (paused) return;
  await refresh();
  const fresh = [...quotes.values()].some((q) => q.ts > 0 && Date.now() - q.ts < 180000);
  schedule((fresh ? state.interval : 60) * 1000);
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

init().catch((e) => console.error('init failed', e));
