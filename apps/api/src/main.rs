use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("running...");
    loop {
//         println!("Ping!");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("Stopped!");
}
