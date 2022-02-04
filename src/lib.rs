#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod card;

#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;
#[cfg(test)] 
mod test_encode;

#[frame_support::pallet]
pub mod pallet {
    use crate::card::CardId;
    use crate::card::Card;
    use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage

    #[pallet::storage]
	#[pallet::getter(fn creators)]
    pub type CreatorRegistry<T: Config> = StorageMap<
        _,
        Blake2_128Concat, T::AccountId,
        (), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cards)]
	pub type CardRegistry<T: Config> = StorageMap<
        _,
        Blake2_128Concat, CardId,
        Card, OptionQuery
        >;

    #[pallet::storage]
    #[pallet::getter(fn owners)]
    pub type CardOwners<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId,
        Blake2_128Concat, CardId,
        u16, OptionQuery
        >;
    
    #[pallet::type_value]
    pub fn DefaultPreviousId() -> CardId { 0 }

    #[pallet::storage]
    #[pallet::getter(fn previous_card_id)]
    pub type PreviousCardId<T: Config> = StorageValue<_, CardId, ValueQuery, DefaultPreviousId>;

    #[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
        pub creator: T::AccountId,
    }

    #[cfg(feature = "std")]
    impl<T: Config> GenesisConfig<T> {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
            <Self as GenesisBuild<T>>::build_storage(self)
        }

        /// Direct implementation of `GenesisBuild::assimilate_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
            <Self as GenesisBuild<T>>::assimilate_storage(self, storage)
        }
    }

    #[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { creator: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
            <CreatorRegistry<T>>::insert(&self.creator, ());
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        /// \[creator, card_id, amount\]
		CardCreated(T::AccountId, CardId, u16),
        /// \[old owner, card_id, new owner\]
        CardTransferred(T::AccountId, CardId, T::AccountId),
        /// \[assigner, new creator\]
        CreatorAssigned(T::AccountId, T::AccountId),
        /// \[assigner, not a creator anymore\]
        CreatorWithdrawn(T::AccountId, T::AccountId)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Can not store a card
		IdStorageOverflow, 
        /// Can not assign to creators
        NoPermission,
        /// Account already a creator
        AccountAlreadyCreator,
        /// Can not remove from creators - account not one
        AccountNotCreator,
        /// Card not owned
        CardNotOwned,
	}

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000+ T::DbWeight::get().writes(1))]
        pub fn set_creator(origin: OriginFor<T>, id: T::AccountId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(<CreatorRegistry<T>>::contains_key(&who), Error::<T>::NoPermission);

            ensure!(!<CreatorRegistry<T>>::contains_key(&id), Error::<T>::AccountAlreadyCreator);
            <CreatorRegistry<T>>::insert(&id, ());
            Self::deposit_event(Event::CreatorAssigned(who, id));
            Ok(().into())
        }

        #[pallet::weight(10_000+ T::DbWeight::get().writes(1))]
        pub fn withdraw_creator(origin: OriginFor<T>, id: T::AccountId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(<CreatorRegistry<T>>::contains_key(&who), Error::<T>::NoPermission);

            ensure!(<CreatorRegistry<T>>::contains_key(&id), Error::<T>::AccountNotCreator);
            <CreatorRegistry<T>>::remove(&id);
            Self::deposit_event(Event::CreatorWithdrawn(who, id));
            Ok(().into())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_card_pack(origin: OriginFor<T>, card: Card, amount: u16) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
            ensure!(<CreatorRegistry<T>>::contains_key(&who), Error::<T>::NoPermission);
            
            let i = Self::previous_card_id();
            let nextid = i.checked_add(1);
            match nextid {
                Some(id) => {
                    // Update storage.
                    <CardRegistry<T>>::insert(id, card);
                    <PreviousCardId<T>>::put(id);
                    <CardOwners<T>>::insert(&who, id, amount);
                    // Emit an event.
                    Self::deposit_event(Event::CardCreated(who, id, amount));
                    // Return a successful DispatchResultWithPostInfo
                    Ok(().into())
                },
                None => Err(Error::<T>::IdStorageOverflow)?
            }	
		}

        // #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn sell(origin: OriginFor<T>, card_id: CardId, amount: u16, account: T::AccountId)   
        //     ->  DispatchResultWithPostInfo {
        //         let who = ensure_signed(origin)?;

        //     Ok(().into())
        // }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(origin: OriginFor<T>, card_id: CardId, account: T::AccountId)
             -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            <CardOwners<T>>::try_mutate_exists(who.clone(), card_id, |amount| {
                match amount {
                    Some(owner_amount) => {
                        if owner_amount == &1u16 {
                            amount.take();
                        } else {
                            let new_amount = owner_amount.checked_sub(1);
                            if let Some(a) = new_amount {
                                *amount = Some(a);
                            } else {
                                Err(Error::<T>::CardNotOwned)?
                            }
                        }

                        match <CardOwners<T>>::try_get(&account, card_id) {
                            Ok(amount) => <CardOwners<T>>::insert(&account, card_id, amount+1),
                            _ => <CardOwners<T>>::insert(&account, card_id, 1),
                        }
                        Self::deposit_event(Event::CardTransferred(who, card_id, account));
                        Ok(().into())
                    },
                    None => {
                        Err(Error::<T>::CardNotOwned)?
                    }
                }
            })
            // match <CardOwners<T>>::try_get(&who, &card_id) {
            //     Ok(owned) => {
            //         if owned < 1 {
            //             <CardOwners<T>>::remove(&who, card_id);
            //             Err(Error::<T>::CardNotOwned)?
            //         }
            //         else if owned == 1 {
            //             <CardOwners<T>>::remove(&who, card_id)
            //         } else {
            //             <CardOwners<T>>::insert(&who, card_id, owned-1)
            //         }

            //         match <CardOwners<T>>::try_get(&account, card_id) {
            //             Ok(amount) => <CardOwners<T>>::insert(&account, card_id, amount+1),
            //             _ => <CardOwners<T>>::insert(&account, card_id, 1),
            //         }
            //         Self::deposit_event(Event::CardTransferred(who, card_id, account));
            //         Ok(().into())
            //     }
            //     _ => Err(Error::<T>::CardNotOwned)?
            // }
        }

        // #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn show_user_cards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
        //     Ok(().into())
        // }

	}
}