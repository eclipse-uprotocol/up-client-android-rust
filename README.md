# up-client-android-rust
Rust client side Library for Android implementation of uProtocol

-------------

# Project layout

## `aidl-rust-codegen`

Contains two items of note:
1. The ability to generate updates to the `IUBus.aidl` and `IUListener.aidl`
   implementations if this project is cloned into a recent (Feb 2024+)
   Android Open Source Project root's `external/rust/`
     * Under `aidl-rust-codegen/aidl` + `aidl-rust-codegen/service_stub`
2. The Rust interfaces generated from `IUBus.aidl` and `IUListener.aidl`
   with some light renaming performed within `IUBus.rs` to use the unmangled
   name for `IUListener`.
     * Under `aidl-rust-codegen/binder_impl`

## `iulistener-example` + `iubus-iulistener-example`

Example code that can be compiled and run on an Android emulator to show
usage between a client and server. Makes use of `aidl-rust-codegen`.

## `up-client-android-rust`

Coming soon!

`aidl-rust-codegen` is the building blocks so that we're able to interact
with `up-android-core` through the defined `IUBus.aidl` and `IUListener.aidl`

# Role within uProtocol

Allows the pluggable uStreamer access to `up-android-core` so that an Android
device with a host transport of Binder is able to communicate with the outside
world over various other transports: Zenoh, sommR, and so on.