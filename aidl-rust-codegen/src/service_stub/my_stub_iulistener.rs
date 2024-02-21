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

use parcelable_stubs::ParcelableUMessage;

/// Implementation of the `IUListener` AIDL interface.
use org_eclipse_uprotocol_core_ubus_iulistener::aidl::org::eclipse::uprotocol::core::ubus::IUListener::IUListener;
use org_eclipse_uprotocol_core_ubus_iulistener::binder;

/// The `MyStubIUListener` implementation.
pub struct MyStubIUListener;

impl binder::Interface for MyStubIUListener {}

impl IUListener for MyStubIUListener {
    fn onReceive(&self, event: &ParcelableUMessage) -> binder::Result<()> {
        Ok(())
    }
}
