use dioxus::prelude::*;
use crate::{Route, db, i18n::{t, Lang}, components::navbar::Navbar};

const ROLES: &[&str] = &["PM","DPM","LO","DLO","MG","GW","MO","OW"];

#[component]
pub fn OrderOfSpeakers(id: String) -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let mut speakers = use_signal(|| vec![String::new(); 8]);
    let nav = navigator();
    let debate_id = id.clone();

    rsx! {
        Navbar { in_speech: false, debate_id: None }
        div { class: "page",
            h1 { {t(&lang, "speakers.title")} }
            form {
                onsubmit: move |e| {
                    e.prevent_default();
                    let owned: Vec<(String, String)> = ROLES.iter().zip(speakers.read().iter())
                        .map(|(r, s)| (r.to_string(), s.clone()))
                        .collect();
                    let refs: Vec<(&str, &str)> = owned.iter().map(|(r,s)| (r.as_str(), s.as_str())).collect();
                    db::save_speakers(&debate_id, &refs);
                    nav.push(Route::SpeechRoute { speaker: "PM".into(), id: debate_id.clone() });
                },
                for (i, role) in ROLES.iter().enumerate() {
                    div { class: "inputdiv",
                        label { {t(&lang, &format!("home.{}", role.to_lowercase()))} }
                        input {
                            r#type: "text",
                            value: "{speakers.read()[i]}",
                            oninput: move |e| speakers.write()[i] = e.value(),
                        }
                    }
                }
                button { r#type: "submit", {t(&lang, "speech.submit")} }
            }
        }
    }
}
