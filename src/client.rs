//! Sample Client for the ZKP Chaum Pedersen Protocol.
//! 
//! The steps are hardcoded but ideally the client should be interactive
//! Perhaps via a REST API or a CLI?



use rand::Rng;
pub mod zkp_auth {
    tonic::include_proto!(r#"zkp_auth"#); 
}

mod prover;

use prover::{ChaumPedersenProver,ZKPProver};
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
    println!("Initialising Client with generator1:{}, generator2:{}, group order:{}", keys.g1,keys.g2,keys.order);
    // II. Calculate secret y1 and y2.
    // This is the secret value.
    let secret_x = 12;

    let prover = ChaumPedersenProver {
        g: keys.g1,
        h: keys.g2,
        q: keys.order,
        p: keys.modulus,
    };

    let reg_keys = prover.generate_registration_keys(secret_x);

    let registration = RegisterRequest{
        user: USERNAME.to_owned(),
        y1: reg_keys.0,
        y2: reg_keys.1,
    };

    let _registration_response = client.register(registration).await?;
    println!("Registered successfully");
    // III. Send Commitment and receive Challenge

    // Generate random k 
    let k: u32 = rand::thread_rng().gen();

    // Generate commitment
    let commitment = prover.generate_commitment(k);

    let challenge_request = AuthenticationChallengeRequest {
        user: USERNAME.to_owned(),
        r1: commitment.0,
        r2: commitment.1,
    };

    let challenge_response = client.create_authentication_challenge(challenge_request).await?.into_inner();
    println!("Authentication Challenge Response - auth_id:{}, c:{}", challenge_response.auth_id, challenge_response.c);
    let answer_request = AuthenticationAnswerRequest{
        auth_id: challenge_response.auth_id,
        s: prover.generate_challenge(k, secret_x, challenge_response.c),
    };

    // IV. Verify Authentication.
    let answer_response = client.verify_authentication(answer_request).await?.into_inner();

    println!("Authentication Answer - Session ID: {}", answer_response.session_id);

    Ok(())
}