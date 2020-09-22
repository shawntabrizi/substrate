//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0-rc6

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl frame_system::WeightInfo for WeightInfo {
	// WARNING! Some components were not used: ["b"]
	fn remark() -> Weight {
		(2_556_000 as Weight)
	}
	fn set_heap_pages() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// WARNING! Some components were not used: ["d"]
	fn set_changes_trie_config() -> Weight {
		(14_192_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn set_storage(i: u32, ) -> Weight {
		(3_517_000 as Weight)
			.saturating_add((1_271_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	fn kill_storage(i: u32, ) -> Weight {
		(4_116_000 as Weight)
			.saturating_add((854_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	fn kill_prefix(p: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((1_839_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn suicide() -> Weight {
		(51_000_000 as Weight)
	}
}
