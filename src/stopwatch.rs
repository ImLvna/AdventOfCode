use std::time::{Duration, Instant};

use crate::clap::ARGS;

#[derive(Clone)]
pub struct Stopwatch {
    name: String,
    pub begining: Option<Instant>,
    pub end: Option<Instant>,
    pub stopped: bool,
}
impl Stopwatch {
    pub fn new(name: String) -> Self {
        return (Stopwatch {
            name,
            begining: None,
            end: None,
            stopped: true,
        })
        .start()
        .clone();
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        return self;
    }

    pub fn stop(&mut self) -> &mut Self {
        self.end = Some(Instant::now());
        self.stopped = true;
        return self;
    }

    pub fn reset(&mut self) -> &mut Self {
        self.end = None;
        if ARGS.lock().unwrap().stopwatch {
            self.begining = Some(Instant::now());
            self.stopped = true;
        };
        return self;
    }
    pub fn start(&mut self) -> &mut Self {
        if ARGS.lock().unwrap().stopwatch {
            self.begining = Some(Instant::now());
            self.stopped = false;
        };
        return self;
    }

    pub fn duration(&mut self) -> Duration {
        return self
            .end
            .unwrap_or(Instant::now())
            .duration_since(self.begining.unwrap_or(Instant::now()));
    }

    pub fn print(&mut self) -> &mut Self {
        let args = ARGS.lock().unwrap();
        if !args.stopwatch {
            return self;
        };
        let duration = &self.duration();
        match args.verbose {
            0 => println!("{} finished in {}ms", self.name, duration.as_millis()),
            _ => println!("{} finished in {}μs", self.name, duration.as_micros()),
        }

        return self;
    }
}
