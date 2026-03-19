#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct Escrow;

#[contractimpl]
impl Escrow {

    // Initialize escrow
    pub fn init(env: Env, buyer: Address, seller: Address, amount: i128) {
        buyer.require_auth();

        env.storage().instance().set(&Symbol::new(&env, "buyer"), &buyer);
        env.storage().instance().set(&Symbol::new(&env, "seller"), &seller);
        env.storage().instance().set(&Symbol::new(&env, "amount"), &amount);
    }

    // Release funds to seller (approved by buyer)
    pub fn release(env: Env) {
        let buyer: Address = env.storage().instance().get(&Symbol::new(&env, "buyer")).unwrap();
        let seller: Address = env.storage().instance().get(&Symbol::new(&env, "seller")).unwrap();

        buyer.require_auth();

        // NOTE: Token transfer logic should go here

        env.storage().instance().remove(&Symbol::new(&env, "buyer"));
        env.storage().instance().remove(&Symbol::new(&env, "seller"));
        env.storage().instance().remove(&Symbol::new(&env, "amount"));
    }

    // Refund buyer (approved by seller)
    pub fn refund(env: Env) {
        let seller: Address = env.storage().instance().get(&Symbol::new(&env, "seller")).unwrap();
        let buyer: Address = env.storage().instance().get(&Symbol::new(&env, "buyer")).unwrap();

        seller.require_auth();

        // NOTE: Token refund logic should go here

        env.storage().instance().remove(&Symbol::new(&env, "buyer"));
        env.storage().instance().remove(&Symbol::new(&env, "seller"));
        env.storage().instance().remove(&Symbol::new(&env, "amount"));
    }

    // View escrow details
    pub fn get_details(env: Env) -> (Address, Address, i128) {
        let buyer: Address = env.storage().instance().get(&Symbol::new(&env, "buyer")).unwrap();
        let seller: Address = env.storage().instance().get(&Symbol::new(&env, "seller")).unwrap();
        let amount: i128 = env.storage().instance().get(&Symbol::new(&env, "amount")).unwrap();

        (buyer, seller, amount)
    }
}