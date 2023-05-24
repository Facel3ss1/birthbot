use mongodm::prelude::*;

use crate::prelude::{utils::*, *};

#[poise::command(slash_command, guild_only)]
pub async fn set(
    context: BotContext<'_>,
    #[description = "Your birthday."] birthday: Birthday,
) -> BotResult<()> {
    // Defer the response to allow time for query execution
    context.defer_or_broadcast().await?;

    let user_id = context.author().id;
    let guild_id = context.guild_id().unwrap(); // PANICS: Will always exist as the command is guild-only

    // Insert or update the member's birthday
    let member_repo = context.data().database.repository::<MemberData>();
    let birthday = member_repo
        .find_one_and_update(
            doc! {
                field!(user_id in MemberData): user_id.to_bson()?,
                field!(guild_id in MemberData): guild_id.to_bson()?,
            },
            doc! {
                Set: {
                    field!(birthday in MemberData): birthday.to_bson()?,
                },
                SetOnInsert: {
                    field!(user_id in MemberData): user_id.to_bson()?,
                    field!(guild_id in MemberData): guild_id.to_bson()?,
                }
            },
            MongoFindOneAndUpdateOptions::builder().upsert(true).build(),
        )
        .await?
        .map(|member_data| member_data.birthday)
        .unwrap(); // PANICS: Will always exist as the document is upserted

    // Display the updated birthday
    utils::embed(&context, true, |embed| {
        embed
            .success()
            .description("Your birthday was successfully set.")
            .field("Birthday", birthday, true)
    })
    .await?;

    Ok(())
}
