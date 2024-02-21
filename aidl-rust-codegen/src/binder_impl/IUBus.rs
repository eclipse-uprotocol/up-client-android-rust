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

/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org.eclipse.uprotocol.core.ubus.iubus-rust-source/gen/org/eclipse/uprotocol/core/ubus/IUBus.rs.d -o out/soong/.intermediates/external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org.eclipse.uprotocol.core.ubus.iubus-rust-source/gen -Nexternal/rust/up-client-android-rust/aidl-rust-codegen/src/aidl external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org/eclipse/uprotocol/core/ubus/IUBus.aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;

use crate::parcelable_stubs;

declare_binder_interface! {
  IUBus["org.eclipse.uprotocol.core.ubus.IUBus"] {
    native: BnUBus(on_transact),
    proxy: BpUBus {
    },
    async: IUBusAsync,
  }
}
pub trait IUBus: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUBus" }
  fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  fn r#unregisterClient(&self, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  fn r#send(&self, _arg_message: &parcelable_stubs::ParcelableUMessage, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  fn r#pull(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>>;
  fn r#enableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  fn r#disableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  fn getDefaultImpl() -> IUBusDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IUBusDefaultRef) -> IUBusDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IUBusAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUBus" }
  fn r#registerClient<'a>(&'a self, _arg_packageName: &'a str, _arg_entity: &'a parcelable_stubs::ParcelableUEntity, _arg_clientToken: &'a binder::SpIBinder, _arg_flags: i32, _arg_listener: &'a binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>>;
  fn r#unregisterClient<'a>(&'a self, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>>;
  fn r#send<'a>(&'a self, _arg_message: &'a parcelable_stubs::ParcelableUMessage, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>>;
  fn r#pull<'a>(&'a self, _arg_uri: &'a parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>>>;
  fn r#enableDispatching<'a>(&'a self, _arg_uri: &'a parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>>;
  fn r#disableDispatching<'a>(&'a self, _arg_uri: &'a parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>>;
}
#[::async_trait::async_trait]
pub trait IUBusAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUBus" }
  async fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  async fn r#unregisterClient(&self, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  async fn r#send(&self, _arg_message: &parcelable_stubs::ParcelableUMessage, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  async fn r#pull(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>>;
  async fn r#enableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  async fn r#disableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
}
impl BnUBus {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IUBus>
  where
    T: IUBusAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
    }
    impl<T, R> IUBus for Wrapper<T, R>
    where
      T: IUBusAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
        self._rt.block_on(self._inner.r#registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener))
      }
      fn r#unregisterClient(&self, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
        self._rt.block_on(self._inner.r#unregisterClient(_arg_clientToken))
      }
      fn r#send(&self, _arg_message: &parcelable_stubs::ParcelableUMessage, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
        self._rt.block_on(self._inner.r#send(_arg_message, _arg_clientToken))
      }
      fn r#pull(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>> {
        self._rt.block_on(self._inner.r#pull(_arg_uri, _arg_count, _arg_flags, _arg_clientToken))
      }
      fn r#enableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
        self._rt.block_on(self._inner.r#enableDispatching(_arg_uri, _arg_flags, _arg_clientToken))
      }
      fn r#disableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
        self._rt.block_on(self._inner.r#disableDispatching(_arg_uri, _arg_flags, _arg_clientToken))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IUBusDefault: Send + Sync {
  fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#unregisterClient(&self, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#send(&self, _arg_message: &parcelable_stubs::ParcelableUMessage, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#pull(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#enableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#disableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#registerClient: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#unregisterClient: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#send: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const r#pull: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const r#enableDispatching: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const r#disableDispatching: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
}
pub type IUBusDefaultRef = Option<std::sync::Arc<dyn IUBusDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IUBusDefaultRef> = std::sync::Mutex::new(None);
impl BpUBus {
  fn build_parcel_registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<binder::binder_impl::Parcel> {

    let mut aidl_data = self.binder.prepare_transact()?;
    info!("before packageName: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    aidl_data.write(_arg_packageName)?;
    info!("after packageName: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    info!("before using write() on uentity: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    aidl_data.write(_arg_entity)?;
    info!("after using write() on uentity: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    info!("before clientToken len: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    aidl_data.write(_arg_clientToken)?;
    info!("after clientToken len: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    info!("before flags len: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    aidl_data.write(&_arg_flags)?;
    info!("after flags len: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    info!("before listener len: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    aidl_data.write(_arg_listener)?;
    info!("after listener len: data_position: {} data_size: {}", aidl_data.get_data_position(), aidl_data.get_data_size());
    Ok(aidl_data)
  }
  fn read_response_registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IUBus>::getDefaultImpl() {
        return _aidl_default_impl.r#registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: parcelable_stubs::ParcelableUStatus = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_unregisterClient(&self, _arg_clientToken: &binder::SpIBinder) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_clientToken)?;
    Ok(aidl_data)
  }
  fn read_response_unregisterClient(&self, _arg_clientToken: &binder::SpIBinder, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IUBus>::getDefaultImpl() {
        return _aidl_default_impl.r#unregisterClient(_arg_clientToken);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: parcelable_stubs::ParcelableUStatus = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_send(&self, _arg_message: &parcelable_stubs::ParcelableUMessage, _arg_clientToken: &binder::SpIBinder) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_message)?;
    aidl_data.write(_arg_clientToken)?;
    Ok(aidl_data)
  }
  fn read_response_send(&self, _arg_message: &parcelable_stubs::ParcelableUMessage, _arg_clientToken: &binder::SpIBinder, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IUBus>::getDefaultImpl() {
        return _aidl_default_impl.r#send(_arg_message, _arg_clientToken);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: parcelable_stubs::ParcelableUStatus = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_pull(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_uri)?;
    aidl_data.write(&_arg_count)?;
    aidl_data.write(&_arg_flags)?;
    aidl_data.write(_arg_clientToken)?;
    Ok(aidl_data)
  }
  fn read_response_pull(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IUBus>::getDefaultImpl() {
        return _aidl_default_impl.r#pull(_arg_uri, _arg_count, _arg_flags, _arg_clientToken);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_enableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_uri)?;
    aidl_data.write(&_arg_flags)?;
    aidl_data.write(_arg_clientToken)?;
    Ok(aidl_data)
  }
  fn read_response_enableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IUBus>::getDefaultImpl() {
        return _aidl_default_impl.r#enableDispatching(_arg_uri, _arg_flags, _arg_clientToken);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: parcelable_stubs::ParcelableUStatus = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_disableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_uri)?;
    aidl_data.write(&_arg_flags)?;
    aidl_data.write(_arg_clientToken)?;
    Ok(aidl_data)
  }
  fn read_response_disableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IUBus>::getDefaultImpl() {
        return _aidl_default_impl.r#disableDispatching(_arg_uri, _arg_flags, _arg_clientToken);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: parcelable_stubs::ParcelableUStatus = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl IUBus for BpUBus {
  fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    let _aidl_data = self.build_parcel_registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#registerClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener, _aidl_reply)
  }
  fn r#unregisterClient(&self, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    let _aidl_data = self.build_parcel_unregisterClient(_arg_clientToken)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#unregisterClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_unregisterClient(_arg_clientToken, _aidl_reply)
  }
  fn r#send(&self, _arg_message: &parcelable_stubs::ParcelableUMessage, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    let _aidl_data = self.build_parcel_send(_arg_message, _arg_clientToken)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#send, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_send(_arg_message, _arg_clientToken, _aidl_reply)
  }
  fn r#pull(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>> {
    let _aidl_data = self.build_parcel_pull(_arg_uri, _arg_count, _arg_flags, _arg_clientToken)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#pull, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_pull(_arg_uri, _arg_count, _arg_flags, _arg_clientToken, _aidl_reply)
  }
  fn r#enableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    let _aidl_data = self.build_parcel_enableDispatching(_arg_uri, _arg_flags, _arg_clientToken)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#enableDispatching, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_enableDispatching(_arg_uri, _arg_flags, _arg_clientToken, _aidl_reply)
  }
  fn r#disableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    let _aidl_data = self.build_parcel_disableDispatching(_arg_uri, _arg_flags, _arg_clientToken)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#disableDispatching, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_disableDispatching(_arg_uri, _arg_flags, _arg_clientToken, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IUBusAsync<P> for BpUBus {
  fn r#registerClient<'a>(&'a self, _arg_packageName: &'a str, _arg_entity: &'a parcelable_stubs::ParcelableUEntity, _arg_clientToken: &'a binder::SpIBinder, _arg_flags: i32, _arg_listener: &'a binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>> {
    let _aidl_data = match self.build_parcel_registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#registerClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener, _aidl_reply)
      }
    )
  }
  fn r#unregisterClient<'a>(&'a self, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>> {
    let _aidl_data = match self.build_parcel_unregisterClient(_arg_clientToken) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#unregisterClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_unregisterClient(_arg_clientToken, _aidl_reply)
      }
    )
  }
  fn r#send<'a>(&'a self, _arg_message: &'a parcelable_stubs::ParcelableUMessage, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>> {
    let _aidl_data = match self.build_parcel_send(_arg_message, _arg_clientToken) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#send, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_send(_arg_message, _arg_clientToken, _aidl_reply)
      }
    )
  }
  fn r#pull<'a>(&'a self, _arg_uri: &'a parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>>> {
    let _aidl_data = match self.build_parcel_pull(_arg_uri, _arg_count, _arg_flags, _arg_clientToken) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#pull, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_pull(_arg_uri, _arg_count, _arg_flags, _arg_clientToken, _aidl_reply)
      }
    )
  }
  fn r#enableDispatching<'a>(&'a self, _arg_uri: &'a parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>> {
    let _aidl_data = match self.build_parcel_enableDispatching(_arg_uri, _arg_flags, _arg_clientToken) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#enableDispatching, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_enableDispatching(_arg_uri, _arg_flags, _arg_clientToken, _aidl_reply)
      }
    )
  }
  fn r#disableDispatching<'a>(&'a self, _arg_uri: &'a parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>> {
    let _aidl_data = match self.build_parcel_disableDispatching(_arg_uri, _arg_flags, _arg_clientToken) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#disableDispatching, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_disableDispatching(_arg_uri, _arg_flags, _arg_clientToken, _aidl_reply)
      }
    )
  }
}
impl IUBus for binder::binder_impl::Binder<BnUBus> {
  fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus> { self.0.r#registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener) }
  fn r#unregisterClient(&self, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> { self.0.r#unregisterClient(_arg_clientToken) }
  fn r#send(&self, _arg_message: &parcelable_stubs::ParcelableUMessage, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> { self.0.r#send(_arg_message, _arg_clientToken) }
  fn r#pull(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_count: i32, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<Option<Vec<Option<parcelable_stubs::ParcelableUMessage>>>> { self.0.r#pull(_arg_uri, _arg_count, _arg_flags, _arg_clientToken) }
  fn r#enableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> { self.0.r#enableDispatching(_arg_uri, _arg_flags, _arg_clientToken) }
  fn r#disableDispatching(&self, _arg_uri: &parcelable_stubs::ParcelableUUri, _arg_flags: i32, _arg_clientToken: &binder::SpIBinder) -> binder::Result<parcelable_stubs::ParcelableUStatus> { self.0.r#disableDispatching(_arg_uri, _arg_flags, _arg_clientToken) }
}
fn on_transact(_aidl_service: &dyn IUBus, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#registerClient => {
      let _arg_packageName: String = _aidl_data.read()?;
      let _arg_entity: parcelable_stubs::ParcelableUEntity = _aidl_data.read()?;
      let _arg_clientToken: binder::SpIBinder = _aidl_data.read()?;
      let _arg_flags: i32 = _aidl_data.read()?;
      let _arg_listener: binder::Strong<dyn crate::binder_impls::IUListener::IUListener> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#registerClient(&_arg_packageName, &_arg_entity, &_arg_clientToken, _arg_flags, &_arg_listener);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#unregisterClient => {
      let _arg_clientToken: binder::SpIBinder = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#unregisterClient(&_arg_clientToken);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#send => {
      let _arg_message: parcelable_stubs::ParcelableUMessage = _aidl_data.read()?;
      let _arg_clientToken: binder::SpIBinder = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#send(&_arg_message, &_arg_clientToken);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#pull => {
      let _arg_uri: parcelable_stubs::ParcelableUUri = _aidl_data.read()?;
      let _arg_count: i32 = _aidl_data.read()?;
      let _arg_flags: i32 = _aidl_data.read()?;
      let _arg_clientToken: binder::SpIBinder = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#pull(&_arg_uri, _arg_count, _arg_flags, &_arg_clientToken);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#enableDispatching => {
      let _arg_uri: parcelable_stubs::ParcelableUUri = _aidl_data.read()?;
      let _arg_flags: i32 = _aidl_data.read()?;
      let _arg_clientToken: binder::SpIBinder = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#enableDispatching(&_arg_uri, _arg_flags, &_arg_clientToken);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#disableDispatching => {
      let _arg_uri: parcelable_stubs::ParcelableUUri = _aidl_data.read()?;
      let _arg_flags: i32 = _aidl_data.read()?;
      let _arg_clientToken: binder::SpIBinder = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#disableDispatching(&_arg_uri, _arg_flags, &_arg_clientToken);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IUBus as _3_org_7_eclipse_9_uprotocol_4_core_4_ubus_5_IUBus;
}
