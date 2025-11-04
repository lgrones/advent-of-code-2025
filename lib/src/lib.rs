use std::{error::Error, fs, panic::Location, path::PathBuf, time::Instant};

#[track_caller]
pub fn read_file<T, F>(initial: char, select: F) -> Result<T, Box<dyn Error>>
where
    F: Fn(String) -> T,
{
    let caller = PathBuf::from(Location::caller().file());
    let path = PathBuf::from(caller.parent().unwrap());
    let file = path.join(format!("{initial}.txt"));

    let contents = fs::read_to_string(file)?;

    Ok(select(contents))
}

pub struct StopWatch {
    instant: Option<Instant>,
}

impl StopWatch {
    pub fn new() -> Self {
        Self { instant: None }
    }

    pub fn start(&mut self) {
        self.instant = Some(Instant::now());
    }

    pub fn stop(&self) {
        if let Some(duration) = self.instant {
            println!(
                "Duration: {}.{}s",
                duration.elapsed().as_secs(),
                duration.elapsed().subsec_nanos()
            )
        } else {
            println!("Stop was called before start")
        }
    }
}
