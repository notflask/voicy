use serenity::{
    all::Message,
    client::Context,
    framework::standard::{macros::command, CommandResult},
};
use songbird::TrackEvent;

use crate::{util::check_msg, voice_event_handler::TrackErrorNotifier};

#[command]
#[owners_only]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild_id, channel_id) = {
        let guild = msg.guild(&ctx.cache).expect("Failed to get message guild");
        let channel_id = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialization")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);

        check_msg(
            msg.reply(
                ctx,
                format!(
                    "Joined to {}",
                    connect_to
                        .name(&ctx.http)
                        .await
                        .expect("Failed to get channel name")
                ),
            )
            .await,
        );
    }

    Ok(())
}
