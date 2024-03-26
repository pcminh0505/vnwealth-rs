use num::Zero;

use crate::pallets::system;
use std::{
    collections::BTreeMap,
    fmt::Debug,
    ops::{AddAssign, SubAssign},
};

pub trait Config: system::Config {
    type AssetName: ToString + Debug + Ord;
    type Currency: Zero + AddAssign + SubAssign;
}

pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AssetName, T::Currency>,
}

impl<T> Pallet<T>
where
    T: Config,
{
    pub fn new() -> Self {
        return Pallet {
            balances: BTreeMap::default(),
        };
    }
}

pub enum Call<T: Config> {
    Mint {
        asset_name: T::AssetName,
        amount: T::Currency,
    },
    Burn {
        asset_name: T::AssetName,
        amount: T::Currency,
    },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Mint { asset_name, amount } => {
                self.balances.insert(asset_name, amount).unwrap();
            }
            Call::Burn { asset_name, amount } => {
                self.balances.insert(asset_name, amount).unwrap();
            }
        }
        Ok(())
    }
}
