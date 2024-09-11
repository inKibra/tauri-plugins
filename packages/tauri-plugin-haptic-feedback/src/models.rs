use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HapticVibratePattern {
    Short,
    Medium,
    Long,
    Custom(CustomPattern),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomPattern {
    pub durations: Vec<u64>,
    pub intensities: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HapticVibrateRequest {
    pub pattern: HapticVibratePattern,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImpactFeedbackStyle {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImpactFeedbackRequest {
    pub style: ImpactFeedbackStyle,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HapticResponse {
    pub success: bool,
}