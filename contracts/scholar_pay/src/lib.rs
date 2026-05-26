#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    token, Address, Env, Map, String, Symbol,
};

const ADMIN: Symbol = symbol_short!("ADMIN");
const SCHOLARS: Symbol = symbol_short!("SCHOLARS");
const GRANTS: Symbol = symbol_short!("GRANTS");
const SCHOOLS: Symbol = symbol_short!("SCHOOLS");

#[contracttype]
#[derive(Clone)]
pub struct School {
    pub wallet: Address,
    pub name: String,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct Scholar {
    pub wallet: Address,
    pub school_id: String,
    pub name: String,
    pub school_wallet: Address,
    pub total_paid: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct TuitionGrant {
    pub grant_id: String,
    pub scholar: Address,
    pub school_wallet: Address,
    pub amount: i128,
    pub semester: String,
    pub disbursed: bool,
}

#[contract]
pub struct ScholarPayContract;

#[contractimpl]
impl ScholarPayContract {

    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
        let schools: Map<Address, School> = Map::new(&env);
        let scholars: Map<Address, Scholar> = Map::new(&env);
        let grants: Map<String, TuitionGrant> = Map::new(&env);
        env.storage().instance().set(&SCHOOLS, &schools);
        env.storage().instance().set(&SCHOLARS, &scholars);
        env.storage().instance().set(&GRANTS, &grants);
    }

    pub fn register_school(
        env: Env,
        caller: Address,
        school_wallet: Address,
        name: String,
    ) {
        caller.require_auth();
        Self::require_admin(&env, &caller);
        let mut schools: Map<Address, School> =
            env.storage().instance().get(&SCHOOLS).unwrap();
        if schools.contains_key(school_wallet.clone()) {
            panic!("school already registered");
        }
        schools.set(school_wallet.clone(), School {
            wallet: school_wallet,
            name,
            is_active: true,
        });
        env.storage().instance().set(&SCHOOLS, &schools);
    }

    pub fn register_scholar(
        env: Env,
        caller: Address,
        wallet: Address,
        school_id: String,
        name: String,
        school_wallet: Address,
    ) {
        caller.require_auth();
        Self::require_admin(&env, &caller);
        let schools: Map<Address, School> =
            env.storage().instance().get(&SCHOOLS).unwrap();
        if !schools.contains_key(school_wallet.clone()) {
            panic!("school not registered");
        }
        let mut scholars: Map<Address, Scholar> =
            env.storage().instance().get(&SCHOLARS).unwrap();
        if scholars.contains_key(wallet.clone()) {
            panic!("scholar already registered");
        }
        scholars.set(wallet.clone(), Scholar {
            wallet,
            school_id,
            name,
            school_wallet,
            total_paid: 0,
        });
        env.storage().instance().set(&SCHOLARS, &scholars);
    }

    pub fn create_grant(
        env: Env,
        caller: Address,
        grant_id: String,
        scholar_wallet: Address,
        amount: i128,
        semester: String,
    ) {
        caller.require_auth();
        Self::require_admin(&env, &caller);
        if amount <= 0 {
            panic!("amount must be positive");
        }
        let mut grants: Map<String, TuitionGrant> =
            env.storage().instance().get(&GRANTS).unwrap();
        if grants.contains_key(grant_id.clone()) {
            panic!("grant ID already exists");
        }
        let scholars: Map<Address, Scholar> =
            env.storage().instance().get(&SCHOLARS).unwrap();
        let scholar = scholars
            .get(scholar_wallet.clone())
            .expect("scholar not registered");
        let locked_school_wallet = scholar.school_wallet.clone();
        grants.set(grant_id.clone(), TuitionGrant {
            grant_id,
            scholar: scholar_wallet,
            school_wallet: locked_school_wallet,
            amount,
            semester,
            disbursed: false,
        });
        env.storage().instance().set(&GRANTS, &grants);
    }

    pub fn disburse(
        env: Env,
        caller: Address,
        grant_id: String,
        token_address: Address,
    ) {
        caller.require_auth();
        let mut grants: Map<String, TuitionGrant> =
            env.storage().instance().get(&GRANTS).unwrap();
        let mut grant = grants
            .get(grant_id.clone())
            .expect("grant not found");
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if caller != grant.scholar && caller != admin {
            panic!("only the scholar or admin can disburse");
        }
        if grant.disbursed {
            panic!("grant already disbursed");
        }
        let schools: Map<Address, School> =
            env.storage().instance().get(&SCHOOLS).unwrap();
        let school = schools
            .get(grant.school_wallet.clone())
            .expect("school not found");
        if !school.is_active {
            panic!("school is no longer active");
        }
        let token = token::Client::new(&env, &token_address);
        token.transfer(
            &env.current_contract_address(),
            &grant.school_wallet,
            &grant.amount,
        );
        grant.disbursed = true;
        grants.set(grant_id, grant.clone());
        env.storage().instance().set(&GRANTS, &grants);
        let mut scholars: Map<Address, Scholar> =
            env.storage().instance().get(&SCHOLARS).unwrap();
        let mut scholar = scholars.get(grant.scholar.clone()).unwrap();
        scholar.total_paid += grant.amount;
        scholars.set(grant.scholar, scholar);
        env.storage().instance().set(&SCHOLARS, &scholars);
    }

    pub fn get_grant(env: Env, grant_id: String) -> TuitionGrant {
        let grants: Map<String, TuitionGrant> =
            env.storage().instance().get(&GRANTS).unwrap();
        grants.get(grant_id).expect("grant not found")
    }

    pub fn get_scholar(env: Env, wallet: Address) -> Scholar {
        let scholars: Map<Address, Scholar> =
            env.storage().instance().get(&SCHOLARS).unwrap();
        scholars.get(wallet).expect("scholar not found")
    }

    pub fn get_school(env: Env, school_wallet: Address) -> School {
        let schools: Map<Address, School> =
            env.storage().instance().get(&SCHOOLS).unwrap();
        schools.get(school_wallet).expect("school not found")
    }

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if *caller != admin {
            panic!("unauthorized: caller is not admin");
        }
    }
}