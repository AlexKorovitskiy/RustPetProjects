use chrono::{DateTime, Utc};

use once_cell::sync::Lazy;

use std::{collections::HashMap, sync::Arc};
use tokio::time::{self, Duration};

use teloxide::{Bot, prelude::Requester, types::ChatId};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct Reminder {
    pub description: String,
    pub date_time: DateTime<chrono::Utc>,
    pub chat_id: ChatId,
}

static LIST: Lazy<Arc<tokio::sync::Mutex<HashMap<ChatId, HashMap<u32, Reminder>>>>> =
    Lazy::new(|| Arc::new(tokio::sync::Mutex::new(HashMap::new())));
static COUNTER: Lazy<Arc<tokio::sync::Mutex<u32>>> =
    Lazy::new(|| Arc::new(tokio::sync::Mutex::new(0)));

const INTERVAL_TO_CHECK_REMINDERS: u64 = 60;

pub async fn remove_reminder_by_id(chat_id: &ChatId, reminder_id: &u32) -> HandlerResult {
    let mutex_rf = Arc::clone(&LIST);
    let mut list = mutex_rf.lock().await;
    let users_chats = list.get_mut(&chat_id).ok_or(format!(
        "Chat not found. chat_id: '{}', reminder_id: '{}'",
        chat_id, reminder_id
    ))?;
    users_chats.remove(&reminder_id).ok_or(format!(
        "Attempt to remove unexisted item. chat_id: '{}', reminder_id: '{}'",
        chat_id, reminder_id
    ))?;
    Ok(())
}

pub async fn create_reminder(
    description: String,
    execution_time: DateTime<chrono::Utc>,
    chat_id: ChatId,
) -> HandlerResult {
    let mut list = LIST.lock().await;
    let reminder = Reminder {
        chat_id: chat_id,
        date_time: execution_time,
        description: description,
    };
    let counter_mutex = Arc::clone(&COUNTER);
    let mut counter = counter_mutex.lock().await;
    *counter += 1;

    let user_chats = list.entry(chat_id).or_insert_with(|| HashMap::new());
    user_chats.insert(*counter, reminder);

    Ok(())
}

pub async fn get_reminders_list(chat_id: ChatId) -> HashMap<u32, Reminder> {
    let mutex = Arc::clone(&LIST);

    let list = mutex.lock().await;

    let list = match list.get(&chat_id) {
        Some(i) => i.clone(),
        None => HashMap::new(),
    };

    list
}

pub async fn run_remind_scheduler(bot: Bot) {
    let mut interval = time::interval(Duration::from_secs(INTERVAL_TO_CHECK_REMINDERS));
    loop {
        interval.tick().await;
        let mutex_rf = Arc::clone(&LIST);
        let mut list_to_remove_reminders = vec![];
        {
            let list = mutex_rf.lock().await;

            for (chat_id, reminders) in list.iter() {
                for (reminder_id, reminder) in reminders {
                    if reminder.date_time <= Utc::now() {
                        let _ = match bot.send_message(*chat_id, &reminder.description).await {
                            Ok(msg) => msg,
                            Err(err) => {
                                eprintln!(
                                    "ErrorMessage: {} Error wile sending message '{}' to {}",
                                    err, reminder.description, chat_id
                                );
                                break;
                            }
                        };

                        list_to_remove_reminders.push((*chat_id, *reminder_id));
                    }
                }
            }
        }

        for (chat_id, reminder_id) in list_to_remove_reminders.iter() {
            let _ = match remove_reminder_by_id(chat_id, reminder_id).await {
                Err(err) => {
                    eprintln!("{}", err)
                }
                _ => println!("removed"),
            };
        }
    }
}
