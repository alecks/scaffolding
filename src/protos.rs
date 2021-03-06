use models::{PermissionOverwrite, TextChannel, User};
use twilight_model::channel::permission_overwrite;

pub mod models {
  tonic::include_proto!("models");
}

pub mod http_client {
  tonic::include_proto!("http_client");
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

impl From<twilight_model::channel::TextChannel> for TextChannel {
  fn from(c: twilight_model::channel::TextChannel) -> Self {
    Self {
      guild_id: c.guild_id.unwrap_or_default().0,
      id: c.id.0,
      last_message_id: c.last_message_id.unwrap_or_default().0,
      last_pin_timestamp: c.last_pin_timestamp.unwrap_or_default(),
      name: c.name,
      nsfw: c.nsfw,
      parent_id: c.parent_id.unwrap_or_default().0,
      permission_overwrites: c
        .permission_overwrites
        .into_iter()
        .map(|x| x.into())
        .collect(),
      position: c.position,
      rate_limit_per_user: c.rate_limit_per_user.unwrap_or_default(),
      topic: c.topic.unwrap_or_default(),
    }
  }
}

impl From<permission_overwrite::PermissionOverwrite> for PermissionOverwrite {
  fn from(p: permission_overwrite::PermissionOverwrite) -> Self {
    let (id, kind) = match p.kind {
      permission_overwrite::PermissionOverwriteType::Member(id) => (id.0, 0),
      permission_overwrite::PermissionOverwriteType::Role(id) => (id.0, 1),
    };
    Self {
      allow: p.allow.bits(),
      deny: p.deny.bits(),
      id: id,
      kind: kind,
    }
  }
}
