// SPDX-License-Identifier: MIT AND Apache-2.0
//! Minimal crate root for a registry-reading module.
//!
//! This is a SKELETON. It deliberately has no chain/sequencer I/O: it only
//! demonstrates the `registry_core` types and filter/IDL patterns that a real
//! module backend would use after decoding on-chain account data. Copy
//! `Cargo.toml.template` to `Cargo.toml` and replace its placeholders to build.

use registry_core::{search_entries_with_name, IdlAvailability, RegistryEntry};

/// Human-readable verification label derived from the on-chain record.
///
/// `deployer_claimed` is a self-assertion; only `is_independently_verified()`
/// (which requires `source_verified` AND a non-empty `verifier`) is treated as
/// independent here.
pub fn verification_label(entry: &RegistryEntry) -> &'static str {
    if entry.is_independently_verified() {
        "independently verified"
    } else {
        "unverified"
    }
}

/// Non-fatal IDL label: metadata is rendered whether or not an IDL is usable.
pub fn idl_label(entry: &RegistryEntry) -> &'static str {
    match entry.idl_availability() {
        IdlAvailability::Available(_) => "IDL available",
        IdlAvailability::Unavailable => "no IDL",
    }
}

/// Filter decoded entries by name, tag (OR) and independent verification.
///
/// Different filter kinds compose with AND; tags compose with OR. This mirrors
/// the semantics of `registry-api`'s `/search` endpoint and the JS client's
/// `search({ name, tags, verified })`.
pub fn find_entries<'a>(
    entries: &'a [RegistryEntry],
    name: Option<&str>,
    tags: &[&str],
    verified_only: bool,
) -> Vec<&'a RegistryEntry> {
    search_entries_with_name(entries, tags, None, verified_only, name)
}
