#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownRemoteSystemCapabilitiesStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownRemoteSystemCapabilitiesStatics {
    type Vtable = IKnownRemoteSystemCapabilitiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8108e380_7f8a_44e4_92cd_03b6469b94a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRemoteSystemCapabilitiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystem(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem {
    type Vtable = IRemoteSystem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed5838cd_1e10_4a8c_b4a6_4e5fd6f97721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystem2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem2 {
    type Vtable = IRemoteSystem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x09dfe4ec_fb8b_4a08_a758_6876435d769e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilityname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystem3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem3 {
    type Vtable = IRemoteSystem3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x72b4b495_b7c6_40be_831b_73562f12ffa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystem4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem4 {
    type Vtable = IRemoteSystem4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf164ffe5_b987_4ca5_9926_fa0438be6273);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemPlatform) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystem5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem5 {
    type Vtable = IRemoteSystem5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeb2ad723_e5e2_4ae2_a7a7_a1097a098e90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystem6(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem6 {
    type Vtable = IRemoteSystem6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd4cda942_c027_533e_9384_3a19b4f7eef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemAddedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f39560f_e534_4697_8836_7abea151516e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAddedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemApp(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemApp {
    type Vtable = IRemoteSystemApp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80e5bcbd_d54d_41b1_9b16_6810a871ed4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemApp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemApp2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemApp2 {
    type Vtable = IRemoteSystemApp2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6369bf15_0a96_577a_8ff6_c35904dfa8f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemApp2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb47947b5_7035_4a5a_b8df_962d8f8431f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistrationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAppRegistrationStatics {
    type Vtable = IRemoteSystemAppRegistrationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x01b99840_cfd2_453f_ae25_c2539f086afd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistrationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6b0dde8e_04d0_40f4_a27f_c2acbbd6b734);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemAuthorizationKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilterFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAuthorizationKindFilterFactory {
    type Vtable = IRemoteSystemAuthorizationKindFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xad65df4d_b66a_45a4_8177_8caed75d9e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystemauthorizationkind: RemoteSystemAuthorizationKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x23278bc3_0d09_52cb_9c6a_eed2940bee43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfoStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionInfoStatics {
    type Vtable = IRemoteSystemConnectionInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xac831e2d_66c5_56d7_a4ce_705d94925ad6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_AppService")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connection: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x84ed4104_8d5e_4d72_8238_7621576c7a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequest2 {
    type Vtable = IRemoteSystemConnectionRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x12df6d6f_bffc_483a_8abe_d34a6c19f92b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequest3 {
    type Vtable = IRemoteSystemConnectionRequest3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xde86c3e7_c9cc_5a50_b8d9_ba7b34bb8d0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequestFactory {
    type Vtable = IRemoteSystemConnectionRequestFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa0a0a20_baeb_4575_b530_810bb9786334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequestStatics {
    type Vtable = IRemoteSystemConnectionRequestStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x86ca143d_8214_425c_8932_db49032d1306);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystemapp: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequestStatics2 {
    type Vtable = IRemoteSystemConnectionRequestStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x460f1027_64ec_598e_a800_4f2ee58def19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectiontoken: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, connectiontoken: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x42d9041f_ee5a_43da_ac6a_6fee25460741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemDiscoveryType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemDiscoveryTypeFilterFactory {
    type Vtable = IRemoteSystemDiscoveryTypeFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f9eb993_c260_4161_92f2_9c021f23fe5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discoverytype: RemoteSystemDiscoveryType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemEnumerationCompletedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc6e83d5f_4030_4354_a060_14f1b22c545d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemEnumerationCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `System_RemoteSystems`*"]
pub struct IRemoteSystemFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemFilter {
    type Vtable = IRemoteSystemFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4a3ba9e4_99eb_45eb_ba16_0367728ff374);
}
impl IRemoteSystemFilter {}
unsafe impl ::windows::runtime::RuntimeType for IRemoteSystemFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4a3ba9e4-99eb-45eb-ba16-0367728ff374}");
}
impl ::core::convert::From<IRemoteSystemFilter> for ::windows::runtime::IUnknown {
    fn from(value: IRemoteSystemFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IRemoteSystemFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IRemoteSystemFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRemoteSystemFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRemoteSystemFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IRemoteSystemFilter> for ::windows::runtime::IInspectable {
    fn from(value: IRemoteSystemFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRemoteSystemFilter> for ::windows::runtime::IInspectable {
    fn from(value: &IRemoteSystemFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IRemoteSystemFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IRemoteSystemFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x38e1c9ec_22c3_4ef6_901a_bbb1c7aad4ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilterFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemKindFilterFactory {
    type Vtable = IRemoteSystemKindFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa1fb18ee_99ea_40bc_9a39_c670aa804a28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystemkinds: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemKindStatics {
    type Vtable = IRemoteSystemKindStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf6317633_ab14_41d0_9553_796aadb882db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemKindStatics2 {
    type Vtable = IRemoteSystemKindStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb9e3a3d0_0466_4749_91e8_65f9d19a96a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemRemovedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8b3d16bb_7306_49ea_b7df_67d5714cb013);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSession(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSession {
    type Vtable = IRemoteSystemSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x69476a01_9ada_490f_9549_d31cb14c9e95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, invitee: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionAddedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd585d754_bc97_4c39_99b4_beca76e04c3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionAddedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionController(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe48b2dd2_6820_4867_b425_d89c0a3ef7ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pparticipant: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionControllerFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionControllerFactory {
    type Vtable = IRemoteSystemSessionControllerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbfcc2f6b_ac3d_4199_82cd_6670a773ef2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionControllerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionCreationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa79812c2_37de_448c_8b83_a30aa3c4ead6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionCreationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemSessionCreationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionDisconnectedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xde0bc69b_77c5_461c_8209_7c6c5d3111ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionDisconnectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemSessionDisconnectedReason) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xff4df648_8b0a_4e9a_9905_69e4b841c588);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3e32cc91_51d7_4766_a121_25516c3b8294);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationListener(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x08f4003f_bc71_49e1_874a_31ddff9a27b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationListener_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5e964a2d_a10d_4edb_8dea_54d20ac19543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20600068_7994_4331_86d1_d89d882585ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdbca4fc3_82b9_4816_9c24_e40e61774bd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xce7b1f04_a03e_41a4_900b_1e79328c1267);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemSessionJoinStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannel(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9524d12a_73d9_4c10_b751_c26784437127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagedata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagedata: ::windows::runtime::RawPtr, participant: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagedata: ::windows::runtime::RawPtr, participants: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannelFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionMessageChannelFactory {
    type Vtable = IRemoteSystemSessionMessageChannelFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x295e1c4a_bd16_4298_b7ce_415482b0e11d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannelFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, session: ::windows::runtime::RawPtr, channelname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, session: ::windows::runtime::RawPtr, channelname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, reliability: RemoteSystemSessionMessageChannelReliability, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x740ed755_8418_4f01_9353_e21c9ecc6cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipant(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7e90058c_acf9_4729_8a17_44e7baed5dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipant_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd35a57d8_c9a1_4bb7_b6b0_79bb91adf93d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x866ef088_de68_4abf_88a1_f90d16274192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdcdd02cc_aa87_4d79_b6cc_4459b3e92075);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemSessionParticipantWatcherStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionRemovedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaf82914e_39a1_4dea_9d63_43798d5bbbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionStatics {
    type Vtable = IRemoteSystemSessionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8524899f_fd20_44e3_9565_e75a3b14c66e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionUpdatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16875069_231e_4c91_8ec8_b3a39d9e55a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x06f31785_2da5_4e58_a78f_9e8d0784ee25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemSessionWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8003e340_0c41_4a62_b6d7_bdbe2b19be2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemSessionWatcherStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatics {
    type Vtable = IRemoteSystemStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa485b392_ff2b_4b47_be62_743f2f140f30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hostname: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatics2 {
    type Vtable = IRemoteSystemStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0c98edca_6f99_4c52_a272_ea4f36471744);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: RemoteSystemAuthorizationKind, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatics3 {
    type Vtable = IRemoteSystemStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9995f16f_0b3c_5ac5_b325_cc73f437dfcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, filters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0c39514e_cbb6_4777_8534_2e0c521affa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemStatusType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilterFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatusTypeFilterFactory {
    type Vtable = IRemoteSystemStatusTypeFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x33cf78fa_d724_4125_ac7a_8d281e44c949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotesystemstatustype: RemoteSystemStatusType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemUpdatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7502ff0e_dbcb_4155_b4ca_b30a04f27627);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5d600c7e_2c07_48c5_889c_455d2b099771);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWatcher2 {
    type Vtable = IRemoteSystemWatcher2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x73436700_19ca_48f9_a4cd_780f7ad58c71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWatcher3 {
    type Vtable = IRemoteSystemWatcher3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf79c0fcf_a913_55d3_8413_418fcf15ba54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74c5c6af_5114_4426_9216_20d81f8519ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RemoteSystemWatcherError) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3fb75873_87c8_5d8f_977e_f69f96d67238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilterFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWebAccountFilterFactory {
    type Vtable = IRemoteSystemWebAccountFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x348a2709_5f4d_5127_b4a7_bf99d5252b1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, account: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[doc = "*Required features: `System_RemoteSystems`*"]
pub struct KnownRemoteSystemCapabilities {}
impl KnownRemoteSystemCapabilities {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn AppService() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn LaunchUri() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSession() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn SpatialEntity() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownRemoteSystemCapabilitiesStatics<R, F: FnOnce(&IKnownRemoteSystemCapabilitiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownRemoteSystemCapabilities, IKnownRemoteSystemCapabilitiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownRemoteSystemCapabilities {
    const NAME: &'static str = "Windows.System.RemoteSystems.KnownRemoteSystemCapabilities";
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystem(pub ::windows::runtime::IInspectable);
impl RemoteSystem {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemStatus>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn IsAvailableByProximity(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`, `Networking`*"]
    pub fn FindByHostNameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Networking::HostName>>(hostname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<RemoteSystem>> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<RemoteSystem>>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateWatcher() -> ::windows::runtime::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`*"]
    pub fn CreateWatcherWithFilters<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>>(filters: Param0) -> ::windows::runtime::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), filters.into_param().abi(), &mut result__).from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RequestAccessAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn IsAvailableBySpatialProximity(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn GetCapabilitySupportedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, capabilityname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), capabilityname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn IsAuthorizationKindEnabled(kind: RemoteSystemAuthorizationKind) -> ::windows::runtime::Result<bool> {
        Self::IRemoteSystemStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn ManufacturerDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn ModelDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Platform(&self) -> ::windows::runtime::Result<RemoteSystemPlatform> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem4>(self)?;
        unsafe {
            let mut result__: RemoteSystemPlatform = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemPlatform>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`*"]
    pub fn Apps(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<RemoteSystemApp>> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<RemoteSystemApp>>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::User>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateWatcherForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(user: Param0) -> ::windows::runtime::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`*"]
    pub fn CreateWatcherWithFiltersForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>>(user: Param0, filters: Param1) -> ::windows::runtime::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), filters.into_param().abi(), &mut result__).from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    pub fn IRemoteSystemStatics<R, F: FnOnce(&IRemoteSystemStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystem, IRemoteSystemStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemStatics2<R, F: FnOnce(&IRemoteSystemStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystem, IRemoteSystemStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemStatics3<R, F: FnOnce(&IRemoteSystemStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystem, IRemoteSystemStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystem;{ed5838cd-1e10-4a8c-b4a6-4e5fd6f97721})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystem {
    type Vtable = IRemoteSystem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed5838cd_1e10_4a8c_b4a6_4e5fd6f97721);
}
impl ::windows::runtime::RuntimeName for RemoteSystem {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystem";
}
impl ::core::convert::From<RemoteSystem> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystem> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystem> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystem> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystem {}
unsafe impl ::core::marker::Sync for RemoteSystem {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemAccessStatus(pub i32);
impl RemoteSystemAccessStatus {
    pub const Unspecified: RemoteSystemAccessStatus = RemoteSystemAccessStatus(0i32);
    pub const Allowed: RemoteSystemAccessStatus = RemoteSystemAccessStatus(1i32);
    pub const DeniedByUser: RemoteSystemAccessStatus = RemoteSystemAccessStatus(2i32);
    pub const DeniedBySystem: RemoteSystemAccessStatus = RemoteSystemAccessStatus(3i32);
}
impl ::core::convert::From<i32> for RemoteSystemAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemAccessStatus;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemAddedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemAddedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystem(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAddedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAddedEventArgs;{8f39560f-e534-4697-8836-7abea151516e})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f39560f_e534_4697_8836_7abea151516e);
}
impl ::windows::runtime::RuntimeName for RemoteSystemAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAddedEventArgs";
}
impl ::core::convert::From<RemoteSystemAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemAddedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemAddedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemAddedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemAddedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemApp(pub ::windows::runtime::IInspectable);
impl RemoteSystemApp {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn IsAvailableByProximity(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn IsAvailableBySpatialProximity(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::User>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn ConnectionToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemApp {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemApp;{80e5bcbd-d54d-41b1-9b16-6810a871ed4f})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemApp {
    type Vtable = IRemoteSystemApp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80e5bcbd_d54d_41b1_9b16_6810a871ed4f);
}
impl ::windows::runtime::RuntimeName for RemoteSystemApp {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemApp";
}
impl ::core::convert::From<RemoteSystemApp> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemApp) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemApp> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemApp) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemApp> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemApp) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemApp> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemApp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemApp {}
unsafe impl ::core::marker::Sync for RemoteSystemApp {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemAppRegistration(pub ::windows::runtime::IInspectable);
impl RemoteSystemAppRegistration {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn SaveAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemAppRegistration>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(user: Param0) -> ::windows::runtime::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<RemoteSystemAppRegistration>(result__)
        })
    }
    pub fn IRemoteSystemAppRegistrationStatics<R, F: FnOnce(&IRemoteSystemAppRegistrationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemAppRegistration, IRemoteSystemAppRegistrationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAppRegistration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAppRegistration;{b47947b5-7035-4a5a-b8df-962d8f8431f4})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb47947b5_7035_4a5a_b8df_962d8f8431f4);
}
impl ::windows::runtime::RuntimeName for RemoteSystemAppRegistration {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAppRegistration";
}
impl ::core::convert::From<RemoteSystemAppRegistration> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemAppRegistration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemAppRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemAppRegistration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemAppRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemAppRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemAppRegistration> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemAppRegistration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemAppRegistration> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemAppRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemAppRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemAppRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemAppRegistration {}
unsafe impl ::core::marker::Sync for RemoteSystemAppRegistration {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemAuthorizationKind(pub i32);
impl RemoteSystemAuthorizationKind {
    pub const SameUser: RemoteSystemAuthorizationKind = RemoteSystemAuthorizationKind(0i32);
    pub const Anonymous: RemoteSystemAuthorizationKind = RemoteSystemAuthorizationKind(1i32);
}
impl ::core::convert::From<i32> for RemoteSystemAuthorizationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemAuthorizationKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAuthorizationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemAuthorizationKind;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemAuthorizationKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemAuthorizationKindFilter(pub ::windows::runtime::IInspectable);
impl RemoteSystemAuthorizationKindFilter {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystemAuthorizationKind(&self) -> ::windows::runtime::Result<RemoteSystemAuthorizationKind> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemAuthorizationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemAuthorizationKind>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Create(remotesystemauthorizationkind: RemoteSystemAuthorizationKind) -> ::windows::runtime::Result<RemoteSystemAuthorizationKindFilter> {
        Self::IRemoteSystemAuthorizationKindFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotesystemauthorizationkind, &mut result__).from_abi::<RemoteSystemAuthorizationKindFilter>(result__)
        })
    }
    pub fn IRemoteSystemAuthorizationKindFilterFactory<R, F: FnOnce(&IRemoteSystemAuthorizationKindFilterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemAuthorizationKindFilter, IRemoteSystemAuthorizationKindFilterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAuthorizationKindFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter;{6b0dde8e-04d0-40f4-a27f-c2acbbd6b734})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6b0dde8e_04d0_40f4_a27f_c2acbbd6b734);
}
impl ::windows::runtime::RuntimeName for RemoteSystemAuthorizationKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter";
}
impl ::core::convert::From<RemoteSystemAuthorizationKindFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemAuthorizationKindFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemAuthorizationKindFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemAuthorizationKindFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemAuthorizationKindFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemAuthorizationKindFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemAuthorizationKindFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemAuthorizationKindFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<RemoteSystemAuthorizationKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemAuthorizationKindFilter) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemAuthorizationKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemAuthorizationKindFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemAuthorizationKindFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemAuthorizationKindFilter {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemConnectionInfo(pub ::windows::runtime::IInspectable);
impl RemoteSystemConnectionInfo {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn IsProximal(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    #[doc = "*Required features: `System_RemoteSystems`, `ApplicationModel_AppService`*"]
    pub fn TryCreateFromAppServiceConnection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::AppService::AppServiceConnection>>(connection: Param0) -> ::windows::runtime::Result<RemoteSystemConnectionInfo> {
        Self::IRemoteSystemConnectionInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connection.into_param().abi(), &mut result__).from_abi::<RemoteSystemConnectionInfo>(result__)
        })
    }
    pub fn IRemoteSystemConnectionInfoStatics<R, F: FnOnce(&IRemoteSystemConnectionInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemConnectionInfo, IRemoteSystemConnectionInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemConnectionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemConnectionInfo;{23278bc3-0d09-52cb-9c6a-eed2940bee43})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x23278bc3_0d09_52cb_9c6a_eed2940bee43);
}
impl ::windows::runtime::RuntimeName for RemoteSystemConnectionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionInfo";
}
impl ::core::convert::From<RemoteSystemConnectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemConnectionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemConnectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemConnectionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemConnectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemConnectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemConnectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemConnectionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemConnectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemConnectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemConnectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemConnectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemConnectionInfo {}
unsafe impl ::core::marker::Sync for RemoteSystemConnectionInfo {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemConnectionRequest(pub ::windows::runtime::IInspectable);
impl RemoteSystemConnectionRequest {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystem(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystem>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystem>>(remotesystem: Param0) -> ::windows::runtime::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotesystem.into_param().abi(), &mut result__).from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystemApp(&self) -> ::windows::runtime::Result<RemoteSystemApp> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemConnectionRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemApp>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateForApp<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystemApp>>(remotesystemapp: Param0) -> ::windows::runtime::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotesystemapp.into_param().abi(), &mut result__).from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn ConnectionToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemConnectionRequest3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateFromConnectionToken<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(connectiontoken: Param0) -> ::windows::runtime::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectiontoken.into_param().abi(), &mut result__).from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateFromConnectionTokenForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, connectiontoken: Param1) -> ::windows::runtime::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), connectiontoken.into_param().abi(), &mut result__).from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn IRemoteSystemConnectionRequestFactory<R, F: FnOnce(&IRemoteSystemConnectionRequestFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemConnectionRequestStatics<R, F: FnOnce(&IRemoteSystemConnectionRequestStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemConnectionRequestStatics2<R, F: FnOnce(&IRemoteSystemConnectionRequestStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemConnectionRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemConnectionRequest;{84ed4104-8d5e-4d72-8238-7621576c7a67})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x84ed4104_8d5e_4d72_8238_7621576c7a67);
}
impl ::windows::runtime::RuntimeName for RemoteSystemConnectionRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionRequest";
}
impl ::core::convert::From<RemoteSystemConnectionRequest> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemConnectionRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemConnectionRequest> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemConnectionRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemConnectionRequest> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemConnectionRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemConnectionRequest> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemConnectionRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemConnectionRequest {}
unsafe impl ::core::marker::Sync for RemoteSystemConnectionRequest {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemDiscoveryType(pub i32);
impl RemoteSystemDiscoveryType {
    pub const Any: RemoteSystemDiscoveryType = RemoteSystemDiscoveryType(0i32);
    pub const Proximal: RemoteSystemDiscoveryType = RemoteSystemDiscoveryType(1i32);
    pub const Cloud: RemoteSystemDiscoveryType = RemoteSystemDiscoveryType(2i32);
    pub const SpatiallyProximal: RemoteSystemDiscoveryType = RemoteSystemDiscoveryType(3i32);
}
impl ::core::convert::From<i32> for RemoteSystemDiscoveryType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemDiscoveryType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemDiscoveryType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemDiscoveryType;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemDiscoveryType {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemDiscoveryTypeFilter(pub ::windows::runtime::IInspectable);
impl RemoteSystemDiscoveryTypeFilter {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystemDiscoveryType(&self) -> ::windows::runtime::Result<RemoteSystemDiscoveryType> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemDiscoveryType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemDiscoveryType>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Create(discoverytype: RemoteSystemDiscoveryType) -> ::windows::runtime::Result<RemoteSystemDiscoveryTypeFilter> {
        Self::IRemoteSystemDiscoveryTypeFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), discoverytype, &mut result__).from_abi::<RemoteSystemDiscoveryTypeFilter>(result__)
        })
    }
    pub fn IRemoteSystemDiscoveryTypeFilterFactory<R, F: FnOnce(&IRemoteSystemDiscoveryTypeFilterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemDiscoveryTypeFilter, IRemoteSystemDiscoveryTypeFilterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemDiscoveryTypeFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter;{42d9041f-ee5a-43da-ac6a-6fee25460741})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x42d9041f_ee5a_43da_ac6a_6fee25460741);
}
impl ::windows::runtime::RuntimeName for RemoteSystemDiscoveryTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter";
}
impl ::core::convert::From<RemoteSystemDiscoveryTypeFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemDiscoveryTypeFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemDiscoveryTypeFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemDiscoveryTypeFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemDiscoveryTypeFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemDiscoveryTypeFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemDiscoveryTypeFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemDiscoveryTypeFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<RemoteSystemDiscoveryTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemDiscoveryTypeFilter) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemDiscoveryTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemDiscoveryTypeFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemDiscoveryTypeFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemDiscoveryTypeFilter {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemEnumerationCompletedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemEnumerationCompletedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemEnumerationCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs;{c6e83d5f-4030-4354-a060-14f1b22c545d})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc6e83d5f_4030_4354_a060_14f1b22c545d);
}
impl ::windows::runtime::RuntimeName for RemoteSystemEnumerationCompletedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs";
}
impl ::core::convert::From<RemoteSystemEnumerationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemEnumerationCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemEnumerationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemEnumerationCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemEnumerationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemEnumerationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemEnumerationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemEnumerationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemEnumerationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemEnumerationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemEnumerationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemEnumerationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemEnumerationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemEnumerationCompletedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemKindFilter(pub ::windows::runtime::IInspectable);
impl RemoteSystemKindFilter {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`*"]
    pub fn RemoteSystemKinds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(remotesystemkinds: Param0) -> ::windows::runtime::Result<RemoteSystemKindFilter> {
        Self::IRemoteSystemKindFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotesystemkinds.into_param().abi(), &mut result__).from_abi::<RemoteSystemKindFilter>(result__)
        })
    }
    pub fn IRemoteSystemKindFilterFactory<R, F: FnOnce(&IRemoteSystemKindFilterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemKindFilter, IRemoteSystemKindFilterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemKindFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemKindFilter;{38e1c9ec-22c3-4ef6-901a-bbb1c7aad4ed})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x38e1c9ec_22c3_4ef6_901a_bbb1c7aad4ed);
}
impl ::windows::runtime::RuntimeName for RemoteSystemKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKindFilter";
}
impl ::core::convert::From<RemoteSystemKindFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemKindFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemKindFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemKindFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemKindFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemKindFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemKindFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemKindFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<RemoteSystemKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemKindFilter) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemKindFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemKindFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemKindFilter {}
#[doc = "*Required features: `System_RemoteSystems`*"]
pub struct RemoteSystemKinds {}
impl RemoteSystemKinds {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Phone() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Hub() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Holographic() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Desktop() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Xbox() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Iot() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Tablet() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Laptop() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IRemoteSystemKindStatics<R, F: FnOnce(&IRemoteSystemKindStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemKinds, IRemoteSystemKindStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemKindStatics2<R, F: FnOnce(&IRemoteSystemKindStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemKinds, IRemoteSystemKindStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for RemoteSystemKinds {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKinds";
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemPlatform(pub i32);
impl RemoteSystemPlatform {
    pub const Unknown: RemoteSystemPlatform = RemoteSystemPlatform(0i32);
    pub const Windows: RemoteSystemPlatform = RemoteSystemPlatform(1i32);
    pub const Android: RemoteSystemPlatform = RemoteSystemPlatform(2i32);
    pub const Ios: RemoteSystemPlatform = RemoteSystemPlatform(3i32);
    pub const Linux: RemoteSystemPlatform = RemoteSystemPlatform(4i32);
}
impl ::core::convert::From<i32> for RemoteSystemPlatform {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemPlatform {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemPlatform {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemPlatform;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemPlatform {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemRemovedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemRemovedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystemId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemRemovedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs;{8b3d16bb-7306-49ea-b7df-67d5714cb013})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8b3d16bb_7306_49ea_b7df_67d5714cb013);
}
impl ::windows::runtime::RuntimeName for RemoteSystemRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs";
}
impl ::core::convert::From<RemoteSystemRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemRemovedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSession(pub ::windows::runtime::IInspectable);
impl RemoteSystemSession {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn ControllerDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn Disconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateParticipantWatcher(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipantWatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionParticipantWatcher>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn SendInvitationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystem>>(&self, invitee: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), invitee.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateWatcher() -> ::windows::runtime::Result<RemoteSystemSessionWatcher> {
        Self::IRemoteSystemSessionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionWatcher>(result__)
        })
    }
    pub fn IRemoteSystemSessionStatics<R, F: FnOnce(&IRemoteSystemSessionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemSession, IRemoteSystemSessionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSession;{69476a01-9ada-490f-9549-d31cb14c9e95})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSession {
    type Vtable = IRemoteSystemSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x69476a01_9ada_490f_9549_d31cb14c9e95);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSession {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSession";
}
impl ::core::convert::From<RemoteSystemSession> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSession> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSession> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSession> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<RemoteSystemSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemSession) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RemoteSystemSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for RemoteSystemSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &RemoteSystemSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSession {}
unsafe impl ::core::marker::Sync for RemoteSystemSession {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionAddedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionAddedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionAddedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs;{d585d754-bc97-4c39-99b4-beca76e04c3f})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd585d754_bc97_4c39_99b4_beca76e04c3f);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionAddedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionAddedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionAddedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionAddedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionController(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionController {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn JoinRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveJoinRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveParticipantAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystemSessionParticipant>>(&self, pparticipant: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pparticipant.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn CreateSessionAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionCreationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<RemoteSystemSessionCreationResult>>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(displayname: Param0) -> ::windows::runtime::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), displayname.into_param().abi(), &mut result__).from_abi::<RemoteSystemSessionController>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateControllerWithSessionOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, RemoteSystemSessionOptions>>(displayname: Param0, options: Param1) -> ::windows::runtime::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), displayname.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<RemoteSystemSessionController>(result__)
        })
    }
    pub fn IRemoteSystemSessionControllerFactory<R, F: FnOnce(&IRemoteSystemSessionControllerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemSessionController, IRemoteSystemSessionControllerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionController;{e48b2dd2-6820-4867-b425-d89c0a3ef7ba})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe48b2dd2_6820_4867_b425_d89c0a3ef7ba);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionController {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionController";
}
impl ::core::convert::From<RemoteSystemSessionController> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionController> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionController> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionController> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionController {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionController {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionCreationResult(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionCreationResult {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemSessionCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionCreationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Session(&self) -> ::windows::runtime::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSession>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionCreationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionCreationResult;{a79812c2-37de-448c-8b83-a30aa3c4ead6})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa79812c2_37de_448c_8b83_a30aa3c4ead6);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionCreationResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionCreationResult";
}
impl ::core::convert::From<RemoteSystemSessionCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionCreationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionCreationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionCreationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionCreationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionCreationResult {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionCreationResult {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemSessionCreationStatus(pub i32);
impl RemoteSystemSessionCreationStatus {
    pub const Success: RemoteSystemSessionCreationStatus = RemoteSystemSessionCreationStatus(0i32);
    pub const SessionLimitsExceeded: RemoteSystemSessionCreationStatus = RemoteSystemSessionCreationStatus(1i32);
    pub const OperationAborted: RemoteSystemSessionCreationStatus = RemoteSystemSessionCreationStatus(2i32);
}
impl ::core::convert::From<i32> for RemoteSystemSessionCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionCreationStatus;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemSessionCreationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionDisconnectedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionDisconnectedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Reason(&self) -> ::windows::runtime::Result<RemoteSystemSessionDisconnectedReason> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionDisconnectedReason = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionDisconnectedReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionDisconnectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs;{de0bc69b-77c5-461c-8209-7c6c5d3111ab})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xde0bc69b_77c5_461c_8209_7c6c5d3111ab);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionDisconnectedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionDisconnectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionDisconnectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionDisconnectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionDisconnectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionDisconnectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionDisconnectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionDisconnectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionDisconnectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionDisconnectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionDisconnectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionDisconnectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionDisconnectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionDisconnectedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemSessionDisconnectedReason(pub i32);
impl RemoteSystemSessionDisconnectedReason {
    pub const SessionUnavailable: RemoteSystemSessionDisconnectedReason = RemoteSystemSessionDisconnectedReason(0i32);
    pub const RemovedByController: RemoteSystemSessionDisconnectedReason = RemoteSystemSessionDisconnectedReason(1i32);
    pub const SessionClosed: RemoteSystemSessionDisconnectedReason = RemoteSystemSessionDisconnectedReason(2i32);
}
impl ::core::convert::From<i32> for RemoteSystemSessionDisconnectedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionDisconnectedReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionDisconnectedReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedReason;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemSessionDisconnectedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionInfo(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionInfo {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn ControllerDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn JoinAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInfo;{ff4df648-8b0a-4e9a-9905-69e4b841c588})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xff4df648_8b0a_4e9a_9905_69e4b841c588);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInfo";
}
impl ::core::convert::From<RemoteSystemSessionInfo> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionInfo> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionInfo {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInfo {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionInvitation(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionInvitation {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Sender(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystem>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionInvitation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitation;{3e32cc91-51d7-4766-a121-25516c3b8294})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3e32cc91_51d7_4766_a121_25516c3b8294);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionInvitation {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitation";
}
impl ::core::convert::From<RemoteSystemSessionInvitation> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionInvitation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitation> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionInvitation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionInvitation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionInvitation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionInvitation> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionInvitation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitation> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionInvitation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionInvitation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionInvitation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitation {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitation {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionInvitationListener(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionInvitationListener {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemSessionInvitationListener, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn InvitationReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveInvitationReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionInvitationListener {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener;{08f4003f-bc71-49e1-874a-31ddff9a27b9})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x08f4003f_bc71_49e1_874a_31ddff9a27b9);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionInvitationListener {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener";
}
impl ::core::convert::From<RemoteSystemSessionInvitationListener> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionInvitationListener) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitationListener> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionInvitationListener) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionInvitationListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionInvitationListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionInvitationListener> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionInvitationListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitationListener> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionInvitationListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionInvitationListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionInvitationListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitationListener {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitationListener {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionInvitationReceivedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionInvitationReceivedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Invitation(&self) -> ::windows::runtime::Result<RemoteSystemSessionInvitation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionInvitation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionInvitationReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs;{5e964a2d-a10d-4edb-8dea-54d20ac19543})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5e964a2d_a10d_4edb_8dea_54d20ac19543);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionInvitationReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionInvitationReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitationReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionInvitationReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionInvitationReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionInvitationReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitationReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionInvitationReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionInvitationReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitationReceivedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionJoinRequest(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionJoinRequest {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Participant(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Accept(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionJoinRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest;{20600068-7994-4331-86d1-d89d882585ee})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20600068_7994_4331_86d1_d89d882585ee);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionJoinRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest";
}
impl ::core::convert::From<RemoteSystemSessionJoinRequest> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionJoinRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinRequest> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionJoinRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionJoinRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionJoinRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionJoinRequest> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionJoinRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinRequest> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionJoinRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionJoinRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionJoinRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinRequest {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinRequest {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionJoinRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionJoinRequestedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn JoinRequest(&self) -> ::windows::runtime::Result<RemoteSystemSessionJoinRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionJoinRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionJoinRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs;{dbca4fc3-82b9-4816-9c24-e40e61774bd8})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdbca4fc3_82b9_4816_9c24_e40e61774bd8);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionJoinRequestedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionJoinRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionJoinRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionJoinRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionJoinRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionJoinRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionJoinRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinRequestedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionJoinResult(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionJoinResult {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemSessionJoinStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionJoinStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionJoinStatus>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Session(&self) -> ::windows::runtime::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSession>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionJoinResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinResult;{ce7b1f04-a03e-41a4-900b-1e79328c1267})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xce7b1f04_a03e_41a4_900b_1e79328c1267);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionJoinResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinResult";
}
impl ::core::convert::From<RemoteSystemSessionJoinResult> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionJoinResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinResult> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionJoinResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionJoinResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionJoinResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionJoinResult> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionJoinResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinResult> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionJoinResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionJoinResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionJoinResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinResult {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinResult {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemSessionJoinStatus(pub i32);
impl RemoteSystemSessionJoinStatus {
    pub const Success: RemoteSystemSessionJoinStatus = RemoteSystemSessionJoinStatus(0i32);
    pub const SessionLimitsExceeded: RemoteSystemSessionJoinStatus = RemoteSystemSessionJoinStatus(1i32);
    pub const OperationAborted: RemoteSystemSessionJoinStatus = RemoteSystemSessionJoinStatus(2i32);
    pub const SessionUnavailable: RemoteSystemSessionJoinStatus = RemoteSystemSessionJoinStatus(3i32);
    pub const RejectedByController: RemoteSystemSessionJoinStatus = RemoteSystemSessionJoinStatus(4i32);
}
impl ::core::convert::From<i32> for RemoteSystemSessionJoinStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionJoinStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionJoinStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionJoinStatus;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemSessionJoinStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionMessageChannel(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionMessageChannel {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Session(&self) -> ::windows::runtime::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSession>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`, `Foundation_Collections`*"]
    pub fn BroadcastValueSetAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, messagedata: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), messagedata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`, `Foundation_Collections`*"]
    pub fn SendValueSetAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>, Param1: ::windows::runtime::IntoParam<'a, RemoteSystemSessionParticipant>>(&self, messagedata: Param0, participant: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), messagedata.into_param().abi(), participant.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`, `Foundation_Collections`*"]
    pub fn SendValueSetToParticipantsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant>>>(&self, messagedata: Param0, participants: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), messagedata.into_param().abi(), participants.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn ValueSetReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveValueSetReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystemSession>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(session: Param0, channelname: Param1) -> ::windows::runtime::Result<RemoteSystemSessionMessageChannel> {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), session.into_param().abi(), channelname.into_param().abi(), &mut result__).from_abi::<RemoteSystemSessionMessageChannel>(result__)
        })
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn CreateWithReliability<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystemSession>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(session: Param0, channelname: Param1, reliability: RemoteSystemSessionMessageChannelReliability) -> ::windows::runtime::Result<RemoteSystemSessionMessageChannel> {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), session.into_param().abi(), channelname.into_param().abi(), reliability, &mut result__).from_abi::<RemoteSystemSessionMessageChannel>(result__)
        })
    }
    pub fn IRemoteSystemSessionMessageChannelFactory<R, F: FnOnce(&IRemoteSystemSessionMessageChannelFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemSessionMessageChannel, IRemoteSystemSessionMessageChannelFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionMessageChannel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel;{9524d12a-73d9-4c10-b751-c26784437127})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9524d12a_73d9_4c10_b751_c26784437127);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionMessageChannel {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel";
}
impl ::core::convert::From<RemoteSystemSessionMessageChannel> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionMessageChannel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionMessageChannel> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionMessageChannel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionMessageChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionMessageChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionMessageChannel> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionMessageChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionMessageChannel> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionMessageChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionMessageChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionMessageChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionMessageChannel {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionMessageChannel {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemSessionMessageChannelReliability(pub i32);
impl RemoteSystemSessionMessageChannelReliability {
    pub const Reliable: RemoteSystemSessionMessageChannelReliability = RemoteSystemSessionMessageChannelReliability(0i32);
    pub const Unreliable: RemoteSystemSessionMessageChannelReliability = RemoteSystemSessionMessageChannelReliability(1i32);
}
impl ::core::convert::From<i32> for RemoteSystemSessionMessageChannelReliability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionMessageChannelReliability {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionMessageChannelReliability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannelReliability;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemSessionMessageChannelReliability {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionOptions(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemSessionOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn IsInviteOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn SetIsInviteOnly(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionOptions;{740ed755-8418-4f01-9353-e21c9ecc6cfc})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x740ed755_8418_4f01_9353_e21c9ecc6cfc);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionOptions {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionOptions";
}
impl ::core::convert::From<RemoteSystemSessionOptions> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionOptions> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionOptions> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionOptions> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionOptions {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionOptions {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionParticipant(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionParticipant {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystem(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystem>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`, `Networking`*"]
    pub fn GetHostNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::HostName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Networking::HostName>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipant {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipant;{7e90058c-acf9-4729-8a17-44e7baed5dcc})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7e90058c_acf9_4729_8a17_44e7baed5dcc);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionParticipant {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipant";
}
impl ::core::convert::From<RemoteSystemSessionParticipant> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionParticipant) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipant> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionParticipant) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionParticipant {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionParticipant {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionParticipant> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionParticipant) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipant> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionParticipant) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionParticipant {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionParticipant {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipant {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipant {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionParticipantAddedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionParticipantAddedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Participant(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipantAddedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs;{d35a57d8-c9a1-4bb7-b6b0-79bb91adf93d})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd35a57d8_c9a1_4bb7_b6b0_79bb91adf93d);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionParticipantAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionParticipantAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionParticipantAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionParticipantAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionParticipantAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionParticipantAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionParticipantAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantAddedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionParticipantRemovedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionParticipantRemovedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Participant(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipantRemovedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs;{866ef088-de68-4abf-88a1-f90d16274192})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x866ef088_de68_4abf_88a1_f90d16274192);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionParticipantRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionParticipantRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionParticipantRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionParticipantRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionParticipantRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionParticipantRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionParticipantRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantRemovedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionParticipantWatcher(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionParticipantWatcher {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipantWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionParticipantWatcherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionParticipantWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipantWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher;{dcdd02cc-aa87-4d79-b6cc-4459b3e92075})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdcdd02cc_aa87_4d79_b6cc_4459b3e92075);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionParticipantWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher";
}
impl ::core::convert::From<RemoteSystemSessionParticipantWatcher> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionParticipantWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionParticipantWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionParticipantWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionParticipantWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionParticipantWatcher> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionParticipantWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionParticipantWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionParticipantWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionParticipantWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantWatcher {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantWatcherStatus(pub i32);
impl RemoteSystemSessionParticipantWatcherStatus {
    pub const Created: RemoteSystemSessionParticipantWatcherStatus = RemoteSystemSessionParticipantWatcherStatus(0i32);
    pub const Started: RemoteSystemSessionParticipantWatcherStatus = RemoteSystemSessionParticipantWatcherStatus(1i32);
    pub const EnumerationCompleted: RemoteSystemSessionParticipantWatcherStatus = RemoteSystemSessionParticipantWatcherStatus(2i32);
    pub const Stopping: RemoteSystemSessionParticipantWatcherStatus = RemoteSystemSessionParticipantWatcherStatus(3i32);
    pub const Stopped: RemoteSystemSessionParticipantWatcherStatus = RemoteSystemSessionParticipantWatcherStatus(4i32);
    pub const Aborted: RemoteSystemSessionParticipantWatcherStatus = RemoteSystemSessionParticipantWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for RemoteSystemSessionParticipantWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionParticipantWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipantWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemSessionParticipantWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionRemovedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionRemovedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionRemovedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs;{af82914e-39a1-4dea-9d63-43798d5bbbd0})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaf82914e_39a1_4dea_9d63_43798d5bbbd0);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionRemovedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionUpdatedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionUpdatedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs;{16875069-231e-4c91-8ec8-b3a39d9e55a3})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16875069_231e_4c91_8ec8_b3a39d9e55a3);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionUpdatedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionValueSetReceivedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionValueSetReceivedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Sender(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation_Collections`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionValueSetReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs;{06f31785-2da5-4e58-a78f-9e8d0784ee25})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x06f31785_2da5_4e58_a78f_9e8d0784ee25);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionValueSetReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionValueSetReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionValueSetReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionValueSetReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionValueSetReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionValueSetReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionValueSetReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionValueSetReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionValueSetReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionValueSetReceivedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionValueSetReceivedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemSessionWatcher(pub ::windows::runtime::IInspectable);
impl RemoteSystemSessionWatcher {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemSessionWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionWatcherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemSessionWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn Updated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionWatcher;{8003e340-0c41-4a62-b6d7-bdbe2b19be2d})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8003e340_0c41_4a62_b6d7_bdbe2b19be2d);
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionWatcher";
}
impl ::core::convert::From<RemoteSystemSessionWatcher> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemSessionWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSessionWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemSessionWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemSessionWatcher> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemSessionWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemSessionWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemSessionWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionWatcher {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemSessionWatcherStatus(pub i32);
impl RemoteSystemSessionWatcherStatus {
    pub const Created: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(0i32);
    pub const Started: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(1i32);
    pub const EnumerationCompleted: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(2i32);
    pub const Stopping: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(3i32);
    pub const Stopped: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(4i32);
    pub const Aborted: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for RemoteSystemSessionWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemSessionWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemStatus(pub i32);
impl RemoteSystemStatus {
    pub const Unavailable: RemoteSystemStatus = RemoteSystemStatus(0i32);
    pub const DiscoveringAvailability: RemoteSystemStatus = RemoteSystemStatus(1i32);
    pub const Available: RemoteSystemStatus = RemoteSystemStatus(2i32);
    pub const Unknown: RemoteSystemStatus = RemoteSystemStatus(3i32);
}
impl ::core::convert::From<i32> for RemoteSystemStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemStatus;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemStatusType(pub i32);
impl RemoteSystemStatusType {
    pub const Any: RemoteSystemStatusType = RemoteSystemStatusType(0i32);
    pub const Available: RemoteSystemStatusType = RemoteSystemStatusType(1i32);
}
impl ::core::convert::From<i32> for RemoteSystemStatusType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemStatusType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemStatusType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemStatusType;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemStatusType {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemStatusTypeFilter(pub ::windows::runtime::IInspectable);
impl RemoteSystemStatusTypeFilter {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystemStatusType(&self) -> ::windows::runtime::Result<RemoteSystemStatusType> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemStatusType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemStatusType>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Create(remotesystemstatustype: RemoteSystemStatusType) -> ::windows::runtime::Result<RemoteSystemStatusTypeFilter> {
        Self::IRemoteSystemStatusTypeFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotesystemstatustype, &mut result__).from_abi::<RemoteSystemStatusTypeFilter>(result__)
        })
    }
    pub fn IRemoteSystemStatusTypeFilterFactory<R, F: FnOnce(&IRemoteSystemStatusTypeFilterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemStatusTypeFilter, IRemoteSystemStatusTypeFilterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemStatusTypeFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter;{0c39514e-cbb6-4777-8534-2e0c521affa2})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0c39514e_cbb6_4777_8534_2e0c521affa2);
}
impl ::windows::runtime::RuntimeName for RemoteSystemStatusTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter";
}
impl ::core::convert::From<RemoteSystemStatusTypeFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemStatusTypeFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemStatusTypeFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemStatusTypeFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemStatusTypeFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemStatusTypeFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemStatusTypeFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemStatusTypeFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<RemoteSystemStatusTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemStatusTypeFilter) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemStatusTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemStatusTypeFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemStatusTypeFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemStatusTypeFilter {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemUpdatedEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemUpdatedEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn RemoteSystem(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs;{7502ff0e-dbcb-4155-b4ca-b30a04f27627})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7502ff0e_dbcb_4155_b4ca_b30a04f27627);
}
impl ::windows::runtime::RuntimeName for RemoteSystemUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs";
}
impl ::core::convert::From<RemoteSystemUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemUpdatedEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemWatcher(pub ::windows::runtime::IInspectable);
impl RemoteSystemWatcher {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoteSystemAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveRemoteSystemAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoteSystemUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveRemoteSystemUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoteSystemRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveRemoteSystemRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn ErrorOccurred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteSystems`, `Foundation`*"]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWatcher;{5d600c7e-2c07-48c5-889c-455d2b099771})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5d600c7e_2c07_48c5_889c_455d2b099771);
}
impl ::windows::runtime::RuntimeName for RemoteSystemWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcher";
}
impl ::core::convert::From<RemoteSystemWatcher> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemWatcher> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemWatcher {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteSystemWatcherError(pub i32);
impl RemoteSystemWatcherError {
    pub const Unknown: RemoteSystemWatcherError = RemoteSystemWatcherError(0i32);
    pub const InternetNotAvailable: RemoteSystemWatcherError = RemoteSystemWatcherError(1i32);
    pub const AuthenticationError: RemoteSystemWatcherError = RemoteSystemWatcherError(2i32);
}
impl ::core::convert::From<i32> for RemoteSystemWatcherError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemWatcherError {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemWatcherError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemWatcherError;i4)");
}
impl ::windows::runtime::DefaultType for RemoteSystemWatcherError {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemWatcherErrorOccurredEventArgs(pub ::windows::runtime::IInspectable);
impl RemoteSystemWatcherErrorOccurredEventArgs {
    #[doc = "*Required features: `System_RemoteSystems`*"]
    pub fn Error(&self) -> ::windows::runtime::Result<RemoteSystemWatcherError> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemWatcherError = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RemoteSystemWatcherError>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemWatcherErrorOccurredEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs;{74c5c6af-5114-4426-9216-20d81f8519ae})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74c5c6af_5114_4426_9216_20d81f8519ae);
}
impl ::windows::runtime::RuntimeName for RemoteSystemWatcherErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs";
}
impl ::core::convert::From<RemoteSystemWatcherErrorOccurredEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemWatcherErrorOccurredEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemWatcherErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemWatcherErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemWatcherErrorOccurredEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemWatcherErrorOccurredEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemWatcherErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemWatcherErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemWatcherErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemWatcherErrorOccurredEventArgs {}
#[doc = "*Required features: `System_RemoteSystems`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RemoteSystemWebAccountFilter(pub ::windows::runtime::IInspectable);
impl RemoteSystemWebAccountFilter {
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `System_RemoteSystems`, `Security_Credentials`*"]
    pub fn Account(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::WebAccount>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `System_RemoteSystems`, `Security_Credentials`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::WebAccount>>(account: Param0) -> ::windows::runtime::Result<RemoteSystemWebAccountFilter> {
        Self::IRemoteSystemWebAccountFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), account.into_param().abi(), &mut result__).from_abi::<RemoteSystemWebAccountFilter>(result__)
        })
    }
    pub fn IRemoteSystemWebAccountFilterFactory<R, F: FnOnce(&IRemoteSystemWebAccountFilterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystemWebAccountFilter, IRemoteSystemWebAccountFilterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemWebAccountFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWebAccountFilter;{3fb75873-87c8-5d8f-977e-f69f96d67238})");
}
unsafe impl ::windows::runtime::Interface for RemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3fb75873_87c8_5d8f_977e_f69f96d67238);
}
impl ::windows::runtime::RuntimeName for RemoteSystemWebAccountFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWebAccountFilter";
}
impl ::core::convert::From<RemoteSystemWebAccountFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemWebAccountFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RemoteSystemWebAccountFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemWebAccountFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RemoteSystemWebAccountFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemWebAccountFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RemoteSystemWebAccountFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemWebAccountFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<RemoteSystemWebAccountFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemWebAccountFilter) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemWebAccountFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemWebAccountFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemWebAccountFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemWebAccountFilter {}
