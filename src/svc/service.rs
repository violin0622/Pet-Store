#![allow(unused_variables)]

use crate::{
    dao::{
        model::{self, NewPet},
        new_connection, DB,
    },
    google::r#type::Date,
    pet_store::{pet_store_server::PetStore, *},
};
use diesel::prelude::*;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

pub struct PetStoreService {
    db: DB,
}

impl PetStoreService {
    pub fn new() -> Self {
        Self { db: DB::new() }
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
                    comment: String::from(""),
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn register_pet(
        &self,
        req: tonic::Request<RegisterPetRequest>,
    ) -> Result<tonic::Response<RegisterPetResponse>, tonic::Status> {
        let conn = &mut new_connection();
        let new_pet: Vec<NewPet> = vec![req.into_inner().into()];

        use crate::dao::schema::pets;
        diesel::insert_into(pets::table)
            .values(new_pet)
            .get_result::<model::Pet>(conn)
            .map_or_else(
                |err| Err(Status::internal(err.to_string())),
                |pet| Ok(Response::new(pet.into())),
            )
    }

    async fn unregister_pet(
        &self,
        request: tonic::Request<UnregisterPetRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Err(Status::unimplemented("Unimplemented!"))
    }

    async fn view_pet(
        &self,
        req: tonic::Request<ViewPetRequest>,
    ) -> Result<tonic::Response<ViewPetResponse>, tonic::Status> {
        self.db.take_pet(req.into_inner().id as i64).map_or_else(
            |err| Err(Status::internal(err.to_string())),
            |pet| Ok(Response::new(pet.into())),
        )
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
