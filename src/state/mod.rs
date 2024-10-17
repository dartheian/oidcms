use crate::parameter::{code::Code, pkce::CodeChallenge, ClientId, Scope};
use axum::extract::FromRequestParts;
use axum::http::Uri;
use derive_more::derive::AsRef;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct AuthSession {
    pub client_id: ClientId,
    pub code_challenge: CodeChallenge,
    pub redirect_uri: Uri,
    pub scope: HashSet<Scope>,
}

pub struct State {
    auth_session: HashMap<Code, AuthSession>,
    rng: StdRng,
}

#[derive(AsRef, Clone, FromRequestParts)]
#[as_ref(forward)]
#[from_request(via(axum::extract::State))]
pub struct AppState(Arc<Mutex<State>>);

impl AppState {
    pub fn new(rng_seed: u64) -> Self {
        let state = State {
            auth_session: Default::default(),
            rng: StdRng::seed_from_u64(rng_seed),
        };
        Self(Arc::new(Mutex::new(state)))
    }
}

pub trait FromRng {
    fn from_rng<R: Rng>(rng: &mut R) -> Self;
}
impl AppState {
    pub fn get(&self, code: &Code) -> Option<AuthSession> {
        self.as_ref().lock().unwrap().auth_session.remove(code)
    }

    pub fn set(&self, code: Code, session: AuthSession) {
        self.as_ref()
            .lock()
            .unwrap()
            .auth_session
            .insert(code, session);
    }

    pub fn random<T: FromRng>(&self) -> T {
        let mut lock = self.as_ref().lock().unwrap();
        T::from_rng(&mut lock.rng)
    }
}
