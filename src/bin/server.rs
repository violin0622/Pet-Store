#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::pet_store::pet_store_server::{PetStore, PetStoreServer};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::Status;

mod pet_store {
    tonic::include_proto!("pet_store");
    pub(crate) const REFLECTION: &[u8] =
        tonic::include_file_descriptor_set!("pet_store_reflection");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let reflection = tonic_reflection::server::Builder::configure()
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
    type PulseStream = ReceiverStream<Result<pet_store::PulseResponse, tonic::Status>>;

    async fn pulse(
        &self,
        request: tonic::Request<tonic::Streaming<pet_store::PulseRequest>>,
    ) -> Result<tonic::Response<Self::PulseStream>, tonic::Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }

    async fn healthy(
        &self,
        request: tonic::Request<pet_store::HealthyRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }
}
