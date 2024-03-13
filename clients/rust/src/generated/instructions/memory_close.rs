//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct MemoryClose {
    /// Lighthouse program
    pub program_id: solana_program::pubkey::Pubkey,
    /// Payer account
    pub payer: solana_program::pubkey::Pubkey,
    /// Memory account
    pub memory: solana_program::pubkey::Pubkey,
}

impl MemoryClose {
    pub fn instruction(
        &self,
        args: MemoryCloseInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: MemoryCloseInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program_id,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.memory,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = MemoryCloseInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::LIGHTHOUSE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct MemoryCloseInstructionData {
    discriminator: u8,
}

impl MemoryCloseInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MemoryCloseInstructionArgs {
    pub memory_id: u8,
    pub memory_bump: u8,
}

/// Instruction builder for `MemoryClose`.
///
/// ### Accounts:
///
///   0. `[]` program_id
///   1. `[writable, signer]` payer
///   2. `[writable]` memory
#[derive(Default)]
pub struct MemoryCloseBuilder {
    program_id: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    memory: Option<solana_program::pubkey::Pubkey>,
    memory_id: Option<u8>,
    memory_bump: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MemoryCloseBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Lighthouse program
    #[inline(always)]
    pub fn program_id(&mut self, program_id: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program_id = Some(program_id);
        self
    }
    /// Payer account
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// Memory account
    #[inline(always)]
    pub fn memory(&mut self, memory: solana_program::pubkey::Pubkey) -> &mut Self {
        self.memory = Some(memory);
        self
    }
    #[inline(always)]
    pub fn memory_id(&mut self, memory_id: u8) -> &mut Self {
        self.memory_id = Some(memory_id);
        self
    }
    #[inline(always)]
    pub fn memory_bump(&mut self, memory_bump: u8) -> &mut Self {
        self.memory_bump = Some(memory_bump);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = MemoryClose {
            program_id: self.program_id.expect("program_id is not set"),
            payer: self.payer.expect("payer is not set"),
            memory: self.memory.expect("memory is not set"),
        };
        let args = MemoryCloseInstructionArgs {
            memory_id: self.memory_id.clone().expect("memory_id is not set"),
            memory_bump: self.memory_bump.clone().expect("memory_bump is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `memory_close` CPI accounts.
pub struct MemoryCloseCpiAccounts<'a, 'b> {
    /// Lighthouse program
    pub program_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer account
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Memory account
    pub memory: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `memory_close` CPI instruction.
pub struct MemoryCloseCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Lighthouse program
    pub program_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer account
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Memory account
    pub memory: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: MemoryCloseInstructionArgs,
}

impl<'a, 'b> MemoryCloseCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: MemoryCloseCpiAccounts<'a, 'b>,
        args: MemoryCloseInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            program_id: accounts.program_id,
            payer: accounts.payer,
            memory: accounts.memory,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program_id.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.memory.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = MemoryCloseInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LIGHTHOUSE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.program_id.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.memory.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `MemoryClose` via CPI.
///
/// ### Accounts:
///
///   0. `[]` program_id
///   1. `[writable, signer]` payer
///   2. `[writable]` memory
pub struct MemoryCloseCpiBuilder<'a, 'b> {
    instruction: Box<MemoryCloseCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MemoryCloseCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MemoryCloseCpiBuilderInstruction {
            __program: program,
            program_id: None,
            payer: None,
            memory: None,
            memory_id: None,
            memory_bump: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Lighthouse program
    #[inline(always)]
    pub fn program_id(
        &mut self,
        program_id: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program_id = Some(program_id);
        self
    }
    /// Payer account
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// Memory account
    #[inline(always)]
    pub fn memory(
        &mut self,
        memory: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.memory = Some(memory);
        self
    }
    #[inline(always)]
    pub fn memory_id(&mut self, memory_id: u8) -> &mut Self {
        self.instruction.memory_id = Some(memory_id);
        self
    }
    #[inline(always)]
    pub fn memory_bump(&mut self, memory_bump: u8) -> &mut Self {
        self.instruction.memory_bump = Some(memory_bump);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = MemoryCloseInstructionArgs {
            memory_id: self
                .instruction
                .memory_id
                .clone()
                .expect("memory_id is not set"),
            memory_bump: self
                .instruction
                .memory_bump
                .clone()
                .expect("memory_bump is not set"),
        };
        let instruction = MemoryCloseCpi {
            __program: self.instruction.__program,

            program_id: self.instruction.program_id.expect("program_id is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            memory: self.instruction.memory.expect("memory is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct MemoryCloseCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    program_id: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    memory: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    memory_id: Option<u8>,
    memory_bump: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
