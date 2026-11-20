<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->

# Result — proclaim-module-starter

**This is a template, not a working verified module.** It exists so a third
party can build their own Logos Core Module against the registry. No
integration has been run here, and no prize criterion is credited.

## What was actually checked (2026-11-20, fresh clone)

| Check | Command | Result |
| --- | --- | --- |
| Clone from plain HTTPS | `git clone https://github.com/goldennftplatform-svg/proclaim-module-starter.git` | exit 0 |
| JS example syntax | `node --check clients/js/search.mjs` | exit 0 |
| Rust template parsed as a manifest | `cargo metadata --manifest-path Cargo.toml.template --no-deps` | exit 101 — expected, it is a `.template` file |
| Secret scan (`ghp_`, `github_pat_`, `PRIVATE KEY`) | literal scan of tracked files | 0 hits |
| SPDX on authored source files | `MIT AND Apache-2.0` | 9/9 |

The full record is in `proclaim-labs/verification/clean-clone/FINDINGS.md`.

## What was NOT checked

- No install, no upstream `proclaim` checkout, no `registry-api` running.
- No Rust example build, no guest build, no `.lgx` package, no `lgs` calls.
- No sequencer, no LEZ, no testnet, no Basecamp.
- No third-party developer has used this template to ship a module (0).
- No adoption, no testimonials, no prize credit.

## What would make this a real result

A third party clones the template, builds a module, points it at a live
registry on testnet 0.4, and publishes their own repo and result. Until then
this repository is reference material only.
