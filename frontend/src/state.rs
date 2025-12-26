use crate::types::*;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppState {
    pub is_running: bool,
    pub net_value: f64,
    pub initial_capital: f64,
    pub total_pnl: f64,
    pub pnl_ratio: f64,
    pub calmar_ratio: f64,
    pub max_drawdown: f64,
    pub aco_status: String,
    pub last_self_healing_time: String,
    pub current_features: FeatureVector,
    pub trades: Vec<TradeHistoryRecord>,
    pub net_value_history: VecDeque<AssetNetValue>,
    pub kline_data: Vec<KLineData>,
    pub is_settings_open: bool,
    pub gateio_api_key: String,
    pub gateio_api_secret: String,
    pub gateio_u_perpetual_url: String,
    pub simulation_capital: f64,
    pub logs: Vec<LogEntry>,
}

#[derive(Clone, Debug)]
pub enum AppAction {
    UpdateSystemStatus(bool, f64, f64, String, String),
    UpdateNetValue(f64, f64, f64, i64),
    UpdateFeatures(FeatureVector),
    AddTrade(TradeHistoryRecord),
    UpdateTrade(TradeHistoryRecord),
    AddKLineData(KLineData),
    UpdateKLineData(Vec<KLineData>),
    ClearData,
    ClearTrades,
    ToggleSettings,
    UpdateGateioApiKeys(String, String, String),
    UpdateSimulationCapital(f64),
    ToggleSystemRunning,
    AddLog(LogEntry),
    ClearLogs,
}

pub fn app_state_reducer(state: &AppState, action: AppAction) -> AppState {
    match action {
        AppAction::UpdateSystemStatus(
            is_running,
            calmar_ratio,
            max_drawdown,
            aco_status,
            last_self_healing_time,
        ) => AppState {
            is_running,
            calmar_ratio,
            max_drawdown,
            aco_status,
            last_self_healing_time,
            ..state.clone()
        },
        AppAction::UpdateNetValue(net_value, initial_capital, total_pnl, timestamp) => {
            let pnl_ratio = if initial_capital != 0.0 {
                (total_pnl / initial_capital) * 100.0
            } else {
                0.0
            };
            let mut net_value_history = state.net_value_history.clone();
            net_value_history.push_back(AssetNetValue {
                net_value,
                initial_capital,
                total_pnl,
                timestamp,
            });
            while net_value_history.len() > 100 {
                net_value_history.pop_front();
            }
            AppState {
                net_value,
                initial_capital,
                total_pnl,
                pnl_ratio,
                net_value_history,
                ..state.clone()
            }
        }
        AppAction::UpdateFeatures(features) => AppState {
            current_features: features,
            ..state.clone()
        },
        AppAction::AddTrade(trade) => {
            let mut trades = state.trades.clone();
            trades.push(trade);
            AppState {
                trades,
                ..state.clone()
            }
        }
        AppAction::UpdateTrade(updated_trade) => {
            let mut trades = state.trades.clone();
            if let Some(pos) = trades.iter().position(|t| t.id == updated_trade.id) {
                trades[pos] = updated_trade;
            }
            AppState {
                trades,
                ..state.clone()
            }
        }
        AppAction::AddKLineData(kline_data) => {
            let mut kline_data_list = state.kline_data.clone();
            kline_data_list.push(kline_data);
            while kline_data_list.len() > 100 {
                kline_data_list.remove(0);
            }
            AppState {
                kline_data: kline_data_list,
                ..state.clone()
            }
        }
        AppAction::UpdateKLineData(data) => AppState {
            kline_data: data,
            ..state.clone()
        },
        AppAction::ClearData => AppState::default(),
        AppAction::ClearTrades => AppState {
            trades: Vec::new(),
            ..state.clone()
        },
        AppAction::ToggleSettings => AppState {
            is_settings_open: !state.is_settings_open,
            ..state.clone()
        },
        AppAction::UpdateGateioApiKeys(api_key, api_secret, u_perpetual_url) => AppState {
            gateio_api_key: api_key,
            gateio_api_secret: api_secret,
            gateio_u_perpetual_url: u_perpetual_url,
            ..state.clone()
        },
        AppAction::UpdateSimulationCapital(capital) => AppState {
            simulation_capital: capital,
            ..state.clone()
        },
        AppAction::ToggleSystemRunning => AppState {
            is_running: !state.is_running,
            ..state.clone()
        },
        AppAction::AddLog(log_entry) => {
            let mut logs = state.logs.clone();
            logs.push(log_entry);
            while logs.len() > 100 {
                logs.remove(0);
            }
            AppState {
                logs,
                ..state.clone()
            }
        }
        AppAction::ClearLogs => AppState {
            logs: Vec::new(),
            ..state.clone()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert!(!state.is_running);
        assert!((state.net_value - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_app_state_update_system_status() {
        let initial = AppState::default();
        let updated = app_state_reducer(
            &initial,
            AppAction::UpdateSystemStatus(
                true,
                2.5,
                10.0,
                "running".to_string(),
                "2025-12-10 12:00:00".to_string(),
            ),
        );
        assert!(updated.is_running);
        assert!((updated.calmar_ratio - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_app_state_update_net_value() {
        let initial = AppState::default();
        let updated = app_state_reducer(
            &initial,
            AppAction::UpdateNetValue(10500.0, 10000.0, 500.0, 1733856000),
        );
        assert!((updated.net_value - 10500.0).abs() < f64::EPSILON);
        assert!((updated.pnl_ratio - 5.0).abs() < f64::EPSILON);
        assert_eq!(updated.net_value_history.len(), 1);
    }

    #[test]
    fn test_app_state_toggle_settings() {
        let initial = AppState::default();
        assert!(!initial.is_settings_open);
        let toggled = app_state_reducer(&initial, AppAction::ToggleSettings);
        assert!(toggled.is_settings_open);
    }

    #[test]
    fn test_app_state_add_trade() {
        let initial = AppState::default();
        let trade = TradeHistoryRecord {
            id: "1".to_string(),
            direction: "long".to_string(),
            entry_price: 100.0,
            exit_price: None,
            entry_time: "2025-12-10 12:00:00".to_string(),
            exit_time: None,
            volume: 1.0,
            status: "open".to_string(),
            pnl: 0.0,
            pnl_ratio: 0.0,
        };
        let updated = app_state_reducer(&initial, AppAction::AddTrade(trade.clone()));
        assert_eq!(updated.trades.len(), 1);
        assert_eq!(updated.trades[0], trade);
    }

    #[test]
    fn test_app_state_toggle_system_running() {
        let initial = AppState::default();
        assert!(!initial.is_running);
        let toggled_on = app_state_reducer(&initial, AppAction::ToggleSystemRunning);
        assert!(toggled_on.is_running);
        let toggled_off = app_state_reducer(&toggled_on, AppAction::ToggleSystemRunning);
        assert!(!toggled_off.is_running);
    }

    #[test]
    fn test_app_state_clear_data() {
        let mut state = AppState::default();
        state.is_running = true;
        state.net_value = 10500.0;
        state.total_pnl = 500.0;
        let cleared = app_state_reducer(&state, AppAction::ClearData);
        assert!(!cleared.is_running);
        assert!((cleared.net_value - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_app_state_add_kline_data() {
        let initial = AppState::default();
        let kline = KLineData {
            timestamp: 1733856000,
            open: 50000.0,
            high: 50500.0,
            low: 49800.0,
            close: 50300.0,
            volume: 1000000,
        };
        let updated = app_state_reducer(&initial, AppAction::AddKLineData(kline.clone()));
        assert_eq!(updated.kline_data.len(), 1);
        assert_eq!(updated.kline_data[0], kline);
    }

    #[test]
    fn test_app_state_add_log() {
        let initial = AppState::default();
        let log = LogEntry {
            time: "2025-12-26 18:00:00".to_string(),
            level: "INFO".to_string(),
            message: "Test message".to_string(),
        };
        let updated = app_state_reducer(&initial, AppAction::AddLog(log.clone()));
        assert_eq!(updated.logs.len(), 1);
        assert_eq!(updated.logs[0], log);
    }

    #[test]
    fn test_app_state_update_gateio_keys() {
        let initial = AppState::default();
        let updated = app_state_reducer(
            &initial,
            AppAction::UpdateGateioApiKeys(
                "test_key".to_string(),
                "test_secret".to_string(),
                "https://api.gateio.ws/api/v4/spot/".to_string(),
            ),
        );
        assert_eq!(updated.gateio_api_key, "test_key");
        assert_eq!(updated.gateio_api_secret, "test_secret");
    }

    #[test]
    fn test_app_state_update_simulation_capital() {
        let initial = AppState::default();
        let updated = app_state_reducer(&initial, AppAction::UpdateSimulationCapital(50000.0));
        assert!((updated.simulation_capital - 50000.0).abs() < f64::EPSILON);
    }
}
