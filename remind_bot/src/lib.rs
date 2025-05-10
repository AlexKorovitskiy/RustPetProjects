use chrono::Utc;

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tokio::time::Duration;

use teloxide::{
    Bot,
    dispatching::{
        UpdateFilterExt, UpdateHandler,
        dialogue::{self, InMemStorage},
    },
    payloads::SendMessageSetters,
    prelude::{Dialogue, Requester},
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message, Update},
    utils::command::BotCommands,
};

pub mod reminder_core;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    GettingReminderTime,
    GettingReminderDescription {
        time_delta: Duration,
    },
    Removing,
}

#[derive(BotCommands, Clone, EnumIter, Debug, Display)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Available commands")]
    Help,
    #[command(description = "Create reminder")]
    Remind,
    #[command(description = "List of reminders")]
    List,
    #[command(description = "Remove reminder")]
    Remove,
    #[command(description = "Cancel")]
    Cancel,
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Remind].endpoint(remind))
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::List].endpoint(send_reminders_to_chat))
                .branch(case![Command::Remove].endpoint(remove_reminder_start)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::Start].branch(dptree::endpoint(receive_product_selection)));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::GettingReminderTime].endpoint(apply_reminder_time))
        .branch(
            case![State::GettingReminderDescription { time_delta }]
                .endpoint(apply_reminder_description),
        )
        .branch(case![State::Removing].endpoint(remove_reminder))
        .branch(dptree::endpoint(invalid_data));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

pub async fn remove_reminder(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let str = msg.text().ok_or("Error during getting message text.")?;
    let reminder_id = str.parse()?;
    reminder_core::remove_reminder_by_id(&dialogue.chat_id(), &reminder_id).await?;

    let str = format!("Reminder removded. Id: {}", str);
    let _ = bot.send_message(dialogue.chat_id(), str).await?;
    let _ = dialogue.update(State::Start).await?;

    Ok(())
}

pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = dialogue.exit().await?;
    let text = "The state has been reset";
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

pub async fn remind(bot: Bot, dialogue: MyDialogue) -> HandlerResult {
    //TODO
    println!("Remind");
    let text = "Input reminder time in format 'mm:ss'";
    bot.send_message(dialogue.chat_id(), text).await?;
    dialogue.update(State::GettingReminderTime).await?;

    Ok(())
}

pub async fn apply_reminder_description(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    time_delta: Duration,
) -> HandlerResult {
    println!(
        "apply_reminder_description. {} secs",
        time_delta.as_secs().to_string()
    );

    let description = msg.text().ok_or("Empty text")?.to_string();
    let execution_time = Utc::now() + time_delta;
    reminder_core::create_reminder(description, execution_time, dialogue.chat_id()).await?;

    dialogue.update(State::Start).await?;

    let text = "Reminder was created";
    bot.send_message(dialogue.chat_id(), text).await?;

    Ok(())
}

pub async fn invalid_data(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let text = msg.text().ok_or("Message is empty.")?;
    println!("Invalid data: {}", text);

    Ok(())
}

pub async fn send_reminders_to_chat(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let reminders = reminder_core::get_reminders_list(dialogue.chat_id()).await;

    for (reminder_id, reminder) in reminders.iter() {
        let text = format!(
            "Id: {} Description: {} DateTime: {}",
            reminder_id,
            reminder.description,
            reminder.date_time.to_string().as_str()
        );
        bot.send_message(dialogue.chat_id(), text).await?;
    }

    Ok(())
}

pub async fn remove_reminder_start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let text = "Input remivder id to remove";
    bot.send_message(dialogue.chat_id(), text).await?;
    dialogue.update(State::Removing).await?;

    Ok(())
}

pub async fn help(bot: Bot, dialogue: MyDialogue) -> HandlerResult {
    println!("Help");

    let inline_keyboard = Command::iter().map(|comand| {
        [InlineKeyboardButton::callback(
            comand.to_string(),
            comand.to_string(),
        )]
    });

    let buttons = InlineKeyboardMarkup::new(inline_keyboard);

    bot.send_message(dialogue.chat_id(), "Choose option:")
        .reply_markup(buttons)
        .await?;

    Ok(())
}

pub async fn apply_reminder_time(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let text = msg.text().ok_or("Empty message")?;
    println!("apply_reminder_time. {}", text);
    let secs = text.parse::<u64>()?;
    let duration = Duration::new(secs, 0);
    let state = State::GettingReminderDescription {
        time_delta: duration,
    };

    dialogue.update(state).await?;
    let text = "Input reminder description";
    bot.send_message(dialogue.chat_id(), text).await?;

    Ok(())
}

pub async fn receive_product_selection(
    bot: Bot,
    dialogue: MyDialogue,
    callback_query: CallbackQuery,
) -> HandlerResult {
    println!("receive_product_selection");

    let command = callback_query.data.ok_or("Empty data")?;
    match command.as_str() {
        "Remind" => remind(bot, dialogue).await?,
        "Help" => help(bot, dialogue).await?,
        others => {
            println!("{}", others);
            todo!();
        }
    };

    Ok(())
}
