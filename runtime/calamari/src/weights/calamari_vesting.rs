// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for calamari_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-21, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("/home/aye/actions-runner/_worker/Manta/Manta/tests/data/fork.json"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=/home/aye/actions-runner/_worker/Manta/Manta/tests/data/fork.json
// --steps=50
// --repeat=40
// --pallet=calamari_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/calamari_vesting.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for calamari_vesting.
pub trait WeightInfo {
	fn update_vesting_schedule() -> Weight;
	fn vest() -> Weight;
	fn vested_transfer() -> Weight;
}

/// Weights for calamari_vesting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> calamari_vesting::WeightInfo for SubstrateWeight<T> {
	/// Storage: CalamariVesting VestingSchedule (r:1 w:1)
	/// Proof Skipped: CalamariVesting VestingSchedule (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn update_vesting_schedule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `118`
		//  Estimated: `1603`
		// Minimum execution time: 14_378_000 picoseconds.
		Weight::from_parts(14_809_000, 1603)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CalamariVesting VestingSchedule (r:1 w:0)
	/// Proof Skipped: CalamariVesting VestingSchedule (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CalamariVesting VestingBalances (r:1 w:1)
	/// Proof Skipped: CalamariVesting VestingBalances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn vest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `462`
		//  Estimated: `4764`
		// Minimum execution time: 46_572_000 picoseconds.
		Weight::from_parts(47_606_000, 4764)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: CalamariVesting VestingBalances (r:1 w:1)
	/// Proof Skipped: CalamariVesting VestingBalances (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CalamariVesting VestingSchedule (r:1 w:0)
	/// Proof Skipped: CalamariVesting VestingSchedule (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn vested_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `232`
		//  Estimated: `4764`
		// Minimum execution time: 84_231_000 picoseconds.
		Weight::from_parts(85_418_000, 4764)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: CalamariVesting VestingSchedule (r:1 w:1)
	/// Proof Skipped: CalamariVesting VestingSchedule (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn update_vesting_schedule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `118`
		//  Estimated: `1603`
		// Minimum execution time: 14_378_000 picoseconds.
		Weight::from_parts(14_809_000, 1603)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CalamariVesting VestingSchedule (r:1 w:0)
	/// Proof Skipped: CalamariVesting VestingSchedule (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CalamariVesting VestingBalances (r:1 w:1)
	/// Proof Skipped: CalamariVesting VestingBalances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn vest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `462`
		//  Estimated: `4764`
		// Minimum execution time: 46_572_000 picoseconds.
		Weight::from_parts(47_606_000, 4764)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: CalamariVesting VestingBalances (r:1 w:1)
	/// Proof Skipped: CalamariVesting VestingBalances (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CalamariVesting VestingSchedule (r:1 w:0)
	/// Proof Skipped: CalamariVesting VestingSchedule (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn vested_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `232`
		//  Estimated: `4764`
		// Minimum execution time: 84_231_000 picoseconds.
		Weight::from_parts(85_418_000, 4764)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}