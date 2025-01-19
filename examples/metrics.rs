use rand::Rng;
use std::{thread, time::Duration};
use anyhow::{Ok, Result};
use concurrency::Metrics;

const N: usize = 2;
const M: usize = 4;


fn main() -> Result<()> {
    let metrics = Metrics::new();
    metrics.inc("req.page.1");
    metrics.inc("call.thread.work.1");

    for idx in 0..N { 
        task_worker(idx, metrics.clone());
    }


    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()>{
    thread::spawn(move || {
        let mut rng = rand::thread_rng();

        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();

    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || loop {
        // process requests
        let mut rng = rand::thread_rng();

        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..5);
        metrics.inc(format!("req.page.{}", page)).unwrap();
    });
    Ok(())
}