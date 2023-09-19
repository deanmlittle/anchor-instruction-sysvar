pub mod instructions;
pub use instructions::*;

pub mod non_executable_program;
pub use non_executable_program::*;

pub struct InstructionSysvar;

// Need to figure out what type this should be. It's *like* a program, but not executable 🤔
impl anchor_lang::Id for InstructionSysvar {
    fn id() -> solana_program::pubkey::Pubkey {
        solana_program::sysvar::instructions::ID
    }
}