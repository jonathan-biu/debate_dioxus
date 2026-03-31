use rusqlite::{Connection, params};
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use crate::types::{Debate, Speech};

static DB: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn init() {
    let db_path = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("debate_dioxus")
        .join("debate.db3");
    std::fs::create_dir_all(db_path.parent().unwrap()).expect("create data dir");
    let conn = Connection::open(&db_path).expect("open db");
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS debates (
            id      TEXT PRIMARY KEY,
            motion  TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS speeches (
            debate_id TEXT NOT NULL,
            role      TEXT NOT NULL,
            speaker   TEXT NOT NULL DEFAULT '',
            speech    TEXT NOT NULL DEFAULT '',
            rebuttal  TEXT NOT NULL DEFAULT '',
            poi       TEXT NOT NULL DEFAULT '',
            PRIMARY KEY (debate_id, role),
            FOREIGN KEY (debate_id) REFERENCES debates(id) ON DELETE CASCADE
        );
    ").expect("init schema");
    DB.set(Mutex::new(conn)).ok();
}

fn with_db<F, T>(f: F) -> T
where F: FnOnce(&Connection) -> T {
    let guard = DB.get().unwrap().lock().unwrap();
    f(&guard)
}

pub fn get_debates() -> Vec<(String, String)> {
    with_db(|conn| {
        let mut stmt = conn.prepare("SELECT id, motion FROM debates ORDER BY rowid").unwrap();
        stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?))).unwrap()
            .filter_map(|r| r.ok()).collect()
    })
}

pub fn create_debate(id: &str, motion: &str) {
    with_db(|conn| {
        conn.execute("INSERT INTO debates (id, motion) VALUES (?1, ?2)", params![id, motion]).unwrap();
        for role in ["PM","LO","DPM","DLO","MG","MO","GW","OW"] {
            conn.execute(
                "INSERT INTO speeches (debate_id, role) VALUES (?1, ?2)",
                params![id, role]
            ).unwrap();
        }
    });
}

pub fn upsert_debate(id: &str, motion: &str) {
    with_db(|conn| {
        conn.execute("INSERT OR REPLACE INTO debates (id, motion) VALUES (?1, ?2)", params![id, motion]).unwrap();
        for role in ["PM","LO","DPM","DLO","MG","MO","GW","OW"] {
            conn.execute(
                "INSERT OR IGNORE INTO speeches (debate_id, role) VALUES (?1, ?2)",
                params![id, role]
            ).unwrap();
        }
    });
}

pub fn delete_debate(id: &str) {
    with_db(|conn| {
        conn.execute("DELETE FROM debates WHERE id=?1", params![id]).unwrap();
    });
}

pub fn get_debate(id: &str) -> Option<Debate> {
    with_db(|conn| {
        let motion: String = conn.query_row(
            "SELECT motion FROM debates WHERE id=?1", params![id], |r| r.get(0)
        ).ok()?;

        let mut stmt = conn.prepare(
            "SELECT role, speaker, speech, rebuttal, poi FROM speeches WHERE debate_id=?1"
        ).unwrap();

        let mut debate = Debate {
            id: id.to_string(),
            motion,
            ..Default::default()
        };

        stmt.query_map(params![id], |r| {
            Ok((
                r.get::<_,String>(0)?,
                r.get::<_,String>(1)?,
                r.get::<_,String>(2)?,
                r.get::<_,String>(3)?,
                r.get::<_,String>(4)?,
            ))
        }).unwrap().filter_map(|r| r.ok()).for_each(|(role, speaker, speech, rebuttal, poi)| {
            let s = Speech { speaker, speech, rebuttal, poi };
            match role.as_str() {
                "PM"  => debate.pm  = s,
                "LO"  => debate.lo  = s,
                "DPM" => debate.dpm = s,
                "DLO" => debate.dlo = s,
                "MG"  => debate.mg  = s,
                "MO"  => debate.mo  = s,
                "GW"  => debate.gw  = s,
                "OW"  => debate.ow  = s,
                _ => {}
            }
        });

        Some(debate)
    })
}

pub fn save_speech(debate_id: &str, role: &str, speaker: &str, speech: &str, rebuttal: &str, poi: &str) {
    with_db(|conn| {
        conn.execute(
            "UPDATE speeches SET speaker=?1, speech=?2, rebuttal=?3, poi=?4
             WHERE debate_id=?5 AND role=?6",
            params![speaker, speech, rebuttal, poi, debate_id, role]
        ).unwrap();
    });
}

pub fn save_speakers(debate_id: &str, speakers: &[(&str, &str)]) {
    with_db(|conn| {
        for (role, name) in speakers {
            conn.execute(
                "UPDATE speeches SET speaker=?1 WHERE debate_id=?2 AND role=?3",
                params![name, debate_id, role]
            ).unwrap();
        }
    });
}
