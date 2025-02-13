use crate::{prelude::*, resources::user::UserResponse};
use azure_core::Context;

#[derive(Debug, Clone)]
pub struct GetUserBuilder {
    client: UserClient,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl GetUserBuilder {
    pub fn new(client: UserClient) -> Self {
        Self {
            client,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> GetUser {
        Box::pin(async move {
            let mut request = self
                .client
                .prepare_request_with_user_name(http::Method::GET);

            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;
            request.set_body(bytes::Bytes::from_static(&[]).into());
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Users),
                    &mut request,
                )
                .await?;

            UserResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type GetUser = futures::future::BoxFuture<'static, crate::Result<UserResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetUserBuilder {
    type Future = GetUser;
    type Output = <GetUser as std::future::Future>::Output;
    fn into_future(self) -> Self::Future {
        Self::into_future(self)
    }
}
