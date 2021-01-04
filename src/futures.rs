use tokio;

mod task;

#[tokio::main]
async fn main() {
    for _ in 0..3 {
        tokio::spawn(async move {task::task()});
    }
}
