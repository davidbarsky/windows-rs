#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveDeviceAddress(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveDeviceAddress {
    type Vtable = IXboxLiveDeviceAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf5bbd279_3c86_4b57_a31a_b9462408fd01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveDeviceAddress_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer_array_size: u32, buffer: *mut u8, byteswritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, otherdeviceaddress: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveNetworkAccessKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveDeviceAddressStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveDeviceAddressStatics {
    type Vtable = IXboxLiveDeviceAddressStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5954a819_4a79_4931_827c_7f503e963263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveDeviceAddressStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, base64: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPair(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveEndpointPair {
    type Vtable = IXboxLiveEndpointPair_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1e9a839b_813e_44e0_b87f_c87a093475e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPair_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveEndpointPairState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairCreationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveEndpointPairCreationResult {
    type Vtable = IXboxLiveEndpointPairCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd9a8bb95_2aab_4d1e_9794_33ecc0dcf0fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairCreationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveEndpointPairCreationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveEndpointPairStateChangedEventArgs {
    type Vtable = IXboxLiveEndpointPairStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x592e3b55_de08_44e7_ac3b_b9b9a169583a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveEndpointPairState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveEndpointPairState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveEndpointPairStatics {
    type Vtable = IXboxLiveEndpointPairStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x64316b30_217a_4243_8ee1_6729281d27db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localSocketAddress_array_size: u32, localsocketaddress: *const u8, remoteSocketAddress_array_size: u32, remotesocketaddress: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localhostname: ::windows::runtime::RawPtr, localport: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, remotehostname: ::windows::runtime::RawPtr, remoteport: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairTemplate(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveEndpointPairTemplate {
    type Vtable = IXboxLiveEndpointPairTemplate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6b286ecf_3457_40ce_b9a1_c0cfe0213ea7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairTemplate_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceaddress: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceaddress: ::windows::runtime::RawPtr, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceaddress: ::windows::runtime::RawPtr, initiatorport: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceaddress: ::windows::runtime::RawPtr, initiatorport: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveSocketKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairTemplateStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveEndpointPairTemplateStatics {
    type Vtable = IXboxLiveEndpointPairTemplateStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1e13137b_737b_4a23_bc64_0870f75655ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairTemplateStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveInboundEndpointPairCreatedEventArgs {
    type Vtable = IXboxLiveInboundEndpointPairCreatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdc183b62_22ba_48d2_80de_c23968bd198b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs_abi(
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
pub struct IXboxLiveQualityOfServiceMeasurement(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveQualityOfServiceMeasurement {
    type Vtable = IXboxLiveQualityOfServiceMeasurement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4d682bce_a5d6_47e6_a236_cfde5fbdf2ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMeasurement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceaddress: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metric: XboxLiveQualityOfServiceMetric, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceaddress: ::windows::runtime::RawPtr, metric: XboxLiveQualityOfServiceMetric, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceaddress: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveQualityOfServiceMeasurementStatics {
    type Vtable = IXboxLiveQualityOfServiceMeasurementStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6e352dca_23cf_440a_b077_5e30857a8234);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, payload_array_size: u32, payload: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMetricResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveQualityOfServiceMetricResult {
    type Vtable = IXboxLiveQualityOfServiceMetricResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaeec53d1_3561_4782_b0cf_d3ae29d9fa87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMetricResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveQualityOfServiceMetric) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXboxLiveQualityOfServicePrivatePayloadResult {
    type Vtable = IXboxLiveQualityOfServicePrivatePayloadResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5a6302ae_6f38_41c0_9fcc_ea6cb978cafc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveDeviceAddress(pub ::windows::runtime::IInspectable);
impl XboxLiveDeviceAddress {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn SnapshotChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<XboxLiveDeviceAddress, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn RemoveSnapshotChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn GetSnapshotAsBase64(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_XboxLive`, `Storage_Streams`*"]
    pub fn GetSnapshotAsBuffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn GetSnapshotAsBytes(&self, buffer: &mut [<u8 as ::windows::runtime::DefaultType>::DefaultType], byteswritten: &mut u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer), byteswritten).ok() }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn Compare<'a, Param0: ::windows::runtime::IntoParam<'a, XboxLiveDeviceAddress>>(&self, otherdeviceaddress: Param0) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), otherdeviceaddress.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn IsValid(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn IsLocal(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn NetworkAccessKind(&self) -> ::windows::runtime::Result<XboxLiveNetworkAccessKind> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveNetworkAccessKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveNetworkAccessKind>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn CreateFromSnapshotBase64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(base64: Param0) -> ::windows::runtime::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), base64.into_param().abi(), &mut result__).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_XboxLive`, `Storage_Streams`*"]
    pub fn CreateFromSnapshotBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows::runtime::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn CreateFromSnapshotBytes(buffer: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), &mut result__).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn GetLocal() -> ::windows::runtime::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn MaxSnapshotBytesSize() -> ::windows::runtime::Result<u32> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IXboxLiveDeviceAddressStatics<R, F: FnOnce(&IXboxLiveDeviceAddressStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XboxLiveDeviceAddress, IXboxLiveDeviceAddressStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveDeviceAddress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveDeviceAddress;{f5bbd279-3c86-4b57-a31a-b9462408fd01})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveDeviceAddress {
    type Vtable = IXboxLiveDeviceAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf5bbd279_3c86_4b57_a31a_b9462408fd01);
}
impl ::windows::runtime::RuntimeName for XboxLiveDeviceAddress {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveDeviceAddress";
}
impl ::core::convert::From<XboxLiveDeviceAddress> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveDeviceAddress) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveDeviceAddress> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveDeviceAddress) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveDeviceAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveDeviceAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveDeviceAddress> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveDeviceAddress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveDeviceAddress> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveDeviceAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveDeviceAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveDeviceAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveDeviceAddress {}
unsafe impl ::core::marker::Sync for XboxLiveDeviceAddress {}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveEndpointPair(pub ::windows::runtime::IInspectable);
impl XboxLiveEndpointPair {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn DeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn GetRemoteSocketAddressBytes(&self, socketaddress: &mut [<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), socketaddress.len() as u32, ::core::mem::transmute_copy(&socketaddress)).ok() }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn GetLocalSocketAddressBytes(&self, socketaddress: &mut [<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), socketaddress.len() as u32, ::core::mem::transmute_copy(&socketaddress)).ok() }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn State(&self) -> ::windows::runtime::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveEndpointPairState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn Template(&self) -> ::windows::runtime::Result<XboxLiveEndpointPairTemplate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveEndpointPairTemplate>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn RemoteDeviceAddress(&self) -> ::windows::runtime::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn RemoteHostName(&self) -> ::windows::runtime::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn RemotePort(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn LocalHostName(&self) -> ::windows::runtime::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn LocalPort(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn FindEndpointPairBySocketAddressBytes(localsocketaddress: &[<u8 as ::windows::runtime::DefaultType>::DefaultType], remotesocketaddress: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<XboxLiveEndpointPair> {
        Self::IXboxLiveEndpointPairStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), localsocketaddress.len() as u32, ::core::mem::transmute(localsocketaddress.as_ptr()), remotesocketaddress.len() as u32, ::core::mem::transmute(remotesocketaddress.as_ptr()), &mut result__).from_abi::<XboxLiveEndpointPair>(result__)
        })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn FindEndpointPairByHostNamesAndPorts<'a, Param0: ::windows::runtime::IntoParam<'a, super::HostName>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::HostName>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(localhostname: Param0, localport: Param1, remotehostname: Param2, remoteport: Param3) -> ::windows::runtime::Result<XboxLiveEndpointPair> {
        Self::IXboxLiveEndpointPairStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), localhostname.into_param().abi(), localport.into_param().abi(), remotehostname.into_param().abi(), remoteport.into_param().abi(), &mut result__).from_abi::<XboxLiveEndpointPair>(result__)
        })
    }
    pub fn IXboxLiveEndpointPairStatics<R, F: FnOnce(&IXboxLiveEndpointPairStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XboxLiveEndpointPair, IXboxLiveEndpointPairStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveEndpointPair {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPair;{1e9a839b-813e-44e0-b87f-c87a093475e4})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveEndpointPair {
    type Vtable = IXboxLiveEndpointPair_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1e9a839b_813e_44e0_b87f_c87a093475e4);
}
impl ::windows::runtime::RuntimeName for XboxLiveEndpointPair {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPair";
}
impl ::core::convert::From<XboxLiveEndpointPair> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveEndpointPair) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveEndpointPair> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveEndpointPair) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveEndpointPair {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveEndpointPair {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveEndpointPair> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveEndpointPair) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveEndpointPair> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveEndpointPair) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveEndpointPair {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveEndpointPair {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPair {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPair {}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationBehaviors(pub u32);
impl XboxLiveEndpointPairCreationBehaviors {
    pub const None: XboxLiveEndpointPairCreationBehaviors = XboxLiveEndpointPairCreationBehaviors(0u32);
    pub const ReevaluatePath: XboxLiveEndpointPairCreationBehaviors = XboxLiveEndpointPairCreationBehaviors(1u32);
}
impl ::core::convert::From<u32> for XboxLiveEndpointPairCreationBehaviors {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XboxLiveEndpointPairCreationBehaviors {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveEndpointPairCreationBehaviors {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationBehaviors;u4)");
}
impl ::windows::runtime::DefaultType for XboxLiveEndpointPairCreationBehaviors {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for XboxLiveEndpointPairCreationBehaviors {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for XboxLiveEndpointPairCreationBehaviors {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveEndpointPairCreationResult(pub ::windows::runtime::IInspectable);
impl XboxLiveEndpointPairCreationResult {
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn DeviceAddress(&self) -> ::windows::runtime::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<XboxLiveEndpointPairCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveEndpointPairCreationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveEndpointPairCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn IsExistingPathEvaluation(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn EndpointPair(&self) -> ::windows::runtime::Result<XboxLiveEndpointPair> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveEndpointPair>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveEndpointPairCreationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult;{d9a8bb95-2aab-4d1e-9794-33ecc0dcf0fe})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveEndpointPairCreationResult {
    type Vtable = IXboxLiveEndpointPairCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd9a8bb95_2aab_4d1e_9794_33ecc0dcf0fe);
}
impl ::windows::runtime::RuntimeName for XboxLiveEndpointPairCreationResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult";
}
impl ::core::convert::From<XboxLiveEndpointPairCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveEndpointPairCreationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveEndpointPairCreationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveEndpointPairCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveEndpointPairCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveEndpointPairCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveEndpointPairCreationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveEndpointPairCreationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveEndpointPairCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveEndpointPairCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairCreationResult {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairCreationResult {}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationStatus(pub i32);
impl XboxLiveEndpointPairCreationStatus {
    pub const Succeeded: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(0i32);
    pub const NoLocalNetworks: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(1i32);
    pub const NoCompatibleNetworkPaths: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(2i32);
    pub const LocalSystemNotAuthorized: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(3i32);
    pub const Canceled: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(4i32);
    pub const TimedOut: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(5i32);
    pub const RemoteSystemNotAuthorized: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(6i32);
    pub const RefusedDueToConfiguration: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(7i32);
    pub const UnexpectedInternalError: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(8i32);
}
impl ::core::convert::From<i32> for XboxLiveEndpointPairCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XboxLiveEndpointPairCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveEndpointPairCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationStatus;i4)");
}
impl ::windows::runtime::DefaultType for XboxLiveEndpointPairCreationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XboxLiveEndpointPairState(pub i32);
impl XboxLiveEndpointPairState {
    pub const Invalid: XboxLiveEndpointPairState = XboxLiveEndpointPairState(0i32);
    pub const CreatingOutbound: XboxLiveEndpointPairState = XboxLiveEndpointPairState(1i32);
    pub const CreatingInbound: XboxLiveEndpointPairState = XboxLiveEndpointPairState(2i32);
    pub const Ready: XboxLiveEndpointPairState = XboxLiveEndpointPairState(3i32);
    pub const DeletingLocally: XboxLiveEndpointPairState = XboxLiveEndpointPairState(4i32);
    pub const RemoteEndpointTerminating: XboxLiveEndpointPairState = XboxLiveEndpointPairState(5i32);
    pub const Deleted: XboxLiveEndpointPairState = XboxLiveEndpointPairState(6i32);
}
impl ::core::convert::From<i32> for XboxLiveEndpointPairState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XboxLiveEndpointPairState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveEndpointPairState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairState;i4)");
}
impl ::windows::runtime::DefaultType for XboxLiveEndpointPairState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveEndpointPairStateChangedEventArgs(pub ::windows::runtime::IInspectable);
impl XboxLiveEndpointPairStateChangedEventArgs {
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn OldState(&self) -> ::windows::runtime::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveEndpointPairState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn NewState(&self) -> ::windows::runtime::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveEndpointPairState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveEndpointPairStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs;{592e3b55-de08-44e7-ac3b-b9b9a169583a})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveEndpointPairStateChangedEventArgs {
    type Vtable = IXboxLiveEndpointPairStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x592e3b55_de08_44e7_ac3b_b9b9a169583a);
}
impl ::windows::runtime::RuntimeName for XboxLiveEndpointPairStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs";
}
impl ::core::convert::From<XboxLiveEndpointPairStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveEndpointPairStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveEndpointPairStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveEndpointPairStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveEndpointPairStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveEndpointPairStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairStateChangedEventArgs {}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveEndpointPairTemplate(pub ::windows::runtime::IInspectable);
impl XboxLiveEndpointPairTemplate {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn InboundEndpointPairCreated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn RemoveInboundEndpointPairCreated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn CreateEndpointPairDefaultAsync<'a, Param0: ::windows::runtime::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceaddress.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn CreateEndpointPairWithBehaviorsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), deviceaddress.into_param().abi(), behaviors, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn CreateEndpointPairForPortsDefaultAsync<'a, Param0: ::windows::runtime::IntoParam<'a, XboxLiveDeviceAddress>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, deviceaddress: Param0, initiatorport: Param1, acceptorport: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), deviceaddress.into_param().abi(), initiatorport.into_param().abi(), acceptorport.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn CreateEndpointPairForPortsWithBehaviorsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, XboxLiveDeviceAddress>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        deviceaddress: Param0,
        initiatorport: Param1,
        acceptorport: Param2,
        behaviors: XboxLiveEndpointPairCreationBehaviors,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), deviceaddress.into_param().abi(), initiatorport.into_param().abi(), acceptorport.into_param().abi(), behaviors, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn SocketKind(&self) -> ::windows::runtime::Result<XboxLiveSocketKind> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveSocketKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveSocketKind>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn InitiatorBoundPortRangeLower(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn InitiatorBoundPortRangeUpper(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn AcceptorBoundPortRangeLower(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn AcceptorBoundPortRangeUpper(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation_Collections`*"]
    pub fn EndpointPairs(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPair>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPair>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn GetTemplateByName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<XboxLiveEndpointPairTemplate> {
        Self::IXboxLiveEndpointPairTemplateStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<XboxLiveEndpointPairTemplate>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation_Collections`*"]
    pub fn Templates() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>> {
        Self::IXboxLiveEndpointPairTemplateStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>>(result__)
        })
    }
    pub fn IXboxLiveEndpointPairTemplateStatics<R, F: FnOnce(&IXboxLiveEndpointPairTemplateStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XboxLiveEndpointPairTemplate, IXboxLiveEndpointPairTemplateStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveEndpointPairTemplate {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate;{6b286ecf-3457-40ce-b9a1-c0cfe0213ea7})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveEndpointPairTemplate {
    type Vtable = IXboxLiveEndpointPairTemplate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6b286ecf_3457_40ce_b9a1_c0cfe0213ea7);
}
impl ::windows::runtime::RuntimeName for XboxLiveEndpointPairTemplate {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate";
}
impl ::core::convert::From<XboxLiveEndpointPairTemplate> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveEndpointPairTemplate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairTemplate> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveEndpointPairTemplate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveEndpointPairTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveEndpointPairTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveEndpointPairTemplate> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveEndpointPairTemplate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairTemplate> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveEndpointPairTemplate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveEndpointPairTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveEndpointPairTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairTemplate {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairTemplate {}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(pub ::windows::runtime::IInspectable);
impl XboxLiveInboundEndpointPairCreatedEventArgs {
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn EndpointPair(&self) -> ::windows::runtime::Result<XboxLiveEndpointPair> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveEndpointPair>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveInboundEndpointPairCreatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs;{dc183b62-22ba-48d2-80de-c23968bd198b})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveInboundEndpointPairCreatedEventArgs {
    type Vtable = IXboxLiveInboundEndpointPairCreatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdc183b62_22ba_48d2_80de_c23968bd198b);
}
impl ::windows::runtime::RuntimeName for XboxLiveInboundEndpointPairCreatedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs";
}
impl ::core::convert::From<XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveInboundEndpointPairCreatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveInboundEndpointPairCreatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveInboundEndpointPairCreatedEventArgs {}
unsafe impl ::core::marker::Sync for XboxLiveInboundEndpointPairCreatedEventArgs {}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XboxLiveNetworkAccessKind(pub i32);
impl XboxLiveNetworkAccessKind {
    pub const Open: XboxLiveNetworkAccessKind = XboxLiveNetworkAccessKind(0i32);
    pub const Moderate: XboxLiveNetworkAccessKind = XboxLiveNetworkAccessKind(1i32);
    pub const Strict: XboxLiveNetworkAccessKind = XboxLiveNetworkAccessKind(2i32);
}
impl ::core::convert::From<i32> for XboxLiveNetworkAccessKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XboxLiveNetworkAccessKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveNetworkAccessKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveNetworkAccessKind;i4)");
}
impl ::windows::runtime::DefaultType for XboxLiveNetworkAccessKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveQualityOfServiceMeasurement(pub ::windows::runtime::IInspectable);
impl XboxLiveQualityOfServiceMeasurement {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XboxLiveQualityOfServiceMeasurement, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation`*"]
    pub fn MeasureAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation_Collections`*"]
    pub fn GetMetricResultsForDevice<'a, Param0: ::windows::runtime::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceaddress.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation_Collections`*"]
    pub fn GetMetricResultsForMetric(&self, metric: XboxLiveQualityOfServiceMetric) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), metric, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn GetMetricResult<'a, Param0: ::windows::runtime::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0, metric: XboxLiveQualityOfServiceMetric) -> ::windows::runtime::Result<XboxLiveQualityOfServiceMetricResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), deviceaddress.into_param().abi(), metric, &mut result__).from_abi::<XboxLiveQualityOfServiceMetricResult>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn GetPrivatePayloadResult<'a, Param0: ::windows::runtime::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0) -> ::windows::runtime::Result<XboxLiveQualityOfServicePrivatePayloadResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), deviceaddress.into_param().abi(), &mut result__).from_abi::<XboxLiveQualityOfServicePrivatePayloadResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation_Collections`*"]
    pub fn Metrics(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation_Collections`*"]
    pub fn DeviceAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<XboxLiveDeviceAddress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<XboxLiveDeviceAddress>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn ShouldRequestPrivatePayloads(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn SetShouldRequestPrivatePayloads(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn TimeoutInMilliseconds(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn SetTimeoutInMilliseconds(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn NumberOfProbesToAttempt(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn SetNumberOfProbesToAttempt(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn NumberOfResultsPending(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation_Collections`*"]
    pub fn MetricResults(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_XboxLive`, `Foundation_Collections`*"]
    pub fn PrivatePayloadResults(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn PublishPrivatePayloadBytes(payload: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), payload.len() as u32, ::core::mem::transmute(payload.as_ptr())).ok() })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn ClearPrivatePayload() -> ::windows::runtime::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn MaxSimultaneousProbeConnections() -> ::windows::runtime::Result<u32> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn SetMaxSimultaneousProbeConnections(value: u32) -> ::windows::runtime::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn IsSystemOutboundBandwidthConstrained() -> ::windows::runtime::Result<bool> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn SetIsSystemOutboundBandwidthConstrained(value: bool) -> ::windows::runtime::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn IsSystemInboundBandwidthConstrained() -> ::windows::runtime::Result<bool> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn SetIsSystemInboundBandwidthConstrained(value: bool) -> ::windows::runtime::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_XboxLive`, `Storage_Streams`*"]
    pub fn PublishedPrivatePayload() -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_XboxLive`, `Storage_Streams`*"]
    pub fn SetPublishedPrivatePayload<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn MaxPrivatePayloadSize() -> ::windows::runtime::Result<u32> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IXboxLiveQualityOfServiceMeasurementStatics<R, F: FnOnce(&IXboxLiveQualityOfServiceMeasurementStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XboxLiveQualityOfServiceMeasurement, IXboxLiveQualityOfServiceMeasurementStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveQualityOfServiceMeasurement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement;{4d682bce-a5d6-47e6-a236-cfde5fbdf2ed})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveQualityOfServiceMeasurement {
    type Vtable = IXboxLiveQualityOfServiceMeasurement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4d682bce_a5d6_47e6_a236_cfde5fbdf2ed);
}
impl ::windows::runtime::RuntimeName for XboxLiveQualityOfServiceMeasurement {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement";
}
impl ::core::convert::From<XboxLiveQualityOfServiceMeasurement> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveQualityOfServiceMeasurement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMeasurement> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveQualityOfServiceMeasurement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveQualityOfServiceMeasurement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveQualityOfServiceMeasurement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveQualityOfServiceMeasurement> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveQualityOfServiceMeasurement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMeasurement> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveQualityOfServiceMeasurement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveQualityOfServiceMeasurement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveQualityOfServiceMeasurement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServiceMeasurement {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServiceMeasurement {}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurementStatus(pub i32);
impl XboxLiveQualityOfServiceMeasurementStatus {
    pub const NotStarted: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(0i32);
    pub const InProgress: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(1i32);
    pub const InProgressWithProvisionalResults: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(2i32);
    pub const Succeeded: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(3i32);
    pub const NoLocalNetworks: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(4i32);
    pub const NoCompatibleNetworkPaths: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(5i32);
    pub const LocalSystemNotAuthorized: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(6i32);
    pub const Canceled: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(7i32);
    pub const TimedOut: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(8i32);
    pub const RemoteSystemNotAuthorized: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(9i32);
    pub const RefusedDueToConfiguration: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(10i32);
    pub const UnexpectedInternalError: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(11i32);
}
impl ::core::convert::From<i32> for XboxLiveQualityOfServiceMeasurementStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XboxLiveQualityOfServiceMeasurementStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveQualityOfServiceMeasurementStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurementStatus;i4)");
}
impl ::windows::runtime::DefaultType for XboxLiveQualityOfServiceMeasurementStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetric(pub i32);
impl XboxLiveQualityOfServiceMetric {
    pub const AverageLatencyInMilliseconds: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(0i32);
    pub const MinLatencyInMilliseconds: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(1i32);
    pub const MaxLatencyInMilliseconds: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(2i32);
    pub const AverageOutboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(3i32);
    pub const MinOutboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(4i32);
    pub const MaxOutboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(5i32);
    pub const AverageInboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(6i32);
    pub const MinInboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(7i32);
    pub const MaxInboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(8i32);
}
impl ::core::convert::From<i32> for XboxLiveQualityOfServiceMetric {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XboxLiveQualityOfServiceMetric {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveQualityOfServiceMetric {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric;i4)");
}
impl ::windows::runtime::DefaultType for XboxLiveQualityOfServiceMetric {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveQualityOfServiceMetricResult(pub ::windows::runtime::IInspectable);
impl XboxLiveQualityOfServiceMetricResult {
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<XboxLiveQualityOfServiceMeasurementStatus> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveQualityOfServiceMeasurementStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveQualityOfServiceMeasurementStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn DeviceAddress(&self) -> ::windows::runtime::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn Metric(&self) -> ::windows::runtime::Result<XboxLiveQualityOfServiceMetric> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveQualityOfServiceMetric = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveQualityOfServiceMetric>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveQualityOfServiceMetricResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult;{aeec53d1-3561-4782-b0cf-d3ae29d9fa87})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveQualityOfServiceMetricResult {
    type Vtable = IXboxLiveQualityOfServiceMetricResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaeec53d1_3561_4782_b0cf_d3ae29d9fa87);
}
impl ::windows::runtime::RuntimeName for XboxLiveQualityOfServiceMetricResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult";
}
impl ::core::convert::From<XboxLiveQualityOfServiceMetricResult> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveQualityOfServiceMetricResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMetricResult> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveQualityOfServiceMetricResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveQualityOfServiceMetricResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveQualityOfServiceMetricResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveQualityOfServiceMetricResult> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveQualityOfServiceMetricResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMetricResult> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveQualityOfServiceMetricResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveQualityOfServiceMetricResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveQualityOfServiceMetricResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServiceMetricResult {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServiceMetricResult {}
#[doc = "*Required features: `Networking_XboxLive`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XboxLiveQualityOfServicePrivatePayloadResult(pub ::windows::runtime::IInspectable);
impl XboxLiveQualityOfServicePrivatePayloadResult {
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<XboxLiveQualityOfServiceMeasurementStatus> {
        let this = self;
        unsafe {
            let mut result__: XboxLiveQualityOfServiceMeasurementStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveQualityOfServiceMeasurementStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_XboxLive`*"]
    pub fn DeviceAddress(&self) -> ::windows::runtime::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_XboxLive`, `Storage_Streams`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveQualityOfServicePrivatePayloadResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult;{5a6302ae-6f38-41c0-9fcc-ea6cb978cafc})");
}
unsafe impl ::windows::runtime::Interface for XboxLiveQualityOfServicePrivatePayloadResult {
    type Vtable = IXboxLiveQualityOfServicePrivatePayloadResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5a6302ae_6f38_41c0_9fcc_ea6cb978cafc);
}
impl ::windows::runtime::RuntimeName for XboxLiveQualityOfServicePrivatePayloadResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult";
}
impl ::core::convert::From<XboxLiveQualityOfServicePrivatePayloadResult> for ::windows::runtime::IUnknown {
    fn from(value: XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServicePrivatePayloadResult> for ::windows::runtime::IUnknown {
    fn from(value: &XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XboxLiveQualityOfServicePrivatePayloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XboxLiveQualityOfServicePrivatePayloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XboxLiveQualityOfServicePrivatePayloadResult> for ::windows::runtime::IInspectable {
    fn from(value: XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServicePrivatePayloadResult> for ::windows::runtime::IInspectable {
    fn from(value: &XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XboxLiveQualityOfServicePrivatePayloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XboxLiveQualityOfServicePrivatePayloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServicePrivatePayloadResult {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServicePrivatePayloadResult {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct XboxLiveSecureSocketsContract(pub u8);
#[doc = "*Required features: `Networking_XboxLive`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XboxLiveSocketKind(pub i32);
impl XboxLiveSocketKind {
    pub const None: XboxLiveSocketKind = XboxLiveSocketKind(0i32);
    pub const Datagram: XboxLiveSocketKind = XboxLiveSocketKind(1i32);
    pub const Stream: XboxLiveSocketKind = XboxLiveSocketKind(2i32);
}
impl ::core::convert::From<i32> for XboxLiveSocketKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XboxLiveSocketKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XboxLiveSocketKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveSocketKind;i4)");
}
impl ::windows::runtime::DefaultType for XboxLiveSocketKind {
    type DefaultType = Self;
}
