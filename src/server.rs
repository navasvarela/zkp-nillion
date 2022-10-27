use rand::Rng;
use store::{Authentication, AuthenticationStore, RegistrationSecret, RegistrationStore, Store};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use verifier::{ChaumPedersenVerifier, ZKPVerifier};
use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, InitialiseRequest, InitialiseResponse, RegisterRequest,
    RegisterResponse,
};

mod zkp_crypto;

mod store;

mod verifier;

pub mod zkp_auth {
    tonic::include_proto!(r#"zkp_auth"#);
}

const SERVER_ADDR: &str = "[::1]:50051";

#[derive(Debug, Default)]
pub struct MyAuth {
    registration_store: store::RegistrationStore,
    authentication_store: store::AuthenticationStore,
    verifier: ChaumPedersenVerifier, 
}

#[tonic::async_trait]
impl Auth for MyAuth {
    /// Initialise the keys g,h and order q.
    /// 
    /// TODO: Support multiple initialisations?
    /// At the moment the initialisation is stored as global
    /// Which is not ideal. Alternatively it should be read from some
    /// configuration file. 
    async fn initialise(
        &self,
        _request: Request<InitialiseRequest>,
    ) -> Result<Response<InitialiseResponse>, Status> {
        let attrs = &self.verifier.attrs;

        let reply = zkp_auth::InitialiseResponse {
            modulus: attrs.modulus,
            order: attrs.order,
            g1: attrs.first_generator,
            g2: attrs.second_generator,
        };       

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
        let registration = self.registration_store.get(authentication_result.user.to_string()).unwrap();

     
        if self.verifier.verify(registration, &authentication_result, answer_request.s) {
            return Err(Status::unauthenticated("Invalid authentication"));
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
        verifier: ChaumPedersenVerifier{
            attrs:  zkp_crypto::generate_keys(),
        }
    };

    Server::builder()
        .add_service(AuthServer::new(auth))
        .serve(addr)
        .await?;

    Ok(())
}
