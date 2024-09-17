use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
struct State {
    code_challenge: String,
    redirect_uri: String,
}

struct StateActor {
    receiver: mpsc::Receiver<Message>,
    codes: HashMap<String, State>,
}

enum Message {
    Put {
        client_id: String,
        code_challenge: String,
        redirect_uri: String,
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
