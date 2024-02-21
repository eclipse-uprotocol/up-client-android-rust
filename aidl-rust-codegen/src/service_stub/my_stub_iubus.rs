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

use parcelable_stubs::{ParcelableUEntity, ParcelableUMessage, ParcelableUStatus, ParcelableUUri};

/// Implementation of the `IUListener` AIDL interface.
use org_eclipse_uprotocol_core_ubus_iulistener::aidl::org::eclipse::uprotocol::core::ubus::IUListener::IUListener;
use org_eclipse_uprotocol_core_ubus_iubus::aidl::org::eclipse::uprotocol::core::ubus::IUBus::IUBus;
use org_eclipse_uprotocol_core_ubus_iulistener::binder;

use binder::{SpIBinder, Strong};

/// The `MyStubIUListener` implementation.
pub struct MyStubIUBus;

impl binder::Interface for MyStubIUBus {}

impl IUBus for MyStubIUBus {
    fn registerClient(&self, packageName: &str, entity: &ParcelableUEntity, clientToken: &SpIBinder, flags: i32, listener: &Strong<(dyn org_eclipse_uprotocol_core_ubus_iubus::mangled::_3_org_7_eclipse_9_uprotocol_4_core_4_ubus_10_IUListener + 'static)>) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    fn unregisterClient(&self, clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    fn send(&self, message: &ParcelableUMessage, clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    fn pull(&self, uri: &ParcelableUUri, count: i32, flags: i32, clientToken: &SpIBinder) -> binder::Result<Option<Vec<Option<ParcelableUMessage>>>> {
        Ok(None)
    }

    fn enableDispatching(&self, uri: &ParcelableUUri, flags: i32, clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    fn disableDispatching(&self, uri: &ParcelableUUri, flags: i32, clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }
}
