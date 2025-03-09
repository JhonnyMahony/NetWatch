use gloo::console;
use gloo::storage::{errors::StorageError, LocalStorage, Storage};
use std::rc::Rc;
use web_sys::{window, MediaQueryList, Window};
use yew::*;

pub enum ThemeAction {
    Light,
    Dark,
}

#[derive(PartialEq, Debug)]
pub struct ThemeState {
    pub current: &'static str,
}

pub fn use_theme_context() -> impl Hook<Output = UseReducerHandle<ThemeState>> + 'static {
    use_reducer(ThemeState::default)
}

impl Default for ThemeState {
    fn default() -> Self {
        let ls_theme_option: Result<String, StorageError> = LocalStorage::get("theme");

        let ls_theme: &str = match &ls_theme_option {
            Ok(theme) => theme,
            _ => {
                let window: Window = window().expect("No Window Object!");
                let match_media_result = window.match_media("(prefers-color-scheme: dark)"); // : Result<Option<MediaQueryList>, JsValue>
                match match_media_result {
                    Ok(match_media_option) => {
                        let match_media: MediaQueryList =
                            match_media_option.expect("MEDIAQUERYLIST NOT SUPPORTED!");
                        if match_media.matches() {
                            "dark"
                        } else {
                            "light"
                        }
                    }
                    _ => "light",
                }
            }
        };

        match ls_theme {
            "dark" => Self { current: "dark" },
            "light" | _ => Self { current: "light" },
        }
    }
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_theme = match action {
            ThemeAction::Light => "light",
            ThemeAction::Dark => "dark",
        };

        match LocalStorage::set("theme", next_theme) {
	    Ok(()) => console::log!(format!("Theme set to {}", next_theme)),
	    _ => console::error!("Couldn't set LocalStorage. Please turn the feature in your Browser on if possible."),
	};

        Self {
            current: next_theme,
        }
        .into()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,
}

pub fn handle_props_class(props: &Props) -> &'static str {
    match props.class {
        Some(class) => class,
        None => "",
    }
}

#[function_component(Light)]
pub fn light(props: &Props) -> Html {
    html! {
    <svg class={ handle_props_class(props) } width="1em" height="1em" viewBox="0 0 16 16" class="nav__svg light_svg" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path d="M3.5 8a4.5 4.5 0 1 1 9 0 4.5 4.5 0 0 1-9 0z"/>
            <path fill-rule="evenodd" d="M8.202.28a.25.25 0 0 0-.404 0l-.91 1.255a.25.25 0 0 1-.334.067L5.232.79a.25.25 0 0 0-.374.155l-.36 1.508a.25.25 0 0 1-.282.19l-1.532-.245a.25.25 0 0 0-.286.286l.244 1.532a.25.25 0 0 1-.189.282l-1.509.36a.25.25 0 0 0-.154.374l.812 1.322a.25.25 0 0 1-.067.333l-1.256.91a.25.25 0 0 0 0 .405l1.256.91a.25.25 0 0 1 .067.334L.79 10.768a.25.25 0 0 0 .154.374l1.51.36a.25.25 0 0 1 .188.282l-.244 1.532a.25.25 0 0 0 .286.286l1.532-.244a.25.25 0 0 1 .282.189l.36 1.508a.25.25 0 0 0 .374.155l1.322-.812a.25.25 0 0 1 .333.067l.91 1.256a.25.25 0 0 0 .405 0l.91-1.256a.25.25 0 0 1 .334-.067l1.322.812a.25.25 0 0 0 .374-.155l.36-1.508a.25.25 0 0 1 .282-.19l1.532.245a.25.25 0 0 0 .286-.286l-.244-1.532a.25.25 0 0 1 .189-.282l1.508-.36a.25.25 0 0 0 .155-.374l-.812-1.322a.25.25 0 0 1 .067-.333l1.256-.91a.25.25 0 0 0 0-.405l-1.256-.91a.25.25 0 0 1-.067-.334l.812-1.322a.25.25 0 0 0-.155-.374l-1.508-.36a.25.25 0 0 1-.19-.282l.245-1.532a.25.25 0 0 0-.286-.286l-1.532.244a.25.25 0 0 1-.282-.189l-.36-1.508a.25.25 0 0 0-.374-.155l-1.322.812a.25.25 0 0 1-.333-.067L8.203.28zM8 2.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11z"/>
        </svg>
    }
}

#[function_component(Dark)]
pub fn dark(props: &Props) -> Html {
    html! {
    <svg class={ handle_props_class(props) } width="1em" height="1em" viewBox="0 0 16 16" class="nav__svg dark_svg" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M14.53 10.53a7 7 0 0 1-9.058-9.058A7.003 7.003 0 0 0 8 15a7.002 7.002 0 0 0 6.53-4.47z"/>
        </svg>
    }
}
