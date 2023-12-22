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

//! Autogenerated weights for pallet_multisig
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
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_multisig.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_multisig.
pub trait WeightInfo {
	fn as_multi_threshold_1(z: u32, ) -> Weight;
	fn as_multi_create(s: u32, z: u32, ) -> Weight;
	fn as_multi_approve(s: u32, z: u32, ) -> Weight;
	fn as_multi_complete(s: u32, z: u32, ) -> Weight;
	fn approve_as_multi_create(s: u32, ) -> Weight;
	fn approve_as_multi_approve(s: u32, ) -> Weight;
	fn cancel_as_multi(s: u32, ) -> Weight;
}

/// Weights for pallet_multisig using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for SubstrateWeight<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_266_000 picoseconds.
		Weight::from_parts(15_663_644, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(691, 0).saturating_mul(z.into()))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 46_400_000 picoseconds.
		Weight::from_parts(36_550_280, 6811)
			// Standard Error: 501
			.saturating_add(Weight::from_parts(118_375, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_472, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `6811`
		// Minimum execution time: 31_065_000 picoseconds.
		Weight::from_parts(20_869_197, 6811)
			// Standard Error: 440
			.saturating_add(Weight::from_parts(109_877, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_494, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `418 + s * (33 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 53_439_000 picoseconds.
		Weight::from_parts(40_781_356, 6811)
			// Standard Error: 792
			.saturating_add(Weight::from_parts(151_013, 0).saturating_mul(s.into()))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_500, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 33_023_000 picoseconds.
		Weight::from_parts(34_286_049, 6811)
			// Standard Error: 583
			.saturating_add(Weight::from_parts(123_300, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `6811`
		// Minimum execution time: 18_913_000 picoseconds.
		Weight::from_parts(19_436_790, 6811)
			// Standard Error: 485
			.saturating_add(Weight::from_parts(110_803, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `487 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 34_350_000 picoseconds.
		Weight::from_parts(35_624_066, 6811)
			// Standard Error: 541
			.saturating_add(Weight::from_parts(122_797, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_266_000 picoseconds.
		Weight::from_parts(15_663_644, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(691, 0).saturating_mul(z.into()))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 46_400_000 picoseconds.
		Weight::from_parts(36_550_280, 6811)
			// Standard Error: 501
			.saturating_add(Weight::from_parts(118_375, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_472, 0).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `6811`
		// Minimum execution time: 31_065_000 picoseconds.
		Weight::from_parts(20_869_197, 6811)
			// Standard Error: 440
			.saturating_add(Weight::from_parts(109_877, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_494, 0).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `418 + s * (33 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 53_439_000 picoseconds.
		Weight::from_parts(40_781_356, 6811)
			// Standard Error: 792
			.saturating_add(Weight::from_parts(151_013, 0).saturating_mul(s.into()))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_500, 0).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 33_023_000 picoseconds.
		Weight::from_parts(34_286_049, 6811)
			// Standard Error: 583
			.saturating_add(Weight::from_parts(123_300, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `6811`
		// Minimum execution time: 18_913_000 picoseconds.
		Weight::from_parts(19_436_790, 6811)
			// Standard Error: 485
			.saturating_add(Weight::from_parts(110_803, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `487 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 34_350_000 picoseconds.
		Weight::from_parts(35_624_066, 6811)
			// Standard Error: 541
			.saturating_add(Weight::from_parts(122_797, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
