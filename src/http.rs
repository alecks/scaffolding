use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use twilight_model::channel::{Channel, GuildChannel};
use twilight_model::id::{ChannelId, UserId};

use crate::protos::http_client::http_client_server::HttpClient as HttpClientDefinition;
use crate::protos::http_client::{BootstrapRequest, EntityRequest};
use crate::protos::models::{TextChannel, User};

const NOT_BOOTSTRAPPED: &str =
  "A HttpClient has not yet been bootstrapped. Call the Bootstrap method.";

#[derive(Default)]
pub struct Client {
  client: Mutex<Option<twilight_http::Client>>,
}

#[tonic::async_trait]
impl HttpClientDefinition for Client {
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

  async fn get_text_channel(
    &self,
    request: Request<EntityRequest>,
  ) -> Result<Response<TextChannel>, Status> {
    if let Some(client) = self.client.lock().await.as_ref() {
      let channel = match client.channel(ChannelId(request.into_inner().id)).await {
        Ok(c) => match c {
          Some(c) => match c {
            Channel::Guild(GuildChannel::Text(c)) => c,
            _ => {
              return Err(Status::not_found(
                "A channel with the provided ID exists, but is not a TextChannel.",
              ))
            }
          },
          None => return Err(Status::not_found("Channel does not exist.")),
        },
        Err(e) => return Err(Status::unknown(e.to_string())),
      };

      return Ok(Response::new(channel.into()));
    }

    Err(Status::failed_precondition(NOT_BOOTSTRAPPED))
  }
}
