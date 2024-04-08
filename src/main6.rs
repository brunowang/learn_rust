use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::task;
use tokio::time::{sleep, timeout, Duration};

async fn job() -> String {
    String::from("job executed")
}

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(1)).await;
    println!("execute after sleeping");
    let ret = timeout(Duration::from_secs(1), job()).await;
    println!("timeout job {}", ret.unwrap());

    let server = TcpListener::bind("0.0.0.0:80").await.unwrap();
    loop {
        let (mut stream, _) = server.accept().await.unwrap();
        task::spawn(async move {
            let mut rsp = String::from("http/1.1 200 OK\r\n\r\n rust server\t");
            let ret = task::spawn_blocking(|| "blocking").await;
            if let Ok(s) = ret {
                rsp.push_str(s);
            }
            stream.write_all(rsp.as_bytes()).await.unwrap();
        })
        .await
        .unwrap();
    }
}
