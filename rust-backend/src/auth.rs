use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::{fmt, str::FromStr};
use uuid::Uuid;
use warp::{
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject,
};

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";

use crate::{error::AuthError, model::user::Claims};

#[derive(Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

pub fn create_jwt(uid: &str, role: &Role) -> Result<String, AuthError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(12))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| AuthError::JWTTokenCreationError)
}

pub async fn authorize(
    (role, headers): (Role, HeaderMap<HeaderValue>),
) -> Result<Uuid, warp::Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(AuthError::JWTTokenError))?;

            if Role::from_str(&decoded.claims.role) != role {
                return Err(reject::custom(AuthError::NoPermissionError));
            }

            /*if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(reject::custom(AuthError::NoPermissionError));
            }*/

            Ok(Uuid::from_str(&decoded.claims.sub).unwrap())
        }
        Err(e) => Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, AuthError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AuthError::NoAuthHeaderError),
    };

    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AuthError::NoAuthHeaderError),
    };

    if !auth_header.starts_with(BEARER) {
        return Err(AuthError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
