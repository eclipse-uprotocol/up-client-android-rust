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

use aidl_rust_codegen::binder_impls::IUListener::IUListener;
use aidl_rust_codegen::parcelable_stubs::ParcelableUMessage;
use binder::Strong;

use up_rust::uprotocol::{UAttributes, UAuthority, UEntity, UMessage, UResource, UUri};

pub fn run() -> anyhow::Result<()> {
    let test_iulistener_service: Strong<dyn IUListener> =
        binder::get_interface("test-iulistener-service").unwrap();
    println!("Do onReceive()");
    let umessage = UMessage {
        attributes: Some(UAttributes {
            source: Some(UUri {
                authority: Some(UAuthority {
                    name: Some("my_vin".to_owned()),
                    ..Default::default()
                })
                .into(),
                entity: Some(UEntity {
                    name: "door".to_owned(),
                    ..Default::default()
                })
                .into(),
                resource: Some(UResource {
                    name: "front_left".to_owned(),
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
    let parcelable_umessage = ParcelableUMessage::from(umessage);
    println!(
        "parcelable_umessage prior to onReceive: {:?}",
        parcelable_umessage
    );
    let res = test_iulistener_service
        .onReceive(&parcelable_umessage)
        .expect("Failed to trigger onReceive");
    println!("Got result: {:?}", res);
    println!("Done!");
    Ok(())
}
