use crate::InstructionsData;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::serialize_utils::read_u16;
use solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked,
};

pub fn load_instruction_data<'a>(
    instruction_sysvar_account: &'a AccountInfo<'a>,
) -> Result<InstructionsData<'a>, ProgramError> {
    let data = instruction_sysvar_account.try_borrow_data()?;
    let current_index = load_current_index_checked(instruction_sysvar_account)?;
    let instruction =
        load_instruction_at_checked(current_index as usize, instruction_sysvar_account)?;
    let mut current = 0;
    let num_of_instructions= match read_u16(&mut current, &**data) {
        Ok(index) => {
            index
        }
        Err(_) => {
            return Err(ProgramError::InvalidArgument);
            // return Err(ReadingNumInstructionsError);
        }
    };

    Ok(InstructionsData {
        current_program: instruction.program_id,
        current_index,
        num_of_instructions,
        instruction_sysvar_account,
    })
}
