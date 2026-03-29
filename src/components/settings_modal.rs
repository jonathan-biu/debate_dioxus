use dioxus::prelude::*;
use crate::{i18n::{t, Lang}, settings::{self, Settings}, components::icons::IconClose};

#[component]
pub fn SettingsModal() -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let mut show = use_context::<Signal<bool>>();
    let mut s = use_context::<Signal<Settings>>();

    let close = move |_| {
        settings::save(&s.read());
        *show.write() = false;
    };

    rsx! {
        div { class: "settings-overlay",
            div { class: "settings-modal",
                div { class: "settings-header",
                    h2 { {t(&lang, "settings.title")} }
                    button { class: "settings-close", onclick: close, IconClose {} }
                }
                div { class: "settings-content",
                    // Language
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.language")} }
                        select {
                            value: "{s.read().language}",
                            onchange: move |e| s.write().language = e.value(),
                            option { value: "en", "English" }
                            option { value: "he", "עברית" }
                        }
                    }
                    // Theme
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.theme")} }
                        select {
                            value: "{s.read().theme}",
                            onchange: move |e| s.write().theme = e.value(),
                            option { value: "light", {t(&lang, "settings.light")} }
                            option { value: "dark",  {t(&lang, "settings.dark")} }
                        }
                    }
                    // Timer
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.timer")} }
                        select {
                            onchange: move |e| {
                                if let Ok(v) = e.value().parse::<u32>() {
                                    s.write().speech_timer_default = v;
                                }
                            },
                            for mins in [5u32, 6, 7, 8, 10] {
                                option {
                                    value: "{mins}",
                                    selected: s.read().speech_timer_default == mins,
                                    "{mins} {t(&lang, \"settings.minutes\")}"
                                }
                            }
                        }
                    }
                    // Font size
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.font_size")} }
                        select {
                            value: "{s.read().font_size}",
                            onchange: move |e| s.write().font_size = e.value(),
                            option { value: "small",       {t(&lang, "settings.small")} }
                            option { value: "medium",      {t(&lang, "settings.medium")} }
                            option { value: "large",       {t(&lang, "settings.large")} }
                            option { value: "extra-large", {t(&lang, "settings.xl")} }
                        }
                    }
                    // Checkboxes
                    div { class: "settings-section",
                        label { class: "settings-checkbox",
                            input { r#type: "checkbox", checked: s.read().enable_sound,
                                onchange: move |e| s.write().enable_sound = e.checked() }
                            span { {t(&lang, "settings.sound")} }
                        }
                        label { class: "settings-checkbox",
                            input { r#type: "checkbox", checked: s.read().include_rebuttal,
                                onchange: move |e| s.write().include_rebuttal = e.checked() }
                            span { {t(&lang, "settings.rebuttal")} }
                        }
                        label { class: "settings-checkbox",
                            input { r#type: "checkbox", checked: s.read().include_poi,
                                onchange: move |e| s.write().include_poi = e.checked() }
                            span { {t(&lang, "settings.poi")} }
                        }
                    }
                }
                div { class: "settings-footer",
                    button { class: "settings-reset",
                        onclick: move |_| { *s.write() = Settings::default(); settings::save(&s.read()); },
                        {t(&lang, "settings.reset")}
                    }
                    button { class: "settings-save", onclick: close, {t(&lang, "settings.close")} }
                }
            }
        }
    }
}
