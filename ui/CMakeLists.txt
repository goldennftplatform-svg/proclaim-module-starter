# SPDX-License-Identifier: MIT AND Apache-2.0
#
# SKELETON ONLY — not a buildable Logos module on its own.
# Requires the Logos Basecamp toolchain (`logos-module-builder`), a Qt/QtQuick
# dev package, and the generated C FFI artifacts from the upstream repo. Adjust
# Qt5 vs Qt6 and the QML resource wiring to match the Basecamp version you use.

cmake_minimum_required(VERSION 3.16)
project(lp0023_registry_module_starter LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)   # needed for Q_OBJECT in registry_backend.h

# TODO: pick the Qt major version required by your Basecamp toolchain.
find_package(Qt5 COMPONENTS Quick REQUIRED)
# find_package(Qt6 COMPONENTS Quick REQUIRED)  # alternative

add_library(lp0023_registry_module_starter SHARED
    registry_backend.cpp
    registry_backend.h
)

target_link_libraries(lp0023_registry_module_starter
    PRIVATE Qt5::Quick
)

# TODO: add Main.qml as a Qt resource / qrc (or a Qt6 qml module), and add the
# generated FFI source/header (`registry_ffi/generated/registry.h`) plus any
# required include dirs and shared library link from the upstream build.

# TODO: package with logos-module-builder into a .lgx for Basecamp. The
# upstream first-party UI is still a scaffold; no installable .lgx is published.
