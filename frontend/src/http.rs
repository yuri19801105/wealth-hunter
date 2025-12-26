use crate::types::*;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Clone)]
pub struct HttpService {
    base_url: String,
}

impl Default for HttpService {
    fn default() -> Self {
        Self {
            base_url: String::new(),
        }
    }
}

impl HttpService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn fetch_json<T>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<&str>,
    ) -> Result<T, JsValue>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let opts = RequestInit::new();
        opts.set_method(method);
        opts.set_mode(RequestMode::Cors);

        if let Some(body_str) = body {
            opts.set_body(&JsValue::from_str(body_str));
        }

        let request = Request::new_with_str_and_init(&url, &opts)?;

        let window = web_sys::window().expect("no global `window` exists");
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        let text_promise = resp.text().expect("resp.text() should return JsValue");
        let text_future = JsFuture::from(text_promise);
        let text_value = text_future.await?;
        let json_str = text_value.as_string().unwrap_or_default();
        serde_json::from_str(&json_str).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub async fn configure_settings(
        &self,
        api_key: &str,
        initial_capital: f64,
    ) -> Result<(), JsValue> {
        let request = ConfigureSettingsRequest {
            api_key: api_key.to_string(),
            initial_capital,
        };
        let body = serde_json::to_string(&request).ok();
        let body_str = body.as_deref();
        self.fetch_json::<()>("/api/configure", "POST", body_str)
            .await
    }

    pub async fn start_system(&self) -> Result<(), JsValue> {
        self.fetch_json::<()>("/api/start", "POST", None).await
    }

    pub async fn stop_system(&self) -> Result<(), JsValue> {
        self.fetch_json::<()>("/api/stop", "POST", None).await
    }

    pub async fn get_status(&self) -> Result<DashboardStatus, JsValue> {
        self.fetch_json("/api/status", "GET", None).await
    }

    pub async fn get_net_value_history(&self) -> Result<Vec<AssetNetValue>, JsValue> {
        self.fetch_json("/api/net-value-history", "GET", None).await
    }

    pub async fn get_trade_history(&self) -> Result<Vec<TradeHistoryRecord>, JsValue> {
        self.fetch_json("/api/trade-history", "GET", None).await
    }

    pub async fn get_kline_data(
        &self,
        symbol: &str,
        interval: &str,
    ) -> Result<Vec<KLineData>, JsValue> {
        let endpoint = format!("/api/kline?symbol={}&interval={}", symbol, interval);
        self.fetch_json(&endpoint, "GET", None).await
    }

    pub async fn health_check(&self) -> Result<bool, JsValue> {
        let result: Result<serde_json::Value, JsValue> =
            self.fetch_json("/api/health", "GET", None).await;
        result
            .map(|v| v.get("status").and_then(|s| s.as_str()) == Some("ok"))
            .map_err(|e| e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_service_default() {
        let service = HttpService::new();
        assert!(service.base_url.is_empty());
    }

    #[test]
    fn test_http_service_with_base_url() {
        let service = HttpService::with_base_url("http://localhost:8080");
        assert_eq!(service.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_configure_settings_request_serialization() {
        let request = ConfigureSettingsRequest {
            api_key: "test_key".to_string(),
            initial_capital: 10000.0,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test_key"));
        assert!(json.contains("10000"));
    }
}
