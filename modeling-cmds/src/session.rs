use schemars::JsonSchema;
use serde::Deserialize;

use crate::shared::PostEffectType;

/// Params for starting the engine.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(default)]
pub struct EngineParams {
    /// Width of the video feed. Must be a multiple of 4.
    pub video_res_width: u32,
    /// Height of the video feed. Must be a multiple of 4.
    pub video_res_height: u32,
    /// Frames per second of the video feed.
    pub fps: u32,
    /// If true, engine will render video frames as fast as it can.
    pub unlocked_framerate: bool,
    /// Engine Post effects (such as SSAO)
    pub post_effect: Option<PostEffectType>,
    /// If true, will start a webrtc connection.
    pub webrtc: bool,
    /// An optional identifier for a pool of engine instances.
    /// The 'default' pool is used when none is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    /// If true, will show the grid at the start of the session.
    pub show_grid: bool,
    /// If given, when the session ends, the modeling commands sent during
    /// the session will be written out to this filename.
    /// For debugging.
    pub replay: Option<String>,
}

impl Default for EngineParams {
    fn default() -> Self {
        Self {
            video_res_width: 1280,
            video_res_height: 720,
            fps: 60,
            unlocked_framerate: false,
            post_effect: None,
            webrtc: true,
            pool: None,
            show_grid: false,
            replay: None,
        }
    }
}

impl EngineParams {
    /// Returns a default EngineParams with the grid enabled.
    /// This is primarily useful for engine testing.
    pub fn default_with_grid() -> Self {
        Self {
            show_grid: true,
            ..Self::default()
        }
    }
}
