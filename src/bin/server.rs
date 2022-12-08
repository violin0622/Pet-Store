#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use pet_store::{
    google::r#type::Date,
    pet_store::pet_store_server::{PetStore, PetStoreServer},
    pet_store::*,
};
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tonic_reflection::server::Builder;

mod reflection {
    pub(crate) const REFLECTION: &[u8] =
        tonic::include_file_descriptor_set!("pet_store_reflection");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let reflection = Builder::configure()
        .register_encoded_file_descriptor_set(reflection::REFLECTION)
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

    async fn healthy(&self, request: Request<HealthyRequest>) -> Result<Response<()>, Status> {
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
                    species: String::from("Dog"),
                    variety: String::from("Dachshund"),
                    // birthday: None::<()>,
                    birthday: Some(Date {
                        year: 2000,
                        month: 12,
                        day: 30,
                    }),
                    comment: String::from(""),
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
        // Err(Status::unimplemented("Unimplemented!"))
    }

    async fn register_pet(
        &self,
        request: tonic::Request<RegisterPetRequest>,
    ) -> Result<tonic::Response<RegisterPetResponse>, tonic::Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }

    async fn unregister_pet(
        &self,
        request: tonic::Request<UnregisterPetRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }
    async fn view_pet(
        &self,
        request: tonic::Request<ViewPetRequest>,
    ) -> Result<tonic::Response<ViewPetResponse>, tonic::Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }
    async fn update_pet(
        &self,
        request: tonic::Request<UpdatePetRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }
    async fn modify_pet(
        &self,
        request: tonic::Request<ModifyPetRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }
}
