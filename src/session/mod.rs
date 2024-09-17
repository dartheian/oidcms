use crate::parameter::{code::Code, pkce::CodeChallenge, ClientId, Scope};
use axum::http::Uri;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct AuthSession {
    pub client_id: ClientId,
    pub code_challenge: CodeChallenge,
    pub redirect_uri: Uri,
    pub scope: HashSet<Scope>,
}

pub type Session = Arc<Mutex<HashMap<Code, AuthSession>>>;

pub fn get(s: Session, code: &Code) -> Option<AuthSession> {
    s.lock().unwrap().remove(code)
}

pub fn set(s: &Session, code: Code, session: AuthSession) {
    s.lock().unwrap().insert(code, session);
}
