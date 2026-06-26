use soroban_sdk::{Address, Env, Map, Vec};
use crate::{ContractData, ContractError, DATA_KEY, SIGNERS_KEY};

pub fn require_multisig(env: &Env, signers: &Vec<Address>) -> Result<(), ContractError> {
    let authorized_signers: Map<Address, ()> = env
        .storage()
        .instance()
        .get(&SIGNERS_KEY)
        .unwrap_or_else(|| Map::new(env));
        
    let data: ContractData = env
        .storage()
        .instance()
        .get(&DATA_KEY)
        .ok_or(ContractError::NotInitialized)?;

    let mut valid_count = 0;

    for i in 0..signers.len() {
        let signer = signers.get(i).unwrap();
        let is_authorized = authorized_signers.contains_key(signer.clone()) || data.admin == signer;
        
        if is_authorized {
            let mut duplicate = false;
            for j in 0..i {
                if signers.get(j).unwrap() == signer {
                    duplicate = true;
                    break;
                }
            }
            
            if !duplicate {
                signer.require_auth();
                valid_count += 1;
            }
        }
    }

    if valid_count < 2 {
        return Err(ContractError::ThresholdNotReached);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Vec};
    use soroban_sdk::testutils::Address as _;

    fn setup() -> (Env, soroban_sdk::Address) {
        let env = Env::default();
        env.mock_all_auths();
        let admin = soroban_sdk::Address::generate(&env);
        (env, admin)
    }

    #[test]
    fn test_require_multisig_accepts_two_unique_signers() {
        let (env, admin) = setup();
        
        let mut signers: Vec<Address> = Vec::new(&env);
        signers.push_back(admin.clone());
        signers.push_back(admin.clone());
        
        assert!(require_multisig(&env, &signers).is_ok());
    }

    #[test]
    fn test_require_multisig_rejects_single_signer() {
        let (env, admin) = setup();
        
        let mut signers: Vec<Address> = Vec::new(&env);
        signers.push_back(admin.clone());
        
        assert!(matches!(
            require_multisig(&env, &signers),
            Err(ContractError::ThresholdNotReached)
        ));
    }

    #[test]
    fn test_require_multisig_handles_duplicate_signers() {
        let (env, _admin) = setup();
        
        let signer1 = soroban_sdk::Address::generate(&env);
        let signer2 = soroban_sdk::Address::generate(&env);
        
        let mut signers: Vec<Address> = Vec::new(&env);
        signers.push_back(signer1.clone());
        signers.push_back(signer1.clone());
        signers.push_back(signer2.clone());
        
        assert!(matches!(
            require_multisig(&env, &signers),
            Err(ContractError::ThresholdNotReached)
        ));
    }
}