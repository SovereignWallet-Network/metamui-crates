//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as SyncDid;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {}

impl_benchmark_test_suite!(SyncDid, crate::mock::new_test_ext(), crate::mock::Test,);
