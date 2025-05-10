use std::sync::Arc;
use tokio_retry::{
    Retry,
    strategy::{ExponentialBackoff, jitter},
};

use teloxide::{
    Bot,
    dispatching::{Dispatcher, dialogue::InMemStorage},
    prelude::Requester,
};

use remind_bot::{State, reminder_core};

const RETRY_DELAY: u64 = 1000;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    tokio::spawn(reminder_core::run_remind_scheduler(bot.clone()));

    Dispatcher::builder(bot.clone(), remind_bot::schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .error_handler(Arc::new(move |err| error_handler(err, bot.clone())))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn error_handler(err: Box<(dyn std::error::Error + Send + Sync + 'static)>, bot: Bot) {
    let retry_strategy = ExponentialBackoff::from_millis(RETRY_DELAY)
        .map(jitter)
        .take(3);

    Retry::spawn(retry_strategy, || async {
        let text = format!("An error occurs: {}", err);
        eprintln!("{}", text);
        let admin_chat_id = "528357584".to_string();
        let _ = bot.send_message(admin_chat_id, text).await;
        Ok::<(), ()>(())
    });
}
