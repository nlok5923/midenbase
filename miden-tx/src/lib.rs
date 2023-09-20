use assembly::{
    ast::{ModuleAst, ProgramAst},
    Assembler, AssemblyContext, AssemblyError,
};
use crypto::{hash::rpo::Rpo256 as Hasher, hash::rpo::RpoDigest as Digest, merkle::NodeIndex};
use miden_lib::{MidenLib, SatKernel};
use miden_objects::{
    accounts::{Account, AccountCode, AccountId},
    notes::{Note, NoteOrigin, NoteScript},
    transaction::{PreparedTransaction, TransactionResult},
    AccountError, BlockHeader, ChainMmr, TransactionResultError,
};
use miden_stdlib::StdLibrary;
use vm_core::{code_blocks::CodeBlock, utils::collections::BTreeMap, Operation, Program};
use vm_processor::{ExecutionError, RecAdviceProvider};

mod compiler;
pub use compiler::{NoteTarget, TransactionComplier};

mod data;
pub use data::DataStore;

mod executor;
pub use executor::TransactionExecutor;

mod prover;
pub use prover::TransactionProver;

mod result;
pub use result::TryFromVmResult;

mod verifier;
pub use verifier::TransactionVerifier;

mod error;
pub use error::{
    DataStoreError, TransactionCompilerError, TransactionError, TransactionExecutorError,
    TransactionProverError, TransactionVerifierError,
};

#[cfg(test)]
mod tests;
