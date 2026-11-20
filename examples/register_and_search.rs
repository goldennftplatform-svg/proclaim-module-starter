// SPDX-License-Identifier: MIT AND Apache-2.0
//! register_and_search.rs — starter example for an LP-0023 registry integration.
//!
//! WHAT THIS IS
//! ------------
//! A conceptual, self-contained example of the four patterns a host-side
//! integration needs: shared Borsh types, PDA derivation, single-entry lookup
//! and name/tag/verified search filters. It does NOT submit transactions and
//! does NOT talk to a sequencer. Filling in live reads is your work: see the
//! `fetch_index` / `fetch_entry` helpers in the upstream repo's
//! `registry-api/src/lib.rs` (they use `wallet::WalletCore` and
//! `LEE_WALLET_HOME_DIR`).
//!
//! EXPECTED CARGO DEPENDENCIES
//! ---------------------------
//! Copy `Cargo.toml.template` to `Cargo.toml` in your own repository and
//! replace every placeholder. `registry_core` is a workspace crate in the
//! published git repository — it is not a crates.io release. Point Cargo at:
//!
//!     https://github.com/goldennftplatform-svg/proclaim
//!
//! e.g. `registry_core = { git = "https://github.com/goldennftplatform-svg/proclaim" }`.
//! PDA derivation below uses the same `nssa` / `nssa_core` types as the upstream
//! `registry-api` crate; copy the exact pinned dependency lines from that
//! repository's root Cargo.toml rather than guessing versions or URLs, then
//! confirm with `cargo metadata`. This file is an example, not a buildable
//! project on its own — there is intentionally no Cargo.toml that pretends to
//! fetch a crate which does not exist.
//!
//! Run (after you have a real manifest): `cargo run --example register_and_search`.
//! Licence: MIT AND Apache-2.0.

use nssa::AccountId;
use nssa_core::program::{PdaSeed, ProgramId};
use registry_core::{search_entries_with_name, IdlAvailability, RegistryEntry, RegistryIndex};

/// Final v5 registry program ID (hex, big-endian text), from the upstream docs.
const REGISTRY_PROGRAM_ID_HEX: &str =
    "4df7d5396b68a16aae6d0f768ef447a0055eb43866827154f71139e486265144";
/// Const seed for the single global index PDA (`registry_index`, right-padded).
const INDEX_SEED: &[u8] = b"registry_index";

/// Parse a 64-char hex program ID into the little-endian `u32` words LEZ uses.
fn program_id_from_hex(s: &str) -> Result<ProgramId, String> {
    let s = s.trim_start_matches("0x");
    if s.len() != 64 {
        return Err(format!("program id hex must be 64 chars, got {}", s.len()));
    }
    let mut pid = [0u32; 8];
    for (i, chunk) in s.as_bytes().chunks(8).enumerate() {
        let word = std::str::from_utf8(chunk)
            .ok()
            .and_then(|x| u32::from_str_radix(x, 16).ok())
            .ok_or_else(|| "invalid program id hex".to_string())?;
        pid[i] = word.swap_bytes(); // hex text is big-endian; ProgramId is LE words
    }
    Ok(pid)
}

/// Right-pad a short const seed to 32 bytes.
fn const_seed(s: &[u8]) -> Result<[u8; 32], String> {
    if s.len() > 32 {
        return Err("const seed exceeds 32 bytes".to_string());
    }
    let mut out = [0u8; 32];
    out[..s.len()].copy_from_slice(s);
    Ok(out)
}

/// PDA of the global registry index: seed is fixed `registry_index`.
fn index_pda(program_id: &ProgramId) -> Result<AccountId, String> {
    let seed = const_seed(INDEX_SEED)?;
    Ok(AccountId::for_public_pda(
        program_id,
        &PdaSeed::new(seed),
    ))
}

/// PDA of one registry entry: the seed is the 32-byte program ID itself
/// (`pda = arg("key")` in the guest; `--key <program-id>` on the CLI).
fn entry_pda(program_id: &ProgramId, key: &[u8; 32]) -> AccountId {
    AccountId::for_public_pda(program_id, &PdaSeed::new(*key))
}

/// In-memory sample record. In a real host you fetch account data and call
/// `RegistryEntry::from_account_data(&bytes)` (Borsh) instead.
fn sample_entry(name: &str, tags: &str, idl_cid: &str, verifier: Option<[u8; 32]>) -> RegistryEntry {
    RegistryEntry {
        program_id: [7u8; 32],
        name: name.into(),
        version: "0.1.0".into(),
        author: [1u8; 32],
        description: "example registry entry".into(),
        tags: tags.into(),
        idl_cid: idl_cid.into(),
        code_cid: "zDv*yXuozL".into(),
        commit: "abc1234".into(),
        build_config: "guest=local,risc0=3.0.5".into(),
        timestamp: 1_700_000_000,
        deployer_claimed: true,
        source_verified: verifier.is_some(),
        verifier,
        verification_method: if verifier.is_some() {
            "reproducible-rebuild:program-id".into()
        } else {
            String::new()
        },
        evidence_cid: String::new(),
    }
}

fn main() {
    // 1. Deterministic PDA derivation — no on-chain lookup needed.
    let program_id = program_id_from_hex(REGISTRY_PROGRAM_ID_HEX).expect("valid program id");
    let idx_account = index_pda(&program_id).expect("valid index seed");
    let key = [7u8; 32]; // one program's 32-byte ID
    let entry_account = entry_pda(&program_id, &key);
    println!("index PDA : {idx_account}");
    println!("entry PDA : {entry_account}");

    // 2. Types: an index holds program IDs; each entry is a Borsh record.
    //    Real code: RegistryIndex::from_account_data(&bytes) and
    //    RegistryEntry::from_account_data(&bytes) after a wallet fetch.
    let index = RegistryIndex {
        program_ids: vec![key],
    };
    println!("indexed programs: {}", index.program_ids.len());

    let entries = vec![
        sample_entry("LP-0023 Registry v5", "registry,spel", "zDv*8iXZ6U", Some([2u8; 32])),
        sample_entry("Wallet Index", "wallet,lez", "none", None),
        sample_entry("registry-browser", "registry,tool", "placeholder", Some([3u8; 32])),
    ];

    // 3. Lookup one entry by its program ID (the entry PDA seed).
    if let Some(found) = entries.iter().find(|e| e.program_id == key) {
        println!("lookup {}: {} {}", found.program_id[0], found.name, found.version);
    }

    // 4. Search filters: name is a case-insensitive substring; tags are
    //    repeatable OR-matched substrings; different kinds compose with AND.
    let by_name = search_entries_with_name(&entries, &[], None, false, Some("registry"));
    println!("by name: {:?}", by_name.iter().map(|e| e.name.as_str()).collect::<Vec<_>>());

    let by_tag = search_entries_with_name(&entries, &["spel", "lez"], None, false, None);
    println!("by tag: {:?}", by_tag.iter().map(|e| e.name.as_str()).collect::<Vec<_>>());

    let verified = search_entries_with_name(&entries, &[], None, true, None);
    println!("verified: {:?}", verified.iter().map(|e| e.name.as_str()).collect::<Vec<_>>());

    let name_and_tag = search_entries_with_name(&entries, &["registry"], None, false, Some("lp-0023"));
    println!("name AND tag: {:?}", name_and_tag.iter().map(|e| e.name.as_str()).collect::<Vec<_>>());

    // 5. Graceful IDL degradation: render metadata regardless of IDL state.
    //    Only attempt CID resolution when the pointer is actually usable.
    for e in &entries {
        let idl = match e.idl_availability() {
            IdlAvailability::Available(cid) => cid,
            IdlAvailability::Unavailable => "(no IDL)".to_string(),
        };
        // CIDs may contain '*' — escape them when placing them into URLs.
        println!("{} {} idl={} {}", e.program_id[0], e.name, idl, verification_label(e));
    }
}

/// `deployer_claimed` is a self-assertion; this is the independent check.
fn verification_label(entry: &RegistryEntry) -> &'static str {
    if entry.is_independently_verified() {
        "verified"
    } else {
        "unverified"
    }
}
