use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct AuthorId;

impl TypeMapKey for AuthorId {
    type Value = Option<u64>;
}

macro_rules! handle_error {
    ($err: expr, $msg: expr) => {
        if let Err(reason) = $err {
            println!("{}: {:?}", reason, $msg);
        }
    };
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.write().await;
        let author_id_ref = data.get::<AuthorId>().unwrap();

        if msg.author.id.0 == author_id_ref.unwrap() {
            return;
        }

        println!("Received: {}", msg.content);
        if msg.content == "ping" {
            let result = msg.channel_id.say(&ctx.http, "pong").await;
            handle_error!(result, "Failed to send a message");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut data = ctx.data.write().await;
        let author_id_ref = data.get_mut::<AuthorId>().unwrap();
        *author_id_ref = Some(ready.user.id.0);

        println!("Ready: {}", ready.user.name);
    }
}

const TOKEN_NAME: &str = "PETROL_TOKEN";

#[tokio::main]
async fn main() {
    let error_msg = format!("Env variable {} must be present", TOKEN_NAME);
    let token = env::var(TOKEN_NAME).expect(&error_msg);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Failed to init the client");

    {
        let mut data = client.data.write().await;
        data.insert::<AuthorId>(None);
    }

    if let Err(reason) = client.start().await {
        println!("Client error: {:?}", reason);
    }
}
