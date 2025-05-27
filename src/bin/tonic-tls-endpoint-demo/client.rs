pub mod pb {
    tonic::include_proto!("grpc.examples.unaryecho");
}

use pb::{echo_client::EchoClient, EchoRequest};
use tokio::time::{sleep, Duration};
use tonic::{transport::{Certificate, ClientTlsConfig, Endpoint, Identity}, Code};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "tls"]);

    let server_root_ca_cert = std::fs::read_to_string(data_dir.join("ca/cert.pem"))?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);

    let client_cert = std::fs::read_to_string(data_dir.join("peer1/cert.pem"))?;
    let client_key = std::fs::read_to_string(data_dir.join("peer1/key.pem"))?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name("peer0.lan")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let endpoint = Endpoint::from_static("https://[::1]:50051")
        .tls_config(tls)?
        .connect_lazy();

    let mut client = EchoClient::new(endpoint.clone());

    loop {
        sleep(Duration::from_secs(1)).await;

        let req = tonic::Request::new(EchoRequest {
            message: "hello".into(),
        });

        match client.unary_echo(req).await {
            Ok(resp) => {
                println!("Response: {:?}", resp.into_inner().message);
            }
            Err(status) if status.code() == Code::Unavailable => {
                eprintln!("Server Unavailable...");
            }
            Err(err) => {
                eprintln!("Request failed: {:?}", err);
            }
        }
    }
}
