use std::sync::OnceLock;

use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::America::New_York;
use chrono_tz::Asia::Shanghai;
use chrono_tz::Tz;
use encoding_rs::GBK;
use serde::Serialize;

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)";

#[derive(Clone, Copy, PartialEq)]
pub enum Source {
    Tencent,
    Sina,
    Eastmoney,
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
    Instrument { id: "sse", name: "上证指数", name_en: "SSE Composite", short: "上证", short_en: "SSE", category: "指数", category_en: "Indices", decimals: 2, code: "sh000001", source: Source::Tencent },
    Instrument { id: "szse", name: "深证成指", name_en: "SZSE Component", short: "深成", short_en: "SZSE", category: "指数", category_en: "Indices", decimals: 2, code: "sz399001", source: Source::Tencent },
    Instrument { id: "chinext", name: "创业板指", name_en: "ChiNext", short: "创业板", short_en: "ChiNext", category: "指数", category_en: "Indices", decimals: 2, code: "sz399006", source: Source::Tencent },
    Instrument { id: "csi300", name: "沪深300", name_en: "CSI 300", short: "沪深300", short_en: "CSI 300", category: "指数", category_en: "Indices", decimals: 2, code: "sh000300", source: Source::Tencent },
    Instrument { id: "star50", name: "科创50", name_en: "STAR 50", short: "科创50", short_en: "STAR 50", category: "指数", category_en: "Indices", decimals: 2, code: "sh000688", source: Source::Tencent },
    Instrument { id: "hsi", name: "恒生指数", name_en: "Hang Seng", short: "恒生", short_en: "HSI", category: "指数", category_en: "Indices", decimals: 2, code: "hkHSI", source: Source::Tencent },
    Instrument { id: "hstech", name: "恒生科技指数", name_en: "Hang Seng Tech", short: "恒科", short_en: "HSTECH", category: "指数", category_en: "Indices", decimals: 2, code: "hkHSTECH", source: Source::Tencent },
    Instrument { id: "nikkei", name: "日经225", name_en: "Nikkei 225", short: "日经", short_en: "Nikkei", category: "指数", category_en: "Indices", decimals: 2, code: "int_nikkei", source: Source::Sina },
    Instrument { id: "kospi", name: "韩国KOSPI", name_en: "KOSPI", short: "韩综", short_en: "KOSPI", category: "指数", category_en: "Indices", decimals: 2, code: "100.KS11", source: Source::Eastmoney },
    Instrument { id: "nasdaq", name: "纳斯达克综合", name_en: "Nasdaq Composite", short: "纳指", short_en: "Nasdaq", category: "指数", category_en: "Indices", decimals: 2, code: "usIXIC", source: Source::Tencent },
    Instrument { id: "sp500", name: "标普500", name_en: "S&P 500", short: "标普", short_en: "S&P 500", category: "指数", category_en: "Indices", decimals: 2, code: "usINX", source: Source::Tencent },
    Instrument { id: "dji", name: "道琼斯工业", name_en: "Dow Jones", short: "道指", short_en: "Dow", category: "指数", category_en: "Indices", decimals: 2, code: "usDJI", source: Source::Tencent },
    Instrument { id: "gold", name: "沪金主连", name_en: "SHFE Gold", short: "沪金", short_en: "SHFE Gold", category: "商品", category_en: "Commodities", decimals: 2, code: "nf_AU0", source: Source::Sina },
    Instrument { id: "brent", name: "布伦特原油", name_en: "Brent Crude", short: "布油", short_en: "Brent", category: "商品", category_en: "Commodities", decimals: 2, code: "hf_OIL", source: Source::Tencent },
    Instrument { id: "xau", name: "伦敦金现货", name_en: "London Gold Spot", short: "伦金", short_en: "Gold", category: "商品", category_en: "Commodities", decimals: 2, code: "hf_XAU", source: Source::Tencent },
    Instrument { id: "fx", name: "美元人民币", name_en: "USD/CNY", short: "美元人民币", short_en: "USD/CNY", category: "外汇", category_en: "FX", decimals: 4, code: "whUSDCNY", source: Source::Tencent },
];

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
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

#[tauri::command]
pub async fn fetch_quotes(ids: Vec<String>) -> Result<Vec<Quote>, String> {
    let mut out: Vec<Quote> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let selected: Vec<&Instrument> = POOL.iter().filter(|i| ids.iter().any(|id| id == i.id)).collect();
    let tencent: Vec<&str> = selected.iter().filter(|i| i.source == Source::Tencent).map(|i| i.code).collect();
    let sina: Vec<&str> = selected.iter().filter(|i| i.source == Source::Sina).map(|i| i.code).collect();
    let eastmoney: Vec<&str> = selected.iter().filter(|i| i.source == Source::Eastmoney).map(|i| i.code).collect();

    if !tencent.is_empty() {
        let url = format!("http://qt.gtimg.cn/q={}", tencent.join(","));
        match client().get(&url).header("User-Agent", UA).send().await {
            Ok(resp) => match resp.bytes().await {
                Ok(bytes) => {
                    let (text, _, _) = GBK.decode(&bytes);
                    out.extend(parse_tencent(&text));
                }
                Err(e) => errors.push(format!("tencent body: {e}")),
            },
            Err(e) => errors.push(format!("tencent: {e}")),
        }
    }

    if !sina.is_empty() {
        let url = format!("https://hq.sinajs.cn/list={}", sina.join(","));
        match client()
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .header("User-Agent", UA)
            .send()
            .await
        {
            Ok(resp) => match resp.bytes().await {
                Ok(bytes) => {
                    let (text, _, _) = GBK.decode(&bytes);
                    out.extend(parse_sina(&text));
                }
                Err(e) => errors.push(format!("sina body: {e}")),
            },
            Err(e) => errors.push(format!("sina: {e}")),
        }
    }

    if !eastmoney.is_empty() {
        for code in eastmoney {
            let url = format!(
                "https://push2.eastmoney.com/api/qt/stock/get?secid={code}&fields=f43,f57,f58,f86,f169,f170&fltt=2&invt=2"
            );
            match client().get(&url).header("User-Agent", UA).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        if let Some(quote) = parse_eastmoney(&text, code) {
                            out.push(quote);
                        }
                    }
                    Err(e) => errors.push(format!("eastmoney body: {e}")),
                },
                Err(e) => errors.push(format!("eastmoney: {e}")),
            }
        }
    }

    if out.is_empty() && !errors.is_empty() {
        return Err(errors.join("; "));
    }
    Ok(out)
}

fn pool_by_code(code: &str) -> Option<&'static Instrument> {
    POOL.iter().find(|i| i.code == code)
}

fn parse_tencent(body: &str) -> Vec<Quote> {
    let mut out = Vec::new();
    for line in body.split(';') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((var, val)) = line.split_once('=') else { continue };
        let code = var.trim().trim_start_matches("v_");
        let Some(inst) = pool_by_code(code) else { continue };
        let val = val.trim().trim_matches('"');
        if val.is_empty() {
            continue;
        }
        let parsed = if code.starts_with("hf_") {
            parse_hf(val)
        } else if code.starts_with("wh_") {
            parse_wh(val)
        } else {
            parse_std(val)
        };
        if let Some((price, change, pct, ts)) = parsed {
            out.push(Quote { id: inst.id.into(), price, change, pct, ts });
        }
    }
    out
}

fn parse_std(val: &str) -> Option<(f64, f64, f64, i64)> {
    let f: Vec<&str> = val.split('~').collect();
    if f.len() < 6 {
        return None;
    }
    let price = f[3].parse::<f64>().ok()?;
    if price <= 0.0 {
        return None;
    }
    let prev = f[4].parse::<f64>().unwrap_or(0.0);
    let idx = f.iter().position(|s| looks_like_datetime(s))?;
    let stamp = f[idx];
    let tz = if stamp.contains('/') || stamp.len() == 14 { Shanghai } else { New_York };
    let ts = ts_from(stamp, tz).unwrap_or(0);
    let change = f.get(idx + 1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(price - prev);
    let pct = f.get(idx + 2).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    Some((price, change, pct, ts))
}

fn parse_hf(val: &str) -> Option<(f64, f64, f64, i64)> {
    let f: Vec<&str> = val.split(',').collect();
    if f.len() < 9 {
        return None;
    }
    let price = f[0].parse::<f64>().ok()?;
    if price <= 0.0 {
        return None;
    }
    let pct = f[1].parse::<f64>().unwrap_or(0.0);
    let prev = f[7].parse::<f64>().unwrap_or(0.0);
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
    let price = f[3].parse::<f64>().ok()?;
    if price <= 0.0 {
        return None;
    }
    let change = f[12].parse::<f64>().unwrap_or(0.0);
    let pct = f[13].parse::<f64>().unwrap_or(0.0);
    let ts = ts_from(f[5], Shanghai).unwrap_or(0);
    Some((price, change, pct, ts))
}

fn parse_sina(body: &str) -> Vec<Quote> {
    let mut out = Vec::new();
    for line in body.split(';') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((var, val)) = line.split_once('=') else { continue };
        let var = var.trim();
        let code = var.strip_prefix("var hq_str_").unwrap_or(var);
        let Some(inst) = pool_by_code(code) else { continue };
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
            out.push(Quote { id: inst.id.into(), price, change, pct, ts });
        }
    }
    out
}

fn parse_nf(val: &str) -> Option<(f64, f64, f64, i64)> {
    let f: Vec<&str> = val.split(',').collect();
    if f.len() < 11 {
        return None;
    }
    let price = f[8].parse::<f64>().ok()?;
    if price <= 0.0 {
        return None;
    }
    let prev = f[10].parse::<f64>().unwrap_or(0.0);
    let change = if prev > 0.0 { price - prev } else { 0.0 };
    let pct = if prev > 0.0 { change / prev * 100.0 } else { 0.0 };
    let time = f[1].trim();
    let ts = if time.len() == 6 && time.chars().all(|c| c.is_ascii_digit()) {
        let date = f.iter().find(|s| s.len() == 10 && s.chars().filter(|c| *c == '-').count() == 2)?;
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
    let price = f[1].parse::<f64>().ok()?;
    if price <= 0.0 {
        return None;
    }
    Some((price, f[2].parse::<f64>().unwrap_or(0.0), f[3].parse::<f64>().unwrap_or(0.0), 0))
}

fn parse_eastmoney(body: &str, code: &str) -> Option<Quote> {
    let inst = pool_by_code(code)?;
    let value: serde_json::Value = serde_json::from_str(body).ok()?;
    let data = value.get("data")?;
    if data.is_null() {
        return None;
    }
    let price = data.get("f43")?.as_f64()?;
    if price <= 0.0 {
        return None;
    }
    let change = data.get("f169").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let pct = data.get("f170").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let ts = data.get("f86").and_then(|v| v.as_i64()).unwrap_or(0) * 1000;
    Some(Quote { id: inst.id.into(), price, change, pct, ts })
}

fn looks_like_datetime(s: &str) -> bool {
    (s.len() == 14 && s.chars().all(|c| c.is_ascii_digit()))
        || (s.len() == 19 && s.contains(':') && s.chars().filter(|c| *c == '/' || *c == '-').count() == 2)
}

fn ts_from(s: &str, tz: Tz) -> Option<i64> {
    let naive = if s.len() == 14 {
        NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S").ok()?
    } else if s.contains('/') {
        NaiveDateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S").ok()?
    } else {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()?
    };
    tz.from_local_datetime(&naive).single().map(|dt| dt.timestamp_millis())
}
