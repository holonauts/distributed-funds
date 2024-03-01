use alloy_primitives::{Address, Signature, U256};
use hdi::prelude::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct Context(pub Vec<U256>);

impl Context {
    pub fn pack(&self) -> Vec<u8> {
        let res: Vec<u8> = self
            .0
            .clone()
            .into_iter()
            .flat_map(|c| c.to_be_bytes::<32>())
            .collect();
        res
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ClaimContext {
    pub context: Context,
    pub context_packed: Vec<u8>,
    pub context_packed_hash: Vec<u8>,
}

impl ClaimContext {
    ///  * [0] this contract address
    ///  * [1] application evm wallet
    ///  * [2] application ActionHash
    ///  * [3] application amount
    ///  * [4] total deposit pool amount
    pub fn new(
        flow_evm_address: Address,
        application_evm_wallet: Address,
        application_action_hash: ActionHash,
        application_amount: U256,
        total_deposit_amount: U256,
    ) -> ExternResult<Self> {
        // build context
        let context = Context(vec![
            U256::try_from(flow_evm_address.into_word())?,
            U256::try_from(application_evm_wallet.into_word())?,
            U256::from_be_slice(application_action_hash.get_raw_32()),
            application_amount,
            total_deposit_amount,
        ]);

        // pack context
        let context_packed = context.pack();

        // Hash packed context
        let context_packed_hash = hash_keccak256(context_packed.clone())?;

        Ok(Self {
            context,
            context_packed,
            context_packed_hash,
        })
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ClaimCoupon {
    pub context: Context,
    pub context_packed: Vec<u8>,
    pub context_packed_hash: Vec<u8>,

    // Result of GrantPool notary_evm_address signing the context_packed_hash
    pub signature: Signature,
}
