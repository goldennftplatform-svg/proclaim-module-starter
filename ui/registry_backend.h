// SPDX-License-Identifier: MIT AND Apache-2.0
//
// SKELETON ONLY — requires the Logos Basecamp toolchain and the generated C FFI
// header from the upstream repo (`registry_ffi/generated/registry.h`). No real
// FFI calls or chain reads are implemented here. This is the *shape* a module
// backend would expose to QML, not working code.

#ifndef REGISTRY_BACKEND_H
#define REGISTRY_BACKEND_H

#include <QObject>
#include <QString>
#include <QVariantList>

class RegistryBackend : public QObject
{
    Q_OBJECT

    // QML binds a ListView to this list of entry maps.
    Q_PROPERTY(QVariantList entries READ entries NOTIFY entriesChanged)

public:
    explicit RegistryBackend(QObject *parent = nullptr);

    QVariantList entries() const;

public slots:
    // TODO: call the generated C FFI (or the read-only HTTP API) and populate
    // m_entries. Keep metadata visible when an IDL CID is empty/placeholder.
    void refresh();
    void searchByName(const QString &name);       // case-insensitive substring
    void searchByTag(const QString &tag);         // OR-matched, repeatable upstream
    void searchVerified(bool verifiedOnly);       // source_verified && verifier

signals:
    void entriesChanged();
    void errorOccurred(const QString &message);

private:
    QVariantList m_entries;
};

#endif // REGISTRY_BACKEND_H
