use futures::{stream, StreamExt};
use par_stream::prelude::*;
use tokio::time::{Duration, sleep};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    stream::iter(1..10).par_then(None, |i| async move {
        wait_and_print(i).await
    }).collect::<Vec<_>>().await;
}

async fn wait_and_print(i: u32) {
    sleep(Duration::from_millis(3000)).await;
    println!("waiting 1 second, iteration {i}");
}