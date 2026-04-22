use crate::{
    auth::{AuthService, OptionalAuthSession, clear_auth_cookie, make_auth_cookie},
    db::DatabaseService,
    error::AppError,
    models::{SigninUser, SignupUser, Token, User},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use slug::slugify;
use ulid::Ulid;

/// Sign up
///
/// Creates a new user account and returns a JWT token and sets an `auth_token` HttpOnly cookie.
///
/// A unique slug is generated from the user's full name. If a slug collision occurs,
/// it will attempt to generate a new slug up to 3 times before returning a conflict error.
#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = SignupUser,
    responses(
        (status = 201, description = "User created, returns JWT token", body = Token),
        (status = 409, description = "Email already in use", body = crate::error::ErrorResponse),
    ),
    tag = "Users"
)]
#[tracing::instrument(skip_all)]
pub async fn signup(
    State(auth): State<AuthService>,
    State(db): State<DatabaseService>,
    jar: CookieJar,
    Json(payload): Json<SignupUser>,
) -> Result<(StatusCode, CookieJar, Json<Token>), AppError> {
    if db.get_user_by_email(&payload.email).await?.is_some() {
        tracing::warn!(email = %payload.email, "signup with already-used email");
        return Err(AppError::Conflict("email already in use".to_owned()));
    }

    let mut slug = slugify(&payload.full_name);
    for attempt in 0..3 {
        if db.get_user_by_slug(&slug).await?.is_some() {
            if attempt == 2 {
                tracing::warn!(slug = %slug, "slug collision after 3 attempts during signup");
                return Err(AppError::Conflict(
                    "failed to generate unique slug after 3 attempts".to_owned(),
                ));
            }
            let suffix = &Ulid::new().to_string()[20..];
            slug = format!("{slug}-{suffix}");
        } else {
            break;
        }
    }

    let id = Ulid::new().to_string();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    db.create_user(
        &id,
        &slug,
        &payload.full_name,
        &payload.email,
        &password_hash,
    )
    .await?;

    tracing::info!(user.id = %id, user.email = %payload.email, "user signed up");
    let token = auth.encode_jwt(&id)?;
    let jar = jar.add(make_auth_cookie(token.clone()));
    Ok((StatusCode::CREATED, jar, Json(Token { token })))
}

/// Sign in
///
/// Authenticates an existing user and sets an `auth_token` HttpOnly cookie.
///
/// The cookie is used automatically for subsequent requests. The token is also returned
/// in the response body for clients that prefer the `Authorization: Bearer` header.
#[utoipa::path(
    post,
    path = "/auth/signin",
    request_body = SigninUser,
    responses(
        (status = 200, description = "Authenticated, returns JWT token", body = Token),
        (status = 401, description = "Invalid email or password", body = crate::error::ErrorResponse),
    ),
    tag = "Users"
)]
#[tracing::instrument(skip_all)]
pub async fn signin(
    State(auth): State<AuthService>,
    State(db): State<DatabaseService>,
    jar: CookieJar,
    Json(payload): Json<SigninUser>,
) -> Result<(CookieJar, Json<Token>), AppError> {
    let user = db.get_user_by_email(&payload.email).await?.ok_or_else(|| {
        tracing::warn!(email = %payload.email, "signin attempt for unknown email");
        AppError::Unauthorized
    })?;

    let parsed_hash = PasswordHash::new(&user.password)?;
    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        tracing::warn!(user.id = %user.id, "invalid password on signin");
        return Err(AppError::Unauthorized);
    }

    tracing::info!(user.id = %user.id, "user signed in");
    let token = auth.encode_jwt(&user.id)?;
    let jar = jar.add(make_auth_cookie(token.clone()));
    Ok((jar, Json(Token { token })))
}

/// Get current user
///
/// Returns the profile of the currently authenticated user.
///
/// Authentication is accepted via `auth_token` cookie (preferred) or
/// `Authorization: Bearer <token>` header.
#[utoipa::path(
    get,
    path = "/users/me",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Current user profile", body = User),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    tag = "Users"
)]
#[tracing::instrument(skip_all)]
pub async fn me(OptionalAuthSession(user): OptionalAuthSession) -> Result<Json<User>, AppError> {
    let user = user.ok_or(AppError::Unauthorized)?;
    tracing::debug!(user.id = %user.id, "fetched current user");
    Ok(Json(user))
}

/// Logout
///
/// Clears the `auth_token` cookie.
#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 204, description = "Logged out, cookie cleared"),
    ),
    tag = "Users"
)]
#[tracing::instrument(skip_all)]
pub async fn logout(jar: CookieJar) -> (StatusCode, CookieJar) {
    let jar = jar.remove(clear_auth_cookie());
    tracing::info!("user logged out");
    (StatusCode::NO_CONTENT, jar)
}
