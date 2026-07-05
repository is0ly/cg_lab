mod app;
pub mod draw;
mod engine;
pub mod input;
pub mod math;
pub mod random;
pub mod render_data;
mod renderer;
pub mod sketch;
mod time;

pub use sketch::Sketch;

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

pub fn run<S>(config: AppConfig)
where
    S: Sketch + 'static,
{
    app::run::<S>(config);
}
