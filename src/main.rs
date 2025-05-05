#[tokio::main]
async fn main() {
    let mut i = 5;
    while i < 10 {
        hello_world().await;
        i = i + 1;
    }
}

async fn hello_world() {
    println!("Hello World");
}
