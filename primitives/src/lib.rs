#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::Perbill;

/// Customized headers.
pub mod header;
pub use header::*;

/// Kate Commitment on Headers.
pub mod kate_commitment;
pub use kate_commitment::*;

/// Application Specific Data Retrieval
pub mod asdr;

pub mod traits;

pub mod well_known_keys {
	/// Public params used to generate Kate commitment
	pub const KATE_PUBLIC_PARAMS: &[u8] = b":kate_public_params:";

	/// Max block length
	pub const BLOCK_LENGTH: &[u8] = b":block_length:";
}

/// We allow `Normal` extrinsics to fill up the block up to 90%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(90);

pub const BLOCK_CHUNK_SIZE: u32 = 64;

/// Money matters.
pub mod currency {

	pub type Balance = u128;

	/// AVL has 18 decimal positions.
	pub const AVL: Balance = 1_000_000_000_000_000_000;

	/// Cents of AVL has 16 decimal positions (100 Cents = $1)
	/// 1 DOLLARS = 10_000_000_000_000_000
	pub const CENTS: Balance = AVL / 100;

	/// Millicent of AVL has 13 decimal positions( 100 mCents = 1 cent).
	pub const MILLICENTS: Balance = CENTS / 1_000;
}

#[repr(u8)]
pub enum InvalidTransactionCustomId {
	/// The AppId is not registered.
	InvalidAppId = 137,
	/// Extrinsic is not allowed for the given `AppId`.
	ForbiddenAppId,
}
