use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("running...");
    loop {
//         println!("Ping!");
        sleep(Duration::from_secs(1)).await;
    }

    println!("Stopped!");
}
