use crate::database::{Database, Fetch};
use crate::shell::eval;
use rusqlite::Result;
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
            if self.debug {
                eprintln!("DEBUG: executing {:?}", jobs[i]);
            }
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
    pub fn show(&mut self) -> Result<()> {
        let jobs = self.database.fetch(Fetch::AllOrderById)?;
        for job in jobs {
            println!("{:?}", job);
        }
        Ok(())
    }
    pub fn revive(&mut self, id: usize) -> Result<()> {
        self.database.revive(id)?;
        Ok(())
    }
}
