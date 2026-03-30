use dioxus::prelude::*;
use dioxus::document::eval;
use crate::{
    Route, db, i18n::{t, Lang}, settings::Settings,
    types::SPEAKER_ORDER,
    components::{navbar::Navbar, timer::Timer},
};



// JS injected once to handle all textarea keyboard shortcuts:
//   Ctrl+B  → wrap selection in '*'
//   Ctrl+D  → wrap selection in '$'
//   Tab     → indent list item (numbers→letters) / Shift+Tab dedent (letters→numbers)
//             also blocks default tab-focus-navigation
//   Enter   → continue list / sub-list, or exit on blank line
//   Ctrl+↑  → focus previous textarea
//   Ctrl+↓  → focus next textarea
const KEYBOARD_JS: &str = r#"
(function() {
  if (window.__speechKeysInstalled) return;
  window.__speechKeysInstalled = true;

  const state = new WeakMap();
  function getState(el) {
    if (!state.has(el)) state.set(el, { counter: 0, subCounter: 0 });
    return state.get(el);
  }

  function wrap(el, marker) {
    const s = el.selectionStart, e = el.selectionEnd;
    el.value = el.value.slice(0, s) + marker + el.value.slice(s, e) + marker + el.value.slice(e);
    el.selectionStart = s + 1; el.selectionEnd = e + 1;
    el.dispatchEvent(new Event('input', { bubbles: true }));
  }

  function insertAt(el, text, pos) {
    el.value = el.value.slice(0, pos) + text + el.value.slice(pos);
    el.selectionStart = el.selectionEnd = pos + text.length;
    el.dispatchEvent(new Event('input', { bubbles: true }));
  }

  function replaceRange(el, text, s, e) {
    el.value = el.value.slice(0, s) + text + el.value.slice(e);
    el.selectionStart = el.selectionEnd = s + text.length;
    el.dispatchEvent(new Event('input', { bubbles: true }));
  }

  function toLetter(n) { return String.fromCharCode(96 + n); }

  // Detect list depth from indentation (each level = 4 spaces)
  function getDepth(line) {
    const m = line.match(/^( *)/);
    return m ? Math.floor(m[1].length / 4) : 0;
  }

  // Build prefix for a given depth and counter
  function makePrefix(depth, counter) {
    const indent = '    '.repeat(depth);
    return depth % 2 === 0
      ? indent + counter + '. '           // even depth → numbers
      : indent + toLetter(counter) + '. '; // odd depth  → letters
  }

  // Parse current line: returns { depth, counter, text } or null
  function parseLine(line) {
    const m = line.match(/^( *)(\d+|[a-z]+)\.\ ?(.*)$/);
    if (!m) return null;
    const depth = Math.floor(m[1].length / 4);
    const raw = m[2];
    const counter = /^\d+$/.test(raw) ? parseInt(raw) : raw.charCodeAt(0) - 96;
    const spaceLen = m[0][m[1].length + m[2].length + 1] === ' ' ? 1 : 0;
    return { depth, counter, text: m[3], prefixLen: m[1].length + m[2].length + 1 + spaceLen };
  }

  document.addEventListener('keydown', function(ev) {
    const el = document.activeElement;
    if (!el || el.tagName !== 'TEXTAREA') return;
    const st = getState(el);
    const ctrl = ev.ctrlKey || ev.metaKey;

    // ── Ctrl+B / Ctrl+D ─────────────────────────────────────────────────────
    if (ctrl && ev.key === 'b') { ev.preventDefault(); wrap(el, '*'); return; }
    if (ctrl && ev.key === 'd') { ev.preventDefault(); wrap(el, '$'); return; }

    // ── Tab : always prevent focus-change; indent/dedent if on a list line ──
    if (ev.key === 'Tab') {
      ev.preventDefault();
      const pos = el.selectionStart;
      const lineStart = el.value.lastIndexOf('\n', pos - 1) + 1;
      const lineEnd = el.value.indexOf('\n', pos);
      const line = el.value.slice(lineStart, lineEnd === -1 ? el.value.length : lineEnd);
      const parsed = parseLine(line);
      if (!parsed) { insertAt(el, '\t', pos); return; }

      if (!ev.shiftKey) {
        // indent: increase depth by 1, reset counter to 1
        const newDepth = parsed.depth + 1;
        replaceRange(el, makePrefix(newDepth, 1), lineStart, lineStart + parsed.prefixLen);
      } else if (parsed.depth > 0) {
        // dedent: decrease depth by 1, restore parent counter
        const newDepth = parsed.depth - 1;
        replaceRange(el, makePrefix(newDepth, st.counter), lineStart, lineStart + parsed.prefixLen);
      }
      return;
    }

    // ── Enter : continue / exit list ────────────────────────────────────────
    if (ev.key === 'Enter') {
      if (ev.shiftKey) return;
      const pos = el.selectionStart;
      const lineStart = el.value.lastIndexOf('\n', pos - 1) + 1;
      const line = el.value.slice(lineStart, pos);
      const parsed = parseLine(line);
      if (!parsed) return;

      ev.preventDefault();
      if (parsed.text.trim() === '') {
        // blank list line: dedent or exit
        if (parsed.depth > 0) {
          replaceRange(el, '\n' + makePrefix(parsed.depth - 1, st.counter), lineStart, pos);
        } else {
          st.counter = 0;
          replaceRange(el, '\n', lineStart, pos);
        }
      } else {
        const next = parsed.counter + 1;
        st.counter = parsed.depth === 0 ? next : st.counter;
        insertAt(el, '\n' + makePrefix(parsed.depth, next), pos);
      }
      return;
    }

    // ── Ctrl+↑ / Ctrl+↓ : navigate between textareas ───────────────────────
    if (ctrl && (ev.key === 'ArrowUp' || ev.key === 'ArrowDown')) {
      ev.preventDefault();
      const all = Array.from(document.querySelectorAll('textarea'));
      const idx = all.indexOf(el);
      if (ev.key === 'ArrowUp'   && idx > 0)              all[idx - 1].focus();
      if (ev.key === 'ArrowDown' && idx < all.length - 1) all[idx + 1].focus();
    }
  });
})();
"#;

#[component]
pub fn Speech(speaker: String, id: String) -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let settings = use_context::<Signal<Settings>>();
    let nav = navigator();

    let mut speech_val   = use_signal(String::new);
    let mut rebuttal_val = use_signal(String::new);
    let mut poi_val      = use_signal(String::new);

    // Store props in signals so use_memo can track changes across re-renders
    let mut speaker_sig = use_signal(|| speaker.clone());
    let mut id_sig      = use_signal(|| id.clone());
    *speaker_sig.write() = speaker.clone();
    *id_sig.write()      = id.clone();

    let loaded = use_memo(move || {
        let sp = speaker_sig.read().clone();
        let id = id_sig.read().clone();
        db::get_debate(&id).map(|d| {
            let s = d.get_speech(&sp);
            (s.speech.clone(), s.rebuttal.clone(), s.poi.clone())
        })
    });
    use_effect(move || {
        if let Some((sp, rb, po)) = loaded() {
            *speech_val.write()   = sp;
            *rebuttal_val.write() = rb;
            *poi_val.write()      = po;
        }
    });

    // Install keyboard handler once
    use_effect(move || { eval(KEYBOARD_JS); });

    let initial     = db::get_debate(&id);
    let init_name   = initial.as_ref().map(|d| d.get_speech(&speaker).speaker.clone()).unwrap_or_default();
    let init_motion = initial.as_ref().map(|d| d.motion.clone()).unwrap_or_default();

    let sp2   = speaker.clone();
    let did2  = id.clone();
    let name2 = init_name.clone();

    let handle_submit = move |e: FormEvent| {
        e.prevent_default();
        db::save_speech(&did2, &sp2, &name2, &speech_val.read(), &rebuttal_val.read(), &poi_val.read());

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
                    textarea { name: "speech", value: "{speech_val}",
                        oninput: move |e| *speech_val.write() = e.value()
                    }
                }
                if !is_pm && inc_rebuttal {
                    div {
                        label { {t(&lang, "speech.rebuttal")} }
                        textarea { name: "rebuttal", value: "{rebuttal_val}",
                            oninput: move |e| *rebuttal_val.write() = e.value()
                        }
                    }
                }
                if inc_poi {
                    div {
                        label { {t(&lang, "speech.poi")} }
                        textarea { name: "poi", value: "{poi_val}",
                            oninput: move |e| *poi_val.write() = e.value()
                        }
                    }
                }
                button { r#type: "submit", {t(&lang, "speech.submit")} }
            }
        }
    }
}
