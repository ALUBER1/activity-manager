pub mod app;
pub mod helper;
pub mod components;
pub mod functions;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
