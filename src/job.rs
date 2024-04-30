use chrono::NaiveDateTime;
use std::fmt;

#[derive(Debug)]
pub struct Job {
    pub id: u32,
    pub cmd: Vec<String>,
    pub done: bool,
    pub succeeded: u32,
    pub failed: u32,
    pub created: NaiveDateTime,
    pub last_executed: Option<NaiveDateTime>,
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_executed: String = self
            .last_executed
            .map(|s| s.format("%Y-%m-%dT%H:%M:%S").to_string())
            .unwrap_or(String::from("null"));
        write!(
            f,
            "id:{} cmd:{:?} done:{} succeeded:{} failed:{} created:{:?} last_executed:{}",
            self.id, self.cmd, self.done, self.succeeded, self.failed, self.created, last_executed,
        )
    }
}
