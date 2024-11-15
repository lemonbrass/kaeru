use app::App;

pub mod app;
pub mod config;
pub mod error;
pub mod gen;
pub mod globals;
pub mod manager;
pub mod package_pack;
pub mod util;

fn main() {
    let _app = App::init();
}
