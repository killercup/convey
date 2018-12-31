use convey;
use failure;

use convey::{human, json};
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), failure::Error> {
    let out = convey::new()
        .add_target(json::file("target/foo.log")?)?
        .add_target(human::stdout()?)?;

    let mut threads = vec![];
    for i in 0..100 {
        let out = out.clone();
        let t = thread::spawn(move || {
            let dur = Duration::from_millis(thread_rng().gen_range(0u64, 1));
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
