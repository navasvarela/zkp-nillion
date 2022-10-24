use rand::Rng;
use store::{RegistrationStore, Store, AuthenticationStore, RegistrationSecret, Authentication};
use tonic::{transport::Server, Request, Response, Status};

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

use uuid::Uuid;

mod store;

pub mod zkp_auth {
    tonic::include_proto!(r#"zkp_auth"#); 
}

#[derive(Debug, Default)]
pub struct MyAuth {
  registrationStore: store::RegistrationStore,
  authenticationStore: store::AuthenticationStore,
}

#[tonic::async_trait]
impl Auth for MyAuth {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let _register_request = request.into_inner();
        let registration = RegistrationSecret{
          y1: _register_request.y1,
          y2: _register_request.y2
        };  
        
        let user = _register_request.user.clone();
        self.registrationStore.insert(user, registration);

        let reply = zkp_auth::RegisterResponse {};

        Ok(Response::new(reply))
    }

    async fn create_authentication_challenge(
      &self,
      request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
      let challenge = request.into_inner();
      let auth_id = Uuid::new_v4().to_string();
      let c: i64 = rand::thread_rng().gen();
      let authentication = Authentication {
        c,
        r1: challenge.r1,
        r2: challenge.r2
      };

      self.authenticationStore.insert(auth_id.clone(), authentication);

      let reply = zkp_auth::AuthenticationChallengeResponse {
        c,
        auth_id: auth_id.clone(),
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
    let auth = MyAuth{
      registrationStore: RegistrationStore::new(),
      authenticationStore: AuthenticationStore::new()
    };

    Server::builder()
        .add_service(AuthServer::new(auth))
        .serve(addr)
        .await?;

    Ok(())
}
