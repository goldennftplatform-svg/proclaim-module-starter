// SPDX-License-Identifier: MIT AND Apache-2.0
//
// SKELETON ONLY — NOT AN INSTALLABLE LOGOS UI MODULE.
// This shows the *shape* of a Logos Basecamp QML module that reads registry
// entries. It needs the Logos Basecamp toolchain (logos-module-builder), a real
// Qt binding and the C++ backend below wired via setContextProperty before it
// will run. No registry data is fetched by this file by itself.

import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root
    width: 640
    height: 480

    // Set from C++: engine.rootContext()->setContextProperty("registryBackend", &backend);
    property var backend: null

    Column {
        anchors.fill: parent
        anchors.margins: 12
        spacing: 8

        Row {
            spacing: 8
            TextField {
                id: nameField
                width: 240
                placeholderText: "name substring"
            }
            Button {
                text: "Search by name"
                enabled: root.backend !== null
                onClicked: root.backend.searchByName(nameField.text)
            }
            Button {
                text: "Tag: spel"
                enabled: root.backend !== null
                onClicked: root.backend.searchByTag("spel")
            }
            Button {
                text: "Verified only"
                enabled: root.backend !== null
                onClicked: root.backend.searchVerified(true)
            }
            Button {
                text: "Refresh"
                enabled: root.backend !== null
                onClicked: root.backend.refresh()
            }
        }

        Text {
            text: "Registry entries (metadata renders even when an IDL is unavailable)"
            font.bold: true
        }

        ListView {
            id: list
            width: parent.width
            height: root.height - 120
            clip: true
            model: root.backend ? root.backend.entries : []
            delegate: Item {
                width: list.width
                height: 52
                Column {
                    Text { text: modelData.name + "  " + modelData.version; font.bold: true }
                    Text {
                        text: modelData.program_id + "  idl="
                              + (modelData.idlAvailable ? modelData.idlCid : "(no IDL)")
                              + "  " + (modelData.verified ? "verified" : "unverified")
                    }
                }
            }
        }
    }

    // Non-fatal backend errors must not blank the view.
    Connections {
        target: root.backend
        function onErrorOccurred(message) { statusLabel.text = message; }
    }
    Text { id: statusLabel; anchors.bottom: parent.bottom; anchors.margins: 12; text: "" }
}
