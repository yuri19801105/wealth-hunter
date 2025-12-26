use crate::dashboard::Dashboard;
use crate::settings::Settings;
use crate::state::{app_state_reducer, AppAction, AppState};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (state, set_state) = signal(AppState::default());
    let dispatch = Action::new(move |action: &AppAction| {
        let action = action.clone();
        set_state.update(|s| *s = app_state_reducer(s, action));
        async {}
    });

    let is_settings_open = move || state.with(|s| s.is_settings_open);

    view! {
        <div class="app">
            <nav class="app-nav">
                <div class="nav-brand">"Wealth Hunter"</div>
                <div class="nav-info">
                    <span class="aco-status">
                        {move || format!("ACO: {}", state.with(|s| s.aco_status.clone()))}
                    </span>
                    <span class="last-healing">
                        {move || format!("上次自愈: {}", state.with(|s| s.last_self_healing_time.clone()))}
                    </span>
                </div>
                <button
                    class="settings-btn"
                    on:click=move |_| { dispatch.dispatch(AppAction::ToggleSettings); }
                >
                    "⚙️ 设置"
                </button>
                <button
                    class="control-btn stop"
                    class:stop={state.with(|s| s.is_running)}
                    on:click=move |_| { dispatch.dispatch(AppAction::ToggleSystemRunning); }
                >
                    {move || if state.with(|s| s.is_running) { "停止系统" } else { "启动系统" }}
                </button>
            </nav>

            <main class="app-main">
                <Dashboard state=state.into() dispatch />
            </main>

            <Show when=move || is_settings_open()>
                <Settings state=state.into() dispatch />
            </Show>
        </div>
    }
}
