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

//! Autogenerated weights for pallet_manta_pay
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
// --pallet=pallet_manta_pay
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_manta_pay.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_manta_pay.
pub trait WeightInfo {
	fn to_private() -> Weight;
	fn to_public() -> Weight;
	fn private_transfer() -> Weight;
	fn public_transfer() -> Weight;
}

/// Weights for pallet_manta_pay using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_manta_pay::WeightInfo for SubstrateWeight<T> {
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoSet (r:1 w:1)
	/// Proof: MantaPay UtxoSet (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetSize (r:1 w:0)
	/// Proof: MantaPay NullifierSetSize (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: MantaPay ShardTrees (r:1 w:1)
	/// Proof: MantaPay ShardTrees (max_values: None, max_size: Some(654), added: 3129, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
	/// Proof: MantaPay UtxoAccumulatorOutputs (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay Shards (r:0 w:1)
	/// Proof: MantaPay Shards (max_values: None, max_size: Some(395), added: 2870, mode: MaxEncodedLen)
	fn to_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4338`
		//  Estimated: `6232`
		// Minimum execution time: 39_976_088_000 picoseconds.
		Weight::from_parts(40_013_767_000, 6232)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
	/// Proof: MantaPay UtxoAccumulatorOutputs (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
	/// Proof: MantaPay NullifierCommitmentSet (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoSet (r:1 w:1)
	/// Proof: MantaPay UtxoSet (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetSize (r:1 w:1)
	/// Proof: MantaPay NullifierSetSize (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: MantaPay ShardTrees (r:1 w:1)
	/// Proof: MantaPay ShardTrees (max_values: None, max_size: Some(654), added: 3129, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
	/// Proof: MantaPay NullifierSetInsertionOrder (max_values: None, max_size: Some(144), added: 2619, mode: MaxEncodedLen)
	/// Storage: MantaPay Shards (r:0 w:1)
	/// Proof: MantaPay Shards (max_values: None, max_size: Some(395), added: 2870, mode: MaxEncodedLen)
	fn to_public() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11190`
		//  Estimated: `6232`
		// Minimum execution time: 52_995_808_000 picoseconds.
		Weight::from_parts(53_043_726_000, 6232)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(12_u64))
	}
	/// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
	/// Proof: MantaPay UtxoAccumulatorOutputs (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
	/// Proof: MantaPay NullifierCommitmentSet (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoSet (r:2 w:2)
	/// Proof: MantaPay UtxoSet (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetSize (r:1 w:1)
	/// Proof: MantaPay NullifierSetSize (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: MantaPay ShardTrees (r:2 w:2)
	/// Proof: MantaPay ShardTrees (max_values: None, max_size: Some(654), added: 3129, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
	/// Proof: MantaPay NullifierSetInsertionOrder (max_values: None, max_size: Some(144), added: 2619, mode: MaxEncodedLen)
	/// Storage: MantaPay Shards (r:0 w:2)
	/// Proof: MantaPay Shards (max_values: None, max_size: Some(395), added: 2870, mode: MaxEncodedLen)
	fn private_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12105`
		//  Estimated: `7248`
		// Minimum execution time: 69_674_315_000 picoseconds.
		Weight::from_parts(69_725_048_000, 7248)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(13_u64))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	fn public_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `969`
		//  Estimated: `6232`
		// Minimum execution time: 52_801_000 picoseconds.
		Weight::from_parts(53_729_000, 6232)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoSet (r:1 w:1)
	/// Proof: MantaPay UtxoSet (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetSize (r:1 w:0)
	/// Proof: MantaPay NullifierSetSize (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: MantaPay ShardTrees (r:1 w:1)
	/// Proof: MantaPay ShardTrees (max_values: None, max_size: Some(654), added: 3129, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
	/// Proof: MantaPay UtxoAccumulatorOutputs (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay Shards (r:0 w:1)
	/// Proof: MantaPay Shards (max_values: None, max_size: Some(395), added: 2870, mode: MaxEncodedLen)
	fn to_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4338`
		//  Estimated: `6232`
		// Minimum execution time: 39_976_088_000 picoseconds.
		Weight::from_parts(40_013_767_000, 6232)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
	/// Proof: MantaPay UtxoAccumulatorOutputs (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
	/// Proof: MantaPay NullifierCommitmentSet (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoSet (r:1 w:1)
	/// Proof: MantaPay UtxoSet (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetSize (r:1 w:1)
	/// Proof: MantaPay NullifierSetSize (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: MantaPay ShardTrees (r:1 w:1)
	/// Proof: MantaPay ShardTrees (max_values: None, max_size: Some(654), added: 3129, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
	/// Proof: MantaPay NullifierSetInsertionOrder (max_values: None, max_size: Some(144), added: 2619, mode: MaxEncodedLen)
	/// Storage: MantaPay Shards (r:0 w:1)
	/// Proof: MantaPay Shards (max_values: None, max_size: Some(395), added: 2870, mode: MaxEncodedLen)
	fn to_public() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11190`
		//  Estimated: `6232`
		// Minimum execution time: 52_995_808_000 picoseconds.
		Weight::from_parts(53_043_726_000, 6232)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(12_u64))
	}
	/// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
	/// Proof: MantaPay UtxoAccumulatorOutputs (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
	/// Proof: MantaPay NullifierCommitmentSet (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: MantaPay UtxoSet (r:2 w:2)
	/// Proof: MantaPay UtxoSet (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetSize (r:1 w:1)
	/// Proof: MantaPay NullifierSetSize (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: MantaPay ShardTrees (r:2 w:2)
	/// Proof: MantaPay ShardTrees (max_values: None, max_size: Some(654), added: 3129, mode: MaxEncodedLen)
	/// Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
	/// Proof: MantaPay NullifierSetInsertionOrder (max_values: None, max_size: Some(144), added: 2619, mode: MaxEncodedLen)
	/// Storage: MantaPay Shards (r:0 w:2)
	/// Proof: MantaPay Shards (max_values: None, max_size: Some(395), added: 2870, mode: MaxEncodedLen)
	fn private_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12105`
		//  Estimated: `7248`
		// Minimum execution time: 69_674_315_000 picoseconds.
		Weight::from_parts(69_725_048_000, 7248)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(13_u64))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	fn public_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `969`
		//  Estimated: `6232`
		// Minimum execution time: 52_801_000 picoseconds.
		Weight::from_parts(53_729_000, 6232)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}