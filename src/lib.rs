#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod card;

#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;

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

    // #[pallet::genesis_config]
	// #[derive(Default)]
	// pub struct GenesisConfig;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. 
        /// \[creator, card_id, amount\]
		CardCreated(T::AccountId, CardId, u16),

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
           // ensure!(<CreatorRegistry<T>>::contains_key(&who), Error::<T>::NoPermission);
            
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

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn sell(origin: OriginFor<T>, card: Card, amount: u16, account: T::AccountId)   
            ->  DispatchResultWithPostInfo {
            
            Ok(().into())
        }

        // #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn transfer(origin: OriginFor<T>, card: Card) -> DispatchResultWithPostInfo {
        //     Ok(().into())
        // }

        // #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn show_user_cards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
        //     Ok(().into())
        // }
	}
}