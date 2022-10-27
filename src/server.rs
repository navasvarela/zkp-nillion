use rand::Rng;
use store::{Authentication, AuthenticationStore, RegistrationSecret, RegistrationStore, Store};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, InitialiseRequest, InitialiseResponse, RegisterRequest,
    RegisterResponse,
};

mod zkp_crypto;

mod store;

pub mod zkp_auth {
    tonic::include_proto!(r#"zkp_auth"#);
}

const SERVER_ADDR: &str = "[::1]:50051";

static mut ZKP_KEYS: InitialiseResponse = InitialiseResponse{g:0,h:0,q:0};


#[derive(Debug, Default)]
pub struct MyAuth {
    registration_store: store::RegistrationStore,
    authentication_store: store::AuthenticationStore,
}

#[tonic::async_trait]
impl Auth for MyAuth {
    /// Initialise the keys g,h and order q.
    /// 
    /// TODO: Support multiple initialisations?
    /// At the moment the initialisation is stored as global
    async fn initialise(
        &self,
        _request: Request<InitialiseRequest>,
    ) -> Result<Response<InitialiseResponse>, Status> {
        let keys = zkp_crypto::generate_keys();

        let reply = zkp_auth::InitialiseResponse {
            g: keys.0,
            h: keys.1,
            q: keys.2,
        };

        unsafe {
          ZKP_KEYS = reply.clone();
        }

        Ok(Response::new(reply))
    }
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let _register_request = request.into_inner();
        let registration = RegistrationSecret {
            y1: _register_request.y1,
            y2: _register_request.y2,
        };

        let user = _register_request.user.clone();
        self.registration_store.insert(user, registration);

        self.registration_store.insert(
            _register_request.user,
            RegistrationSecret {
                y1: _register_request.y1,
                y2: _register_request.y2,
            },
        );

        let reply = zkp_auth::RegisterResponse {};

        Ok(Response::new(reply))
    }

    /// Create authentication challenge.
    /// 
    /// - Receives the challenge (r1,r2)
    /// - Generates random c
    /// - Stores the Authentication Challenge
    /// - Returns authentication ID and c to user.
    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let challenge = request.into_inner();
        let auth_id = Uuid::new_v4().to_string();
        // Making c small to prevent overflow when raising g and h to c.
        let c: i64 = rand::thread_rng().gen_range(2..10);
        let authentication = Authentication {
            c,
            r1: challenge.r1,
            r2: challenge.r2,
            user: challenge.user,
        };

        self.authentication_store
            .insert(auth_id.clone(), authentication);

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

        let answer_request = request.into_inner();
        let authentication_result = match self.authentication_store.get(answer_request.auth_id) {
          Some(value) => value,
          None => panic!("Invalid Authentication ID"),
        };
        // We don't expect to have a problem here though it would be good to do some error 
        // checking nonetheless.
        let registration = self.registration_store.get(authentication_result.user).unwrap();

        // To prevent overflows we calculate equalities using logarithms.  
        // The Chaum-Pedersen original verification is:
        // r1 = g^s*y1^c (mod q), r2 = h^s*y2^c (mod q)
        unsafe {
          let first = (answer_request.s as f64) + authentication_result.c as f64 * (registration.y1 as f64).log(ZKP_KEYS.g as f64) != (authentication_result.r1 as f64).log(ZKP_KEYS.g as f64);
          let second = (answer_request.s as f64) + authentication_result.c as f64 * (registration.y2 as f64).log(ZKP_KEYS.h as f64) != (authentication_result.r2 as f64).log(ZKP_KEYS.h as f64) ;
          if first || second {
            return Err(Status::unauthenticated("Invalid authentication"));
          }
        }
        

        let reply = zkp_auth::AuthenticationAnswerResponse {
            session_id: Uuid::new_v4().to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SERVER_ADDR.parse()?;
    let auth = MyAuth {
        registration_store: RegistrationStore::new(),
        authentication_store: AuthenticationStore::new(),
    };

    Server::builder()
        .add_service(AuthServer::new(auth))
        .serve(addr)
        .await?;

    Ok(())
}
