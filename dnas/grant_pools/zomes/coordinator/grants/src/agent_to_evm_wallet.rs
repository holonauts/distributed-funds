use hdk::prelude::*;
use grants_integrity::*;
use alloy_primitives::Address;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddEvmWalletForAgentInput {
    pub agent: AgentPubKey,
    pub evm_wallet: Address,
}
#[hdk_extern]
pub fn add_agent_to_evm_wallet_for_agent(input: AddEvmWalletForAgentInput) -> ExternResult<()> {
    create_link(input.agent.clone(), input.agent, LinkTypes::AgentToEvmWallet, input.evm_wallet.to_vec())?;

    Ok(())    
}

#[hdk_extern]
pub fn get_agent_to_evm_wallets_for_agent(agent: AgentPubKey) -> ExternResult<Vec<String>> {
    let links = get_links(agent, LinkTypes::AgentToEvmWallet, None)?;
    
    let agent_to_evm_wallet: Vec<String> = links
        .into_iter()
        .map(|link| 
          String::from_utf8(link.tag.into_inner())
            .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Error converting link tag to string: {:?}", e))))
        )
        .collect::<ExternResult<Vec<String>>>()?;

    Ok(agent_to_evm_wallet)
}