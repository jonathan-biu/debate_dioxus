use crate::{
    components::{navbar::Navbar, icons::{IconChevronUp, IconChevronDown}},
    db,
    i18n::{t, Lang},
    settings::Settings,
    types::{Debate, SPEAKER_ORDER},
    Route,
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum ViewMode {
    Cards,
    Table,
    Placement,
}

#[derive(Clone, PartialEq)]
struct GroupPlacement {
    id: &'static str,
    placement: usize,
    points: f32,
    notes: String,
}

fn default_placements() -> Vec<GroupPlacement> {
    vec![
        GroupPlacement {
            id: "OG",
            placement: 1,
            points: 3.0,
            notes: String::new(),
        },
        GroupPlacement {
            id: "OO",
            placement: 2,
            points: 2.0,
            notes: String::new(),
        },
        GroupPlacement {
            id: "CG",
            placement: 3,
            points: 1.0,
            notes: String::new(),
        },
        GroupPlacement {
            id: "CO",
            placement: 4,
            points: 0.0,
            notes: String::new(),
        },
    ]
}

fn placement_color(p: usize) -> &'static str {
    match p {
        1 => "#FFD700",
        2 => "#C0C0C0",
        3 => "#CD7F32",
        _ => "#E5E5E5",
    }
}

fn format_text(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' => {
                let inner: String = chars.by_ref().take_while(|&x| x != '*').collect();
                out.push_str(&format!(
                    "<strong style=\"color: var(--highlight-red, red);\">{inner}</strong>"
                ));
            }
            '$' => {
                let inner: String = chars.by_ref().take_while(|&x| x != '$').collect();
                out.push_str(&format!(
                    "<strong style=\"color: var(--highlight-blue, blue);\">{inner}</strong>"
                ));
            }
            '\n' => out.push_str("<br>"),
            '\t' => out.push_str("&nbsp;&nbsp;&nbsp;&nbsp;"),
            _ => out.push(c),
        }
    }
    out
}

#[component]
pub fn Home() -> Element {
    home_inner(None)
}

#[component]
pub fn HomeWithId(id: String) -> Element {
    home_inner(Some(id))
}

fn home_inner(param_id: Option<String>) -> Element {
    let lang_ctx = use_context::<Lang>();
    let lang = lang_ctx.0.read().clone();
    let settings = use_context::<Signal<Settings>>();
    let nav = navigator();

    let mut debates = use_signal(|| db::get_debates());
    let mut selected_id = use_signal(|| param_id);
    let mut view_mode = use_signal(|| ViewMode::Cards);
    let mut placements = use_signal(|| default_placements());

    let sync_version = use_context::<Signal<u32>>();
    use_effect(move || {
        let _ = sync_version.read();
        *debates.write() = db::get_debates();
        *selected_id.write() = None;
    });

    let debate = use_memo(move || {
        selected_id
            .read()
            .as_ref()
            .and_then(|id| db::get_debate(id))
    });

    rsx! {
        Navbar { in_speech: false, debate_id: None }
        div { class: "page",
            h1 {
                {
                    debate
                        .read()
                        .as_ref()
                        .map(|d| d.motion.clone())
                        .unwrap_or_else(|| t(&lang, "home.select_motion").to_string())
                }
            }

            select {
                class: "motion-select",
                value: selected_id.read().clone().unwrap_or_default(),
                onchange: move |e| {
                    let v = e.value();
                    *selected_id.write() = if v.is_empty() { None } else { Some(v) };
                    *placements.write() = default_placements();
                },
                option { value: "", disabled: true, {t(&lang, "home.select_motion")} }
                for (id , motion) in debates.read().iter() {
                    option { value: "{id}", "{motion}" }
                }
            }

            if selected_id.read().is_some() {
                button {
                    onclick: move |_| {
                        if let Some(id) = selected_id.read().clone() {
                            db::delete_debate(&id);
                            *debates.write() = db::get_debates();
                        }
                        *selected_id.write() = None;
                    },
                    {t(&lang, "home.delete_motion")}
                }
                span { style: "display:inline-block;width:8px;" }
                button {
                    onclick: move |_| {
                        if let Some(id) = selected_id.read().clone() {
                            nav.push(Route::SpeechRoute { speaker: "PM".to_string(), id });
                        }
                    },
                    {t(&lang, "home.edit_motion")}
                }
            }

            if let Some(d) = debate.read().clone() {
                div { class: "view-toggle",
                    button {
                        class: if *view_mode.read() == ViewMode::Cards { "view-toggle-button active" } else { "view-toggle-button" },
                        onclick: move |_| *view_mode.write() = ViewMode::Cards,
                        {t(&lang, "home.card_view")}
                    }
                    button {
                        class: if *view_mode.read() == ViewMode::Table { "view-toggle-button active" } else { "view-toggle-button" },
                        onclick: move |_| *view_mode.write() = ViewMode::Table,
                        {t(&lang, "home.table_view")}
                    }
                    button {
                        class: if *view_mode.read() == ViewMode::Placement { "view-toggle-button active" } else { "view-toggle-button" },
                        onclick: move |_| *view_mode.write() = ViewMode::Placement,
                        {t(&lang, "home.placement_view")}
                    }
                }

                div { class: "content-layout",
                    match *view_mode.read() {
                        ViewMode::Cards => rsx! {
                            CardsView {
                                debate: d.clone(),
                                lang: lang.clone(),
                                settings: settings.read().clone(),
                            }
                        },
                        ViewMode::Table => rsx! {
                            TableView {
                                debate: d.clone(),
                                lang: lang.clone(),
                                settings: settings.read().clone(),
                            }
                        },
                        ViewMode::Placement => rsx! {
                            PlacementView { debate: d.clone(), lang: lang.clone(), placements }
                        },
                    }
                }
            }
        }
    }
}

// ── Cards View ────────────────────────────────────────────────────────────────

#[component]
fn CardsView(debate: Debate, lang: String, settings: Settings) -> Element {
    let groups: &[(&str, &[&str])] = &[
        ("home.og", &["PM", "DPM"]),
        ("home.oo", &["LO", "DLO"]),
        ("home.cg", &["MG", "GW"]),
        ("home.co", &["MO", "OW"]),
    ];

    rsx! {
        div { class: "container",
            for (group_key , roles) in groups {
                div { class: "group",
                    h2 { {t(&lang, group_key)} }
                    for role in *roles {
                        {
                            let sp = debate.get_speech(role);
                            let role_label = t(&lang, &format!("home.{}", role.to_lowercase()));
                            let title = format!("{role_label} - {}", sp.speaker);
                            let speech_html = format_text(&sp.speech);
                            let rebuttal_html = format_text(&sp.rebuttal);
                            let poi_html = format_text(&sp.poi);
                            let show_rebuttal = *role != "PM" && settings.include_rebuttal
                                && !sp.rebuttal.is_empty();
                            let show_poi = settings.include_poi;
                            rsx! {
                                div { class: "speaker-card",
                                    h3 { "{title}" }
                                    h4 { {t(&lang, "home.arguments")} }
                                    p { dangerous_inner_html: "{speech_html}" }
                                    if show_rebuttal {
                                        h4 { {t(&lang, "home.rebuttal")} }
                                        p { dangerous_inner_html: "{rebuttal_html}" }
                                    }
                                    if show_poi {
                                        h4 { {t(&lang, "home.poi")} }
                                        p { dangerous_inner_html: "{poi_html}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Table View ────────────────────────────────────────────────────────────────

#[component]
fn TableView(debate: Debate, lang: String, settings: Settings) -> Element {
    rsx! {
        div { class: "comparison-table-container",
            h2 { {t(&lang, "home.comparison")} }
            div { class: "table-wrapper",
                table { class: "comparison-table",
                    thead {
                        tr {
                            th { {t(&lang, "home.speaker")} }
                            th { {t(&lang, "home.team")} }
                            th { {t(&lang, "home.arguments")} }
                            if settings.include_rebuttal {
                                th { {t(&lang, "home.rebuttal")} }
                            }
                            if settings.include_poi {
                                th { {t(&lang, "home.poi")} }
                            }
                        }
                    }
                    tbody {
                        for role in SPEAKER_ORDER {
                            {
                                let sp = debate.get_speech(role);
                                let is_gov = ["PM", "DPM", "MG", "GW"].contains(role);
                                let team = if is_gov {
                                    t(&lang, "home.government")
                                } else {
                                    t(&lang, "home.opposition")
                                };
                                let row_class = if is_gov { "government" } else { "opposition" };
                                let role_label = t(&lang, &format!("home.{}", role.to_lowercase()));
                                let speech_html = format_text(&sp.speech);
                                let rebuttal_html = format_text(&sp.rebuttal);
                                let poi_html = format_text(&sp.poi);
                                let speaker_name = sp.speaker.clone();
                                let inc_rebuttal = settings.include_rebuttal;
                                let inc_poi = settings.include_poi;
                                let is_pm = *role == "PM";
                                rsx! {
                                    tr { class: "{row_class}",
                                        td {
                                            div { class: "speaker-title", "{role_label}" }
                                            div { class: "speaker-name", "{speaker_name}" }
                                        }
                                        td { "{team}" }
                                        td { class: "content-cell",
                                            div { class: "content-text", dangerous_inner_html: "{speech_html}" }
                                        }
                                        if inc_rebuttal {
                                            td { class: "content-cell",
                                                if is_pm {
                                                    "—"
                                                } else {
                                                    div {
                                                        class: "content-text",
                                                        dangerous_inner_html: "{rebuttal_html}",
                                                    }
                                                }
                                            }
                                        }
                                        if inc_poi {
                                            td { class: "content-cell",
                                                div { class: "content-text", dangerous_inner_html: "{poi_html}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Placement View ────────────────────────────────────────────────────────────

#[component]
fn PlacementView(
    debate: Debate,
    lang: String,
    mut placements: Signal<Vec<GroupPlacement>>,
) -> Element {
    let sorted = use_memo(move || {
        let mut p = placements.read().clone();
        p.sort_by_key(|g| g.placement);
        p
    });

    rsx! {
        div { class: "placement-table-container",
            h2 { {t(&lang, "home.placement_table")} }
            table { class: "placement-table",
                thead {
                    tr {
                        th { {t(&lang, "home.placement")} }
                        th { {t(&lang, "home.group")} }
                        th { {t(&lang, "home.speakers")} }
                        th { {t(&lang, "home.points")} }
                        th { {t(&lang, "home.notes")} }
                        th { {t(&lang, "home.actions")} }
                    }
                }
                tbody {
                    for g in sorted.read().iter() {
                        {
                            let gid = g.id;
                            let color = placement_color(g.placement);
                            let group_name = t(&lang, &format!("home.{}", gid.to_lowercase())).to_string();
                            let speakers: Vec<(String, String)> = match gid {
                                "OG" => {
                                    vec![
                                        ("PM".into(), debate.pm.speaker.clone()),
                                        ("DPM".into(), debate.dpm.speaker.clone()),
                                    ]
                                }
                                "OO" => {
                                    vec![
                                        ("LO".into(), debate.lo.speaker.clone()),
                                        ("DLO".into(), debate.dlo.speaker.clone()),
                                    ]
                                }
                                "CG" => {
                                    vec![
                                        ("MG".into(), debate.mg.speaker.clone()),
                                        ("GW".into(), debate.gw.speaker.clone()),
                                    ]
                                }
                                _ => {
                                    vec![
                                        ("MO".into(), debate.mo.speaker.clone()),
                                        ("OW".into(), debate.ow.speaker.clone()),
                                    ]
                                }
                            };
                            let pts = g.points;
                            let notes = g.notes.clone();
                            let placement = g.placement;
                            rsx! {
                                tr { style: "background-color: {color}20",
                                    td { "{placement}" }
                                    td { "{group_name}" }
                                    td {
                                        for (role , name) in &speakers {
                                            div {
                                                span { "{role}: " }
                                                span { "{name}" }
                                            }
                                        }
                                    }
                                    td {
                                        input {
                                            r#type: "number",
                                            value: "{pts}",
                                            min: "0",
                                            max: "100",
                                            step: "0.5",
                                            oninput: move |e| {
                                                if let Ok(v) = e.value().parse::<f32>() {
                                                    if let Some(g) = placements.write().iter_mut().find(|g| g.id == gid) {
                                                        g.points = v;
                                                    }
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        textarea {
                                            value: "{notes}",
                                            rows: "2",
                                            oninput: move |e| {
                                                if let Some(g) = placements.write().iter_mut().find(|g| g.id == gid) {
                                                    g.notes = e.value();
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        button {
                                            disabled: placement == 1,
                                            onclick: move |_| {
                                                let cur = placements
                                                    .read()
                                                    .iter()
                                                    .find(|g| g.id == gid)
                                                    .map(|g| g.placement)
                                                    .unwrap_or(1);
                                                let new_p = cur.saturating_sub(1).max(1);
                                                if new_p != cur {
                                                    let mut p = placements.write();
                                                    if let Some(other) = p
                                                        .iter_mut()
                                                        .find(|g| g.id != gid && g.placement == new_p)
                                                    {
                                                        other.placement = cur;
                                                    }
                                                    if let Some(g) = p.iter_mut().find(|g| g.id == gid) {
                                                        g.placement = new_p;
                                                    }
                                                }
                                            },
                                            IconChevronUp {}
                                        }
                                        button {
                                            disabled: placement == 4,
                                            onclick: move |_| {
                                                let cur = placements
                                                    .read()
                                                    .iter()
                                                    .find(|g| g.id == gid)
                                                    .map(|g| g.placement)
                                                    .unwrap_or(4);
                                                let new_p = (cur + 1).min(4);
                                                if new_p != cur {
                                                    let mut p = placements.write();
                                                    if let Some(other) = p
                                                        .iter_mut()
                                                        .find(|g| g.id != gid && g.placement == new_p)
                                                    {
                                                        other.placement = cur;
                                                    }
                                                    if let Some(g) = p.iter_mut().find(|g| g.id == gid) {
                                                        g.placement = new_p;
                                                    }
                                                }
                                            },
                                            IconChevronDown {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
