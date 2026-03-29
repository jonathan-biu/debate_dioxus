use dioxus::prelude::*;
use crate::{Route, i18n::{t, Lang}, components::icons::IconSettings};

#[component]
pub fn Navbar(in_speech: bool, debate_id: Option<String>) -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let mut show_settings = use_context::<Signal<bool>>();

    rsx! {
        nav { class: "navbar",
            div { class: "navbar-center",
                ul { class: "nav-links",
                    if in_speech {
                        {
                            let id = debate_id.clone().unwrap_or_default();
                            let roles = ["PM","LO","DPM","DLO","MG","MO","GW","OW"];
                            rsx! {
                                for role in roles {
                                    li {
                                        Link {
                                            to: Route::SpeechRoute { speaker: role.to_string(), id: id.clone() },
                                            {t(&lang, &format!("nav.{}", role.to_lowercase()))}
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        li { Link { to: Route::HomeRoute {}, {t(&lang, "nav.home")} } }
                        li { Link { to: Route::CreateRoute {}, {t(&lang, "nav.new_motion")} } }
                    }
                }
            }
            div { class: "navbar-right",
                button {
                    class: "settings-button",
                    onclick: move |_| *show_settings.write() = true,
                    IconSettings {}
                }
            }
        }
    }
}
