#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::pet_store::{
    pet_store_server::{PetStore, PetStoreServer},
    HostingPetsResponse, HostringPetsRequest, ListPetsRequest, ListPetsResponse, PulseRequest,
    PulseResponse,
};
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tonic_reflection::server::Builder;

mod pet_store {
    tonic::include_proto!("pet_store");
    pub(crate) const REFLECTION: &[u8] =
        tonic::include_file_descriptor_set!("pet_store_reflection");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let reflection = Builder::configure()
        .register_encoded_file_descriptor_set(pet_store::REFLECTION)
        .build()?;
    let pet_store_svc = PetStoreServer::new(PetStoreServiceServer);

    Server::builder()
        .add_service(reflection)
        .add_service(pet_store_svc)
        .serve("[::1]:8000".parse()?)
        .await?;

    Ok(())
}

pub struct PetStoreServiceServer;

#[tonic::async_trait]
impl PetStore for PetStoreServiceServer {
    type PulseStream = ReceiverStream<Result<PulseResponse, Status>>;
    async fn pulse(
        &self,
        request: Request<Streaming<PulseRequest>>,
    ) -> Result<Response<Self::PulseStream>, Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }

    async fn healthy(
        &self,
        request: Request<pet_store::HealthyRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    type ListPetsStream = ReceiverStream<Result<ListPetsResponse, Status>>;
    async fn list_pets(
        &self,
        request: Request<ListPetsRequest>,
    ) -> Result<Response<Self::ListPetsStream>, Status> {
        let (tx, rx) = mpsc::channel(10);
        let mut counter = 0;
        let mut interval = time::interval(Duration::from_millis(100));
        tokio::spawn(async move {
            while counter < 10 {
                interval.tick().await;
                counter += 1;
                tx.send(Ok(ListPetsResponse {
                    id: counter,
                    name: format!("Pet No.{counter}"),
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
        // Err(Status::unimplemented("Unimplemented!"))
    }

    async fn hosting_pets(
        &self,
        req: Request<Streaming<HostringPetsRequest>>,
    ) -> Result<Response<HostingPetsResponse>, Status> {
        let mut req_stream = req.into_inner();
        let mut pets = Vec::new();

        // 如果有导入 stkio:streamExt 的话，下面这一句也可以
        // while let Some(Ok(msg)) = req_stream.next().await {
        while let Ok(Some(msg)) = req_stream.message().await {
            println!("Receving pet {msg:?}");
            pets.push(msg.clone());
        }
        let count = pets.len();
        println!("Hosting {count} pets. All pets {pets:?} accepted");
        Ok(Response::new(HostingPetsResponse { accept: true }))
    }
}
