use solana_program  :: {
    account_info::AccountInfo,
    entrypoint::{self, ProgramResult},
    pubkey::Pubkey
};
     
entrypoint!(process_instruction);

fn process_instruction (
    program_id:: &Pubkey,
    accounts:: &[Account_Info],
    instruction_data:: &[u8]
) -> ProgramResult {

    Ok(())
}