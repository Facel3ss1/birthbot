use crate::prelude::*;

mod get;
pub use get::*;

mod set;
pub use set::*;

mod unset;
pub use unset::*;

/// Parent command for all birthday channel-related subcommands.
///
/// This command cannot actually be called by itself - it requires a subcommand.
#[poise::command(
    slash_command,
    subcommands("get", "set", "unset"),
    guild_only,
    on_error = "util::report_framework_error"
)]
pub async fn channel(_: BotContext<'_>) -> BotResult<()> {
    unreachable!() // PANICS: Will never be reached as the command is slash-only, and parent slash commands cannot be called without subcommands
}
