<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->

# Integration checklist — clone to registered entry (independent developer)

This checklist is for an **independent** author. It assumes you have read the
root `README.md` and accept that using this template is **not** adoption. Every
step below is performed by you, from your own account, on infrastructure you
control. Nothing here submits a transaction on a public network: the upstream
project is **localnet only**, with **testnet 0.4 pending**.

## 0. Before you start

- [ ] You have your own public forge account and a new repository for your work.
- [ ] You understand the honest baseline: third-party UI modules **0/5**;
      registered programs **1/50** (the registry's own entry, localnet);
      distinct independent authors **1/20**. Do not inflate any count.
- [ ] You have Rust + the risc0 toolchain, the LSSA wallet CLI (`wallet`), and
      `logos-scaffold` (`lgs`) available. No Docker is required for the local
      toolchain workflow described in the upstream `LP0023.md`.
- [ ] You will use **disposable** accounts with no real funds.

## 1. Clone and orient

```bash
git clone https://github.com/goldennftplatform-svg/proclaim.git
cd proclaim
# Read these before changing anything:
#   README.md, LP0023.md, docs/INTEGRATION.md, clients/js/README.md
```

Record the upstream commit SHA you built against (`git rev-parse HEAD`) — it is
part of your evidence trail.

## 2. Get a disposable sequencer

There is **no public always-on sequencer endpoint** and no confirmed public
prebuilt binary/container. Use a fresh, short-lived standalone LEZ sequencer;
do not claim a public testnet deployment.

```bash
# Builds/pins the standalone sequencer, wallet, spel and spel-client-gen;
# copies the LEZ debug wallet config into .scaffold/wallet and seeds an account.
lgs setup

# Verify the test node is healthy before starting it.
lgs test-node prepare
lgs test-node doctor

# Start a fresh real standalone sequencer on the LEZ debug port 3040.
lgs test-node start --port 3040

# Inspect it (the node selector is the basename of state_dir, not an "id").
lgs test-node status --node <selector> --json
```

Notes:

- Port **3040** is the LEZ debug wallet and v0.2.4 sequencer default. The
  upstream `scaffold.toml`'s `3041` is a known discrepancy; do not mix the two.
- `lgs wallet --` and `lgs spel --` set `LEE_WALLET_HOME_DIR` and
  `NSSA_WALLET_HOME_DIR` for the child process. When running host tools
  directly, export `LEE_WALLET_HOME_DIR="$PWD/.scaffold/wallet"` yourself.
- Use the **public disposable LEZ debug identities** documented upstream (LEZ
  v0.2.4 `Justfile` / debug config). They hold no real funds and are only valid
  on a fresh local node. **Do not copy any private key into your repository,
  this template, an issue or a post.** Treat all keys as sensitive by default.
- Host-side helpers need `RUSTFLAGS='--cfg getrandom_backend="custom"'`.

## 3. Prepare and register **your own** program

1. Build your guest and generate its IDL (`make idl` → `registry-idl.json`).
2. Record the 32-byte program ID in hex and base58
   (`lgs spel -- program-id <bin> --format hex`). This is your `--key`.
3. Upload the IDL and a source archive of the exact commit to Logos Storage via
   `tools/launch_storage_host.sh` + `tools/qtro_storage_upload.py` (QtRO on a
   Unix socket, **not** HTTP). Capture the CIDs and **quote them** — Logos CIDs
   can contain `*`, e.g. `'zDv*8iXZ6U'`.
4. Derive PDAs locally (no on-chain lookup needed):

   ```bash
   PID=4df7d5396b68a16aae6d0f768ef447a0055eb43866827154f71139e486265144
   lgs spel -- pda entry --key "$PID"
   lgs spel -- pda index   # 9mr5hePKo24ZHNztzVpuRND7PeAsR17yQ2M5HHPKcbw3
   ```

5. Register once, from your own author account. The global index may need a
   one-time `init-index --authority "$AUTHOR"`.

   ```bash
   lgs spel -- register --key "$PID" \
     --name "<YOUR PROGRAM NAME>" --version "0.1.0" \
     --description "<ONE-LINE PURPOSE>" \
     --tags "registry,lez,<YOUR TAGS>" \
     --idl-cid "$IDL_CID" --code-cid "$CODE_CID" \
     --commit "$COMMIT" \
     --build-config "guest=local,risc0=3.0.5,target=riscv32im-risc0-zkvm-elf" \
     --timestamp "$(date +%s)" \
     --deployer-claimed true --author "$AUTHOR"
   ```

   Leave `source_verified`, `verifier`, `verification_method` and `evidence_cid`
   empty at registration. Only the original `author` may later `update`.

6. **Capture the transaction hash** from the `register` submission. This is a
   required evidence item. Label it `localnet` until testnet 0.4 exists.

## 4. Optional but valuable: independent verification

A **different account** (never your registering `author`) fetches `code_cid`,
rebuilds from `commit`/`build_config`, wraps the ELF in R0BF, derives the
program ID and compares it. `scripts/verify-source.sh` automates the clean-room
rebuild and prints a ready-to-submit `mark-verified` command plus an attestation
template. Record the `mark-verified` tx hash too. The guest rejects
self-verification (`verifier` must differ from `author`).

A source-changing `update` clears any prior attestation — re-verify afterwards.

## 5. Validate reads (no transactions)

```bash
source ~/.cargo/env
export RUSTFLAGS='--cfg getrandom_backend="custom"'
export LEE_WALLET_HOME_DIR="$PWD/.scaffold/wallet"
make api    # read-only JSON API on 127.0.0.1:3030

curl -s http://127.0.0.1:3030/health
curl -s http://127.0.0.1:3030/index
curl -sG http://127.0.0.1:3030/search --data-urlencode 'name=<YOUR NAME>' \
  --data-urlencode 'tag=<YOUR TAG>' --data-urlencode 'verified=true'
curl -s "http://127.0.0.1:3030/entry/<YOUR BASE58 PROGRAM ID>"
```

Then, in your own copy of this template:

- copy `Cargo.toml.template` → `Cargo.toml`, fill placeholders, and run
  `cargo run --example register_and_search`;
- `npm install ./clients/js` (from the upstream checkout) and run
  `clients/js/search.mjs` with `REGISTRY_API_URL=http://127.0.0.1:3030`;
- confirm an entry with an empty/placeholder IDL still renders (IDL state must
  never break metadata rendering).

## 6. Build your module

- Rust host: use `registry_core` types, `PDA` helpers and
  `search_entries_with_name` as shown in `examples/register_and_search.rs`.
- JS/UI: use the client shape in `clients/js/search.mjs`.
- QML/C++: treat `ui/` as a skeleton only. You need the Logos Basecamp
  toolchain (`logos-module-builder`) and the generated FFI header to produce an
  installable `.lgx`; none is published yet upstream.

Commit this work to **your** repository with a genuine, spread-out commit
history (see §8). Do not commit private keys, tokens or secrets.

## 7. What to record as evidence

| # | Evidence item | Where to publish |
| --- | --- | --- |
| 1 | Public forge repo URL, owned by you | Your forge profile / README |
| 2 | Genuine, spread-out commit history (several commits by you over time) | That repo's public commit log |
| 3 | Program name + 32-byte ID in hex and base58 | Repo README and your public post |
| 4 | `register` transaction hash, labelled `localnet` (and `mark-verified` tx if any) | Repo README / evidence log |
| 5 | Timestamped public link describing what you built and registered | Your own post/release/readme with a visible date |
| 6 | Network label (`localnet`, not testnet/official zone) | Alongside every tx hash and link |

Keep the link and its date in the same place you claim the item. A localnet
record must be labelled localnet and must not be presented as official-zone
adoption.

## 8. Honest anti-manufacturing rules

These are hard rules, not suggestions:

- **No alternate accounts.** One author per distinct identity. Do not create
  sockpuppets to simulate multiple authors, verifiers, or supporters. Repeated
  registrations from one account do not increase the distinct-author count.
- **No bulk imports.** Do not seed fake registrations or mass-import entries to
  simulate volume. Registrations must come from developers actually deploying
  their own programs. A bulk copy of this template with one commit is not a
  spread-out history and is not independent work.
- **No self-verification.** `verifier` must be a different account from
  `author`. `deployer_claimed` is a self-assertion and is not evidence of
  independent use.
- **No paid/incentivized posts presented as organic.** If a post was paid for
  or incentivized, say so explicitly; never present it as an organic
  testimonial.
- **No self-testimonials or cross-posting.** Testimonials must be written and
  posted by a user other than the submitter, from their own account, with a
  link. Do not write, rewrite or cross-post testimonials from the project's own
  accounts, and do not count one person's Discord and X posts as two
  independent supporters without noting they are from one person.
- **No fabricated results.** Do not claim a test, build, cycle count, testnet
  deployment or `.lgx` release that you did not actually run or publish.
- **No secrets.** Never publish private keys, tokens or capability tokens in a
  repo, issue, post or evidence file.

## 9. Definition of done (for your own work)

- [ ] My own program is deployed and registered; tx hash recorded and labelled.
- [ ] My public repo has a genuine, spread-out commit history authored by me.
- [ ] My module/client reads entries and degrades gracefully on missing IDLs.
- [ ] A timestamped public link from my own account points to the work.
- [ ] I did not manufacture volume, verification, accounts or testimonials.

This checklist is guidance for honest work. Filling it in does not itself
change any LP-0023 adoption count; an independent reviewer decides that from
the published evidence.
