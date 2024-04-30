use chrono::NaiveDateTime;

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
