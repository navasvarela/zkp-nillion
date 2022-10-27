//! Sample Client for the ZKP Chaum Pedersen Protocol.

use num::ToPrimitive;
use num::bigint::BigInt;
use modpow::modpow;
use rand::Rng;
pub mod zkp_auth {
    tonic::include_proto!(r#"zkp_auth"#); 
}

use zkp_auth::auth_client::AuthClient;
use zkp_auth::{AuthenticationAnswerRequest,AuthenticationChallengeRequest,RegisterRequest, InitialiseRequest};

const USERNAME: &str = "Test User";
const SERVER_ADDR: &str = "http://[::1]:50051";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthClient::connect(SERVER_ADDR).await?;
    // I. Call to initialise, receive the keys and log them. 
    let initialise_response = client.initialise(InitialiseRequest{}).await?;
    let keys = initialise_response.into_inner();
    println!("Initialising Client with g:{}, h:{}, q:{}", keys.g,keys.h,keys.q);
    // II. Calculate secret y1 and y2.
    // Send registration request

    // This is the secret value.
    let secret_x = 12;

    let registration = RegisterRequest{
        user: USERNAME.to_owned(),
        y1: modpow(&keys.g.to_be(), &BigInt::from(secret_x), &BigInt::from(keys.q)).to_i64(),
        y2: keys.h.pow(secret_x),
    };

    let _registration_response = client.register(registration).await?;
    println!("Registered successfully");
    // III. Send Authentication Challenge

    // Generate random k 
    // Limiting range to try and prevent overflow
    let k: u32 = rand::thread_rng().gen_range(2..10);

    let challenge_request = AuthenticationChallengeRequest {
        user: USERNAME.to_owned(),
        r1: keys.g.pow(k),
        r2: keys.g.pow(k)
    };

    let challenge_response = client.create_authentication_challenge(challenge_request).await?.into_inner();
    println!("Authentication Challenge Response - auth_id:{}, c:{}", challenge_response.auth_id, challenge_response.c);
    let answer_request = AuthenticationAnswerRequest{
        auth_id: challenge_response.auth_id,
        s: (k as i64 - challenge_response.c*(secret_x as i64)) % keys.q
    };

    // IV. Verify Authentication.
    let answer_response = client.verify_authentication(answer_request).await?.into_inner();

    println!("Authentication Answer - Session ID: {}", answer_response.session_id);

    Ok(())
}