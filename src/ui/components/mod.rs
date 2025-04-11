use theme::{use_theme_context, ThemeState};
use yew::prelude::*;

pub mod sidenavbar;
pub mod theme;

#[derive(Clone, PartialEq)]
pub struct AppContext {
    pub theme: UseReducerHandle<ThemeState>,
    pub theme_cycle: Vec<&'static str>,

    pub interfaces: UseStateHandle<Vec<String>>,
    pub interface: UseStateHandle<String>,
}

#[derive(PartialEq, Properties)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component]
pub fn AppProvider(props: &AuthProviderProps) -> Html {
    let theme: UseReducerHandle<ThemeState> = use_theme_context();
    let theme_cycle: Vec<&str> = vec!["light", "dark"];

    let interfaces = use_state(|| vec!["any".to_string()]);
    let interface = use_state(|| String::from("any"));

    html! {
        <ContextProvider<AppContext> context={AppContext {
            theme: theme.clone(),
            theme_cycle: theme_cycle,
            interfaces,
            interface

        }}>
        <main class ={theme.current}>
                <div class="dark:bg-gray-900 min-h-screen">
            { for props.children.iter() }
                </div>
        </main>
        </ContextProvider<AppContext>>

    }
}
