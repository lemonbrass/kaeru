use app::App;

pub mod app;
pub mod cli;
pub mod config;
pub mod error;
pub mod gen;
pub mod genman;
pub mod globals;
pub mod manager;
pub mod util;

fn main() {
    let _app = App::init();
}
