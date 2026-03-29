import json, sqlite3

with open("db.json") as f:
    debates = json.load(f)

con = sqlite3.connect("debate.db3")
cur = con.cursor()

roles = ["PM", "LO", "DPM", "DLO", "MG", "MO", "GW", "OW"]

for d in debates:
    cur.execute("INSERT OR IGNORE INTO debates (id, motion) VALUES (?, ?)", (d["id"], d["motion"]))
    for role in roles:
        s = d.get(role, {})
        cur.execute(
            "INSERT OR REPLACE INTO speeches (debate_id, role, speaker, speech, rebuttal, poi) VALUES (?, ?, ?, ?, ?, ?)",
            (d["id"], role, s.get("speaker", ""), s.get("speech", ""), s.get("rebuttal", ""), s.get("POI", ""))
        )

con.commit()
con.close()
print("Done.")
