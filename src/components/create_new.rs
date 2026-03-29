use dioxus::prelude::*;
use crate::{Route, db, i18n::{t, Lang}, components::navbar::Navbar};

#[component]
pub fn CreateNew() -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let mut motion = use_signal(|| String::new());
    let mut error  = use_signal(|| String::new());
    let nav = navigator();

    rsx! {
        Navbar { in_speech: false, debate_id: None }
        div { class: "page",
            h1 { {t(&lang, "create.motion")} }
            form {
                onsubmit: move |e| {
                    e.prevent_default();
                    let m = motion.read().trim().to_string();
                    if m.is_empty() {
                        *error.write() = "Motion cannot be empty".into();
                        return;
                    }
                    let id = format!("{}", std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
                    db::create_debate(&id, &m);
                    nav.push(Route::SpeakersRoute { id });
                },
                input {
                    r#type: "text",
                    value: "{motion}",
                    placeholder: t(&lang, "create.placeholder"),
                    oninput: move |e| *motion.write() = e.value(),
                }
                button { r#type: "submit", {t(&lang, "create.start")} }
            }
            if !error.read().is_empty() {
                p { style: "color:red", "{error}" }
            }
        }
    }
}
