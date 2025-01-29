use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

pub struct AllowedRules {
    pub program: Pubkey,
    pub before: bool,
    pub after: bool,
}

pub struct InstructionsData<'a> {
    pub current_program: Pubkey,
    pub current_index: u16,
    pub num_of_instructions: u16,
    pub instruction_sysvar_account: &'a AccountInfo<'a>,
}
