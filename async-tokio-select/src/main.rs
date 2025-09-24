use std::time::Duration;
use tokio::sync::mpsc;
use tokio::{io, time};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::IntervalStream;

async fn do_rx_event(cmd: Option<String>) {
    // do something
}
async fn do_tick() {
    // do something
}

async fn multi_handler(
    mut rx1: mpsc::Receiver<String>,
    mut rx2: mpsc::Receiver<String>,
) -> std::io::Result<()> {
    let mut tick =
        IntervalStream::new(time::interval(Duration::from_millis(200)));

    loop {
        tokio::select! {
           cmd = rx1.recv() => {
                do_rx_event(cmd).await;
            }
           cmd = rx2.recv() => {
                do_rx_event(cmd).await;
            }
            cyclic = tick.next() => {
                println!("tick");
                do_tick().await;
            }
        }
    }

    Ok(())
}



#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx1, rx1) = mpsc::channel::<String>(10);
    let (tx2, rx2) = mpsc::channel::<String>(10);

    match multi_handler(rx1, rx2).await {
        Ok(_) => {Ok(())},
        Err(err) => {Ok(())}
    }
}