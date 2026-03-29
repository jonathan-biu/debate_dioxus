use dioxus::prelude::*;
use std::collections::HashMap;
use crate::{
    Route, db, i18n::{t, Lang}, settings::Settings,
    types::SPEAKER_ORDER,
    components::{navbar::Navbar, timer::Timer},
};

// (speech, rebuttal, poi)
type Edits = HashMap<String, (String, String, String)>;

#[component]
pub fn Speech(speaker: String, id: String) -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let settings = use_context::<Signal<Settings>>();
    let nav = navigator();

    // Single signal holding edits for ALL speakers — persists across speaker nav
    // but is keyed so each speaker has independent text
    let mut edits: Signal<Edits> = use_signal(|| HashMap::new());

    // Load from DB into edits map if not already present for this speaker
    let sp_key = speaker.clone();
    let did_load = id.clone();
    use_effect(move || {
        let key = sp_key.clone();
        if !edits.read().contains_key(&key) {
            if let Some(d) = db::get_debate(&did_load) {
                let s = d.get_speech(&key);
                edits.write().insert(key, (s.speech.clone(), s.rebuttal.clone(), s.poi.clone()));
            }
        }
    });

    let current = edits.read().get(&speaker).cloned().unwrap_or_default();
    let (speech_val, rebuttal_val, poi_val) = current;

    let initial = db::get_debate(&id);
    let init_name   = initial.as_ref().map(|d| d.get_speech(&speaker).speaker.clone()).unwrap_or_default();
    let init_motion = initial.as_ref().map(|d| d.motion.clone()).unwrap_or_default();

    let sp2   = speaker.clone();
    let did2  = id.clone();
    let name2 = init_name.clone();

    let handle_submit = move |e: FormEvent| {
        e.prevent_default();
        let (sp_val, rb_val, po_val) = edits.read().get(&sp2).cloned().unwrap_or_default();
        db::save_speech(&did2, &sp2, &name2, &sp_val, &rb_val, &po_val);

        let idx = SPEAKER_ORDER.iter().position(|&r| r == sp2.as_str()).unwrap_or(0);
        if idx + 1 < SPEAKER_ORDER.len() {
            nav.push(Route::SpeechRoute { speaker: SPEAKER_ORDER[idx + 1].to_string(), id: did2.clone() });
        } else {
            nav.push(Route::HomeWithId { id: did2.clone() });
        }
    };

    let role_label   = t(&lang, &format!("home.{}", speaker.to_lowercase())).to_string();
    let title        = format!("{role_label} - {init_name}");
    let is_pm        = speaker == "PM";
    let inc_rebuttal = settings.read().include_rebuttal;
    let inc_poi      = settings.read().include_poi;
    let sp3 = speaker.clone();
    let sp4 = speaker.clone();
    let sp5 = speaker.clone();

    rsx! {
        Navbar { in_speech: true, debate_id: Some(id.clone()) }
        div { class: "page",
            h1 { "{init_motion}" }
            div { class: "speech-header",
                h2 { "{title}" }
                Timer {}
            }
            form { onsubmit: handle_submit,
                div {
                    label { {t(&lang, "speech.arguments")} }
                    textarea { name: "speech", rows: "8", value: "{speech_val}",
                        oninput: move |e| {
                            let v = e.value();
                            if let Some(entry) = edits.write().get_mut(&sp3) { entry.0 = v; }
                        }
                    }
                }
                if !is_pm && inc_rebuttal {
                    div {
                        label { {t(&lang, "speech.rebuttal")} }
                        textarea { name: "rebuttal", rows: "6", value: "{rebuttal_val}",
                            oninput: move |e| {
                                let v = e.value();
                                if let Some(entry) = edits.write().get_mut(&sp4) { entry.1 = v; }
                            }
                        }
                    }
                }
                if inc_poi {
                    div {
                        label { {t(&lang, "speech.poi")} }
                        textarea { name: "poi", rows: "4", value: "{poi_val}",
                            oninput: move |e| {
                                let v = e.value();
                                if let Some(entry) = edits.write().get_mut(&sp5) { entry.2 = v; }
                            }
                        }
                    }
                }
                button { r#type: "submit", {t(&lang, "speech.submit")} }
            }
        }
    }
}
