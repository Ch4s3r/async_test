use futures::{stream, StreamExt};
use tokio::time::{Duration, sleep};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    stream::iter(1..10).then(|i| async move {
        wait_and_print(i).await
    }).collect::<Vec<_>>().await;
}

async fn wait_and_print(i: u32) {
    sleep(Duration::from_millis(333)).await;
    println!("waiting 1 second, iteration {i}");
}