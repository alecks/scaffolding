use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use twilight_model::id::UserId;

use crate::protos::models::User;
use crate::protos::rest_client::rest_client_server::RestClient as RestClientDefinition;
use crate::protos::rest_client::{BootstrapRequest, EntityRequest};

const NOT_BOOTSTRAPPED: &str =
  "A RestClient has not yet been bootstrapped. Call the Bootstrap method.";

#[derive(Default)]
pub struct Client {
  client: Mutex<Option<twilight_http::Client>>,
}

#[tonic::async_trait]
impl RestClientDefinition for Client {
  async fn bootstrap(&self, request: Request<BootstrapRequest>) -> Result<Response<User>, Status> {
    {
      let mut client = self.client.lock().await;
      *client = Some(twilight_http::Client::new(request.into_inner().token));
    }

    let me = match self
      .client
      .lock()
      .await
      .as_ref()
      .unwrap()
      .current_user()
      .await
    {
      Ok(u) => u,
      Err(e) => return Err(Status::unknown(e.to_string())),
    };

    tracing::info!(message = "A new client has been bootstrapped.", %me.id);
    Ok(Response::new(me.into()))
  }

  async fn get_user(&self, request: Request<EntityRequest>) -> Result<Response<User>, Status> {
    if let Some(client) = self.client.lock().await.as_ref() {
      let user = match client.user(UserId(request.into_inner().id)).await {
        Ok(u) => match u {
          Some(u) => u,
          None => return Err(Status::not_found("User does not exist.")),
        },
        Err(e) => return Err(Status::unknown(e.to_string())),
      };

      return Ok(Response::new(user.into()));
    }

    Err(Status::failed_precondition(NOT_BOOTSTRAPPED))
  }
}
