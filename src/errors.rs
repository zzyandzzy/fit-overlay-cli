#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Environment variable not found: {0}")]
    VarNotFoundError(String),

    #[error("Args error: {0}")]
    ArgsError(String),

    #[error("IO error, {0:?}")]
    IOError(#[from] std::io::Error),

    #[error("FFmpeg error, {0:?}")]
    FFmpegError(#[from] ffmpeg_sidecar::error::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
