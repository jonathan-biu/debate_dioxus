use dioxus::prelude::*;
use crate::{i18n::{t, Lang}, settings::{self, Settings}, components::icons::IconClose, sync};

#[component]
pub fn SettingsModal() -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let mut show = use_context::<Signal<bool>>();
    let mut s = use_context::<Signal<Settings>>();
    let mut sync_version = use_context::<Signal<u32>>();
    let mut sync_status: Signal<Option<Result<(), String>>> = use_signal(|| None);

    let save = move || settings::save(&s.read());

    let close = move |_| { *show.write() = false; };

    rsx! {
        div { class: "settings-overlay",
            div { class: "settings-modal",
                div { class: "settings-header",
                    h2 { {t(&lang, "settings.title")} }
                    button { class: "settings-close", onclick: close, IconClose {} }
                }
                div { class: "settings-content",
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.language")} }
                        select {
                            value: "{s.read().language}",
                            onchange: move |e| { s.write().language = e.value(); save(); },
                            option { value: "en", "English" }
                            option { value: "he", "עברית" }
                        }
                    }
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.theme")} }
                        select {
                            value: "{s.read().theme}",
                            onchange: move |e| { s.write().theme = e.value(); save(); },
                            option { value: "light",  {t(&lang, "settings.light")} }
                            option { value: "dark",   {t(&lang, "settings.dark")} }
                            option { value: "sepia",  {t(&lang, "settings.sepia")} }
                            option { value: "ocean",  {t(&lang, "settings.ocean")} }
                            option { value: "forest", {t(&lang, "settings.forest")} }
                        }
                    }
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.timer")} }
                        input {
                            r#type: "number", min: "1", max: "60",
                            value: "{s.read().speech_timer_default}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u32>() { s.write().speech_timer_default = v; save(); }
                            },
                        }
                        span { " {t(&lang, \"settings.minutes\")}" }
                    }
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.font_size")} }
                        select {
                            value: "{s.read().font_size}",
                            onchange: move |e| { s.write().font_size = e.value(); save(); },
                            option { value: "small",       {t(&lang, "settings.small")} }
                            option { value: "medium",      {t(&lang, "settings.medium")} }
                            option { value: "large",       {t(&lang, "settings.large")} }
                            option { value: "extra-large", {t(&lang, "settings.xl")} }
                        }
                    }
                    div { class: "settings-section",
                        label { class: "settings-checkbox",
                            input { r#type: "checkbox", checked: s.read().enable_sound,
                                onchange: move |e| { s.write().enable_sound = e.checked(); save(); } }
                            span { {t(&lang, "settings.sound")} }
                        }
                        label { class: "settings-checkbox",
                            input { r#type: "checkbox", checked: s.read().include_rebuttal,
                                onchange: move |e| { s.write().include_rebuttal = e.checked(); save(); } }
                            span { {t(&lang, "settings.rebuttal")} }
                        }
                        label { class: "settings-checkbox",
                            input { r#type: "checkbox", checked: s.read().include_poi,
                                onchange: move |e| { s.write().include_poi = e.checked(); save(); } }
                            span { {t(&lang, "settings.poi")} }
                        }
                        label { class: "settings-checkbox",
                            input { r#type: "checkbox", checked: s.read().always_on_top,
                                onchange: move |e| { s.write().always_on_top = e.checked(); save(); } }
                            span { {t(&lang, "settings.always_top")} }
                        }
                    }
                    // Cloud Sync
                    div { class: "settings-section",
                        h3 { {t(&lang, "settings.sync_title")} }
                        input {
                            r#type: "text",
                            placeholder: t(&lang, "settings.sync_url_placeholder"),
                            value: "{s.read().turso_url}",
                            oninput: move |e| { s.write().turso_url = e.value(); save(); },
                        }
                        input {
                            r#type: "password",
                            placeholder: t(&lang, "settings.sync_token_placeholder"),
                            value: "{s.read().turso_token}",
                            oninput: move |e| { s.write().turso_token = e.value(); save(); },
                        }
                        div { class: "sync-buttons", style: "display:flex;gap:8px;",
                            button {
                                disabled: s.read().turso_url.is_empty() || s.read().turso_token.is_empty(),
                                onclick: move |_| {
                                    let url   = s.read().turso_url.clone();
                                    let token = s.read().turso_token.clone();
                                    *sync_status.write() = None;
                                    spawn(async move {
                                        let res = sync::push(&url, &token).await;
                                        if res.is_ok() { *sync_version.write() += 1; }
                                        *sync_status.write() = Some(res);
                                    });
                                },
                                {t(&lang, "settings.sync_push")}
                            }
                            button {
                                disabled: s.read().turso_url.is_empty() || s.read().turso_token.is_empty(),
                                onclick: move |_| {
                                    let url   = s.read().turso_url.clone();
                                    let token = s.read().turso_token.clone();
                                    *sync_status.write() = None;
                                    spawn(async move {
                                        let res = sync::pull(&url, &token).await;
                                        if res.is_ok() { *sync_version.write() += 1; }
                                        *sync_status.write() = Some(res);
                                    });
                                },
                                {t(&lang, "settings.sync_pull")}
                            }
                        }
                        if let Some(status) = sync_status.read().as_ref() {
                            match status {
                                Ok(_)    => rsx! { p { style: "color: green", {t(&lang, "settings.sync_ok")} } },
                                Err(msg) => rsx! { p { style: "color: red",   "{msg}" } },
                            }
                        }
                    }
                }
                div { class: "settings-footer",
                    button { class: "settings-reset",
                        onclick: move |_| { *s.write() = Settings::default(); save(); },
                        {t(&lang, "settings.reset")}
                    }
                    button { class: "settings-save", onclick: close, {t(&lang, "settings.close")} }
                }
            }
        }
    }
}
