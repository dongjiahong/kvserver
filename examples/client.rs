use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kvserver::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9527";
    // 链接服务器
    let stream = TcpStream::connect(addr).await?;
    // 使用AsyncProstStream来处理TCP Frame
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();
    // 生产一个HSET命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".into());
    // 发送HSET命令
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }
    Ok(())
}
