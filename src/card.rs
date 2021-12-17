use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use fixed_hash::construct_fixed_hash;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

construct_fixed_hash! {
    /// 256 bit hash type
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(Encode, Decode)]
    pub struct H256(32);
}

pub type CardId = u32;
pub type CardUniqueIdentity = H256;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct Card {
    pub name: Vec<u8>,
    pub card_type: CardType,
    pub color: Color,
    pub rules: Vec<u8>,
    pub image: H256,
}

pub type Color = u8;

pub const WHITE: Color = 1;
pub const BLACK: Color = 2;
pub const RED: Color = 4;
pub const GREEN: Color = 8;
pub const BLUE: Color = 16;
pub const COLORLESS: Color = 0;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub enum CardType {
    Sorcery,
    Instant,
    Creature,
    Aura,
    Land
}

impl Default for CardType {
    fn default() -> Self {CardType::Sorcery}
}