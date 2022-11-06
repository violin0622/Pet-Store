#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::pet_store::{pet_store_client::PetStoreClient, HealthyRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let mut client = PetStoreClient::connect("http://[::1]:8000").await?;

    client
        .healthy(Request::new(HealthyRequest {}))
        .await
        .expect("Healthy service should return no error");

    println!("Server is healthy!");
    Ok(())
}

pub mod pet_store {
    tonic::include_proto!("pet_store");
}
