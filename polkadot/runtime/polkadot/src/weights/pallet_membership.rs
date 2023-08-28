// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-e8ezs4ez-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_membership
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 17_443_000 picoseconds.
		Weight::from_parts(18_272_399, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 376
			.saturating_add(Weight::from_parts(33_633, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 19_826_000 picoseconds.
		Weight::from_parts(20_859_732, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 667
			.saturating_add(Weight::from_parts(33_155, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 20_151_000 picoseconds.
		Weight::from_parts(20_774_114, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 447
			.saturating_add(Weight::from_parts(44_052, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn reset_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 19_846_000 picoseconds.
		Weight::from_parts(20_903_563, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 865
			.saturating_add(Weight::from_parts(149_306, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 20_523_000 picoseconds.
		Weight::from_parts(21_705_085, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 948
			.saturating_add(Weight::from_parts(44_568, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Members (r:1 w:0)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: TechnicalMembership Prime (r:0 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (32 ±0)`
		//  Estimated: `4687 + m * (32 ±0)`
		// Minimum execution time: 8_032_000 picoseconds.
		Weight::from_parts(8_386_682, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 190
			.saturating_add(Weight::from_parts(9_724, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: TechnicalMembership Prime (r:0 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_061_000 picoseconds.
		Weight::from_parts(3_304_217, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 80
			.saturating_add(Weight::from_parts(273, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}