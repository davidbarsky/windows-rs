#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct INamedResource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INamedResource {
    type Vtable = INamedResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1c98c219_0b13_4240_89a5_d495dc189a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcecontext: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcecontext: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceCandidate(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceCandidate {
    type Vtable = IResourceCandidate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaf5207d9_c433_4764_b3fd_8fa6bfbcbadc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, qualifiername: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceCandidate2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceCandidate2 {
    type Vtable = IResourceCandidate2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x69e5b468_f6fc_4013_aaa2_d53f1757d3b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceCandidate3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceCandidate3 {
    type Vtable = IResourceCandidate3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x08ae97f8_517a_4674_958c_4a3c7cd2cc6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ResourceCandidateKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceContext(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceContext {
    type Vtable = IResourceContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2fa22f4b_707e_4b27_ad0d_d0d8cd468fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, qualifiernames: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languages: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceContextStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceContextStatics {
    type Vtable = IResourceContextStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x98be9d6c_6338_4b31_99df_b2b442f17149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceContextStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceContextStatics2 {
    type Vtable = IResourceContextStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x41f752ef_12af_41b9_ab36_b1eb4b512460);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, qualifiernames: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceContextStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceContextStatics3 {
    type Vtable = IResourceContextStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20cf492c_af0f_450b_9da6_106dd0c29a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, persistence: ResourceQualifierPersistence) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceContextStatics4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceContextStatics4 {
    type Vtable = IResourceContextStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x22eb9ccd_fb31_4bfa_b86b_df9d9d7bdc39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceManager {
    type Vtable = IResourceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf744d97b_9988_44fb_abd6_5378844cfa8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, files: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, files: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceManager2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9d66fe6c_a4d7_4c23_9e85_675f304c252d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceManagerStatics {
    type Vtable = IResourceManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1cc0fdfc_69ee_4e43_9901_47f12687baf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcereference: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceMap(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceMap {
    type Vtable = IResourceMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x72284824_db8c_42f8_b08c_53ff357dad82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, context: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reference: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceQualifier(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceQualifier {
    type Vtable = IResourceQualifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x785da5b2_4afd_4376_a888_c5f9a6b7a05c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceQualifier_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NamedResource(pub ::windows::runtime::IInspectable);
impl NamedResource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Candidates(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn Resolve(&self) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn ResolveForContext<'a, Param0: ::windows::runtime::IntoParam<'a, ResourceContext>>(&self, resourcecontext: Param0) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), resourcecontext.into_param().abi(), &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn ResolveAll(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn ResolveAllForContext<'a, Param0: ::windows::runtime::IntoParam<'a, ResourceContext>>(&self, resourcecontext: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), resourcecontext.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NamedResource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.NamedResource;{1c98c219-0b13-4240-89a5-d495dc189a00})");
}
unsafe impl ::windows::runtime::Interface for NamedResource {
    type Vtable = INamedResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1c98c219_0b13_4240_89a5_d495dc189a00);
}
impl ::windows::runtime::RuntimeName for NamedResource {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.NamedResource";
}
impl ::core::convert::From<NamedResource> for ::windows::runtime::IUnknown {
    fn from(value: NamedResource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NamedResource> for ::windows::runtime::IUnknown {
    fn from(value: &NamedResource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NamedResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a NamedResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NamedResource> for ::windows::runtime::IInspectable {
    fn from(value: NamedResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NamedResource> for ::windows::runtime::IInspectable {
    fn from(value: &NamedResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NamedResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NamedResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NamedResource {}
unsafe impl ::core::marker::Sync for NamedResource {}
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceCandidate(pub ::windows::runtime::IInspectable);
impl ResourceCandidate {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Qualifiers(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn IsMatch(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn IsMatchAsDefault(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn IsDefault(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn ValueAsString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation`, `Storage`*"]
    pub fn GetValueAsFileAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn GetQualifierValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, qualifiername: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), qualifiername.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation`, `Storage_Streams`*"]
    pub fn GetValueAsStreamAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows::runtime::Interface::cast::<IResourceCandidate2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ResourceCandidateKind> {
        let this = &::windows::runtime::Interface::cast::<IResourceCandidate3>(self)?;
        unsafe {
            let mut result__: ResourceCandidateKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceCandidateKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceCandidate {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceCandidate;{af5207d9-c433-4764-b3fd-8fa6bfbcbadc})");
}
unsafe impl ::windows::runtime::Interface for ResourceCandidate {
    type Vtable = IResourceCandidate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaf5207d9_c433_4764_b3fd_8fa6bfbcbadc);
}
impl ::windows::runtime::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidate";
}
impl ::core::convert::From<ResourceCandidate> for ::windows::runtime::IUnknown {
    fn from(value: ResourceCandidate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceCandidate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceCandidate> for ::windows::runtime::IInspectable {
    fn from(value: ResourceCandidate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceCandidate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceCandidate {}
unsafe impl ::core::marker::Sync for ResourceCandidate {}
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const String: ResourceCandidateKind = ResourceCandidateKind(0i32);
    pub const File: ResourceCandidateKind = ResourceCandidateKind(1i32);
    pub const EmbeddedData: ResourceCandidateKind = ResourceCandidateKind(2i32);
}
impl ::core::convert::From<i32> for ResourceCandidateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ResourceCandidateKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceCandidateKind;i4)");
}
impl ::windows::runtime::DefaultType for ResourceCandidateKind {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceCandidateVectorView(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ResourceCandidateVectorView {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, ResourceCandidate>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ResourceCandidate as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<ResourceCandidate>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ResourceCandidate>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ResourceCandidate>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for ResourceCandidateVectorView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.ApplicationModel.Resources.Core.ResourceCandidate;{af5207d9-c433-4764-b3fd-8fa6bfbcbadc})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for ResourceCandidateVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_abi<ResourceCandidate>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for ResourceCandidateVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceCandidateVectorView> for ::windows::runtime::IUnknown {
    fn from(value: ResourceCandidateVectorView) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceCandidateVectorView> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceCandidateVectorView) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceCandidateVectorView> for ::windows::runtime::IInspectable {
    fn from(value: ResourceCandidateVectorView) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceCandidateVectorView> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceCandidateVectorView) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceCandidateVectorView> for super::super::super::Foundation::Collections::IVectorView<ResourceCandidate> {
    fn from(value: ResourceCandidateVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceCandidateVectorView> for super::super::super::Foundation::Collections::IVectorView<ResourceCandidate> {
    fn from(value: &ResourceCandidateVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> for &ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceCandidateVectorView> for super::super::super::Foundation::Collections::IIterable<ResourceCandidate> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceCandidateVectorView) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceCandidateVectorView> for super::super::super::Foundation::Collections::IIterable<ResourceCandidate> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceCandidateVectorView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> for &ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ResourceCandidate>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceCandidateVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceCandidateVectorView {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceContext(pub ::windows::runtime::IInspectable);
impl ResourceContext {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceContext, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn QualifierValues(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn ResetQualifierValues<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, qualifiernames: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), qualifiernames.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn OverrideToMatch<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>>(&self, result: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), result.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn Clone(&self) -> ::windows::runtime::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceContext>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn SetLanguages<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(&self, languages: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), languages.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn CreateMatchingContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>>(result: Param0) -> ::windows::runtime::Result<ResourceContext> {
        Self::IResourceContextStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), result.into_param().abi(), &mut result__).from_abi::<ResourceContext>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceContext>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn SetGlobalQualifierValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(key: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn ResetGlobalQualifierValues() -> ::windows::runtime::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn ResetGlobalQualifierValuesForSpecifiedQualifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(qualifiernames: Param0) -> ::windows::runtime::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), qualifiernames.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn GetForViewIndependentUse() -> ::windows::runtime::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceContext>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn SetGlobalQualifierValueWithPersistence<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(key: Param0, value: Param1, persistence: ResourceQualifierPersistence) -> ::windows::runtime::Result<()> {
        Self::IResourceContextStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), persistence).ok() })
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `UI`*"]
    pub fn GetForUIContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::UIContext>>(context: Param0) -> ::windows::runtime::Result<ResourceContext> {
        Self::IResourceContextStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn IResourceContextStatics<R, F: FnOnce(&IResourceContextStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceContext, IResourceContextStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics2<R, F: FnOnce(&IResourceContextStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceContext, IResourceContextStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics3<R, F: FnOnce(&IResourceContextStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceContext, IResourceContextStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics4<R, F: FnOnce(&IResourceContextStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceContext, IResourceContextStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceContext;{2fa22f4b-707e-4b27-ad0d-d0d8cd468fd2})");
}
unsafe impl ::windows::runtime::Interface for ResourceContext {
    type Vtable = IResourceContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2fa22f4b_707e_4b27_ad0d_d0d8cd468fd2);
}
impl ::windows::runtime::RuntimeName for ResourceContext {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContext";
}
impl ::core::convert::From<ResourceContext> for ::windows::runtime::IUnknown {
    fn from(value: ResourceContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceContext> for ::windows::runtime::IInspectable {
    fn from(value: ResourceContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceContext {}
unsafe impl ::core::marker::Sync for ResourceContext {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceContextLanguagesVectorView(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ResourceContextLanguagesVectorView {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<::windows::runtime::HSTRING>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for ResourceContextLanguagesVectorView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for ResourceContextLanguagesVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_abi<::windows::runtime::HSTRING>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for ResourceContextLanguagesVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceContextLanguagesVectorView> for ::windows::runtime::IUnknown {
    fn from(value: ResourceContextLanguagesVectorView) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceContextLanguagesVectorView> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceContextLanguagesVectorView) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceContextLanguagesVectorView> for ::windows::runtime::IInspectable {
    fn from(value: ResourceContextLanguagesVectorView) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceContextLanguagesVectorView> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceContextLanguagesVectorView) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceContextLanguagesVectorView> for super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING> {
    fn from(value: ResourceContextLanguagesVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceContextLanguagesVectorView> for super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING> {
    fn from(value: &ResourceContextLanguagesVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> for &ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceContextLanguagesVectorView> for super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceContextLanguagesVectorView) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceContextLanguagesVectorView> for super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceContextLanguagesVectorView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>> for &ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceContextLanguagesVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceContextLanguagesVectorView {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ResourceContextLanguagesVectorView {
    type Item = ::windows::runtime::HSTRING;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ResourceContextLanguagesVectorView {
    type Item = ::windows::runtime::HSTRING;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
pub struct ResourceLayoutInfo {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ResourceSubtreeCount: u32,
    pub NamedResourceCount: u32,
    pub Checksum: i32,
}
impl ResourceLayoutInfo {}
impl ::core::default::Default for ResourceLayoutInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ResourceLayoutInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ResourceLayoutInfo").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("ResourceSubtreeCount", &self.ResourceSubtreeCount).field("NamedResourceCount", &self.NamedResourceCount).field("Checksum", &self.Checksum).finish()
    }
}
impl ::core::cmp::PartialEq for ResourceLayoutInfo {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.ResourceSubtreeCount == other.ResourceSubtreeCount && self.NamedResourceCount == other.NamedResourceCount && self.Checksum == other.Checksum
    }
}
impl ::core::cmp::Eq for ResourceLayoutInfo {}
unsafe impl ::windows::runtime::Abi for ResourceLayoutInfo {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ResourceLayoutInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.Resources.Core.ResourceLayoutInfo;u4;u4;u4;u4;i4)");
}
impl ::windows::runtime::DefaultType for ResourceLayoutInfo {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceManager(pub ::windows::runtime::IInspectable);
impl ResourceManager {
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn MainResourceMap(&self) -> ::windows::runtime::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceMap>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn AllResourceMaps(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn DefaultContext(&self) -> ::windows::runtime::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceContext>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`, `Storage`*"]
    pub fn LoadPriFiles<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>>(&self, files: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), files.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`, `Storage`*"]
    pub fn UnloadPriFiles<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>>(&self, files: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), files.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetAllNamedResourcesForPackage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ResourceLayoutInfo>>(&self, packagename: Param0, resourcelayoutinfo: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<NamedResource>> {
        let this = &::windows::runtime::Interface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagename.into_param().abi(), resourcelayoutinfo.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<NamedResource>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetAllSubtreesForPackage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ResourceLayoutInfo>>(&self, packagename: Param0, resourcelayoutinfo: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<ResourceMap>> {
        let this = &::windows::runtime::Interface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packagename.into_param().abi(), resourcelayoutinfo.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceMap>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn Current() -> ::windows::runtime::Result<ResourceManager> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceManager>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn IsResourceReference<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(resourcereference: Param0) -> ::windows::runtime::Result<bool> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), resourcereference.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IResourceManagerStatics<R, F: FnOnce(&IResourceManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceManager, IResourceManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceManager;{f744d97b-9988-44fb-abd6-5378844cfa8b})");
}
unsafe impl ::windows::runtime::Interface for ResourceManager {
    type Vtable = IResourceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf744d97b_9988_44fb_abd6_5378844cfa8b);
}
impl ::windows::runtime::RuntimeName for ResourceManager {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceManager";
}
impl ::core::convert::From<ResourceManager> for ::windows::runtime::IUnknown {
    fn from(value: ResourceManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceManager> for ::windows::runtime::IInspectable {
    fn from(value: ResourceManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceManager {}
unsafe impl ::core::marker::Sync for ResourceManager {}
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceMap(pub ::windows::runtime::IInspectable);
impl ResourceMap {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, resource: Param0) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), resource.into_param().abi(), &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn GetValueForContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ResourceContext>>(&self, resource: Param0, context: Param1) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), resource.into_param().abi(), context.into_param().abi(), &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn GetSubtree<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reference: Param0) -> ::windows::runtime::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), reference.into_param().abi(), &mut result__).from_abi::<ResourceMap>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<NamedResource> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<NamedResource>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceMap {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82})");
}
unsafe impl ::windows::runtime::Interface for ResourceMap {
    type Vtable = IResourceMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x72284824_db8c_42f8_b08c_53ff357dad82);
}
impl ::windows::runtime::RuntimeName for ResourceMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMap";
}
impl ::core::convert::From<ResourceMap> for ::windows::runtime::IUnknown {
    fn from(value: ResourceMap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceMap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceMap> for ::windows::runtime::IInspectable {
    fn from(value: ResourceMap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> for ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> for &ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMap> for super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMap> for super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>> for ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>> for &ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, NamedResource>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ResourceMap {}
unsafe impl ::core::marker::Sync for ResourceMap {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ResourceMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ResourceMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceMapIterator(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapIterator {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Current(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn HasCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn MoveNext(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetMany(&self, items: &mut [<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource> as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for ResourceMapIterator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;rc(Windows.ApplicationModel.Resources.Core.NamedResource;{1c98c219-0b13-4240-89a5-d495dc189a00}))))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for ResourceMapIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_abi<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for ResourceMapIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapIterator> for ::windows::runtime::IUnknown {
    fn from(value: ResourceMapIterator) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapIterator> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceMapIterator) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceMapIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceMapIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapIterator> for ::windows::runtime::IInspectable {
    fn from(value: ResourceMapIterator) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapIterator> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceMapIterator) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceMapIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceMapIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapIterator> for super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>> {
    fn from(value: ResourceMapIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapIterator> for super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>> {
    fn from(value: &ResourceMapIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> for ResourceMapIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> for &ResourceMapIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, NamedResource>>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceMapMapView(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapMapView {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<ResourceMap>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap>>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for ResourceMapMapView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapMapView;pinterface({e480ce40-a338-4ada-adcf-272272e48cb9};string;rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for ResourceMapMapView {
    type Vtable = super::super::super::Foundation::Collections::IMapView_abi<::windows::runtime::HSTRING, ResourceMap>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for ResourceMapMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapView> for ::windows::runtime::IUnknown {
    fn from(value: ResourceMapMapView) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapView> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceMapMapView) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceMapMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceMapMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapView> for ::windows::runtime::IInspectable {
    fn from(value: ResourceMapMapView) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapView> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceMapMapView) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceMapMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceMapMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapView> for super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap> {
    fn from(value: ResourceMapMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapView> for super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap> {
    fn from(value: &ResourceMapMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap>> for ResourceMapMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap>> for &ResourceMapMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ResourceMap>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMapMapView> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceMapMapView) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMapMapView> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceMapMapView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> for ResourceMapMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> for &ResourceMapMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapMapView {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ResourceMapMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ResourceMapMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceMapMapViewIterator(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapMapViewIterator {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Current(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn HasCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn MoveNext(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetMany(&self, items: &mut [<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap> as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for ResourceMapMapViewIterator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82}))))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for ResourceMapMapViewIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_abi<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for ResourceMapMapViewIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapViewIterator> for ::windows::runtime::IUnknown {
    fn from(value: ResourceMapMapViewIterator) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapViewIterator> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceMapMapViewIterator) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapViewIterator> for ::windows::runtime::IInspectable {
    fn from(value: ResourceMapMapViewIterator) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapViewIterator> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceMapMapViewIterator) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapViewIterator> for super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>> {
    fn from(value: ResourceMapMapViewIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapViewIterator> for super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>> {
    fn from(value: &ResourceMapMapViewIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> for &ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ResourceMap>>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapMapViewIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapMapViewIterator {}
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceQualifier(pub ::windows::runtime::IInspectable);
impl ResourceQualifier {
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn QualifierName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn QualifierValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn IsDefault(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn IsMatch(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
    pub fn Score(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceQualifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifier;{785da5b2-4afd-4376-a888-c5f9a6b7a05c})");
}
unsafe impl ::windows::runtime::Interface for ResourceQualifier {
    type Vtable = IResourceQualifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x785da5b2_4afd_4376_a888_c5f9a6b7a05c);
}
impl ::windows::runtime::RuntimeName for ResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifier";
}
impl ::core::convert::From<ResourceQualifier> for ::windows::runtime::IUnknown {
    fn from(value: ResourceQualifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceQualifier> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceQualifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceQualifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceQualifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceQualifier> for ::windows::runtime::IInspectable {
    fn from(value: ResourceQualifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceQualifier> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceQualifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceQualifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceQualifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceQualifier {}
unsafe impl ::core::marker::Sync for ResourceQualifier {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceQualifierMapView(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierMapView {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for ResourceQualifierMapView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView;pinterface({e480ce40-a338-4ada-adcf-272272e48cb9};string;string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for ResourceQualifierMapView {
    type Vtable = super::super::super::Foundation::Collections::IMapView_abi<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for ResourceQualifierMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierMapView> for ::windows::runtime::IUnknown {
    fn from(value: ResourceQualifierMapView) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierMapView> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceQualifierMapView) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceQualifierMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierMapView> for ::windows::runtime::IInspectable {
    fn from(value: ResourceQualifierMapView) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierMapView> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceQualifierMapView) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceQualifierMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierMapView> for super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    fn from(value: ResourceQualifierMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierMapView> for super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    fn from(value: &ResourceQualifierMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for &ResourceQualifierMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierMapView> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceQualifierMapView) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierMapView> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceQualifierMapView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for &ResourceQualifierMapView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierMapView {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ResourceQualifierMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ResourceQualifierMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceQualifierObservableMap(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierObservableMap {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation`, `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::MapChangedEventHandler<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(&self, vhnd: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation`, `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for ResourceQualifierObservableMap {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap;pinterface({65df2bf5-bf39-41b5-aebc-5a9d865e472b};string;string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for ResourceQualifierObservableMap {
    type Vtable = super::super::super::Foundation::Collections::IObservableMap_abi<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for ResourceQualifierObservableMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierObservableMap> for ::windows::runtime::IUnknown {
    fn from(value: ResourceQualifierObservableMap) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierObservableMap> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceQualifierObservableMap) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierObservableMap> for ::windows::runtime::IInspectable {
    fn from(value: ResourceQualifierObservableMap) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierObservableMap> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceQualifierObservableMap) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    fn from(value: ResourceQualifierObservableMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    fn from(value: &ResourceQualifierObservableMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceQualifierObservableMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceQualifierObservableMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceQualifierObservableMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceQualifierObservableMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierObservableMap {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ResourceQualifierObservableMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ResourceQualifierObservableMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `ApplicationModel_Resources_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ResourceQualifierPersistence(pub i32);
impl ResourceQualifierPersistence {
    pub const None: ResourceQualifierPersistence = ResourceQualifierPersistence(0i32);
    pub const LocalMachine: ResourceQualifierPersistence = ResourceQualifierPersistence(1i32);
}
impl ::core::convert::From<i32> for ResourceQualifierPersistence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ResourceQualifierPersistence {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ResourceQualifierPersistence {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceQualifierPersistence;i4)");
}
impl ::windows::runtime::DefaultType for ResourceQualifierPersistence {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceQualifierVectorView(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierVectorView {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<ResourceQualifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ResourceQualifier>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, ResourceQualifier>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ResourceQualifier as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Core`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<ResourceQualifier>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ResourceQualifier>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for ResourceQualifierVectorView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.ApplicationModel.Resources.Core.ResourceQualifier;{785da5b2-4afd-4376-a888-c5f9a6b7a05c})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for ResourceQualifierVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_abi<ResourceQualifier>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for ResourceQualifierVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierVectorView> for ::windows::runtime::IUnknown {
    fn from(value: ResourceQualifierVectorView) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierVectorView> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceQualifierVectorView) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierVectorView> for ::windows::runtime::IInspectable {
    fn from(value: ResourceQualifierVectorView) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierVectorView> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceQualifierVectorView) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierVectorView> for super::super::super::Foundation::Collections::IVectorView<ResourceQualifier> {
    fn from(value: ResourceQualifierVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierVectorView> for super::super::super::Foundation::Collections::IVectorView<ResourceQualifier> {
    fn from(value: &ResourceQualifierVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> for &ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierVectorView> for super::super::super::Foundation::Collections::IIterable<ResourceQualifier> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceQualifierVectorView) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierVectorView> for super::super::super::Foundation::Collections::IIterable<ResourceQualifier> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceQualifierVectorView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> for &ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierVectorView {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
