use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};
use hello_world::{greeter_client::GreeterClient};
use tonic::transport::Channel;

use chrono::Local;
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let time = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}]-Got a request: {:?}", time, request);

        let response = hello_world::HelloResponse {
            message: format!("[{}] Hello, {}!", time, request.into_inner().name).into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
pub async fn go() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();

    let greeter = MyGreeter::default();

    let svc = GreeterServer::new(greeter);

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
pub async fn hi() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = GreeterClient::new(channel);

    let request = tonic::Request::new(HelloRequest {
        name: "World".into(),
    });

    let response = client.say_hello(request).await?.into_inner();

    println!("Greeting: {}", response.message);

    Ok(())
}