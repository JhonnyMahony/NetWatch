mod ui;

use ui::app::Watch;
use ui::components::AppProvider;
use yew::{function_component, html, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
    <AppProvider>
        <Watch />
    </AppProvider>

    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
