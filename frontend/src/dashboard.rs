use crate::state::{AppAction, AppState};
use leptos::prelude::*;

#[component]
pub fn Dashboard(state: Signal<AppState>, dispatch: Action<AppAction, ()>) -> impl IntoView {
    let trades = move || state.with(|s| s.trades.clone());
    let logs = move || {
        state.with(|s| {
            s.logs
                .iter()
                .rev()
                .enumerate()
                .map(|(i, log)| (i, log.clone()))
                .collect::<Vec<_>>()
        })
    };

    view! {
        <div class="dashboard">
            <header class="dashboard-header">
                <h1>"A-PCO 智能交易系统"</h1>
                <div class="status-badge" class:running={state.with(|s| s.is_running)}>
                    {move || if state.with(|s| s.is_running) { "运行中" } else { "已停止" }}
                </div>
            </header>

            <div class="stats-grid">
                <div class="stat-card">
                    <h3>"净值"</h3>
                    <p class="stat-value">{move || format!("{:.2}", state.with(|s| s.net_value))}</p>
                </div>
                <div class="stat-card">
                    <h3>"总盈亏"</h3>
                    <p class="stat-value pnl">
                        {move || format!("{:.2} ({:.2}%)", state.with(|s| s.total_pnl), state.with(|s| s.pnl_ratio))}
                    </p>
                </div>
                <div class="stat-card">
                    <h3>"卡玛比率"</h3>
                    <p class="stat-value">{move || format!("{:.2}", state.with(|s| s.calmar_ratio))}</p>
                </div>
                <div class="stat-card">
                    <h3>"最大回撤"</h3>
                    <p class="stat-value warning">{move || format!("{:.2}%", state.with(|s| s.max_drawdown))}</p>
                </div>
            </div>

            <div class="features-section">
                <h2>"当前特征"</h2>
                <div class="features-grid">
                    <div class="feature-item">
                        <span class="feature-label">"香农熵"</span>
                        <span class="feature-value">
                            {move || format!("{:.4}", state.with(|s| s.current_features.shannon_entropy))}
                        </span>
                    </div>
                    <div class="feature-item">
                        <span class="feature-label">"ATR"</span>
                        <span class="feature-value">
                            {move || format!("{:.2}", state.with(|s| s.current_features.atr))}
                        </span>
                    </div>
                    <div class="feature-item">
                        <span class="feature-label">"波动率"</span>
                        <span class="feature-value">
                            {move || format!("{:.4}", state.with(|s| s.current_features.garch_volatility))}
                        </span>
                    </div>
                </div>
            </div>

            <div class="trades-section">
                <h2>"交易记录"</h2>
                <div class="trades-table-actions">
                    <button
                        class="btn btn-secondary btn-sm"
                        on:click=move |_| { dispatch.dispatch(AppAction::ClearTrades); }
                    >
                        "清空记录"
                    </button>
                </div>
                <div class="trades-table">
                    <table>
                        <thead>
                            <tr>
                                <th>"方向"</th>
                                <th>"入场价"</th>
                                <th>"状态"</th>
                                <th>"盈亏"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For each=trades key=|t| t.id.clone() children=move |trade| {
                                let direction = trade.direction.clone();
                                let status = trade.status.clone();
                                let is_long = direction == "LONG";
                                view! {
                                    <tr class="trade-row" class:long=is_long class:short=!is_long>
                                        <td>{direction}</td>
                                        <td>{format!("{:.2}", trade.entry_price)}</td>
                                        <td>{status}</td>
                                        <td class="pnl">{format!("{:.2}", trade.pnl)}</td>
                                    </tr>
                                }
                            }/>
                        </tbody>
                    </table>
                </div>
            </div>

            <div class="logs-section">
                <h2>"系统日志"</h2>
                <div class="logs-container">
                    <For each=logs key=|&(i, _)| i children=move |(_, log)| {
                        let time = log.time.clone();
                        let level = log.level.clone();
                        let message = log.message.clone();
                        let is_error = level == "ERROR";
                        let is_warn = level == "WARN";
                        view! {
                            <div class="log-entry" class:error=is_error class:warn=is_error && !is_error class:info=!is_error && !is_warn>
                                <span class="log-time">{time}</span>
                                <span class="log-level">[{level}]</span>
                                <span class="log-message">{message}</span>
                            </div>
                        }
                    }/>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_component_renders() {
        let _dashboard = Dashboard;
    }
}
