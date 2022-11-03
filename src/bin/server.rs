#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::pet_store::pet_store_server::{PetStore, PetStoreServer};
use tokio_stream::wrappers::ReceiverStream;
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
    type PulseStream = ReceiverStream<Result<pet_store::PulseResponse, Status>>;

    async fn pulse(
        &self,
        request: Request<Streaming<pet_store::PulseRequest>>,
    ) -> Result<Response<Self::PulseStream>, Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }

    async fn healthy(
        &self,
        request: Request<pet_store::HealthyRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
