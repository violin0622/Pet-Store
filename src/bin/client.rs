#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::pet_store::{pet_store_client::PetStoreClient, HealthyRequest, HostringPetsRequest};
use async_stream::stream;
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

    let outbound = stream! {
        let mut interval = time::interval(Duration::from_millis(100));
        let mut new_id = 0;

        loop {
            let time = interval.tick().await;
            new_id += 1;
            let pet = HostringPetsRequest{ id: new_id, name: String::from("David") };

            if new_id==10{
                break;
            }
            println!("Hosting pet {pet:?}");
            yield pet;
        }
    };
    client
        .hosting_pets(Request::new(outbound))
        .await
        .expect("Pets should be accepted");
    println!("All pets have already been hosted to Pet Store");

    Ok(())
}

pub mod pet_store {
    tonic::include_proto!("pet_store");
}
