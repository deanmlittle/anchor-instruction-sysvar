use anchor_lang::error::ErrorCode;
use anchor_lang::prelude::Error;
use anchor_lang::{Accounts, AccountsExit, Key, Result, ToAccountInfos, ToAccountMetas, Id, AccountDeserialize};
use solana_program::account_info::AccountInfo;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use std::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Clone)]
pub struct NonExecutableProgram<'info, T> {
    info: AccountInfo<'info>,
    _phantom: PhantomData<T>,
}

impl<'info, T: fmt::Debug> fmt::Debug for NonExecutableProgram<'info, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NonExecutableProgram").field("info", &self.info).finish()
    }
}

impl<'a, T> NonExecutableProgram<'a, T> {
    pub(crate) fn new(info: AccountInfo<'a>) -> NonExecutableProgram<'a, T> {
        Self {
            info,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: Id> TryFrom<&AccountInfo<'a>> for NonExecutableProgram<'a, T> {
    type Error = Error;
    /// Deserializes the given `info` into a `Program`.
    fn try_from(info: &AccountInfo<'a>) -> Result<Self> {
        if info.key != &T::id() {
            return Err(Error::from(ErrorCode::InvalidProgramId).with_pubkeys((*info.key, T::id())));
        }
        if info.executable {
            return Err(ErrorCode::ConstraintExecutable.into());
        }

        Ok(NonExecutableProgram::new(info.clone()))
    }
}

impl<'info, T: Id> Accounts<'info> for NonExecutableProgram<'info, T> {
    #[inline(never)]
    fn try_accounts(
        _program_id: &Pubkey,
        accounts: &mut &[AccountInfo<'info>],
        _ix_data: &[u8],
        _bumps: &mut BTreeMap<String, u8>,
        _reallocs: &mut BTreeSet<Pubkey>,
    ) -> Result<Self> {
        if accounts.is_empty() {
            return Err(ErrorCode::AccountNotEnoughKeys.into());
        }
        let account = &accounts[0];
        *accounts = &accounts[1..];
        NonExecutableProgram::try_from(account)
    }
}

impl<'info, T> ToAccountMetas for NonExecutableProgram<'info, T> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.info.is_signer);
        let meta = match self.info.is_writable {
            false => AccountMeta::new_readonly(*self.info.key, is_signer),
            true => AccountMeta::new(*self.info.key, is_signer),
        };
        vec![meta]
    }
}

impl<'info, T> ToAccountInfos<'info> for NonExecutableProgram<'info, T> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.info.clone()]
    }
}

impl<'info, T> AsRef<AccountInfo<'info>> for NonExecutableProgram<'info, T> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        &self.info
    }
}

impl<'info, T> Deref for NonExecutableProgram<'info, T> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl<'info, T> AccountsExit<'info> for NonExecutableProgram<'info, T> {}

impl<'info, T: AccountDeserialize> Key for NonExecutableProgram<'info, T> {
    fn key(&self) -> Pubkey {
        *self.info.key
    }
}