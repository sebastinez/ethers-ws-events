pub use org_mod::*;
#[allow(clippy::too_many_arguments)]
mod org_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            self as ethers_contract,
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            self as ethers_core,
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::{self as ethers_providers, Middleware},
    };
    #[doc = "Org was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ORG_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> = ethers_contract::Lazy::new(
        || {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"tag\",\"type\":\"uint32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"multihash\",\"type\":\"bytes\"}],\"name\":\"Anchored\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"}],\"name\":\"NameChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnerChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\"}],\"name\":\"Unanchored\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"ADDR_REVERSE_NODE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\"},{\"internalType\":\"uint32\",\"name\":\"tag\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"multihash\",\"type\":\"bytes\"}],\"name\":\"anchor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"anchors\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"tag\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"multihash\",\"type\":\"bytes\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"recoverFunds\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"},{\"internalType\":\"contract ENS\",\"name\":\"ens\",\"type\":\"address\"}],\"name\":\"setName\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\"}],\"name\":\"unanchor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
        },
    );
    #[derive(Clone)]
    pub struct Org<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for Org<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for Org<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Org))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> Org<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(address.into(), ORG_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `ADDR_REVERSE_NODE` (0x7cf8a2eb) function"]
        pub fn addr_reverse_node(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([124, 248, 162, 235], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `anchor` (0x68f1fbf8) function"]
        pub fn anchor(
            &self,
            id: [u8; 32],
            tag: u32,
            multihash: Vec<u8>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 241, 251, 248], (id, tag, multihash))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `anchors` (0xb01b6d53) function"]
        pub fn anchors(
            &self,
            p0: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, (u32, Vec<u8>)> {
            self.0
                .method_hash([176, 27, 109, 83], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `recoverFunds` (0x58609754) function"]
        pub fn recover_funds(
            &self,
            token: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([88, 96, 151, 84], (token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setName` (0xec07c2d1) function"]
        pub fn set_name(
            &self,
            name: String,
            ens: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([236, 7, 194, 209], (name, ens))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOwner` (0x13af4035) function"]
        pub fn set_owner(
            &self,
            new_owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unanchor` (0xcbd56601) function"]
        pub fn unanchor(&self, id: [u8; 32]) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 213, 102, 1], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Anchored` event"]
        pub fn anchored_filter(&self) -> ethers_contract::builders::Event<M, AnchoredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NameChanged` event"]
        pub fn name_changed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NameChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerChanged` event"]
        pub fn owner_changed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, OwnerChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unanchored` event"]
        pub fn unanchored_filter(&self) -> ethers_contract::builders::Event<M, UnanchoredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, OrgEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Anchored", abi = "Anchored(bytes32,uint32,bytes)")]
    pub struct AnchoredFilter {
        pub id: [u8; 32],
        pub tag: u32,
        pub multihash: Vec<u8>,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "NameChanged", abi = "NameChanged(string)")]
    pub struct NameChangedFilter {
        pub name: String,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "OwnerChanged", abi = "OwnerChanged(address)")]
    pub struct OwnerChangedFilter {
        pub new_owner: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Unanchored", abi = "Unanchored(bytes32)")]
    pub struct UnanchoredFilter {
        pub id: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum OrgEvents {
        AnchoredFilter(AnchoredFilter),
        NameChangedFilter(NameChangedFilter),
        OwnerChangedFilter(OwnerChangedFilter),
        UnanchoredFilter(UnanchoredFilter),
    }
    impl ethers_core::abi::Tokenizable for OrgEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AnchoredFilter::from_token(token.clone()) {
                return Ok(OrgEvents::AnchoredFilter(decoded));
            }
            if let Ok(decoded) = NameChangedFilter::from_token(token.clone()) {
                return Ok(OrgEvents::NameChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerChangedFilter::from_token(token.clone()) {
                return Ok(OrgEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = UnanchoredFilter::from_token(token.clone()) {
                return Ok(OrgEvents::UnanchoredFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                OrgEvents::AnchoredFilter(element) => element.into_token(),
                OrgEvents::NameChangedFilter(element) => element.into_token(),
                OrgEvents::OwnerChangedFilter(element) => element.into_token(),
                OrgEvents::UnanchoredFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for OrgEvents {}
    impl ethers_contract::EthLogDecode for OrgEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AnchoredFilter::decode_log(log) {
                return Ok(OrgEvents::AnchoredFilter(decoded));
            }
            if let Ok(decoded) = NameChangedFilter::decode_log(log) {
                return Ok(OrgEvents::NameChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerChangedFilter::decode_log(log) {
                return Ok(OrgEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = UnanchoredFilter::decode_log(log) {
                return Ok(OrgEvents::UnanchoredFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
