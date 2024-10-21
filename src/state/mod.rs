use crate::config::Configuration;
use crate::parameter::{code::Code, pkce::CodeChallenge, ClientId, Scope};
use axum::extract::FromRequestParts;
use axum::http::Uri;
use derive_more::derive::AsRef;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

pub struct AuthSession {
    pub client_id: ClientId,
    pub code_challenge: CodeChallenge,
    pub redirect_uri: Uri,
    pub scope: HashSet<Scope>,
}

pub struct Vars {
    pub expiration: u64,
    pub issuer: Uri,
    pub secret: Vec<u8>,
}

impl From<Configuration> for Vars {
    fn from(config: Configuration) -> Self {
        Self {
            expiration: config.expiration,
            issuer: config.issuer,
            secret: config.secret,
        }
    }
}

pub struct State {
    auth_session: HashMap<Code, AuthSession>,
    rng: StdRng,
    vars: Vars,
}

#[derive(AsRef, Clone, FromRequestParts)]
#[as_ref(forward)]
#[from_request(via(axum::extract::State))]
pub struct AppState(Arc<RwLock<State>>);

impl AppState {
    pub fn get_session(&self, code: &Code) -> Option<AuthSession> {
        self.as_ref().write().unwrap().auth_session.remove(code)
    }

    pub fn set_session(&self, code: Code, session: AuthSession) {
        self.as_ref()
            .write()
            .unwrap()
            .auth_session
            .insert(code, session);
    }

    pub fn gen_random<T: FromRng>(&self) -> T {
        let mut lock = self.as_ref().write().unwrap();
        T::from_rng(&mut lock.rng)
    }

    pub fn expiration(&self) -> u64 {
        self.0.read().unwrap().vars.expiration
    }

    pub fn issuer(&self) -> Uri {
        self.0.read().unwrap().vars.issuer.clone()
    }

    pub fn secret(&self) -> Vec<u8> {
        self.0.read().unwrap().vars.secret.clone()
    }
}

impl From<Configuration> for AppState {
    fn from(configuration: Configuration) -> Self {
        let state = State {
            auth_session: Default::default(),
            rng: StdRng::seed_from_u64(configuration.rng_seed),
            vars: configuration.into(),
        };
        Self(Arc::new(RwLock::new(state)))
    }
}
pub trait FromRng {
    fn from_rng<R: Rng>(rng: &mut R) -> Self;
}
