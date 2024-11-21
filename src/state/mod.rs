use crate::bounded_string::{NonEmptyString, SecureString};
use crate::config::Configuration;
use crate::crypto::strong_random_bytes;
use crate::data::pkce::CodeChallenge;
use crate::data::{Scope, Secret};
use axum::extract::FromRequestParts;
use axum::http::Uri;
use derive_more::derive::AsRef;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    address: Address,
    email_verified: bool,
    email: NonEmptyString,
    family_name: NonEmptyString,
    given_name: NonEmptyString,
    locale: NonEmptyString,
    middle_name: NonEmptyString,
    name: NonEmptyString,
    nickname: NonEmptyString,
    phone_number: NonEmptyString,
    updated_at: u64,
    zoneinfo: NonEmptyString,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Address {
    country: NonEmptyString,
    locality: NonEmptyString,
    postal_code: NonEmptyString,
    region: NonEmptyString,
    street_address: NonEmptyString,
}

pub struct AuthSession {
    pub client_id: SecureString,
    pub code_challenge: CodeChallenge,
    pub redirect_uri: Uri,
    pub scope: HashSet<Scope>,
    pub user_id: SecureString,
}

pub struct Vars {
    pub audience: Uri,
    pub client_secret: SecureString,
    pub expiration: u64,
    pub issuer: Uri,
    pub secret: Secret,
    pub required_scopes: HashSet<Scope>,
}

impl From<Configuration> for Vars {
    fn from(config: Configuration) -> Self {
        Self {
            audience: config.audience,
            client_secret: config.client_secret,
            expiration: config.expiration,
            issuer: config.issuer,
            secret: config.secret,
            required_scopes: [Scope::Profile, Scope::Email, Scope::Address, Scope::Phone].into(),
        }
    }
}

pub struct State {
    auth_sessions: HashMap<SecureString, AuthSession>,
    rng: StdRng,
    user: User,
    vars: Vars,
}

#[derive(AsRef, Clone, FromRequestParts)]
#[as_ref(forward)]
#[from_request(via(axum::extract::State))]
pub struct AppState(Arc<RwLock<State>>);

impl AppState {
    pub fn get_session(&self, code: &SecureString) -> Option<AuthSession> {
        self.as_ref().write().unwrap().auth_sessions.remove(code)
    }

    pub fn set_session(&self, code: SecureString, session: AuthSession) {
        self.as_ref()
            .write()
            .unwrap()
            .auth_sessions
            .insert(code, session);
    }

    pub fn get_user(&self) -> User {
        self.as_ref().read().unwrap().user.clone()
    }

    pub fn gen_secure_string(&self) -> SecureString {
        let mut lock = self.as_ref().write().unwrap();
        strong_random_bytes(&mut lock.rng).try_into().unwrap()
    }

    pub fn audience(&self) -> Uri {
        self.0.read().unwrap().vars.audience.clone()
    }

    pub fn client_secret(&self) -> SecureString {
        self.0.read().unwrap().vars.client_secret.clone()
    }

    pub fn expiration(&self) -> u64 {
        self.0.read().unwrap().vars.expiration
    }

    pub fn issuer(&self) -> Uri {
        self.0.read().unwrap().vars.issuer.clone()
    }

    pub fn secret(&self) -> Secret {
        self.0.read().unwrap().vars.secret.clone()
    }

    pub fn required_scopes(&self) -> HashSet<Scope> {
        self.0.read().unwrap().vars.required_scopes.clone()
    }
}

impl From<Configuration> for AppState {
    fn from(configuration: Configuration) -> Self {
        let state = State {
            auth_sessions: Default::default(),
            rng: StdRng::seed_from_u64(configuration.rng_seed),
            user: configuration.user.clone(),
            vars: configuration.into(),
        };
        Self(Arc::new(RwLock::new(state)))
    }
}
