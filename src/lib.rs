#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod card;

#[frame_support::pallet]
pub mod pallet {
	use crate::card::Card;
use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use super::*;

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
	#[pallet::getter(fn cards)]
	pub type CardRegistry<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId,
        Blake2_128Concat, u32,
        Card, OptionQuery
        >;
    
    #[pallet::type_value]
    pub fn DefaultNextId() -> u32 { 0 }

    #[pallet::storage]
    #[pallet::getter(fn previous_card_id)]
    pub type PreviousCardId<T: Config> = StorageValue<_, u32, ValueQuery, DefaultNextId>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. 
        /// [something, who]
		CardCreated(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		IdStorageOverflow, 
	}

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create(origin: OriginFor<T>, card: Card) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
            let i = Self::previous_card_id();

            let nextid = i.checked_add(1);
            match nextid {
                Some(id) => {
                    // Update storage.
                    <CardRegistry<T>>::insert(&who, id, card);
                    <PreviousCardId<T>>::put(id);
                    // Emit an event.
                    Self::deposit_event(Event::CardCreated(i, who));
                    // Return a successful DispatchResultWithPostInfo
                    Ok(().into())
                },
                None => Err(Error::<T>::IdStorageOverflow)?
            }	
		}

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn sell(origin: OriginFor<T>, card: Card) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(origin: OriginFor<T>, card: Card) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn show_user_cards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }
	}
}