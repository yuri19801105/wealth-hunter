use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AssetNetValue {
    pub net_value: f64,
    pub initial_capital: f64,
    pub total_pnl: f64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FeatureVector {
    pub shannon_entropy: f64,
    pub fft_spectrum: Vec<f64>,
    pub short_ma: f64,
    pub long_ma: f64,
    pub atr: f64,
    pub cks_trend_strength: f64,
    pub instantaneous_entropy_increase: f64,
    pub normalized_gap: f64,
    pub garch_volatility: f64,
    pub fractal_dimension: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TradeHistoryRecord {
    pub id: String,
    pub direction: String,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub entry_time: String,
    pub exit_time: Option<String>,
    pub volume: f64,
    pub status: String,
    pub pnl: f64,
    pub pnl_ratio: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct KLineData {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct LogEntry {
    pub time: String,
    pub level: String,
    pub message: String,
}

impl Default for LogEntry {
    fn default() -> Self {
        Self {
            time: String::new(),
            level: String::new(),
            message: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct DashboardStatus {
    pub calmar_ratio: f64,
    pub max_drawdown: f64,
    pub timestamp: i64,
    pub is_running: bool,
    pub total_equity: f64,
    pub daily_change: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ConfigureSettingsRequest {
    pub api_key: String,
    pub initial_capital: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WebsocketMessage {
    StatusUpdate(DashboardStatus),
    NetValueUpdate(AssetNetValue),
    FeatureUpdate(FeatureVector),
    TradeUpdate(TradeHistoryRecord),
    KLineUpdate(KLineData),
    SystemEvent(String),
    LogUpdate(LogEntry),
}
