mod errors;
mod structs;
mod utils;

use crate::structs::{AllowedRules, InstructionsData};
use crate::utils::load_instruction_data;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::sysvar::instructions::load_instruction_at_checked;

pub fn allowed_only<'a>(
    instruction_sysvar_account: &'a AccountInfo<'a>,
    rules: Vec<AllowedRules>,
) -> Result<bool, ProgramError> {
    let instruction_data = load_instruction_data(instruction_sysvar_account)?;

    for index in 0..instruction_data.num_of_instructions {
        let instruction = load_instruction_at_checked(index as usize, instruction_sysvar_account)?;
        let rule: &AllowedRules = match rules
            .iter()
            .find(|rule| rule.program == instruction.program_id)
        {
            Some(r) => r,
            None => {
                return Ok(false);
            }
        };

        if !rule.before && index > instruction_data.current_index {
            return Ok(false);
        }
        if !rule.after && index < instruction_data.current_index {
            return Ok(false);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
