//! GPUI integration for `alhazen-ms-core`: a `VideoPlayer` entity and a `video_view` element.

mod cover;
mod element;
mod player;

pub use alhazen_ms_core::{self, Metadata, Picture, PlayerConfig, PlayerEvent, PlayerState, Source};
pub use element::{VideoView, video_view};
pub use player::VideoPlayer;
