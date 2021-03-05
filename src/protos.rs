use models::User;

pub mod models {
  tonic::include_proto!("models");
}

pub mod rest_client {
  tonic::include_proto!("rest_client");
}

// TODO: Possibly implement a macro to make this cleaner.
impl From<twilight_model::user::CurrentUser> for User {
  fn from(u: twilight_model::user::CurrentUser) -> Self {
    Self {
      avatar: u.avatar.unwrap_or_default(),
      bot: u.bot,
      discriminator: u.discriminator,
      email: u.email.unwrap_or_default(),
      id: u.id.0,
      locale: u.locale.unwrap_or_default(),
      mfa_enabled: u.mfa_enabled,
      name: u.name,
      system: false,
      verified: u.verified.unwrap_or_default(),
    }
  }
}

impl From<twilight_model::user::User> for User {
  fn from(u: twilight_model::user::User) -> Self {
    Self {
      avatar: u.avatar.unwrap_or_default(),
      bot: u.bot,
      discriminator: u.discriminator,
      email: u.email.unwrap_or_default(),
      id: u.id.0,
      locale: u.locale.unwrap_or_default(),
      mfa_enabled: u.mfa_enabled.unwrap_or_default(),
      name: u.name,
      system: u.system.unwrap_or_default(),
      verified: u.verified.unwrap_or_default(),
    }
  }
}
