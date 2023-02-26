use pet_store::{pet_store::pet_store_server::PetStoreServer, svc::PetStoreService};
use tonic::transport::Server;
use tonic_reflection::server::Builder;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod reflection {
    pub(crate) const REFLECTION: &[u8] =
        tonic::include_file_descriptor_set!("pet_store_reflection");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry().with(fmt::layer()).init();
    info!("Hello, world!");
    let reflection = Builder::configure()
        .register_encoded_file_descriptor_set(reflection::REFLECTION)
        .build()?;
    let pet_store_svc = PetStoreServer::new(PetStoreService::new());

    Server::builder()
        .add_service(reflection)
        .add_service(pet_store_svc)
        .serve("[::1]:8000".parse()?)
        .await?;

    Ok(())
}
