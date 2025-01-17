use crate::*;

pub fn join_game(
    client: &Client,
    user: Pubkey,
    game: Pubkey,
    color: chess::Color,
) -> ClientResult<()> {
    let join_game_ix = Instruction {
        program_id: chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new(game, false),
        ],
        data: chess::instruction::JoinGame { color }.data(),
    };

    send_and_confirm_tx(
        &client,
        [join_game_ix].to_vec(),
        None,
        "join_game".to_string(),
    )?;

    Ok(())
}