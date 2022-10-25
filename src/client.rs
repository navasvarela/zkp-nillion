pub mod zkp_auth {
    tonic::include_proto!(r#"zkp_auth"#); 
}

use zkp_auth::auth_client::AuthClient;
use zkp_auth::{AuthenticationAnswerRequest,AuthenticationChallengeRequest,RegisterRequest};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthClient::connect("http://[::1]:10000").await?;
    // I. Call to initialise, receive the keys and log them. 


    let registration = RegisterRequest{
        user: "Test User".to_owned(),
        y1: 1,
        y2: 2
    };

    let registration_response = client.register(registration);

    Ok(())
}