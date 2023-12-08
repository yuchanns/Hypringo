use anyhow::Result;
use async_trait::async_trait;
use serde_derive::Deserialize;

use super::Hyprctl;

#[async_trait]
pub trait Monitors {
    async fn get_monitors(&self) -> Result<Vec<Monitor>>;
}

#[async_trait]
impl Monitors for Hyprctl {
    async fn get_monitors(&self) -> Result<Vec<Monitor>> {
        let data = self.write("j/monitors".to_string()).await?;
        let monitors: Vec<Monitor> = serde_json::from_str(data.as_str())?;
        Ok(monitors)
    }
}

#[derive(Deserialize, Debug)]
pub struct Monitor {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "refreshRate")]
    pub refresh_rate: f64,
    pub x: i32,
    pub y: i32,
    #[serde(rename = "activeWorkspace")]
    pub active_workspace: Workspace,
    #[serde(rename = "specialWorkspace")]
    pub special_workspace: Workspace,
    pub reserved: [i32; 4],
    pub scale: f64,
    pub transform: i32,
    pub focused: bool,
    #[serde(rename = "dpmsStatus")]
    pub dpms_status: bool,
    pub vrr: bool,
    #[serde(rename = "activelyTearing")]
    pub actively_tearing: bool,
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}