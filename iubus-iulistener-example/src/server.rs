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

use aidl_rust_codegen::binder_impls::IUBus::{BnUBus, IUBus};
use aidl_rust_codegen::binder_impls::IUListener::IUListener;
use aidl_rust_codegen::parcelable_stubs::{
    ParcelableUEntity, ParcelableUMessage, ParcelableUStatus, ParcelableUUri,
};
use binder::Interface;
use binder::{BinderFeatures, SpIBinder, Strong};
use up_rust::uprotocol::{UAttributes, UAuthority, UMessage, UResource, UUri};

pub struct TestCallingClientIUListenerService;

impl Interface for TestCallingClientIUListenerService {}

impl IUBus for TestCallingClientIUListenerService {
    fn registerClient(
        &self,
        _packageName: &str,
        entity: &ParcelableUEntity,
        _clientToken: &SpIBinder,
        _flags: i32,
        listener: &Strong<(dyn IUListener + 'static)>,
    ) -> binder::Result<ParcelableUStatus> {
        let umessage = UMessage {
            attributes: Some(UAttributes {
                source: Some(UUri {
                    authority: Some(UAuthority {
                        name: Some("super_cool_authority".to_owned()),
                        ..Default::default()
                    })
                    .into(),
                    entity: Some(entity.as_ref().clone()).into(),
                    resource: Some(UResource {
                        name: "super_cool_resource".to_owned(),
                        ..Default::default()
                    })
                    .into(),
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            })
            .into(),
            ..Default::default()
        };

        println!("umessage we're sending over onReceive(): \n{:?}", umessage);

        let res = listener.as_ref().onReceive(&umessage.into());
        println!("after calling onReceive in the server: res: {:?}", res);

        Ok(ParcelableUStatus::default())
    }

    fn unregisterClient(&self, _clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    fn send(
        &self,
        _message: &ParcelableUMessage,
        _clientToken: &SpIBinder,
    ) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    fn pull(
        &self,
        _uri: &ParcelableUUri,
        _count: i32,
        _flags: i32,
        _clientToken: &SpIBinder,
    ) -> binder::Result<Option<Vec<Option<ParcelableUMessage>>>> {
        Ok(None)
    }

    fn enableDispatching(
        &self,
        _uri: &ParcelableUUri,
        _flags: i32,
        _clientToken: &SpIBinder,
    ) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    fn disableDispatching(
        &self,
        _uri: &ParcelableUUri,
        _flags: i32,
        _clientToken: &SpIBinder,
    ) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }
}

pub fn run() -> anyhow::Result<()> {
    let test_calling_client_iulistener_service = TestCallingClientIUListenerService;
    let test_calling_client_iulistener_service_binder = BnUBus::new_binder(
        test_calling_client_iulistener_service,
        BinderFeatures::default(),
    );
    binder::add_service(
        "test-calling-client-iulistener-service",
        test_calling_client_iulistener_service_binder.as_binder(),
    )
    .expect("Failed to register service?");
    println!("Running!");
    binder::ProcessState::join_thread_pool();
    anyhow::Ok(())
}
