use binder::{BinderFeatures, SpIBinder, Strong};
use aidl_rust_codegen::binder_impls::IUListener::{BnUListener, IUListener};
use aidl_rust_codegen::binder_impls::IUBus::{BnUBus, IUBus};
use aidl_rust_codegen::parcelable_stubs::{ParcelableUMessage, ParcelableUEntity, ParcelableUStatus, ParcelableUUri};
use binder::{Interface, Result as BinderResult,
             binder_impl::{BorrowedParcel, UnstructuredParcelable},
};
use up_rust::uprotocol::{UAttributes, UAuthority, UEntity, UMessage, UPayload, UResource, UUri};

pub struct TestCallingClientIUListenerService;

impl Interface for TestCallingClientIUListenerService {}

impl IUBus for TestCallingClientIUListenerService {
    fn registerClient(&self, packageName: &str, entity: &ParcelableUEntity, clientToken: &SpIBinder, flags: i32, listener: &Strong<(dyn IUListener + 'static)>) -> binder::Result<ParcelableUStatus> {
        let umessage = UMessage {
            source: Some(UUri {
                authority: Some(UAuthority {
                    name: Some("super_cool_authority".to_owned()),
                    ..Default::default()
                }).into(),
                entity: Some(entity.as_ref().clone()).into(),
                resource: Some(UResource {
                    name: "super_cool_resource".to_owned(),
                    ..Default::default()
                }).into(),
                ..Default::default()
            }).into(),
            ..Default::default()
        };

        println!("umessage we're sending over onReceive(): \n{:?}", umessage);

        let res = listener.as_ref().onReceive(&umessage.into());
        println!("after calling onReceive in the server: res: {:?}", res);

        Ok(ParcelableUStatus::default())
    }

    fn unregisterClient(&self, clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    fn send(&self, message: &ParcelableUMessage, clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    // TODO: Discuss what the Bundle extras are for... doesn't appear to be available in Rust (yet?)
    //  @nullable ParcelableUMessage[] pull(in ParcelableUUri uri, int count, in @nullable Bundle extras, IBinder clientToken);
    fn pull(&self, uri: &ParcelableUUri, count: i32, clientToken: &SpIBinder) -> binder::Result<Option<Vec<Option<ParcelableUMessage>>>> {
        Ok(None)
    }

    // TODO: Discuss what the Bundle extras are for... doesn't appear to be available in Rust (yet?)
    // ParcelableUStatus enableDispatching(in ParcelableUUri uri, in @nullable Bundle extras, IBinder clientToken);
    fn enableDispatching(&self, uri: &ParcelableUUri, clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }

    // TODO: Discuss what the Bundle extras are for... doesn't appear to be available in Rust (yet?)
    // ParcelableUStatus disableDispatching(in ParcelableUUri uri, in @nullable Bundle extras, IBinder clientToken);
    fn disableDispatching(&self, uri: &ParcelableUUri, clientToken: &SpIBinder) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }
}

pub fn run() -> anyhow::Result<()> {
    let test_calling_client_iulistener_service = TestCallingClientIUListenerService;
    let test_calling_client_iulistener_service_binder = BnUBus::new_binder(test_calling_client_iulistener_service, BinderFeatures::default());
    binder::add_service("test-calling-client-iulistener-service", test_calling_client_iulistener_service_binder.as_binder())
        .expect("Failed to register service?");
    println!("Running!");
    binder::ProcessState::join_thread_pool();
    anyhow::Ok(())
}
