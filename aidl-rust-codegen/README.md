# `aidl-rust-codegen`

A small tutorial on the portion of this crate devoted to generating Rust
interfaces for the `up-client-android-java`'s `IUBus.aidl` and `IUListener.aidl`

_Note_: When developing Rust software to compile out of the Android Open Source 
Project (AOSP) it's necessary to use the Soong build system. Some details may
be omitted which are more general to Soong, but can be found via a web search.

______

# Step 1: Generating the Rust source for the AIDLs

## Clone & Repo Sync a new AOSP

Or use an existing one you have, so long as you've synced it sometime past
Feb 2024. (Some recent updates were made to support AIDL custom parcelables
in Rust which are present from that rough date forward).

## Download Updated AIDL files from `up-android-client-java`

This portion could be automated, but the next few steps are so involved that
it frankly doesn't make much sense to do so.

### Download AIDL files

Navigate to the repo for [`up-client-android-java`](https://github.com/eclipse-uprotocol/up-client-android-java)

Download the following AIDL files and place them in 
`aidl-rust-codegen/org/eclipse/uprotocol/core/ubus`
* `IUBus.aidl`
* `IUListener.aidl`

## Update `service_stub`s to match AIDL updates

In order to convince Soong to generate the Rust source files corresponding
to the AIDL files, we will build some stubs for `IUBus` and `IUListener`.
These are located under `aidl-rust-codegen/service_stub`.

_Note_: These stubs are not used in the final product of `aidl-rust-codegen`,
the true contents will later be copied into `aidl-rust-codegen/binder_impl`
from a Soong intermediate build folder.

We need to update the contents of `my_stub_iubus.rs` and `my_stub_iulistener.rs`
according to the new parameters, return values, names of functions, and so on.
So you'll now need to update `aidl-rust-codegen/service_stub` to match the new
AIDL files.

## Build the `service_stub`s

### Prep for building with Soong

As per normal when building out of the AOSP, make sure to source the 
`build/envsetup.sh` script and select whichever target you wish from `lunch`.

### Run the build

From within `aidl-rust-codegen/src` run:
```bash
m libmystubiubus
```

### Details

This will kick off Soong to go build the `service_stub/my_stub_iubus.rs`
and brings in the appropriate dependencies from the `aidl` folder so that
it will be understood how to interpret the various types. Read
`aidl-rust-codegen/src/Android.bp` and `aidl-rust-codegen/src/aidl/Android.bp`
for more details how the dependencies are listed. Android AIDL documentation
is also good, but it's lagging the most recent AIDL => Rust capabilities.

## Copy in the newly generated Rust source for AIDLs

Navigate to Soong intermediate build folder and copy generated source files
from there into `aidl-rust-codegen/binder_impl`.

Inside of:
```
<aosp-root>/out/soong/.intermediates/external/rust/<up-client-android-rust>/aidl/org.eclipse.uprotocol.ubus.iubus-rust-source/gen/org/eclipse/uprotocol/mystubiubus/
```
you should find both an `IUBus.rs` and `IUListener.rs`.

### Copying the contents

Copy the contents of those files carefully into 
`aidl-rust-codegen/src/binder_impl`, ideally using a diff tool.

Take note that there are some intentional changes made to the generated
`IUBus.rs` and `IUListener.rs` which are already here, namely:

#### Ensuring `UnstructuredParcelable`s have their definitions found

The inclusion of:
```rust
use crate::parcelable_stubs;
```
at the top of the file, to ensure that the binder implementations can
find the definitions of the `UnstructuredParcelable`s defined in 
`aidl-rust-codegen/aidl/parcelable_stubs/`. These are necessary because
we have defined the serialization and deserialization of several `UFoo`
we communicate over the `IUBus` via Protobuf.

#### Ensuring `IUListener` can be found in `IUListener.rs` from `IUBus.rs`

Originally, the mangled name of the `IUListener` is used in `IUBus.rs`
in the generated Rust source but it's quite unpleasant to have to type and
use, so we have replaced each instance of the mangled name with `IUListener`
in `IUBus.rs`.

_______

# Step 2: Updating `up-client-android-rust` and examples as necessary

Given that we have just now retrieved and carefully updated the contents
of `aidl-rust-codegen/src/binder_impl`, if we go to try to build, we will
likely encounter errors as the example code and `up-client-android-rust`

The scope of these changes varies, but this portions is fairly "normal"
and we can ensure that we can now compile using `cargo-ndk` with a command
like the follow:
```bash
cargo ndk -t x86_64 --platform=29 --bindgen build
```

Note that in order to build in this fashion you must have already installed
* An Android NDK installed and exported as:
  * ```bash
    export ANDROID_NDK_HOME="/home/<your-username>/Android/Sdk/ndk"
    ```
* `cargo-ndk`
* The relevant `rustup` toolchains
  * See [here](https://doc.rust-lang.org/rustc/platform-support/android.html)
    for a list of available targets. If you are building and testing on your
    local computer with an Android emulator, you'd likely choose `x86_64-linux-android`
    as you're likely to see better emulator performance than picking a different
    architecture
* A `libbinder_ndk.so` which has been built from source from the AOSP
  * We need this because there are certain symbols stripped from the
    distributed version of this shared library which we need access to
    in support of usage of `libbinder_rs`.
  * You'd copy this into the appropriate sysroot. In the case of the above
    build targeting API level 29, we'd copy it from:
    * ```
      <aosp-root>/out/target/product/generic_x86_64/system/lib/libbinder_ndk.so
      ```
  * Into:
    * ```
      /home/<your-username>/Android/Sdk/ndk/<ndk-version>/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/x86_64-linux-android/29/
      ```
  * _Note_ how the architecture in the `cargo-ndk` command lines up with where
    we look in Soong's build output per architecture and which API level 
    folder we place `libbinder_ndk.so` in when put in our `ANDROID_NDK_HOME`
