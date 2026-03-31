mod components;
mod db;
mod i18n;
mod settings;
mod sync;
mod types;

use components::{
    create_new::CreateNew,
    home::{Home, HomeWithId},
    order_of_speakers::OrderOfSpeakers,
    settings_modal::SettingsModal,
    speech::Speech,
};
use dioxus::desktop::use_window;
use dioxus::prelude::*;
use i18n::Lang;
use settings::Settings;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const APP_ICON_ICO: &[u8] = include_bytes!("../assets/debate_logo.ico");

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[route("/")]
    HomeRoute {},
    #[route("/home/:id")]
    HomeWithId { id: String },
    #[route("/create")]
    CreateRoute {},
    #[route("/speakers/:id")]
    SpeakersRoute { id: String },
    #[route("/speech/:speaker/:id")]
    SpeechRoute { speaker: String, id: String },
}

fn main() {
    db::init();

    let mut config = dioxus::desktop::Config::new();
    if let Some(icon) = load_app_icon() {
        config = config.with_icon(icon);
    }

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

fn load_app_icon() -> Option<dioxus::desktop::tao::window::Icon> {
    let icon_dir = ico::IconDir::read(std::io::Cursor::new(APP_ICON_ICO)).ok()?;
    let best = icon_dir
        .entries()
        .iter()
        .max_by_key(|entry| u32::from(entry.width()) * u32::from(entry.height()))?;
    let image = best.decode().ok()?;

    dioxus::desktop::tao::window::Icon::from_rgba(
        image.rgba_data().to_vec(),
        image.width(),
        image.height(),
    )
    .ok()
}

#[component]
fn App() -> Element {
    let settings: Signal<Settings> = use_signal(|| settings::load());
    let show_settings: Signal<bool> = use_signal(|| false);
    let lang: Memo<String> = use_memo(move || settings.read().language.clone());

    provide_context(settings);
    provide_context(show_settings);
    provide_context(Lang(lang));

    use_effect(move || {
        let s = settings.read();
        let theme = s.theme.clone();
        let fs = s.font_size.clone();
        let dir = if s.language == "he" { "rtl" } else { "ltr" };
        let _ = document::eval(&format!(
            "document.documentElement.setAttribute('data-theme','{theme}');
             document.documentElement.setAttribute('data-font-size','{fs}');
             document.documentElement.dir='{dir}';"
        ));
    });

    let window = use_window();
    use_effect(move || {
        window.set_always_on_top(settings.read().always_on_top);
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
        if *show_settings.read() {
            SettingsModal {}
        }
    }
}

// Route components
#[component]
fn HomeRoute() -> Element {
    rsx! {
        Home {}
    }
}

#[component]
fn CreateRoute() -> Element {
    rsx! {
        CreateNew {}
    }
}

#[component]
fn SpeakersRoute(id: String) -> Element {
    rsx! {
        OrderOfSpeakers { id }
    }
}

#[component]
fn SpeechRoute(speaker: String, id: String) -> Element {
    rsx! {
        Speech { key: "{speaker}-{id}", speaker, id }
    }
}
