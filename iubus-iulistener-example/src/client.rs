/*
 * Copyright (c) 2024 General Motors GTO LLC
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 * SPDX-FileType: SOURCE
 * SPDX-FileCopyrightText: 2023 General Motors GTO LLC
 * SPDX-License-Identifier: Apache-2.0
 */

use binder::{binder_impl::Binder, BinderFeatures, Interface, Strong};

use aidl_rust_codegen::binder_impls::IUBus::IUBus;
use aidl_rust_codegen::binder_impls::IUListener::{BnUListener, IUListener};
use aidl_rust_codegen::parcelable_stubs::ParcelableUMessage;

use up_rust::uprotocol::UEntity;

pub struct MyIUListener;

impl Interface for MyIUListener {}

impl IUListener for MyIUListener {
    fn onReceive(&self, event: &ParcelableUMessage) -> binder::Result<()> {
        println!("received ParcelableUMessage: {:?}", event);
        Ok(())
    }
}

pub fn run() -> anyhow::Result<()> {
    let test_calling_client_iulistener_service: Strong<dyn IUBus> =
        binder::get_interface("test-calling-client-iulistener-service").unwrap();

    let my_package_name = "super_cool_client";
    let uentity = UEntity {
        name: "ustreamer_glue".to_string(),
        version_major: Some(1),
        ..Default::default()
    };

    println!(
        "my_entity we're sending over with registerClient():\n{:?}",
        uentity
    );

    let my_flags: i32 = 0;

    let my_iulistener = MyIUListener;
    let my_iulistener_binder = BnUListener::new_binder(my_iulistener, BinderFeatures::default());
    println!("Running!");

    let client_token = Binder::new(()).as_binder();

    println!("Call registerClient()");
    let _parcelable_ustatus = test_calling_client_iulistener_service.registerClient(
        &my_package_name,
        &uentity.into(),
        &client_token,
        my_flags,
        &my_iulistener_binder,
    );
    binder::ProcessState::join_thread_pool();

    anyhow::Ok(())
}
