use crate::database::{Database, Fetch};
use crate::shell::eval;
use rusqlite::Result;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

pub struct Controller {
    pub database: Database,
    pub debug: bool,
}
impl Controller {
    pub fn new(debug: bool) -> Result<Self> {
        let database = Database::new()?;
        Ok(Self { database, debug })
    }
    pub fn push(&self, cmd: &Vec<String>) -> Result<()> {
        self.database.push(cmd)
    }
    pub fn pop_then_execute(&mut self, num: usize, interval: u64) -> Result<()> {
        let jobs = self.database.fetch(Fetch::NotDone)?;
        for i in 0..num {
            if i >= jobs.len() {
                eprintln!("No more q!!");
                break;
            }
            eprintln!("Executing {}", jobs[i]);
            match eval(&jobs[i].cmd) {
                Ok(code) => {
                    self.database.finish(jobs[i].id, code)?;
                }
                Err(err) => {
                    eprintln!("Error {:?}", err);
                    self.database.finish(jobs[i].id, 255)?;
                }
            }
            if i + 1 < num {
                sleep(Duration::from_secs(interval));
            }
        }
        Ok(())
    }
    pub fn show(&self, tail: usize) -> Result<()> {
        let jobs = self.database.fetch(if tail == 0 {
            Fetch::AllOrderById
        } else {
            Fetch::TailOrderById(tail)
        })?;
        let mut handle = io::stdout().lock();
        for job in jobs {
            if let Err(_) = writeln!(handle, "{}", job) {
                break;
            }
        }
        Ok(())
    }
    pub fn revive(&self, id: usize) -> Result<()> {
        self.database.revive(id)?;
        Ok(())
    }
}
