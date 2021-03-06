use std::{io, fmt, error};
use cardano::block::{self, HeaderHash};
use cardano_storage;
use cbor_event;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),

    NewCannotInitializeBlockchainDirectory(cardano_storage::Error),

    ListNoBlockchains,
    ListPermissionsDenied,
    ListBlockchainInvalidName(::storage_units::utils::directory_name::DirectoryNameError),

    ForwardHashDoesNotExist(HeaderHash),

    GetBlockDoesNotExist(HeaderHash),
    GetInvalidBLock(HeaderHash),

    CatMalformedBlock(cbor_event::Error),

    VerifyInvalidBlock(block::Error),
    VerifyMalformedBlock(cbor_event::Error),

    VerifyChainGenesisHashNotFound(HeaderHash),
    VerifyChainInvalidGenesisPrevHash(HeaderHash, HeaderHash), // (Expected, got)
    BlockchainIsNotValid(usize),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self { Error::IoError(e) }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(_) => write!(f, "I/O Error"),

            Error::NewCannotInitializeBlockchainDirectory(_) => write!(f, "Cannot Initialise the blockchain directory"),
            Error::ListNoBlockchains                         => write!(f, "No local blockchains yet"),
            Error::ListPermissionsDenied                     => write!(f, "No local blockchains (permission denied to the cardano-cli directory, check the `root-dir` option of the CLI)"),
            Error::ListBlockchainInvalidName(_)              => write!(f, "Blockchain with invalid name"),
            Error::ForwardHashDoesNotExist(hh)               => write!(f, "Cannot forward the blockchain to non existant hash `{}`", hh),
            Error::GetBlockDoesNotExist(hh)                  => write!(f, "Block `{}` does not exist", hh),
            Error::GetInvalidBLock(hh)                       => write!(f, "Block `{}` cannot be read from the local storage", hh),
            Error::CatMalformedBlock(_)                      => write!(f, "Unsupported or corrupted block"),
            Error::VerifyInvalidBlock(_)                     => write!(f, "Block is not valid"),
            Error::VerifyMalformedBlock(_)                   => write!(f, "Unsupported or corrupted block"),
            Error::VerifyChainGenesisHashNotFound(hh)        => write!(f, "Genesis data for given blockchain not found ({})", hh),
            Error::VerifyChainInvalidGenesisPrevHash(eh, hh) => write!(f, "Genesis data invalid: expected previous hash {} different from the one provided {}", eh, hh),
            Error::BlockchainIsNotValid(num_invalid_blocks)  => write!(f, "Blockchain has {} invalid blocks", num_invalid_blocks),
        }
    }
}
impl error::Error for Error {
    fn cause(&self) -> Option<& error::Error> {
        match self {
            Error::IoError(ref err) => Some(err),
            Error::NewCannotInitializeBlockchainDirectory(ref err) => Some(err),
            Error::ListBlockchainInvalidName(ref err) => Some(err),
            Error::CatMalformedBlock(ref err) => Some(err),
            Error::VerifyInvalidBlock(ref err) => Some(err),
            Error::VerifyMalformedBlock(ref err) => Some(err),
            _ => None
        }
    }
}
