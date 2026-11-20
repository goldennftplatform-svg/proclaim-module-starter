// SPDX-License-Identifier: MIT AND Apache-2.0
//
// Zero-dependency ESM example for the published `clients/js` API shape of the
// LP-0023 read-only registry client (`@lp0023/registry-client`). The client
// itself has no runtime dependencies and uses global fetch (Node 18+).
//
// Install from a checkout of https://github.com/goldennftplatform-svg/proclaim:
//   npm install ./clients/js
// Then, with `registry-api` running on 127.0.0.1:3030:
//   node search.mjs
//
// No Cargo.toml or package.json is included here on purpose: this is an
// example, not a publishable package. Do not present running it as adoption.

import { RegistryClient, RegistryApiError, idlAvailability } from '@lp0023/registry-client';

const baseUrl = process.env.REGISTRY_API_URL ?? 'http://127.0.0.1:3030';
const client = new RegistryClient(baseUrl);

// Render metadata first; IDL resolution is a separate, optional step.
function render(entry) {
  const idl = idlAvailability(entry) === 'available' ? entry.idl_cid : '(no IDL)';
  const verified = entry.source_verified && entry.verifier ? 'verified' : 'unverified';
  console.log(`${entry.program_id}  ${entry.name} ${entry.version}  idl=${idl}  ${verified}`);
}

function label(err) {
  return err instanceof RegistryApiError
    ? `API ${err.status}: ${JSON.stringify(err.body)}` // status 0 = fetch failed
    : String(err);
}

try {
  // Construct client / health probe.
  const health = await client.health();
  console.log('health:', health.status);

  // Lookup: index and full entry list.
  const ids = await client.index();
  console.log(`indexed programs: ${ids.length}`);
  const all = await client.entries();
  console.log(`entries: ${all.length}`);

  // Search: name is a case-insensitive substring; tags repeat with OR;
  // verified returns only independently verified entries; limit caps results.
  const byName = await client.search({ name: 'registry', limit: 10 });
  console.log(`by name: ${byName.length}`);
  byName.forEach(render);

  const byTag = await client.search({ tags: ['spel', 'lez'], limit: 10 });
  console.log(`by tag: ${byTag.length}`);
  byTag.forEach(render);

  const verified = await client.search({ verified: true, limit: 10 });
  console.log(`verified: ${verified.length}`);
  verified.forEach(render);

  // Single-entry lookup by base58 program ID. A missing entry throws a
  // RegistryApiError (404), so handle it without aborting the whole demo.
  const sampleId = ids[0] ?? '6FMaVQ6CmbAVNHgwsmmeBe5kiiKkQRFtqu4GUQ2vAf2T';
  try {
    const one = await client.entry(sampleId);
    console.log('one entry:', one.name, one.commit, one.verifier ?? 'unverified');
    render(one);
  } catch (err) {
    console.log(`entry lookup skipped (${label(err)})`);
  }
} catch (err) {
  // RegistryApiError carries `status` (0 for a failed fetch) and parsed `body`.
  console.error(label(err));
  process.exitCode = 1;
}
