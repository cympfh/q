use crate::job::Job;
use chrono::Local;
use rusqlite::{Connection, Result};

pub enum Fetch {
    NotDone,
    AllOrderById,
}

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(".q")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS jobs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                cmd JSON NOT NULL,
                done BOOL NOT NULL,
                succeeded INTEGER NOT NULL,
                failed INTEGER NOT NULL,
                created DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_executed DATETIME
            )",
            (),
        )?;
        Ok(Self { conn })
    }
    pub fn push(&self, cmd: &Vec<String>) -> Result<()> {
        let cmddata = serde_json::to_string(cmd).expect("json serialize");
        self.conn.execute(
            "INSERT INTO jobs (cmd, done, succeeded, failed)
            VALUES (?1, ?2, ?3, ?4)",
            (cmddata, false, 0, 0),
        )?;
        Ok(())
    }
    pub fn fetch(&self, f: Fetch) -> Result<Vec<Job>> {
        let mut stmt = match f {
            Fetch::AllOrderById => self.conn.prepare(
                "SELECT
                    id,
                    cmd,
                    done,
                    succeeded,
                    failed,
                    created,
                    last_executed
                FROM jobs
                ORDER BY id DESC
            ",
            ),
            Fetch::NotDone => self.conn.prepare(
                "SELECT
                    id,
                    cmd,
                    done,
                    succeeded,
                    failed,
                    created,
                    last_executed
                FROM jobs
                WHERE done IS FALSE
            ",
            ),
        }?;
        let jobs = stmt.query_map([], |row| {
            Ok(Job {
                id: row.get(0)?,
                cmd: serde_json::from_str(row.get::<_, String>(1)?.as_str())
                    .expect("JSON deserialize"),
                done: row.get(2)?,
                succeeded: row.get(3)?,
                failed: row.get(4)?,
                created: row.get(5)?,
                last_executed: row.get(6)?,
            })
        })?;
        jobs.collect()
    }
    /// done=false で上書き
    pub fn revive(&self, id: usize) -> Result<()> {
        let n = self.conn.execute(
            "
            UPDATE jobs
            SET done = FALSE
            WHERE id = ?1
            ",
            (id,),
        )?;
        if n == 0 {
            eprintln!("Not found job (id={})", id);
        }
        Ok(())
    }
    /// コマンド実行終了
    pub fn finish(&self, id: u32, code: i32) -> Result<()> {
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        if code == 0 {
            self.conn.execute(
                "
            UPDATE jobs
            SET done = TRUE,
                succeeded = succeeded + 1,
                last_executed = ?2
            WHERE id = ?1
            ",
                (id, now),
            )?;
        } else {
            self.conn.execute(
                "
            UPDATE jobs
            SET done = TRUE,
                failed = failed + 1,
                last_executed = ?2
            WHERE id = ?1
            ",
                (id, now),
            )?;
        }
        Ok(())
    }
}
