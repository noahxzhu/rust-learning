
use tokio::{sync::{oneshot, mpsc}, time::sleep, time::Duration, spawn};

#[derive(Debug)]
struct Work {
    input: u32,
    respond_on: oneshot::Sender<u32>,
}

async fn worker(mut work_queue: mpsc::Receiver<Work>)  {
    let mut iterations = 0;
    loop {
        tokio::select! {
            Some(work) = work_queue.recv() => {
                sleep(Duration::from_millis(10)).await;
                work.respond_on.send(work.input * 1000).expect("failed to send response");
                iterations += 1;
            }
        }
    }
}

async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
    let (tx, rx) = oneshot::channel();
    work_queue.send(Work {
        input,
        respond_on: tx,
    }).await.expect("failed to send on work queue");

    rx.await.expect("failed waiting for response")
}

#[tokio::main]
async fn main() {
let (tx, rx) = mpsc::channel(10);
    spawn(worker(rx));
    for i in 0..100 {
        let resp = do_work(&tx, i).await;
        println!("work result for iteration {i}: {resp}");
    }
}
