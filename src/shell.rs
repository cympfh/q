use chrono::Local;
use std::io;
use std::process::Command;

pub fn eval(cmd: &Vec<String>) -> io::Result<i32> {
    let bin = &cmd[0];
    let args = &cmd[1..];
    let start = Local::now();

    let mut child = Command::new(bin)
        .args(args)
        .stdout(io::stdout())
        .stderr(io::stderr())
        .spawn()?;
    let code = child.wait()?.code().unwrap_or(255);

    let end = Local::now();
    let duration = end.signed_duration_since(start);
    eprintln!(
        "Exit code: {}, Execution time: {}.{:03}",
        code,
        duration.num_seconds(),
        duration.num_milliseconds(),
    );
    Ok(code)
}
