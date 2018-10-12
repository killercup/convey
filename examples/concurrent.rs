extern crate failure;
extern crate output;
extern crate rand;

use output::{human, json};
use rand::distributions::Distribution;
use rand::distributions::Range;
use rand::thread_rng;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), failure::Error> {
    let out = output::new()
        .add_target(json::file("target/foo.log")?)
        .add_target(human::stdout()?);

    let mut threads = vec![];
    for i in 0..100 {
        let mut out = out.clone();
        let t = thread::spawn(move || {
            let dur = Duration::from_millis({
                let mut rng = thread_rng();
                let range = Range::new(0u64, 1);
                range.sample(&mut rng)
            });
            thread::sleep(dur);
            out.print(&format!("thread {} says hello", i))
        });
        threads.push(t);
    }

    threads.into_iter().for_each(|t| {
        t.join().unwrap().unwrap();
    });

    Ok(())
}
