use tonic::{transport::Server, Request, Response, Status};

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

pub mod zkp_auth {
    tonic::include_proto!(r#"zkp_auth"#); 
}

#[derive(Debug, Default)]
pub struct MyAuth {}

#[tonic::async_trait]
impl Auth for MyAuth {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = zkp_auth::RegisterResponse {};

        Ok(Response::new(reply))
    }

    async fn create_authentication_challenge(
      &self,
      request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
      
      let reply = zkp_auth::AuthenticationChallengeResponse {
        c: 2,
        auth_id: "1".to_owned()
      };

      Ok(Response::new(reply))
    }

    async fn verify_authentication(
      &self,
      request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {

      let reply = zkp_auth::AuthenticationAnswerResponse{
        session_id: "1".to_owned()
      };

      Ok(Response::new(reply))
    }
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let auth = MyAuth::default();

    Server::builder()
        .add_service(AuthServer::new(auth))
        .serve(addr)
        .await?;

    Ok(())
}
