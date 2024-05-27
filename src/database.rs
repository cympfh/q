use crate::job::Job;
use chrono::Local;
use rusqlite::{Connection, Result, Row};

pub enum Fetch {
    NotDone,
    AllOrderById(String),
    TailOrderById(String, usize),
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
                created DATETIME NOT NULL,
                last_executed DATETIME
            )",
            (),
        )?;
        Ok(Self { conn })
    }
    pub fn push(&self, cmd: &Vec<String>) -> Result<()> {
        let cmddata = serde_json::to_string(cmd).expect("json serialize");
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        self.conn.execute(
            "INSERT INTO jobs (cmd, done, succeeded, failed, created)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (cmddata, false, 0, 0, now),
        )?;
        Ok(())
    }
    pub fn fetch(&self, f: Fetch) -> Result<Vec<Job>> {
        let mapper = |row: &Row| {
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
        };
        match f {
            Fetch::NotDone => self
                .conn
                .prepare(
                    "SELECT id, cmd, done, succeeded, failed, created, last_executed FROM jobs WHERE done IS FALSE",
                )?
                .query_map((), mapper)?.collect(),
            Fetch::AllOrderById(filter) => {
                let query = format!(
                    "SELECT id, cmd, done, succeeded, failed, created, last_executed
                        FROM jobs
                        WHERE 1 == 1 AND ({})
                        ORDER BY id",
                    filter);
                self
                .conn
                .prepare(query.as_str())?
                .query_map((), mapper)?.collect()
            },
            Fetch::TailOrderById(filter, n) =>{
                let query = format!(
                    "SELECT id, cmd, done, succeeded, failed, created, last_executed FROM (
                        SELECT * FROM jobs
                        WHERE 1 == 1 AND ({})
                        ORDER BY id DESC
                        LIMIT ?1
                    ) ORDER BY id",
                    filter
                );
                self.conn
                .prepare(query.as_str())?
                .query_map((n,), mapper)?.collect()
            },
         }
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
    /// レコードを削除
    pub fn rm(&self, id: usize) -> Result<()> {
        let n = self.conn.execute(
            "
            DELETE FROM jobs
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
