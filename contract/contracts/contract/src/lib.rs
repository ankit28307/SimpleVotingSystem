#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Map, String};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // Initialize proposal with zero votes
    pub fn create_proposal(env: Env, proposal: Symbol) {
        let mut proposals: Map<Symbol, u32> =
            env.storage().instance().get(&symbol_short!("PROPS")).unwrap_or(Map::new(&env));

        if proposals.contains_key(proposal.clone()) {
            panic!("Proposal already exists");
        }

        proposals.set(proposal.clone(), 0);
        env.storage().instance().set(&symbol_short!("PROPS"), &proposals);
    }

    // Vote for a proposal
    pub fn vote(env: Env, proposal: Symbol) {
        let mut proposals: Map<Symbol, u32> =
            env.storage().instance().get(&symbol_short!("PROPS")).unwrap_or(Map::new(&env));

        if !proposals.contains_key(proposal.clone()) {
            panic!("Proposal does not exist");
        }

        let count = proposals.get(proposal.clone()).unwrap();
        proposals.set(proposal.clone(), count + 1);

        env.storage().instance().set(&symbol_short!("PROPS"), &proposals);
    }

    // Get vote count
    pub fn get_votes(env: Env, proposal: Symbol) -> u32 {
        let proposals: Map<Symbol, u32> =
            env.storage().instance().get(&symbol_short!("PROPS")).unwrap_or(Map::new(&env));

        proposals.get(proposal).unwrap_or(0)
    }
}