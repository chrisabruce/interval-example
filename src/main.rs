use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::Duration;

fn main() {
    println!("Starting...");

    let task = Interval::new_interval(Duration::from_secs(5))
        .take(5)
        .for_each(|instant| {
            println!("fire; instant={:?}", instant);
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}
