#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use async_stream::stream;
use pet_store::pet_store::{pet_store_client::PetStoreClient, HealthyRequest, ListPetsRequest};
use tokio::time::{self, Duration};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let mut client = PetStoreClient::connect("http://[::1]:8000").await?;
    let start = time::Instant::now();

    client
        .healthy(Request::new(HealthyRequest {}))
        .await
        .expect("Healthy service should return no error");
    println!("Server is healthy!");

    let mut stream = client
        .list_pets(Request::new(ListPetsRequest {}))
        .await
        .expect("There should be pets.")
        .into_inner();

    let mut pets = vec![];
    while let Ok(Some(pet)) = stream.message().await {
        pets.push(pet.clone());
        println!("Received pet {pet:?}");
    }

    Ok(())
}
