use std::net::SocketAddr;
use tonic::transport::Server;

use backend::route::*;

use backend::sample2_mod::sample2_server::Sample2Server;
use backend::sample_mod::sample_server::SampleServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:50052".parse()?;

    let sample_service = SampleServer::new(sample::MySample::default());
    let sample2_service = Sample2Server::new(sample2::MySample2::default());

    println!("server listening at {}:{}", &addr.ip(), &addr.port());
    Server::builder()
        .add_service(sample_service)
        .add_service(sample2_service)
        .serve(addr)
        .await?;

    Ok(())
}
