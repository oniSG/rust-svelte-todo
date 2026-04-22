use std::convert::Infallible;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_extra::extract::cookie::{Cookie, SameSite};
use time::Duration;

use crate::{
    db::DatabaseService,
    error::AppError,
    models::{Claims, User},
};

pub const AUTH_COOKIE: &str = "auth_token";

/// Builds a signed-in `auth_token` cookie with secure defaults.
pub fn make_auth_cookie(token: String) -> Cookie<'static> {
    Cookie::build((AUTH_COOKIE, token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::days(7))
        .build()
}

/// Builds an expired `auth_token` cookie that instructs the browser to delete it.
pub fn clear_auth_cookie() -> Cookie<'static> {
    Cookie::build(AUTH_COOKIE).path("/").build()
}

/// Owns the JWT secret and a Database handle.
/// All authentication logic lives here — token encoding, decoding, user lookup.
#[derive(Clone)]
pub struct AuthService {
    db: DatabaseService,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(db: DatabaseService, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }

    pub fn encode_jwt(&self, id: &str) -> Result<String, AppError> {
        let exp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .as_secs()
            .saturating_add(60 * 60 * 24 * 7); // 7 days

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &Claims {
                id: id.to_owned(),
                exp,
            },
            &jsonwebtoken::EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    fn decode_jwt(&self, token: &str) -> Option<Claims> {
        let mut validation = jsonwebtoken::Validation::default();
        validation.required_spec_claims = std::collections::HashSet::new();

        jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .ok()
    }

    #[tracing::instrument(skip_all)]
    pub async fn try_authenticate(&self, parts: &mut Parts) -> Option<User> {
        // Check HttpOnly cookie first, fall back to Authorization header.
        let cookie_token = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .and_then(|cookies_str| {
                cookies_str.split(';').find_map(|part| {
                    part.trim()
                        .strip_prefix(&format!("{AUTH_COOKIE}="))
                        .map(ToOwned::to_owned)
                })
            });

        let header_token = || {
            parts
                .headers
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .and_then(|v| v.split(',').next())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned)
        };

        let token = cookie_token.or_else(header_token)?;

        let claims = self.decode_jwt(&token).or_else(|| {
            tracing::warn!("jwt decode failed");
            None
        })?;

        match self.db.get_user_by_id(&claims.id).await {
            Ok(Some(user)) => Some(user),
            Ok(None) => {
                tracing::warn!(user_id = %claims.id, "jwt references unknown user");
                None
            }
            Err(err) => {
                tracing::error!(error = %err, "db error during authentication");
                None
            }
        }
    }
}

/// Extractor for routes that work for both anonymous and authenticated users.
/// Always succeeds — use `session.0` to get `Option<User>`.
#[derive(Debug, Clone)]
pub struct OptionalAuthSession(pub Option<User>);

impl<S> FromRequestParts<S> for OptionalAuthSession
where
    AuthService: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth = AuthService::from_ref(state);
        Ok(Self(auth.try_authenticate(parts).await))
    }
}
