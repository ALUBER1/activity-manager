pub mod app;
pub mod components;
pub mod utils;
pub mod models;
pub mod errors;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
