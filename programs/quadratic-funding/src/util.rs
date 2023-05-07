use anchor_lang::prelude::*;
use anchor_lang::system_program;

pub const MAX_NAME_LEN: usize = 50;

pub fn set_and_maybe_realloc<'info, T>(
    account: &mut Account<'info, T>,
    new_data: T,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()>
where
    T: AccountDeserialize
        + AccountSerialize
        + borsh::BorshDeserialize
        + borsh::BorshSerialize
        + Clone,
{
    let account_info = account.to_account_info();

    // See if it needs to be reallocated
    let new_account_size = (new_data.try_to_vec()?).len();
    if new_account_size > account_info.data_len() {
        // Determine additional rent required
        let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
        let additional_rent_to_fund = lamports_required - account_info.lamports();

        // Perform transfer of additional rent
        system_program::transfer(
            CpiContext::new(
                system_program,
                system_program::Transfer {
                    from: payer,
                    to: account_info.clone(),
                },
            ),
            additional_rent_to_fund,
        )?;

        // Serialize new data
        account_info.realloc(new_account_size, false)?;
    }
    account.set_inner(new_data);
    Ok(())
}
