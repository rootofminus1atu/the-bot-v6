use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("You can use this command only in guilds.")]
    GuildOnly,
    #[error("Some database error occurred.")]
    SomeDbError
}