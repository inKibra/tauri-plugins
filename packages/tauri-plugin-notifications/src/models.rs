use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPermissionStatus {
    pub status: String, // "prompt" | "denied" | "granted"
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRegistrationStatus {
    pub is_registered: bool,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRegistrationResult {
    pub success: bool,
    pub token: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationEventType {
    BackgroundTap,
    ForegroundTap,
    ForegroundDelivery,
    BackgroundDelivery,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationEvent {
    pub type_: NotificationEventType,
    pub payload: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchNotificationResult {
    pub success: bool,
}