use dioxus::prelude::*;
use std::time::Duration;
use crate::{i18n::{t, Lang}, settings::Settings};

const BELL: Asset = asset!("/assets/bell.mp3");
const NEGATIVE_LIMIT: i32 = -15;

#[component]
pub fn Timer() -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let settings = use_context::<Signal<Settings>>();

    let default_secs = (settings.read().speech_timer_default * 60) as i32;
    let mut seconds   = use_signal(|| default_secs);
    let mut is_active = use_signal(|| false);
    let mut silenced  = use_signal(|| false);

    // Tick every second via coroutine
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        loop {
            async_std::task::sleep(Duration::from_secs(1)).await;
            if *is_active.read() {
                let cur = *seconds.read();
                if cur > NEGATIVE_LIMIT {
                    let new_val = cur - 1;
                    *seconds.write() = new_val;

                    if !*silenced.read() && settings.read().enable_sound {
                        let orig = (settings.read().speech_timer_default * 60) as i32;
                        let elapsed = orig - new_val;
                        if elapsed == 60 || new_val == 60 || new_val == 0 {
                            play_bell(1);
                        }
                        if new_val == NEGATIVE_LIMIT {
                            play_bell(3);
                        }
                    }
                } else {
                    *is_active.write() = false;
                }
            }
        }
    });

    let secs = *seconds.read();
    let abs = secs.unsigned_abs();
    let display = format!("{}{:02}:{:02}", if secs < 0 { "-" } else { "" }, abs / 60, abs % 60);

    let orig = (settings.read().speech_timer_default * 60) as i32;
    let bg_style = if orig - secs >= 60 && secs > 60 {
        "background:green;color:white".to_string()
    } else if secs <= 0 {
        "background:red;color:white".to_string()
    } else {
        String::new()
    };

    rsx! {
        div { class: "timer",
            div { class: "timer-display", style: "{bg_style}", "{display}" }
            button {
                disabled: *is_active.read() || secs == NEGATIVE_LIMIT,
                onclick: move |_| *is_active.write() = true,
                {t(&lang, "timer.start")}
            }
            button {
                disabled: !*is_active.read(),
                onclick: move |_| *is_active.write() = false,
                {t(&lang, "timer.pause")}
            }
            button {
                onclick: move |_| {
                    *is_active.write() = false;
                    *seconds.write() = (settings.read().speech_timer_default * 60) as i32;
                },
                {t(&lang, "timer.reset")}
            }
            button {
                onclick: move |_| *silenced.write() ^= true,
                if *silenced.read() { {t(&lang, "timer.unmute")} } else { {t(&lang, "timer.silence")} }
            }
        }
    }
}

fn play_bell(times: u32) {
    let bell_path = BELL.to_string();
    spawn(async move {
        for i in 0..times {
            if i > 0 {
                async_std::task::sleep(Duration::from_millis(500)).await;
            }
            let _ = document::eval(&format!("new Audio('{bell_path}').play()"));
        }
    });
}
