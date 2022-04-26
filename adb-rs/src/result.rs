use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdbError {
    #[error("io error: {}", _0)]
    Io(std::io::Error),

    #[error("data crc mismatch")]
    Crc,

    #[error("auth not supported")]
    AuthNotSupported,

    #[error("unknown command: {:x}", _0)]
    UnknownCommand(u32),

    #[error("unexpected command: {:?}", _0)]
    UnexpectedCommand(crate::message::Command),

    #[error("unexpected data: {:?}", _0)]
    UnexpectedData(Vec<u8>),

    #[error("disconnected")]
    Disconnected,

    #[error("fail: {}", _0)]
    Fail(String),
}

impl AdbError {
    pub fn from_unexpected_command_u32(cmd: u32) -> Self {
        use crate::message::Command;
        use num_traits::FromPrimitive;
        if let Some(cmd) = Command::from_u32(cmd) {
            AdbError::UnexpectedCommand(cmd)
        } else {
            AdbError::UnknownCommand(cmd)
        }
    }
}

pub type AdbResult<T> = Result<T, AdbError>;

impl From<::std::io::Error> for AdbError {
    fn from(err: ::std::io::Error) -> AdbError {
        AdbError::Io(err)
    }
}
