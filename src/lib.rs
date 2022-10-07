#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

use std::time::Duration;
use tokio::time::sleep;

#[napi]
pub async fn click_input(x: f64, y: f64, typing: Option<String>) {
  autopilot::mouse::move_to(autopilot::geometry::Point::new(x, y)).expect("move mouse err");

  sleep(Duration::from_millis(100)).await;

  autopilot::mouse::click(autopilot::mouse::Button::Left, Some(100));

  if let Some(msg) = typing {
    autopilot::key::type_string(msg.as_str(), &[], 2000., 0.);
    sleep(Duration::from_millis(100)).await;
  }
}
