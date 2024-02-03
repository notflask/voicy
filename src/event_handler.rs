use std::env;
use std::str::FromStr;

use serenity::all::{GuildId, Message, UserId};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use songbird::input::cached::Compressed;
use songbird::input::{File, Input};
use voicy_tts::{VoicyTTS, VoicyTTSOptions, VoicyTools};

use crate::sound::CachedSound;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if !msg.is_private() {
            return;
        }

        let owner_id: UserId = UserId::from_str(
            env::var("OWNER_ID")
                .expect("Failed to get OWNER_ID from .env")
                .as_str(),
        )
        .expect("Failed to get owner_id")
        .into();

        if !(msg.author.id == owner_id) {
            return;
        }

        let manager = songbird::get(&ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        let guild_id: GuildId = GuildId::from_str(
            env::var("GUILD_ID")
                .expect("Failed to get GUILD_ID from .env")
                .as_str(),
        )
        .expect("Failed to get guild_id")
        .into();

        if let Some(handler_lock) = manager.get(guild_id) {
            let mut handler = handler_lock.lock().await;

            let mut options = VoicyTTSOptions::default();
            options.set_filename("tmp/raw_message.mp3".to_string());
            options.set_language(voicy_tts::Language::Russian);
            options.set_text(msg.content);

            match VoicyTTS::to_file(&options) {
                Ok(_) => {}
                Err(why) => panic!("Error on generating mp3 from text: {:#?}", why),
            }

            VoicyTools::speed_up("tmp/raw_message.mp3", "tmp/message.mp3", 1.5);

            let source = Compressed::new(
                File::new("tmp/message.mp3").into(),
                songbird::driver::Bitrate::BitsPerSecond(32768),
            )
            .await
            .expect("These paramteres are well-defined");

            let source = CachedSound::Compressed(source);

            handler.play_input(Input::from(source));
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected to Discord Gateway", ready.user.name);
    }
}
