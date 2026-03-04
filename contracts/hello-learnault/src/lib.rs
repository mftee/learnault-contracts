#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

/// HelloLearnault contract - A simple greeting contract for the Learnault platform.
/// This contract demonstrates the basic structure of a Soroban smart contract
/// and serves as a CI/CD test contract.
#[contract]
pub struct HelloLearnault;

#[contractimpl]
impl HelloLearnault {
    /// Returns a personalized greeting message.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `name` - The name to include in the greeting
    ///
    /// # Returns
    /// A vector containing the greeting message parts
    pub fn hello(env: Env, name: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), symbol_short!("Learnault"), name]
    }

    /// Returns a welcome message for new users.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `user` - The user to welcome
    ///
    /// # Returns
    /// A vector containing the welcome message parts
    pub fn welcome(env: Env, user: Symbol) -> Vec<Symbol> {
        vec![
            &env,
            symbol_short!("Welcome"),
            symbol_short!("to"),
            symbol_short!("Learnault"),
            user,
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, vec, Env};

    #[test]
    fn test_hello() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloLearnault);
        let client = HelloLearnaultClient::new(&env, &contract_id);

        let result = client.hello(&symbol_short!("Dev"));
        assert_eq!(
            result,
            vec![
                &env,
                symbol_short!("Hello"),
                symbol_short!("Learnault"),
                symbol_short!("Dev")
            ]
        );
    }

    #[test]
    fn test_welcome() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloLearnault);
        let client = HelloLearnaultClient::new(&env, &contract_id);

        let result = client.welcome(&symbol_short!("Alice"));
        assert_eq!(
            result,
            vec![
                &env,
                symbol_short!("Welcome"),
                symbol_short!("to"),
                symbol_short!("Learnault"),
                symbol_short!("Alice")
            ]
        );
    }
}
