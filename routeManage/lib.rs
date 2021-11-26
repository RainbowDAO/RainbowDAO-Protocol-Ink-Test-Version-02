#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::route_manage::{
    RouteManage,
};

#[ink::contract]
mod route_manage {
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };

    #[ink(storage)]
     #[derive(Default)]
    pub struct RouteManage {
        route_manage:AccountId,
        contract_addrs:StorageHashMap<String, AccountId>,        
    }
    #[ink(event)]
    pub struct Contract{
        #[ink(topic)]
        contract_name:String,
        #[ink(topic)]
        contract_addr:AccountId,
    }

    impl RouteManage {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(route_manage:AccountId) -> Self {
            Self {
                route_manage,
                contract_addrs:StorageHashMap::new(),
             }
        }
    #[ink(message)]
    pub fn add_route(&mut self ,contract_name:String, contract_addr:AccountId) ->bool{
        //only core or governance
        //assert!();
        let caller = self.env().caller();
        assert!(self.route_manage == caller);
        self.contract_addrs.insert(contract_name,contract_addr);
        true
    }

    #[ink(message)]
    pub fn change_route(&mut self,name:String,contract_addr:AccountId) -> bool {
        // self.only_core(Self::env().caller());
        self.contract_addrs[&name] = contract_addr;
        true
    }

    #[ink(message)]
    pub fn get_contract_by_name(&self, contract_name:String) -> AccountId {
        self.contract_addrs.get(&contract_name).unwrap().clone()
    }

    #[ink(message)]
    pub fn get_manager_addr(&self) ->AccountId {
        return self.route_manage;
    }
    ///set a new contract address
    // #[ink(message)]
    // pub fn set_new_contract_addr(&mut self,set_contract_name:String, set_new_addr:AccountId, ) ->bool{
    //     assert!(self.route_manage == self.env().caller());
    //     let mut contract_instance: DaoManage = ink_env::call::FromAccountId::from_account_id(self.dao_manage_addr);
    //     contract_instance.set_new_addr(set_contract_name,set_new_addr);
    //     true
    // }
    ///Replacing an Administrator
    #[ink(message)]
    pub fn transfer_owner(&mut self,new_owner: AccountId,) ->bool {
        // only core or govnance
        //assert!()!
        assert!(self.route_manage == self.env().caller());
        self.route_manage = new_owner;
        true
    }
    }
}
