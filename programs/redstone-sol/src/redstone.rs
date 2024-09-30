use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;

pub fn recover_address(
    message: &[u8],
    signature: &[u8],
) -> Result<SignerAddress> {
    let recovery_byte = signature[64];
    let recovery_id =
        recovery_byte - (if recovery_byte >= 27 { 27 } else { 0 });
    let msg_hash = keccak256(message);
    let res = secp256k1_recover(&msg_hash, recovery_id, &signature[..64]);
    match res {
        Ok(pubkey) => {
            let key_hash = keccak256(&pubkey.to_bytes()[1..]);
            Ok(key_hash[12..].try_into().unwrap())
        }
        Err(_e) => {
            #[cfg(feature = "dev")]
            msg!("Invalid signature: {:?}: {:?}", signature, _e);
            Err(RedstoneError::InvalidSignature.into())
        }
    }
}

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    anchor_lang::solana_program::keccak::hash(data).to_bytes()
}
