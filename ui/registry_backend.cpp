// SPDX-License-Identifier: MIT AND Apache-2.0
//
// SKELETON ONLY — NOT FUNCTIONAL.
// Wire this up with the Logos Basecamp toolchain and the generated C FFI from
// the upstream repository (registry_ffi/generated/registry.h, produced by
// `make ffi-gen`), or call the read-only JSON API on 127.0.0.1:3030.
// The methods below intentionally do no chain I/O and emit a clear
// "not implemented" error so nobody mistakes this for a working integration.

#include "registry_backend.h"

#include <QVariantMap>

RegistryBackend::RegistryBackend(QObject *parent) : QObject(parent) {}

QVariantList RegistryBackend::entries() const
{
    return m_entries;
}

void RegistryBackend::refresh()
{
    // TODO: fetch /entries (or the FFI equivalent), decode each RegistryEntry,
    // and expose fields including program_id, name, version, idl_cid,
    // source_verified and verifier. Emit entriesChanged() on success.
    emit errorOccurred(QStringLiteral("skeleton: refresh() is not implemented"));
}

void RegistryBackend::searchByName(const QString &name)
{
    // TODO: apply the same case-insensitive substring semantics as
    // registry_core::matches_name / GET /search?name=...
    Q_UNUSED(name);
    emit errorOccurred(QStringLiteral("skeleton: searchByName() is not implemented"));
}

void RegistryBackend::searchByTag(const QString &tag)
{
    // TODO: apply OR-matched, case-insensitive tag semantics as in
    // registry_core::matches_any_tag / repeatable GET /search?tag=...
    Q_UNUSED(tag);
    emit errorOccurred(QStringLiteral("skeleton: searchByTag() is not implemented"));
}

void RegistryBackend::searchVerified(bool verifiedOnly)
{
    // TODO: filter with is_independently_verified(): source_verified must be
    // true AND verifier must be non-empty. deployer_claimed is not proof.
    Q_UNUSED(verifiedOnly);
    emit errorOccurred(QStringLiteral("skeleton: searchVerified() is not implemented"));
}
