use futures::{stream, StreamExt};
use futures::future::join_all;
use par_stream::prelude::*;
use tokio::time::{Duration, sleep};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    stream::iter(1..10).par_then(None, |i| async move {
        wait_and_print(i).await
    }).collect::<Vec<_>>().await;
}

// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let futures = stream::iter(1..10).map(|i| async move {
//         wait_and_print(i).await
//     }).collect::<Vec<_>>().await;
//     join_all(futures).await;
// }

async fn wait_and_print(i: u32) {
    sleep(Duration::from_millis(3000)).await;
    println!("waiting 1 second, iteration {i}");
}