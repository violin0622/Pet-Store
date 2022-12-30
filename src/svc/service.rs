#![allow(unused_variables)]

use crate::{
    dao::{
        model::{self, NewPet},
        new_connection,
    },
    google::r#type::Date,
    pet_store::{pet_store_server::PetStore, *},
};
use diesel::prelude::*;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

pub struct PetStoreService {}

impl PetStoreService {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl PetStore for PetStoreService {
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
                    birthday: None::<Date>,
                    // birthday: Some(Date {
                    //     year: 2000,
                    //     month: 12,
                    //     day: 30,
                    // }),
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
        req: tonic::Request<RegisterPetRequest>,
    ) -> Result<tonic::Response<RegisterPetResponse>, tonic::Status> {
        let conn = &mut new_connection();
        let new_pet: Vec<NewPet> = vec![req.into_inner().into()];

        use crate::dao::schema::pets;
        let insert_result = diesel::insert_into(pets::table)
            .values(new_pet)
            // .get_results(conn)
            .get_result::<model::Pet>(conn);
        match insert_result {
            Ok(pet) => Ok(Response::new(pet.into())),
            Err(err) => Err(Status::internal(err.to_string())),
        }
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
