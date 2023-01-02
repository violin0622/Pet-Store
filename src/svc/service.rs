#![allow(unused_variables)]

use crate::{
    dao::DB,
    pet_store::{pet_store_server::PetStore, *},
};
use tokio::sync::mpsc;
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
        self.db.list_pets().map_or_else(
            |err| Err(Status::internal(err.to_string())),
            |pets| {
                let (tx, rx) = mpsc::channel(10);
                tokio::spawn(async move {
                    for pet in pets {
                        tx.send(Ok(pet.into())).await.unwrap();
                    }
                });
                Ok(Response::new(ReceiverStream::new(rx)))
            },
        )
    }

    async fn register_pet(
        &self,
        req: tonic::Request<RegisterPetRequest>,
    ) -> Result<tonic::Response<RegisterPetResponse>, tonic::Status> {
        self.db.insert_pet(req.into_inner().into()).map_or_else(
            |err| Err(Status::internal(err.to_string())),
            |pet| Ok(Response::new(pet.into())),
        )
    }

    async fn unregister_pet(
        &self,
        req: tonic::Request<UnregisterPetRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        self.db.delete_pet(req.into_inner().id as i64).map_or_else(
            |err| Err(Status::internal(err.to_string())),
            |_| Ok(Response::new(())),
        )
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
