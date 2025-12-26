use crate::state::{AppAction, AppState};
use leptos::prelude::*;

#[component]
pub fn Settings(state: Signal<AppState>, dispatch: Action<AppAction, ()>) -> impl IntoView {
    let is_running = move || state.with(|s| s.is_running);
    let api_key = RwSignal::new(String::new());
    let api_secret = RwSignal::new(String::new());
    let url = RwSignal::new(String::new());
    let capital = RwSignal::new(String::new());

    view! {
        <div class="settings-modal-overlay">
            <div class="settings-modal">
                <div class="settings-header">
                    <h2>"系统设置"</h2>
                    <button
                        class="close-btn"
                        on:click=move |_| { dispatch.dispatch(AppAction::ToggleSettings); }
                    >
                        "×"
                    </button>
                </div>

                <div class="settings-section">
                    <h3>"Gate.io API 配置"</h3>
                    <div class="form-group">
                        <label>"API Key"</label>
                        <input
                            type="password"
                            prop:value=api_key.get()
                            placeholder="请输入 API Key"
                            disabled=is_running()
                            on:input=move |e| {
                                api_key.set(event_target_value(&e));
                            }
                        />
                    </div>
                    <div class="form-group">
                        <label>"API Secret"</label>
                        <input
                            type="password"
                            prop:value=api_secret.get()
                            placeholder="请输入 API Secret"
                            disabled=is_running()
                            on:input=move |e| {
                                api_secret.set(event_target_value(&e));
                            }
                        />
                    </div>
                    <div class="form-group">
                        <label>"U-Perpetual URL"</label>
                        <input
                            type="text"
                            prop:value=url.get()
                            placeholder="https://api.gateio.ws/api/v4/..."
                            disabled=is_running()
                            on:input=move |e| {
                                url.set(event_target_value(&e));
                            }
                        />
                    </div>
                </div>

                <div class="settings-section">
                    <h3>"模拟交易设置"</h3>
                    <div class="form-group">
                        <label>"初始资金 (USDT)"</label>
                        <input
                            type="number"
                            prop:value=capital.get()
                            placeholder="请输入初始资金"
                            disabled=is_running()
                            on:input=move |e| {
                                capital.set(event_target_value(&e));
                            }
                        />
                    </div>
                </div>

                <div class="settings-actions">
                    <button
                        class="btn btn-primary"
                        disabled=is_running()
                        on:click=move |_| {
                            dispatch.dispatch(AppAction::UpdateGateioApiKeys(
                                api_key.get(),
                                api_secret.get(),
                                url.get(),
                            ));
                            if let Ok(c) = capital.get().parse::<f64>() {
                                dispatch.dispatch(AppAction::UpdateSimulationCapital(c));
                            }
                            dispatch.dispatch(AppAction::ToggleSettings);
                        }
                    >
                        "保存设置"
                    </button>
                    <button
                        class="btn btn-secondary"
                        on:click=move |_| { dispatch.dispatch(AppAction::ToggleSettings); }
                    >
                        "取消"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_component_renders() {
        let _settings = Settings;
    }
}
