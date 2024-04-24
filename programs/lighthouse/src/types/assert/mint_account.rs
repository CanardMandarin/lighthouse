use super::{Assert, LogLevel};
use crate::error::LighthouseError;
use crate::utils::unpack_coption_key;
use crate::{
    types::assert::evaluate::{EquatableOperator, Evaluate, IntegerOperator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

// Should be externalized somehere else
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct OptionalPubkey {
    value: Option<Pubkey>,
    operator: EquatableOperator,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct U64 {
    value: u64,
    operator: IntegerOperator,
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct U8 {
    value: u8,
    operator: IntegerOperator,
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Bool {
    value: bool,
    operator: EquatableOperator,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum MintAccountAssertion {
    MintAuthority(OptionalPubkey),
    Supply(U64),
    Decimals(U8),
    IsInitialized(Bool),
    FreezeAuthority(OptionalPubkey),
}

pub trait Evaluator {
    fn get_range_for_field(&self) -> std::ops::Range<usize>;
    fn evaluate2(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()>;
}

impl Evaluator for MintAccountAssertion {
    fn get_range_for_field(&self) -> std::ops::Range<usize> {
        match *self {
            MintAccountAssertion::MintAuthority { .. } => 0..36,
            MintAccountAssertion::Supply { .. } => 36..44,
            MintAccountAssertion::Decimals { .. } => 44..45,
            MintAccountAssertion::IsInitialized { .. } => 45..46,
            MintAccountAssertion::FreezeAuthority { .. } => 46..82,
        }
    }

    fn evaluate2(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()> {
        let range = self.get_range_for_field();

        let data = account
            .try_borrow_mut_data()
            .map_err(LighthouseError::failed_borrow_err)?;

        let data_slice = data
            .get(range.clone())
            .ok_or_else(|| LighthouseError::oob_err(range))?;

        match self {
            MintAccountAssertion::MintAuthority(inner)
            | MintAccountAssertion::FreezeAuthority(inner) => <Option<&Pubkey>>::evaluate(
                &unpack_coption_key(data_slice)?,
                &inner.value.as_ref(),
                &inner.operator,
                log_level,
            ),
            // TODO: handle other variants
            MintAccountAssertion::Supply(_) => todo!(),
            MintAccountAssertion::Decimals(_) => todo!(),
            MintAccountAssertion::IsInitialized(_) => todo!(),
        }
    }
}

impl Assert<&AccountInfo<'_>> for MintAccountAssertion {
    fn evaluate(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()> {
        Ok(())
    }
}
