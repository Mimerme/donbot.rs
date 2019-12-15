pub mod twitch;
//pub use twitch::get_channel;
pub use twitch::download_clip;

//This is using the Helix API
pub use twitch::get_helix_top_clips;
