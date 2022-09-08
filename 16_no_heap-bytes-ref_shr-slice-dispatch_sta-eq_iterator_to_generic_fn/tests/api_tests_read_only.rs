#![no_std]
#![feature(generic_associated_types)]

use no_heap_bytes_ref_shr_slice_dispatch_sta_eq_iterator_to_generic_fn as dna;
use test_harness::api_tests_read_only::Tests;

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna<'a>;
    type Rna<'a> = dna::Rna<'a>;
}

#[test]
fn all_tests() {
    T::all_tests();
}
