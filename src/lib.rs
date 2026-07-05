mod app;
pub mod draw;
mod engine;
mod input;
pub mod math;
mod random;
pub mod render_data;
mod renderer;
mod sketch;
mod time;

#[derive(Debug, Clone)]

pub struct AppConfig {
    pub title: String,

    pub width: u16,

    pub height: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "cg_lab".to_string(),
            width: 500,
            height: 500,
        }
    }
}
pub fn run(config: AppConfig) {
    app::run(config);
}
