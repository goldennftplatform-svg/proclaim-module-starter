<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->

# LP-0023 registry — starter template for an independent Logos UI module

This is a **starter template**: copyable source, manifests and checklists that an
independent developer can use to build a Logos UI module which *reads* the LEZ
Program Registry. It is **not** a third-party module, it is **not** an
installable `.lgx`, and it is **not** adoption evidence.

Upstream registry repo: <https://github.com/goldennftplatform-svg/proclaim>
(dual-licensed `MIT AND Apache-2.0`).

## Who this is for

You are an independent developer with your own public forge account (GitHub,
GitLab, Codeberg, …). You want to:

- write a small Logos UI module (Qt/QML or a JS client) that lists, looks up and
  filters registry entries; and/or
- deploy your **own** LEZ program, register it, and publish honest evidence of
  that work.

You do not need permission to use this template. You do need your own identity,
your own repository, and your own program to make the work count as independent.

## Honest current state of the upstream project

Read this before you plan anything:

- The full lifecycle (deploy → register → search → independent `mark_verified`
  → inspect) works on **localnet only**.
- **Testnet 0.4 is pending**; there is no testnet deployment yet.
- The first-party `ui/registry/` module is a **scaffold**. An installable,
  published `.lgx` package is **pending**.
- Independent third-party UI modules integrating the SDK: **0/5** as of the
  2026-11-19 snapshot in `ADOPTION_TRACKER.md`. There is no adoption to assume.

## Using this template does NOT count as third-party adoption

Copying these files, filling in the placeholders, or running the examples
locally does **not** by itself satisfy any LP-0023 adoption criterion. A
template supplied by the registry project is first-party scaffolding; adoption
means an **independent** author did real work and published durable evidence of
it. In particular:

- Merely using `Cargo.toml.template`, `examples/register_and_search.rs`,
  `clients/js/search.mjs` or `ui/` is not an integration.
- Registering the registry's own example program is not your program.
- A localnet-only result is a localnet result. It must be labelled as such and
  must not be presented as official-zone/testnet adoption.
- `deployer_claimed` is a self-assertion, not proof of independent use.
- A self-verification (`verifier` equal to `author`) is rejected on-chain and
  would not count even if it were accepted.

## Exact evidence an independent author must publish

To be counted honestly, publish **all** of the following from your own account:

1. **A public forge repository** under your own account, containing the module
   or program source. It must have a **genuine, spread-out commit history**:
   several distinct commits authored by you over real intervals, showing the
   work as it happened. A single squashed import, a bulk copy of this template,
   or a one-shot "hello" commit with no follow-up is not spread-out history and
   will not be counted as independent adoption.
2. **The program you register**: your own LEZ program (not the registry's demo
   program), with its 32-byte program ID recorded in both hex and base58, and
   the exact commit/archives that produced the deployed ELF present in that
   public repository.
3. **The registry entry transaction**: the transaction hash for your `register`
   call (and, if you did one, the `mark-verified` transaction), recorded on the
   same network you actually used. Label it `localnet` while it is localnet.
4. **A timestamped public link**: a public post, release note, README section or
   module listing, from your own account, with a visible date, that points to
   the repository and states what you built and what you registered. Keep the
   link durable; a private message or a screenshot with no public URL is not
   evidence.

Also required by the tracker's counting rules:

- one author per distinct account/identity — multiple registrations from the
  same account do not increase the independent-author count;
- the verifier account must differ from the author account;
- no alternate accounts to simulate multiple authors or testimonials;
- no bulk imports to simulate volume;
- no paid or incentivized posts presented as organic;
- no testimonials written by the project's own accounts or cross-posted as if
  they came from different people.

See `docs/INTEGRATION-CHECKLIST.md` for the full clone-to-entry walkthrough and
the anti-manufacturing rules in detail.

## Contents

| Path | What it is | Status |
| --- | --- | --- |
| `README.md` | This file: scope, audience, honesty statement, evidence requirements | Informational |
| `Cargo.toml.template` | Rust manifest **template** with placeholders; copy and edit | Not a real crate manifest |
| `src/lib.rs` | Minimal crate root showing `registry_core` types/filters | Skeleton, no chain I/O |
| `examples/register_and_search.rs` | PDA derivation, types, lookup and search filters | Runnable-style example; needs upstream deps |
| `clients/js/search.mjs` | Zero-dependency ESM example using the published JS client API | Example only |
| `clients/js/README.md` | How to obtain the client and run the example | Informational |
| `ui/Main.qml` | QML view shape for a registry-reading module | Skeleton |
| `ui/registry_backend.h` / `ui/registry_backend.cpp` | C++/FFI backend shape | Skeleton, no working FFI |
| `ui/CMakeLists.txt` | Build-file skeleton | Skeleton |
| `docs/INTEGRATION-CHECKLIST.md` | Clone → disposable sequencer → register → evidence | Checklist |
| `LICENSE-MIT`, `LICENSE-APACHE` | Dual-license texts / pointers | Legal |
| `RESULT.md` | Files, validation actually performed, scope disclaimers | Report |

## Quick orientation

- **Rust/sdk path:** copy `Cargo.toml.template` to `Cargo.toml`, replace every
  `<...>` placeholder, then read `examples/register_and_search.rs`. Do **not**
  expect a `registry_core` crate on crates.io; the template points at the git
  repository and documents where to copy the exact pinned dependencies from.
- **JS path:** from a checkout of the upstream repo, `npm install ./clients/js`,
  then run `clients/js/search.mjs` against a running `registry-api`.
- **UI path:** `ui/` is deliberately only a skeleton. You need the Logos
  Basecamp toolchain (`logos-module-builder`) and the generated C FFI header
  from the upstream repo to turn it into a real module.

All authored text and source files carry the SPDX identifier
`MIT AND Apache-2.0`.
