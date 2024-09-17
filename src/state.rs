use axum::http::Uri;
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

use crate::pkce::PkceCode;

#[derive(Debug, PartialEq)]
pub struct State {
    code_challenge: PkceCode,
    redirect_uri: Uri,
}

struct StateActor {
    receiver: mpsc::Receiver<Message>,
    codes: HashMap<String, State>,
}

enum Message {
    Put {
        client_id: String,
        code_challenge: PkceCode,
        redirect_uri: Uri,
        respond_to: oneshot::Sender<()>,
    },
    Get {
        client_id: String,
        respond_to: oneshot::Sender<Option<State>>,
    },
}

impl StateActor {
    fn new(receiver: mpsc::Receiver<Message>) -> Self {
        StateActor {
            receiver,
            codes: HashMap::default(),
        }
    }
    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Put {
                client_id,
                code_challenge,
                redirect_uri,
                respond_to,
            } => {
                let _ = self.codes.insert(
                    client_id,
                    State {
                        code_challenge,
                        redirect_uri,
                    },
                );
                respond_to
                    .send(())
                    .expect("Error while sendig message back");
            }
            Message::Get {
                client_id,
                respond_to,
            } => {
                respond_to
                    .send(self.codes.remove(&client_id))
                    .expect("Error while sendig message back");
            }
        }
    }
}

async fn run(mut actor: StateActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct StateActorHandle {
    sender: mpsc::Sender<Message>,
}

impl StateActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(1024);
        let actor = StateActor::new(receiver);
        tokio::spawn(run(actor));
        Self { sender }
    }

    pub async fn put_code_challenge(
        &self,
        client_id: String,
        code_challenge: PkceCode,
        redirect_uri: Uri,
    ) {
        let (send, recv) = oneshot::channel();
        let msg = Message::Put {
            client_id,
            code_challenge,
            redirect_uri,
            respond_to: send,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }

    pub async fn get_code_challenge(&self, client_id: String) -> Option<State> {
        let (send, recv) = oneshot::channel();
        let msg = Message::Get {
            client_id,
            respond_to: send,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn happy_path() {
        let client_id = "1234".to_string();
        let code_challenge =
            PkceCode::try_from("E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM").unwrap();
        let redirect_uri = Uri::from_static("http://prima.com/test");
        let actor_handle = StateActorHandle::new();
        let retrieved = actor_handle.get_code_challenge(client_id.clone()).await;
        assert!(retrieved.is_none(), "This id should be empty");
        actor_handle
            .put_code_challenge(
                client_id.clone(),
                code_challenge.clone(),
                redirect_uri.clone(),
            )
            .await;
        let retrieved = actor_handle.get_code_challenge(client_id.clone()).await;
        assert_eq!(
            Some(State {
                code_challenge,
                redirect_uri
            }),
            retrieved,
            "Retrieved state not matching with inserted one"
        );
        let retrieved = actor_handle.get_code_challenge(client_id.clone()).await;
        assert!(retrieved.is_none(), "This id should have been already consumed");
    }

    #[tokio::test]
    async fn overwrite_id() {
        let client_id = "1234".to_string();
        let code_challenge =
            PkceCode::try_from("E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM").unwrap();
        let redirect_uri = Uri::from_static("http://prima.com/test");
        let actor_handle = StateActorHandle::new();
        actor_handle
            .put_code_challenge(
                client_id.clone(),
                code_challenge.clone(),
                redirect_uri.clone(),
            )
            .await;
        let new_challenge = PkceCode::try_from("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk").unwrap();

        actor_handle.put_code_challenge(client_id.clone(), new_challenge.clone(), redirect_uri.clone()).await;
        let retrieved = actor_handle.get_code_challenge(client_id.clone()).await;
        assert_eq!(
            Some(State {
                code_challenge: new_challenge,
                redirect_uri
            }),
            retrieved,
            "Id was not overwritten"
        );
    }
}
