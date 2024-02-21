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

use aidl_rust_codegen::binder_impls::IUListener::{BnUListener, IUListener};
use aidl_rust_codegen::parcelable_stubs::ParcelableUMessage;
use binder::BinderFeatures;
use binder::Interface;

pub struct TestIUListenerService;

impl Interface for TestIUListenerService {}

impl IUListener for TestIUListenerService {
    fn onReceive(&self, event: &ParcelableUMessage) -> binder::Result<()> {
        println!("received ParcelableUMessage: {:?}", event);

        Ok(())
    }
}

pub fn run() -> anyhow::Result<()> {
    let test_iulistener_service = TestIUListenerService;
    let test_iulistener_service_binder =
        BnUListener::new_binder(test_iulistener_service, BinderFeatures::default());
    binder::add_service(
        "test-iulistener-service",
        test_iulistener_service_binder.as_binder(),
    )
    .expect("Failed to register service?");
    println!("Running!");
    binder::ProcessState::join_thread_pool();
    anyhow::Ok(())
}
