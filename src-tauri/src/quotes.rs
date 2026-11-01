//! 行情数据源适配层。
//!
//! - 品种由 `POOL` 定义；`fetch_quotes` 按品种 `source` 分组请求，单源失败不影响其余源：
//!   腾讯（GBK，`qt.gtimg.cn`）、新浪（GBK，`hq.sinajs.cn`）、东方财富（JSON，`push2.eastmoney.com`）。
//! - 解析器均为纯函数（响应文本 → `Quote`）：
//!   腾讯 `parse_std` / `parse_hf` / `parse_wh`，新浪 `parse_nf` / `parse_int`，东财 `parse_eastmoney`。
//! - 时间戳统一为 epoch 毫秒（`Quote::ts`）：`ts_from` 把 14 位或含 `/` 的时间串按 Asia/Shanghai 解释，
//!   含 `-` 的 19 位串按 America/New_York；hf / wh / nf 固定 Asia/Shanghai；东财 f86 为 Unix 秒（×1000）。
//!   解析失败或源未提供时间时为 0，前端据此判断数据新鲜度（>180s 自动降频）。
//! - 新增品种时必须确认源与解析器匹配，见 AGENTS.md「新增 / 修改品种清单」。

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::America::New_York;
use chrono_tz::Asia::Shanghai;
use chrono_tz::Tz;
use encoding_rs::GBK;
use futures_util::future::{join3, join_all};
use serde::{Deserialize, Serialize};

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)";
const MAX_SELECTED: usize = 4;
const MAX_BODY_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy, PartialEq)]
pub enum Source {
    Tencent,
    Sina,
    Eastmoney,
}

impl Source {
    fn as_str(self) -> &'static str {
        match self {
            Self::Tencent => "tencent",
            Self::Sina => "sina",
            Self::Eastmoney => "eastmoney",
        }
    }

    fn from_custom(value: &str) -> Option<Self> {
        match value {
            "tencent" => Some(Self::Tencent),
            "sina" => Some(Self::Sina),
            "eastmoney" => Some(Self::Eastmoney),
            _ => None,
        }
    }
}

pub struct Instrument {
    pub id: &'static str,
    pub name: &'static str,
    pub name_en: &'static str,
    pub short: &'static str,
    pub short_en: &'static str,
    pub category: &'static str,
    pub category_en: &'static str,
    pub decimals: u8,
    pub code: &'static str,
    pub source: Source,
}

pub const POOL: &[Instrument] = &[
    Instrument {
        id: "sse",
        name: "上证指数",
        name_en: "SSE Composite",
        short: "上证",
        short_en: "SSE",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "sh000001",
        source: Source::Tencent,
    },
    Instrument {
        id: "szse",
        name: "深证成指",
        name_en: "SZSE Component",
        short: "深成",
        short_en: "SZSE",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "sz399001",
        source: Source::Tencent,
    },
    Instrument {
        id: "chinext",
        name: "创业板指",
        name_en: "ChiNext",
        short: "创业板",
        short_en: "ChiNext",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "sz399006",
        source: Source::Tencent,
    },
    Instrument {
        id: "csi300",
        name: "沪深300",
        name_en: "CSI 300",
        short: "沪深300",
        short_en: "CSI 300",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "sh000300",
        source: Source::Tencent,
    },
    Instrument {
        id: "star50",
        name: "科创50",
        name_en: "STAR 50",
        short: "科创50",
        short_en: "STAR 50",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "sh000688",
        source: Source::Tencent,
    },
    Instrument {
        id: "csi500",
        name: "中证500",
        name_en: "CSI 500",
        short: "中证500",
        short_en: "CSI 500",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "sh000905",
        source: Source::Tencent,
    },
    Instrument {
        id: "hsi",
        name: "恒生指数",
        name_en: "Hang Seng",
        short: "恒生",
        short_en: "HSI",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "hkHSI",
        source: Source::Tencent,
    },
    Instrument {
        id: "hstech",
        name: "恒生科技指数",
        name_en: "Hang Seng Tech",
        short: "恒科",
        short_en: "HSTECH",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "hkHSTECH",
        source: Source::Tencent,
    },
    Instrument {
        id: "nikkei",
        name: "日经225",
        name_en: "Nikkei 225",
        short: "日经",
        short_en: "Nikkei",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "int_nikkei",
        source: Source::Sina,
    },
    Instrument {
        id: "kospi",
        name: "韩国KOSPI",
        name_en: "KOSPI",
        short: "韩综",
        short_en: "KOSPI",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "100.KS11",
        source: Source::Eastmoney,
    },
    Instrument {
        id: "twii",
        name: "台湾加权",
        name_en: "Taiwan Weighted",
        short: "台湾加权",
        short_en: "TWSE",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "100.TWII",
        source: Source::Eastmoney,
    },
    Instrument {
        id: "as51",
        name: "澳大利亚标普200",
        name_en: "S&P/ASX 200",
        short: "澳洲200",
        short_en: "ASX 200",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "100.AS51",
        source: Source::Eastmoney,
    },
    Instrument {
        id: "ftse",
        name: "英国富时100",
        name_en: "FTSE 100",
        short: "富时100",
        short_en: "FTSE",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "100.FTSE",
        source: Source::Eastmoney,
    },
    Instrument {
        id: "dax",
        name: "德国DAX",
        name_en: "DAX",
        short: "DAX",
        short_en: "DAX",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "100.GDAXI",
        source: Source::Eastmoney,
    },
    Instrument {
        id: "nasdaq",
        name: "纳斯达克综合",
        name_en: "Nasdaq Composite",
        short: "纳指",
        short_en: "Nasdaq",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "usIXIC",
        source: Source::Tencent,
    },
    Instrument {
        id: "sp500",
        name: "标普500",
        name_en: "S&P 500",
        short: "标普",
        short_en: "S&P 500",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "usINX",
        source: Source::Tencent,
    },
    Instrument {
        id: "dji",
        name: "道琼斯工业",
        name_en: "Dow Jones",
        short: "道指",
        short_en: "Dow",
        category: "指数",
        category_en: "Indices",
        decimals: 2,
        code: "usDJI",
        source: Source::Tencent,
    },
    Instrument {
        id: "gold",
        name: "沪金主连",
        name_en: "SHFE Gold",
        short: "沪金",
        short_en: "SHFE Gold",
        category: "商品",
        category_en: "Commodities",
        decimals: 2,
        code: "nf_AU0",
        source: Source::Sina,
    },
    Instrument {
        id: "brent",
        name: "布伦特原油",
        name_en: "Brent Crude",
        short: "布油",
        short_en: "Brent",
        category: "商品",
        category_en: "Commodities",
        decimals: 2,
        code: "hf_OIL",
        source: Source::Tencent,
    },
    Instrument {
        id: "xau",
        name: "伦敦金现货",
        name_en: "London Gold Spot",
        short: "伦金",
        short_en: "Gold",
        category: "商品",
        category_en: "Commodities",
        decimals: 2,
        code: "hf_XAU",
        source: Source::Tencent,
    },
    Instrument {
        id: "wti",
        name: "WTI原油",
        name_en: "WTI Crude",
        short: "WTI",
        short_en: "WTI",
        category: "商品",
        category_en: "Commodities",
        decimals: 2,
        code: "hf_CL",
        source: Source::Tencent,
    },
    Instrument {
        id: "xag",
        name: "COMEX白银",
        name_en: "COMEX Silver",
        short: "纽约银",
        short_en: "COMEX Silver",
        category: "商品",
        category_en: "Commodities",
        decimals: 2,
        code: "hf_SI",
        source: Source::Tencent,
    },
    Instrument {
        id: "silver",
        name: "沪银主连",
        name_en: "SHFE Silver",
        short: "沪银",
        short_en: "SHFE Silver",
        category: "商品",
        category_en: "Commodities",
        decimals: 2,
        code: "nf_AG0",
        source: Source::Sina,
    },
    Instrument {
        id: "copper",
        name: "沪铜主连",
        name_en: "SHFE Copper",
        short: "沪铜",
        short_en: "SHFE Copper",
        category: "商品",
        category_en: "Commodities",
        decimals: 2,
        code: "nf_CU0",
        source: Source::Sina,
    },
    Instrument {
        id: "fx",
        name: "美元人民币",
        name_en: "USD/CNY",
        short: "美元人民币",
        short_en: "USD/CNY",
        category: "外汇",
        category_en: "FX",
        decimals: 4,
        code: "whUSDCNY",
        source: Source::Tencent,
    },
    Instrument {
        id: "eurusd",
        name: "欧元美元",
        name_en: "EUR/USD",
        short: "欧元美元",
        short_en: "EUR/USD",
        category: "外汇",
        category_en: "FX",
        decimals: 4,
        code: "whEURUSD",
        source: Source::Tencent,
    },
    Instrument {
        id: "usdhkd",
        name: "美元港币",
        name_en: "USD/HKD",
        short: "美元港币",
        short_en: "USD/HKD",
        category: "外汇",
        category_en: "FX",
        decimals: 4,
        code: "whUSDHKD",
        source: Source::Tencent,
    },
    Instrument {
        id: "eurcny",
        name: "欧元人民币",
        name_en: "EUR/CNY",
        short: "欧元人民币",
        short_en: "EUR/CNY",
        category: "外汇",
        category_en: "FX",
        decimals: 4,
        code: "whEURCNY",
        source: Source::Tencent,
    },
    Instrument {
        id: "hkdcny",
        name: "港币人民币",
        name_en: "HKD/CNY",
        short: "港币人民币",
        short_en: "HKD/CNY",
        category: "外汇",
        category_en: "FX",
        decimals: 4,
        code: "whHKDCNY",
        source: Source::Tencent,
    },
    Instrument {
        id: "cnyjpy",
        name: "人民币日元",
        name_en: "CNY/JPY",
        short: "人民币日元",
        short_en: "CNY/JPY",
        category: "外汇",
        category_en: "FX",
        decimals: 4,
        code: "whCNYJPY",
        source: Source::Tencent,
    },
];

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(4))
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(3))
            .user_agent(UA)
            .build()
            .expect("http client")
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    pub id: String,
    pub name: String,
    pub name_en: String,
    pub short: String,
    pub short_en: String,
    pub category: String,
    pub category_en: String,
    pub decimals: u8,
    pub code: String,
}

#[tauri::command]
pub fn get_instruments() -> Vec<InstrumentInfo> {
    POOL.iter()
        .map(|i| InstrumentInfo {
            id: i.id.into(),
            name: i.name.into(),
            name_en: i.name_en.into(),
            short: i.short.into(),
            short_en: i.short_en.into(),
            category: i.category.into(),
            category_en: i.category_en.into(),
            decimals: i.decimals,
            code: i.code.into(),
        })
        .collect()
}

#[derive(Serialize, Clone)]
pub struct Quote {
    pub id: String,
    pub price: f64,
    pub change: f64,
    pub pct: f64,
    pub ts: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomInstrument {
    pub id: String,
    pub code: String,
    pub source: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteFailure {
    pub id: String,
    pub source: String,
    pub message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchResponse {
    pub quotes: Vec<Quote>,
    pub failures: Vec<QuoteFailure>,
}

#[derive(Clone)]
struct RequestItem {
    id: String,
    code: String,
    source: Source,
}

#[derive(Default)]
struct GroupOutcome {
    quotes: Vec<Quote>,
    failures: Vec<QuoteFailure>,
}

#[tauri::command]
pub async fn fetch_quotes(ids: Vec<String>) -> Result<FetchResponse, String> {
    validate_count(ids.len())?;
    let mut seen = HashSet::new();
    let mut items = Vec::with_capacity(ids.len());
    for id in ids {
        if !seen.insert(id.clone()) {
            return Err(format!("duplicate instrument id: {id}"));
        }
        let instrument = POOL
            .iter()
            .find(|instrument| instrument.id == id)
            .ok_or_else(|| format!("unknown instrument id: {id}"))?;
        items.push(RequestItem {
            id,
            code: instrument.code.to_string(),
            source: instrument.source,
        });
    }
    Ok(fetch_items(items).await)
}

#[tauri::command]
pub async fn fetch_custom_quotes(items: Vec<CustomInstrument>) -> Result<FetchResponse, String> {
    validate_count(items.len())?;
    let mut seen_ids = HashSet::new();
    let mut seen_codes = HashSet::new();
    let mut requests = Vec::with_capacity(items.len());
    for item in items {
        let source = Source::from_custom(&item.source)
            .ok_or_else(|| format!("unsupported source: {}", item.source))?;
        validate_custom_id(&item.id)?;
        validate_custom_code(&item.code, source)?;
        if !seen_ids.insert(item.id.clone()) {
            return Err(format!("duplicate custom id: {}", item.id));
        }
        if !seen_codes.insert((source.as_str(), item.code.clone())) {
            return Err(format!("duplicate custom code: {}", item.code));
        }
        requests.push(RequestItem {
            id: item.id,
            code: item.code,
            source,
        });
    }
    Ok(fetch_items(requests).await)
}

fn validate_count(count: usize) -> Result<(), String> {
    if count > MAX_SELECTED {
        Err(format!("at most {MAX_SELECTED} instruments may be fetched"))
    } else {
        Ok(())
    }
}

fn validate_custom_id(id: &str) -> Result<(), String> {
    if id.is_empty()
        || id.len() > 64
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
    {
        return Err("invalid custom instrument id".into());
    }
    Ok(())
}

fn validate_custom_code(code: &str, source: Source) -> Result<(), String> {
    if code.is_empty()
        || code.len() > 32
        || !code
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.'))
    {
        return Err("invalid custom instrument code".into());
    }
    if source == Source::Sina && !(code.starts_with("nf_") || code.starts_with("int_")) {
        return Err("Sina custom codes must start with nf_ or int_".into());
    }
    if source == Source::Eastmoney {
        let Some((market, symbol)) = code.split_once('.') else {
            return Err("Eastmoney custom codes must use market.symbol format".into());
        };
        if market.is_empty() || symbol.is_empty() || symbol.contains('.') {
            return Err("Eastmoney custom codes must use market.symbol format".into());
        }
    }
    Ok(())
}

async fn fetch_items(items: Vec<RequestItem>) -> FetchResponse {
    let tencent = items
        .iter()
        .filter(|item| item.source == Source::Tencent)
        .cloned()
        .collect();
    let sina = items
        .iter()
        .filter(|item| item.source == Source::Sina)
        .cloned()
        .collect();
    let eastmoney = items
        .into_iter()
        .filter(|item| item.source == Source::Eastmoney)
        .collect();
    let (tencent, sina, eastmoney) = join3(
        fetch_tencent_group(tencent),
        fetch_sina_group(sina),
        fetch_eastmoney_group(eastmoney),
    )
    .await;
    let mut response = FetchResponse {
        quotes: Vec::new(),
        failures: Vec::new(),
    };
    for mut outcome in [tencent, sina, eastmoney] {
        response.quotes.append(&mut outcome.quotes);
        response.failures.append(&mut outcome.failures);
    }
    response
}

async fn fetch_tencent_group(items: Vec<RequestItem>) -> GroupOutcome {
    if items.is_empty() {
        return GroupOutcome::default();
    }
    let codes = items
        .iter()
        .map(|item| item.code.as_str())
        .collect::<Vec<_>>()
        .join(",");
    let result = async {
        let response = client()
            .get("https://qt.gtimg.cn/")
            .query(&[("q", codes)])
            .send()
            .await
            .map_err(|error| format!("request failed: {error}"))?;
        let bytes = read_body_limited(response).await?;
        let (text, _, _) = GBK.decode(&bytes);
        let ids: HashMap<&str, &str> = items
            .iter()
            .map(|item| (item.code.as_str(), item.id.as_str()))
            .collect();
        Ok(parse_tencent_with(&text, |code| {
            ids.get(code).map(|id| (*id).to_string())
        }))
    }
    .await;
    batch_outcome(items, result)
}

async fn fetch_sina_group(items: Vec<RequestItem>) -> GroupOutcome {
    if items.is_empty() {
        return GroupOutcome::default();
    }
    let codes = items
        .iter()
        .map(|item| item.code.as_str())
        .collect::<Vec<_>>()
        .join(",");
    let result = async {
        let url = format!("https://hq.sinajs.cn/list={codes}");
        let response = client()
            .get(url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await
            .map_err(|error| format!("request failed: {error}"))?;
        let bytes = read_body_limited(response).await?;
        let (text, _, _) = GBK.decode(&bytes);
        let ids: HashMap<&str, &str> = items
            .iter()
            .map(|item| (item.code.as_str(), item.id.as_str()))
            .collect();
        Ok(parse_sina_with(&text, |code| {
            ids.get(code).map(|id| (*id).to_string())
        }))
    }
    .await;
    batch_outcome(items, result)
}

async fn fetch_eastmoney_group(items: Vec<RequestItem>) -> GroupOutcome {
    let futures = items.into_iter().map(|item| async move {
        let result = async {
            let response = client()
                .get("https://push2.eastmoney.com/api/qt/stock/get")
                .query(&[
                    ("secid", item.code.as_str()),
                    ("fields", "f43,f57,f58,f86,f169,f170"),
                    ("fltt", "2"),
                    ("invt", "2"),
                ])
                .send()
                .await
                .map_err(|error| format!("request failed: {error}"))?;
            let bytes = read_body_limited(response).await?;
            let text = std::str::from_utf8(&bytes)
                .map_err(|error| format!("invalid UTF-8 response: {error}"))?;
            parse_eastmoney_with(text, &item.id)
                .ok_or_else(|| "response did not contain a valid quote".to_string())
        }
        .await;
        (item, result)
    });
    let mut outcome = GroupOutcome::default();
    for (item, result) in join_all(futures).await {
        match result {
            Ok(quote) => outcome.quotes.push(quote),
            Err(message) => outcome.failures.push(QuoteFailure {
                id: item.id,
                source: item.source.as_str().into(),
                message,
            }),
        }
    }
    outcome
}

fn batch_outcome(items: Vec<RequestItem>, result: Result<Vec<Quote>, String>) -> GroupOutcome {
    match result {
        Ok(quotes) => {
            let returned: HashSet<&str> = quotes.iter().map(|quote| quote.id.as_str()).collect();
            let failures = items
                .iter()
                .filter(|item| !returned.contains(item.id.as_str()))
                .map(|item| QuoteFailure {
                    id: item.id.clone(),
                    source: item.source.as_str().into(),
                    message: "response did not contain a valid quote".into(),
                })
                .collect();
            GroupOutcome { quotes, failures }
        }
        Err(message) => GroupOutcome {
            quotes: Vec::new(),
            failures: items
                .into_iter()
                .map(|item| QuoteFailure {
                    id: item.id,
                    source: item.source.as_str().into(),
                    message: message.clone(),
                })
                .collect(),
        },
    }
}

async fn read_body_limited(mut response: reqwest::Response) -> Result<Vec<u8>, String> {
    let status = response.status();
    if !status.is_success() {
        return Err(format!("HTTP {status}"));
    }
    if response
        .content_length()
        .is_some_and(|length| length > MAX_BODY_BYTES as u64)
    {
        return Err("response body is too large".into());
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| format!("failed to read response: {error}"))?
    {
        if body.len().saturating_add(chunk.len()) > MAX_BODY_BYTES {
            return Err("response body is too large".into());
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

#[cfg(test)]
fn pool_by_code(code: &str) -> Option<&'static Instrument> {
    POOL.iter().find(|i| i.code == code)
}

#[cfg(test)]
fn parse_tencent(body: &str) -> Vec<Quote> {
    parse_tencent_with(body, |code| pool_by_code(code).map(|i| i.id.to_string()))
}

fn parse_tencent_with(body: &str, lookup: impl Fn(&str) -> Option<String>) -> Vec<Quote> {
    let mut out = Vec::new();
    for line in body.split(';') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((var, val)) = line.split_once('=') else {
            continue;
        };
        let code = var.trim().trim_start_matches("v_");
        let Some(id) = lookup(code) else { continue };
        let val = val.trim().trim_matches('"');
        if val.is_empty() {
            continue;
        }
        let parsed = if code.starts_with("hf_") {
            parse_hf(val)
        } else if code.starts_with("wh") {
            parse_wh(val)
        } else {
            parse_std(val)
        };
        if let Some((price, change, pct, ts)) = parsed {
            out.push(Quote {
                id,
                price,
                change,
                pct,
                ts,
            });
        }
    }
    out
}

fn parse_std(val: &str) -> Option<(f64, f64, f64, i64)> {
    let f: Vec<&str> = val.split('~').collect();
    if f.len() < 6 {
        return None;
    }
    let price = parse_positive(f[3])?;
    let prev = parse_positive(f[4]);
    let idx = f.iter().position(|s| looks_like_datetime(s))?;
    let stamp = f[idx];
    let tz = if stamp.contains('/') || stamp.len() == 14 {
        Shanghai
    } else {
        New_York
    };
    let ts = ts_from(stamp, tz).unwrap_or(0);
    let change = match f.get(idx + 1) {
        Some(value) => parse_finite(value)?,
        None => prev.map(|previous| price - previous)?,
    };
    let pct = match f.get(idx + 2) {
        Some(value) => parse_finite(value)?,
        None => prev.map(|previous| change / previous * 100.0)?,
    };
    Some((price, change, pct, ts))
}

fn parse_hf(val: &str) -> Option<(f64, f64, f64, i64)> {
    let f: Vec<&str> = val.split(',').collect();
    if f.len() < 9 {
        return None;
    }
    let price = parse_positive(f[0])?;
    let pct = parse_finite(f[1])?;
    let prev = parse_positive(f[7])?;
    let ts = match (f.get(12), f.get(6)) {
        (Some(d), Some(t)) => ts_from(&format!("{d} {t}"), Shanghai).unwrap_or(0),
        _ => 0,
    };
    Some((price, price - prev, pct, ts))
}

fn parse_wh(val: &str) -> Option<(f64, f64, f64, i64)> {
    let f: Vec<&str> = val.split('~').collect();
    if f.len() < 14 {
        return None;
    }
    let price = parse_positive(f[3])?;
    let change = parse_finite(f[12])?;
    let pct = parse_finite(f[13])?;
    let ts = ts_from(f[5], Shanghai).unwrap_or(0);
    Some((price, change, pct, ts))
}

#[cfg(test)]
fn parse_sina(body: &str) -> Vec<Quote> {
    parse_sina_with(body, |code| pool_by_code(code).map(|i| i.id.to_string()))
}

fn parse_sina_with(body: &str, lookup: impl Fn(&str) -> Option<String>) -> Vec<Quote> {
    let mut out = Vec::new();
    for line in body.split(';') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((var, val)) = line.split_once('=') else {
            continue;
        };
        let var = var.trim();
        let code = var.strip_prefix("var hq_str_").unwrap_or(var);
        let Some(id) = lookup(code) else { continue };
        let val = val.trim().trim_matches('"');
        if val.is_empty() {
            continue;
        }
        let parsed = if code.starts_with("nf_") {
            parse_nf(val)
        } else if code.starts_with("int_") {
            parse_int(val)
        } else {
            None
        };
        if let Some((price, change, pct, ts)) = parsed {
            out.push(Quote {
                id,
                price,
                change,
                pct,
                ts,
            });
        }
    }
    out
}

fn parse_nf(val: &str) -> Option<(f64, f64, f64, i64)> {
    let f: Vec<&str> = val.split(',').collect();
    if f.len() < 11 {
        return None;
    }
    let price = parse_positive(f[8])?;
    let prev = parse_positive(f[10])?;
    let change = price - prev;
    let pct = change / prev * 100.0;
    let time = f[1].trim();
    let ts = if time.len() == 6 && time.chars().all(|c| c.is_ascii_digit()) {
        let date = f
            .iter()
            .find(|s| s.len() == 10 && s.chars().filter(|c| *c == '-').count() == 2)?;
        let combined = format!("{date} {}:{}:{}", &time[0..2], &time[2..4], &time[4..6]);
        ts_from(&combined, Shanghai).unwrap_or(0)
    } else {
        0
    };
    Some((price, change, pct, ts))
}

fn parse_int(val: &str) -> Option<(f64, f64, f64, i64)> {
    let f: Vec<&str> = val.split(',').collect();
    if f.len() < 4 {
        return None;
    }
    let price = parse_positive(f[1])?;
    Some((price, parse_finite(f[2])?, parse_finite(f[3])?, 0))
}

#[cfg(test)]
fn parse_eastmoney(body: &str, code: &str) -> Option<Quote> {
    parse_eastmoney_with(body, pool_by_code(code)?.id)
}

fn parse_eastmoney_with(body: &str, id: &str) -> Option<Quote> {
    let value: serde_json::Value = serde_json::from_str(body).ok()?;
    let data = value.get("data")?;
    if data.is_null() {
        return None;
    }
    let price = finite_value(data.get("f43")?.as_f64()?).filter(|value| *value > 0.0)?;
    let change = finite_value(data.get("f169")?.as_f64()?)?;
    let pct = finite_value(data.get("f170")?.as_f64()?)?;
    let ts = data
        .get("f86")
        .and_then(|value| value.as_i64())
        .and_then(|seconds| seconds.checked_mul(1000))
        .filter(|value| *value > 0)
        .unwrap_or(0);
    Some(Quote {
        id: id.into(),
        price,
        change,
        pct,
        ts,
    })
}

fn parse_finite(value: &str) -> Option<f64> {
    finite_value(value.parse().ok()?)
}

fn parse_positive(value: &str) -> Option<f64> {
    parse_finite(value).filter(|number| *number > 0.0)
}

fn finite_value(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn looks_like_datetime(s: &str) -> bool {
    (s.len() == 14 && s.chars().all(|c| c.is_ascii_digit()))
        || (s.len() == 19
            && s.contains(':')
            && s.chars().filter(|c| *c == '/' || *c == '-').count() == 2)
}

fn ts_from(s: &str, tz: Tz) -> Option<i64> {
    let naive = if s.len() == 14 {
        NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S").ok()?
    } else if s.contains('/') {
        NaiveDateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S").ok()?
    } else {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()?
    };
    tz.from_local_datetime(&naive)
        .single()
        .map(|dt| dt.timestamp_millis())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
    }

    #[test]
    fn parse_std_reads_a_share_row() {
        let val = "1~上证指数~000001~3350.13~3328.10~0~20260916150000~22.03~0.66~0~0";
        let (price, change, pct, ts) = parse_std(val).unwrap();
        approx(price, 3350.13);
        approx(change, 22.03);
        approx(pct, 0.66);
        assert_eq!(ts, ts_from("20260916150000", Shanghai).unwrap());
    }

    #[test]
    fn parse_std_treats_slashed_stamp_as_shanghai() {
        let val = "1~恒生指数~HSI~26000.00~25900.00~0~2026/10/05 16:00:00~100.00~0.39";
        let (_, _, _, ts) = parse_std(val).unwrap();
        assert_eq!(ts, ts_from("2026/10/05 16:00:00", Shanghai).unwrap());
    }

    #[test]
    fn parse_std_treats_dashed_stamp_as_new_york() {
        let val = "104~纳斯达克综合~IXIC~18000.00~17950.00~0~2026-10-05 16:00:00~50.00~0.28";
        let (price, _, _, ts) = parse_std(val).unwrap();
        approx(price, 18000.0);
        assert_eq!(ts, ts_from("2026-10-05 16:00:00", New_York).unwrap());
    }

    #[test]
    fn parse_std_rejects_short_rows_and_zero_price() {
        assert!(parse_std("1~x~y").is_none());
        assert!(parse_std("1~x~y~0~0~0~20260916150000~0~0").is_none());
        assert!(parse_std("1~x~y~NaN~1~0~20260916150000~0~0").is_none());
        assert!(parse_std("1~x~y~1~1~0~20260916150000~NaN~0").is_none());
    }

    #[test]
    fn parse_hf_reads_comma_format() {
        let val = "70.50,1.23,0,0,0,0,15:30:00,69.64,0,0,0,0,2026-10-05,0";
        let (price, change, pct, ts) = parse_hf(val).unwrap();
        approx(price, 70.50);
        approx(change, 0.86);
        approx(pct, 1.23);
        assert_eq!(ts, ts_from("2026-10-05 15:30:00", Shanghai).unwrap());
    }

    #[test]
    fn parse_wh_reads_tilde_format() {
        let val = "310~美元人民币~USDCNY~6.7050~0~20261001025956~6.7065~6.7050~6.7055~6.7030~6.7050~6.7072~-0.0015~-0.02~-0.08~-0.02~-0.21~-1.47~-4.03~7.1430~6.6950~2026-09-30";
        let (price, change, pct, ts) = parse_wh(val).unwrap();
        approx(price, 6.7050);
        approx(change, -0.0015);
        approx(pct, -0.02);
        assert_eq!(ts, ts_from("20261001025956", Shanghai).unwrap());
    }

    #[test]
    fn parse_nf_computes_change_from_prev_close() {
        let val = "沪金主连,150000,0,0,0,0,0,0,860.52,0,859.80,0,0,2026-10-05";
        let (price, change, pct, ts) = parse_nf(val).unwrap();
        approx(price, 860.52);
        approx(change, 0.72);
        assert!((pct - 0.08374).abs() < 1e-4, "unexpected pct {pct}");
        assert_eq!(ts, ts_from("2026-10-05 15:00:00", Shanghai).unwrap());
    }

    #[test]
    fn parse_nf_returns_zero_ts_without_time_field() {
        let val = "沪金主连,,0,0,0,0,0,0,860.52,0,859.80,0,0,2026-10-05";
        let (_, _, _, ts) = parse_nf(val).unwrap();
        assert_eq!(ts, 0);
    }

    #[test]
    fn parse_int_reads_price_change_pct() {
        let (price, change, pct, ts) = parse_int("日经225,45000.00,120.50,0.27").unwrap();
        approx(price, 45000.00);
        approx(change, 120.50);
        approx(pct, 0.27);
        assert_eq!(ts, 0);
    }

    #[test]
    fn parse_eastmoney_reads_json_payload() {
        let body = r#"{"data":{"f43":3325.31,"f169":12.34,"f170":0.37,"f86":1789542000}}"#;
        let q = parse_eastmoney(body, "100.KS11").unwrap();
        assert_eq!(q.id, "kospi");
        approx(q.price, 3325.31);
        approx(q.change, 12.34);
        approx(q.pct, 0.37);
        assert_eq!(q.ts, 1_789_542_000_000);
        assert!(parse_eastmoney(r#"{"data":null}"#, "100.KS11").is_none());
        assert!(parse_eastmoney("not json", "100.KS11").is_none());
        assert!(parse_eastmoney(r#"{"data":{"f43":1,"f86":1}}"#, "100.KS11").is_none());
    }

    #[test]
    fn parse_tencent_routes_rows_by_code() {
        let body = r#"v_sh000001="1~上证指数~000001~3350.13~3328.10~0~20260916150000~22.03~0.66";"#;
        let out = parse_tencent(body);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "sse");
        approx(out[0].price, 3350.13);
        assert_eq!(out[0].ts, ts_from("20260916150000", Shanghai).unwrap());
    }

    #[test]
    fn parse_sina_routes_rows_by_var_name() {
        let body =
            r#"var hq_str_nf_AU0="沪金主连,150000,0,0,0,0,0,0,860.52,0,859.80,0,0,2026-10-05";"#;
        let out = parse_sina(body);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "gold");
    }

    #[test]
    fn looks_like_datetime_accepts_known_stamps() {
        assert!(looks_like_datetime("20260916150000"));
        assert!(looks_like_datetime("2026/10/05 16:00:00"));
        assert!(looks_like_datetime("2026-10-05 16:00:00"));
        assert!(!looks_like_datetime("150000"));
        assert!(!looks_like_datetime("2026-10-05"));
    }

    #[test]
    fn ts_from_converts_to_epoch_millis() {
        assert_eq!(
            ts_from("20260101000000", Shanghai).unwrap(),
            1_767_196_800_000
        );
        assert_eq!(
            ts_from("2026/01/01 00:00:00", Shanghai).unwrap(),
            1_767_196_800_000
        );
        assert_eq!(
            ts_from("2026-01-01 00:00:00", New_York).unwrap(),
            1_767_243_600_000
        );
        assert!(ts_from("not a date", Shanghai).is_none());
    }

    #[test]
    fn parse_tencent_routes_new_hf_instrument() {
        let body = r#"v_hf_CL="87.92,-1.69,87.93,87.95,90.05,87.62,16:59:00,89.43,89.27,0,5,3,2026-10-06,纽约原油";"#;
        let out = parse_tencent(body);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "wti");
        approx(out[0].price, 87.92);
        approx(out[0].change, -1.51);
        approx(out[0].pct, -1.69);
        assert_eq!(out[0].ts, ts_from("2026-10-06 16:59:00", Shanghai).unwrap());
    }

    #[test]
    fn parse_tencent_routes_new_fx_instrument() {
        let body = r#"v_whEURCNY="310~欧元人民币~EURCNY~7.5418~0~20261006170121~7.5253~7.5250~7.5421~7.5123~7.5398~7.5402~0.0165~0.22~-1.22~-1.95~-3.13~-2.95~-8.10~8.3912~7.4848~2026-10-06";"#;
        let out = parse_tencent(body);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "eurcny");
        approx(out[0].price, 7.5418);
        approx(out[0].change, 0.0165);
        approx(out[0].pct, 0.22);
    }

    #[test]
    fn parse_sina_routes_new_futures_instrument() {
        let body =
            r#"var hq_str_nf_AG0="沪银主连,150000,0,0,0,0,0,0,8600.00,0,8540.00,0,0,2026-10-06";"#;
        let out = parse_sina(body);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "silver");
        approx(out[0].price, 8600.00);
        approx(out[0].change, 60.00);
    }

    #[test]
    fn parse_eastmoney_routes_new_global_index() {
        let body = r#"{"rc":0,"rt":4,"data":{"f43":10590.77,"f57":"FTSE","f58":"英国富时100","f86":1791277138,"f169":92.83,"f170":0.88}}"#;
        let q = parse_eastmoney(body, "100.FTSE").unwrap();
        assert_eq!(q.id, "ftse");
        approx(q.price, 10590.77);
        approx(q.change, 92.83);
        approx(q.pct, 0.88);
        assert_eq!(q.ts, 1_791_277_138_000);
    }

    #[test]
    fn pool_ids_and_codes_are_unique() {
        let mut ids: Vec<&str> = POOL.iter().map(|i| i.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), POOL.len());
        let mut codes: Vec<&str> = POOL.iter().map(|i| i.code).collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), POOL.len());
    }

    #[test]
    fn parse_tencent_with_accepts_custom_lookup() {
        let body = r#"v_whEURUSD="310~欧元美元~EURUSD~1.1241~0~20261006165822~1.1219~1.1220~1.1246~1.1202~1.1241~1.1242~0.0022~0.20~-1.25~-1.95~-2.99~-1.53~-4.29~1.2065~1.1161~2026-10-06";"#;
        let out = parse_tencent_with(body, |code| {
            (code == "whEURUSD").then(|| "c_whEURUSD".to_string())
        });
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "c_whEURUSD");
        approx(out[0].price, 1.1241);
        approx(out[0].change, 0.0022);
        approx(out[0].pct, 0.20);
    }

    #[test]
    fn parse_sina_with_accepts_custom_lookup() {
        let body = r#"var hq_str_nf_RB0="螺纹钢主连,150000,0,0,0,0,0,0,3200.00,0,3180.00,0,0,2026-10-06";"#;
        let out = parse_sina_with(body, |code| {
            (code == "nf_RB0").then(|| "c_nf_RB0".to_string())
        });
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "c_nf_RB0");
        approx(out[0].price, 3200.00);
        approx(out[0].change, 20.00);
    }

    #[test]
    fn parse_eastmoney_with_accepts_custom_id() {
        let body = r#"{"rc":0,"rt":4,"data":{"f43":49822.55,"f57":"TWII","f58":"台湾加权","f86":1791264325,"f169":110.51,"f170":0.22}}"#;
        let q = parse_eastmoney_with(body, "c_100.TWII").unwrap();
        assert_eq!(q.id, "c_100.TWII");
        approx(q.price, 49822.55);
        assert_eq!(q.ts, 1_791_264_325_000);
    }

    #[test]
    fn validates_custom_instrument_boundaries() {
        assert!(validate_custom_code("whEURUSD", Source::Tencent).is_ok());
        assert!(validate_custom_code("nf_AU0", Source::Sina).is_ok());
        assert!(validate_custom_code("100.KS11", Source::Eastmoney).is_ok());
        assert!(validate_custom_code("sh600000&fields=x", Source::Tencent).is_err());
        assert!(validate_custom_code("sh600000", Source::Sina).is_err());
        assert!(validate_custom_code("KS11", Source::Eastmoney).is_err());
        assert!(validate_custom_id("c_100.KS11").is_ok());
        assert!(validate_custom_id("bad id").is_err());
        assert!(validate_count(4).is_ok());
        assert!(validate_count(5).is_err());
    }

    #[test]
    fn batch_outcome_reports_every_missing_quote() {
        let items = vec![
            RequestItem {
                id: "one".into(),
                code: "sh000001".into(),
                source: Source::Tencent,
            },
            RequestItem {
                id: "two".into(),
                code: "sz399001".into(),
                source: Source::Tencent,
            },
        ];
        let outcome = batch_outcome(
            items,
            Ok(vec![Quote {
                id: "one".into(),
                price: 1.0,
                change: 0.0,
                pct: 0.0,
                ts: 1,
            }]),
        );
        assert_eq!(outcome.quotes.len(), 1);
        assert_eq!(outcome.failures.len(), 1);
        assert_eq!(outcome.failures[0].id, "two");
    }
}
