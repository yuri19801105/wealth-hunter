use crate::types::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[derive(Clone)]
pub struct WebSocketService {
    socket: Rc<RefCell<Option<WebSocket>>>,
    on_message: Rc<RefCell<Option<Box<dyn Fn(WebsocketMessage)>>>>,
}

impl Default for WebSocketService {
    fn default() -> Self {
        Self {
            socket: Rc::new(RefCell::new(None)),
            on_message: Rc::new(RefCell::new(None)),
        }
    }
}

impl WebSocketService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, url: &str) -> Result<(), JsValue> {
        let ws = WebSocket::new(url)?;
        let on_message = self.on_message.clone();
        let socket_clone = self.socket.clone();

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
            if let Some(data) = event.data().as_string() {
                if let Ok(msg) = serde_json::from_str::<WebsocketMessage>(&data) {
                    if let Some(callback) = on_message.borrow_mut().as_mut() {
                        callback(msg);
                    }
                }
            }
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |_event: ErrorEvent| {
            web_sys::console::error_1(&"WebSocket error occurred".into());
        });
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let onopen_callback = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
            web_sys::console::log_1(&"WebSocket connected".into());
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        *socket_clone.borrow_mut() = Some(ws);
        Ok(())
    }

    pub fn disconnect(&self) {
        if let Some(ws) = self.socket.borrow().as_ref() {
            if ws.ready_state() == web_sys::WebSocket::OPEN {
                let _ = ws.close();
            }
        }
        *self.socket.borrow_mut() = None;
    }

    pub fn send(&self, message: &str) -> Result<(), JsValue> {
        if let Some(ws) = self.socket.borrow().as_ref() {
            if ws.ready_state() == web_sys::WebSocket::OPEN {
                ws.send_with_str(message)?;
            }
        }
        Ok(())
    }

    pub fn set_on_message<F>(&mut self, callback: F)
    where
        F: Fn(WebsocketMessage) + 'static,
    {
        *self.on_message.borrow_mut() = Some(Box::new(callback));
    }

    pub fn is_connected(&self) -> bool {
        self.socket
            .borrow()
            .as_ref()
            .map_or(false, |ws| ws.ready_state() == web_sys::WebSocket::OPEN)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SubscribeRequest {
    pub channel: String,
}

impl WebSocketService {
    pub fn subscribe(&self, channel: &str) -> Result<(), JsValue> {
        let request = SubscribeRequest {
            channel: channel.to_string(),
        };
        let json = serde_json::to_string(&request).unwrap_or_default();
        self.send(&json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_service_default() {
        let service = WebSocketService::new();
        assert!(!service.is_connected());
    }

    #[test]
    fn test_websocket_service_set_callback() {
        let mut service = WebSocketService::new();
        service.set_on_message(|_msg| {
            // Callback test
        });
        assert!(service.on_message.borrow().is_some());
    }
}
