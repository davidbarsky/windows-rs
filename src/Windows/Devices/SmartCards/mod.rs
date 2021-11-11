#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CardAddedEventArgs(pub ::windows::runtime::IInspectable);
impl CardAddedEventArgs {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SmartCard(&self) -> ::windows::runtime::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCard>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CardAddedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.CardAddedEventArgs;{18bbef98-f18b-4dd3-b118-dfb2c8e23cc6})");
}
unsafe impl ::windows::runtime::Interface for CardAddedEventArgs {
    type Vtable = ICardAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18bbef98_f18b_4dd3_b118_dfb2c8e23cc6);
}
impl ::windows::runtime::RuntimeName for CardAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.CardAddedEventArgs";
}
impl ::core::convert::From<CardAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CardAddedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CardAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CardAddedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CardAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CardAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CardAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CardAddedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CardAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CardAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CardAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CardAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CardAddedEventArgs {}
unsafe impl ::core::marker::Sync for CardAddedEventArgs {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CardRemovedEventArgs(pub ::windows::runtime::IInspectable);
impl CardRemovedEventArgs {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SmartCard(&self) -> ::windows::runtime::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCard>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CardRemovedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.CardRemovedEventArgs;{15331aaf-22d7-4945-afc9-03b46f42a6cd})");
}
unsafe impl ::windows::runtime::Interface for CardRemovedEventArgs {
    type Vtable = ICardRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x15331aaf_22d7_4945_afc9_03b46f42a6cd);
}
impl ::windows::runtime::RuntimeName for CardRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.CardRemovedEventArgs";
}
impl ::core::convert::From<CardRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CardRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CardRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CardRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CardRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CardRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CardRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CardRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CardRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CardRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CardRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CardRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CardRemovedEventArgs {}
unsafe impl ::core::marker::Sync for CardRemovedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICardAddedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICardAddedEventArgs {
    type Vtable = ICardAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18bbef98_f18b_4dd3_b118_dfb2c8e23cc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICardAddedEventArgs_abi(
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
pub struct ICardRemovedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICardRemovedEventArgs {
    type Vtable = ICardRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x15331aaf_22d7_4945_afc9_03b46f42a6cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICardRemovedEventArgs_abi(
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
pub struct IKnownSmartCardAppletIds(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownSmartCardAppletIds {
    type Vtable = IKnownSmartCardAppletIds_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7b04d8d8_95b4_4c88_8cea_411e55511efc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSmartCardAppletIds_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCard(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCard {
    type Vtable = ISmartCard_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b718871_6434_43f4_b55a_6a29623870aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCard_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroup(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAppletIdGroup {
    type Vtable = ISmartCardAppletIdGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7db165e6_6264_56f4_5e03_c86385395eb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardEmulationCategory) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmartCardEmulationCategory) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardEmulationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmartCardEmulationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroup2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAppletIdGroup2 {
    type Vtable = ISmartCardAppletIdGroup2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6b0ef9dc_9956_4a62_8d4e_d37a68ebc3a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroup2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAppletIdGroupFactory {
    type Vtable = ISmartCardAppletIdGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9105eb4d_4a65_4e41_8061_cbe83f3695e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, appletids: ::windows::runtime::RawPtr, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupRegistration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAppletIdGroupRegistration {
    type Vtable = ISmartCardAppletIdGroupRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdf1208d1_31bb_5596_43b1_6d69a0257b3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardAppletIdGroupActivationPolicy) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, policy: SmartCardAppletIdGroupActivationPolicy, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, apdus: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupRegistration2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAppletIdGroupRegistration2 {
    type Vtable = ISmartCardAppletIdGroupRegistration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f5508d8_98a7_4f2e_91d9_6cfcceda407f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupRegistration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, props: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAppletIdGroupStatics {
    type Vtable = ISmartCardAppletIdGroupStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xab2899a9_e76c_45cf_bf1d_90eaa6205927);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAutomaticResponseApdu {
    type Vtable = ISmartCardAutomaticResponseApdu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x52152bab_c63e_4531_a857_d756d99b986a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAutomaticResponseApdu2 {
    type Vtable = ISmartCardAutomaticResponseApdu2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x44aebb14_559d_4531_4e51_89db6fa8a57a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAutomaticResponseApdu3 {
    type Vtable = ISmartCardAutomaticResponseApdu3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbf43da74_6576_4392_9367_fe3bc9e2d496);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu3_abi(
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
pub struct ISmartCardAutomaticResponseApduFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardAutomaticResponseApduFactory {
    type Vtable = ISmartCardAutomaticResponseApduFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe97ea2fa_d02c_4c55_b02a_8cff7fa9f05b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApduFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandapdu: ::windows::runtime::RawPtr, responseapdu: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardChallengeContext(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardChallengeContext {
    type Vtable = ISmartCardChallengeContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x192a5319_c9c4_4947_81cc_44794a61ef91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardChallengeContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, response: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, response: ::windows::runtime::RawPtr, formatcard: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, response: ::windows::runtime::RawPtr, formatcard: bool, newcardid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, response: ::windows::runtime::RawPtr, newadministrativekey: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardConnect(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardConnect {
    type Vtable = ISmartCardConnect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2fdf87e5_028d_491e_a058_3382c3986f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardConnect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardConnection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardConnection {
    type Vtable = ISmartCardConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7edb991a_a81a_47bc_a649_156be6b7f231);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGenerator(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramGenerator {
    type Vtable = ISmartCardCryptogramGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe39f587b_edd3_4e49_b594_0ff5e4d0c76f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storagekeyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Core")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, materialpackagename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, cryptogrammaterialpackage: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, materialname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, challenge: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, promptingbehavior: SmartCardUnlockPromptingBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, materialpackagename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGenerator2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramGenerator2 {
    type Vtable = ISmartCardCryptogramGenerator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7116aa34_5d6d_4b4a_96a3_efa47d2a7e25);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGenerator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: ::windows::runtime::RawPtr, cryptogramplacementsteps: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storagekeyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGeneratorStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramGeneratorStatics {
    type Vtable = ISmartCardCryptogramGeneratorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x09933910_cb9c_4015_967d_5234f3b02900);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGeneratorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGeneratorStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramGeneratorStatics2 {
    type Vtable = ISmartCardCryptogramGeneratorStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x09bdf5e5_b4bd_4e23_a588_74469204c128);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGeneratorStatics2_abi(
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
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2798e029_d687_4c92_86c6_399e9a0ecb09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e6a8a5c_9773_46c4_a32f_b1e543159e04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8c7ce857_a7e7_489d_b9d6_368061515012);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialCharacteristics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramMaterialCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialCharacteristics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfc9ac5cc_c1d7_4153_923b_a2d43c6c8d49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialCharacteristics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramMaterialType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramMaterialProtectionMethod) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramMaterialPackageCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialPackageCharacteristics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xffb58e1f_0692_4c47_93cf_34d91f9dcd00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramMaterialPackageFormat) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialPossessionProof(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramMaterialPossessionProof {
    type Vtable = ISmartCardCryptogramMaterialPossessionProof_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe5b9ab8c_a141_4135_9add_b0d2e3aa1fc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialPossessionProof_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramPlacementStep(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramPlacementStep {
    type Vtable = ISmartCardCryptogramPlacementStep_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x947b03eb_8342_4792_a2e5_925636378a53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramPlacementStep_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramAlgorithm) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmartCardCryptogramAlgorithm) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramPlacementOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmartCardCryptogramPlacementOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramStorageKeyCharacteristics {
    type Vtable = ISmartCardCryptogramStorageKeyCharacteristics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8552546e_4457_4825_b464_635471a39f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramStorageKeyAlgorithm) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramStorageKeyInfo {
    type Vtable = ISmartCardCryptogramStorageKeyInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x77b0f00d_b097_4f61_a26a_9561639c9c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Cryptography_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Core"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptographicKeyAttestationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyInfo2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardCryptogramStorageKeyInfo2 {
    type Vtable = ISmartCardCryptogramStorageKeyInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x000440f9_f7fd_417d_89e1_fbb0382adc4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyInfo2_abi(
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
pub struct ISmartCardEmulator(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulator {
    type Vtable = ISmartCardEmulator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdfb906b2_875e_47e5_8077_e8bff1b1c6fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardEmulatorEnablementPolicy) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulator2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulator2 {
    type Vtable = ISmartCardEmulator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfe3fc0b8_8529_411a_807b_48edc2a0ab44);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulatorApduReceivedEventArgs {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd55d1576_69d2_5333_5b5f_f8c0d6e9f09f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, responseapdu: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardAutomaticResponseStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulatorApduReceivedEventArgs2 {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8bf93df0_22e1_4238_8610_94ce4a965425);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, responseapdu: ::windows::runtime::RawPtr, nextstate: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgsWithCryptograms_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd550bac7_b7bf_4e29_9294_0c4ac3c941bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, responsetemplate: ::windows::runtime::RawPtr, cryptogramplacementsteps: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, responsetemplate: ::windows::runtime::RawPtr, cryptogramplacementsteps: ::windows::runtime::RawPtr, nextstate: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    type Vtable = ISmartCardEmulatorConnectionDeactivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2186d8d3_c5eb_5262_43df_62a0a1b55557);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardEmulatorConnectionDeactivatedReason) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulatorConnectionProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulatorConnectionProperties {
    type Vtable = ISmartCardEmulatorConnectionProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e2ca5ee_f969_507d_6cf9_34e2d18df311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorConnectionProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardEmulatorConnectionSource) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulatorStatics {
    type Vtable = ISmartCardEmulatorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7a9bfc4b_c4d3_494f_b8a2_6215d81e85b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulatorStatics2 {
    type Vtable = ISmartCardEmulatorStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x69ae9f8a_b775_488b_8436_6c1e28ed731f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appletidgroup: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, registration: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardEmulatorStatics3 {
    type Vtable = ISmartCardEmulatorStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x59ea142a_9f09_43f5_8565_cfa8148e4cb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics3_abi(
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
pub struct ISmartCardPinPolicy(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardPinPolicy {
    type Vtable = ISmartCardPinPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x183ce184_4db6_4841_ac9e_2ac1f39b7304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinPolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmartCardPinCharacterPolicyOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmartCardPinCharacterPolicyOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmartCardPinCharacterPolicyOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmartCardPinCharacterPolicyOption) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardPinResetDeferral(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardPinResetDeferral {
    type Vtable = ISmartCardPinResetDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18c94aac_7805_4004_85e4_bbefac8f6884);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinResetDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardPinResetRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardPinResetRequest {
    type Vtable = ISmartCardPinResetRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x12fe3c4d_5fb9_4e8e_9ff6_61f475124fef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinResetRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, response: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardProvisioning(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardProvisioning {
    type Vtable = ISmartCardProvisioning_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x19eeedbd_1fab_477c_b712_1a2c5af1fd6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioning_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardProvisioning2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardProvisioning2 {
    type Vtable = ISmartCardProvisioning2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x10fd28eb_3f79_4b66_9b7c_11c149b7d0bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioning2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardProvisioningStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardProvisioningStatics {
    type Vtable = ISmartCardProvisioningStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x13882848_0d13_4e70_9735_51daeca5254f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioningStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, card: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, friendlyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, administrativekey: ::windows::runtime::RawPtr, pinpolicy: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, friendlyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, administrativekey: ::windows::runtime::RawPtr, pinpolicy: ::windows::runtime::RawPtr, cardid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, card: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardProvisioningStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardProvisioningStatics2 {
    type Vtable = ISmartCardProvisioningStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3447c6a8_c9a0_4bd6_b50d_251f4e8d3a62);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioningStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, friendlyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, administrativekey: ::windows::runtime::RawPtr, pinpolicy: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, friendlyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, administrativekey: ::windows::runtime::RawPtr, pinpolicy: ::windows::runtime::RawPtr, cardid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardReader(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardReader {
    type Vtable = ISmartCardReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1074b4e0_54c2_4df0_817a_14c14378f06c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardReaderKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
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
pub struct ISmartCardReaderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardReaderStatics {
    type Vtable = ISmartCardReaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x103c04e1_a1ca_48f2_a281_5b6f669af107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardReaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: SmartCardReaderKind, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardTriggerDetails {
    type Vtable = ISmartCardTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f9bf11e_39ef_4f2b_b44f_0a9155b177bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmartCardTriggerType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardTriggerDetails2 {
    type Vtable = ISmartCardTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2945c569_8975_4a51_9e1a_5f8a76ee51af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, arguments: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, arguments: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, behavior: SmartCardLaunchBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmartCardTriggerDetails3 {
    type Vtable = ISmartCardTriggerDetails3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb3e2c27d_18c6_4ba8_8376_ef03d4912666);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_SmartCards`*"]
pub struct KnownSmartCardAppletIds {}
impl KnownSmartCardAppletIds {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn PaymentSystemEnvironment() -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        Self::IKnownSmartCardAppletIds(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn ProximityPaymentSystemEnvironment() -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        Self::IKnownSmartCardAppletIds(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    pub fn IKnownSmartCardAppletIds<R, F: FnOnce(&IKnownSmartCardAppletIds) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownSmartCardAppletIds, IKnownSmartCardAppletIds> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownSmartCardAppletIds {
    const NAME: &'static str = "Windows.Devices.SmartCards.KnownSmartCardAppletIds";
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCard(pub ::windows::runtime::IInspectable);
impl SmartCard {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Reader(&self) -> ::windows::runtime::Result<SmartCardReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetStatusAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn GetAnswerToResetAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn ConnectAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardConnection>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardConnect>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardConnection>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCard {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCard;{1b718871-6434-43f4-b55a-6a29623870aa})");
}
unsafe impl ::windows::runtime::Interface for SmartCard {
    type Vtable = ISmartCard_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b718871_6434_43f4_b55a_6a29623870aa);
}
impl ::windows::runtime::RuntimeName for SmartCard {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCard";
}
impl ::core::convert::From<SmartCard> for ::windows::runtime::IUnknown {
    fn from(value: SmartCard) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCard> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCard) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCard> for ::windows::runtime::IInspectable {
    fn from(value: SmartCard) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCard> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCard) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCard {}
unsafe impl ::core::marker::Sync for SmartCard {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardActivationPolicyChangeResult(pub i32);
impl SmartCardActivationPolicyChangeResult {
    pub const Denied: SmartCardActivationPolicyChangeResult = SmartCardActivationPolicyChangeResult(0i32);
    pub const Allowed: SmartCardActivationPolicyChangeResult = SmartCardActivationPolicyChangeResult(1i32);
}
impl ::core::convert::From<i32> for SmartCardActivationPolicyChangeResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardActivationPolicyChangeResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardActivationPolicyChangeResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardActivationPolicyChangeResult;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardActivationPolicyChangeResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardAppletIdGroup(pub ::windows::runtime::IInspectable);
impl SmartCardAppletIdGroup {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardAppletIdGroup, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn AppletIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SmartCardEmulationCategory(&self) -> ::windows::runtime::Result<SmartCardEmulationCategory> {
        let this = self;
        unsafe {
            let mut result__: SmartCardEmulationCategory = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardEmulationCategory>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetSmartCardEmulationCategory(&self, value: SmartCardEmulationCategory) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SmartCardEmulationType(&self) -> ::windows::runtime::Result<SmartCardEmulationType> {
        let this = self;
        unsafe {
            let mut result__: SmartCardEmulationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardEmulationType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetSmartCardEmulationType(&self, value: SmartCardEmulationType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn AutomaticEnablement(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetAutomaticEnablement(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>>(displayname: Param0, appletids: Param1, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType) -> ::windows::runtime::Result<SmartCardAppletIdGroup> {
        Self::ISmartCardAppletIdGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), displayname.into_param().abi(), appletids.into_param().abi(), emulationcategory, emulationtype, &mut result__).from_abi::<SmartCardAppletIdGroup>(result__)
        })
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn MaxAppletIds() -> ::windows::runtime::Result<u16> {
        Self::ISmartCardAppletIdGroupStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn Logo(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SetLogo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SecureUserAuthenticationRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetSecureUserAuthenticationRequired(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ISmartCardAppletIdGroupFactory<R, F: FnOnce(&ISmartCardAppletIdGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardAppletIdGroup, ISmartCardAppletIdGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardAppletIdGroupStatics<R, F: FnOnce(&ISmartCardAppletIdGroupStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardAppletIdGroup, ISmartCardAppletIdGroupStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardAppletIdGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAppletIdGroup;{7db165e6-6264-56f4-5e03-c86385395eb1})");
}
unsafe impl ::windows::runtime::Interface for SmartCardAppletIdGroup {
    type Vtable = ISmartCardAppletIdGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7db165e6_6264_56f4_5e03_c86385395eb1);
}
impl ::windows::runtime::RuntimeName for SmartCardAppletIdGroup {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAppletIdGroup";
}
impl ::core::convert::From<SmartCardAppletIdGroup> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardAppletIdGroup) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardAppletIdGroup> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardAppletIdGroup) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardAppletIdGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardAppletIdGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardAppletIdGroup> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardAppletIdGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardAppletIdGroup> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardAppletIdGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardAppletIdGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardAppletIdGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardAppletIdGroup {}
unsafe impl ::core::marker::Sync for SmartCardAppletIdGroup {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardAppletIdGroupActivationPolicy(pub i32);
impl SmartCardAppletIdGroupActivationPolicy {
    pub const Disabled: SmartCardAppletIdGroupActivationPolicy = SmartCardAppletIdGroupActivationPolicy(0i32);
    pub const ForegroundOverride: SmartCardAppletIdGroupActivationPolicy = SmartCardAppletIdGroupActivationPolicy(1i32);
    pub const Enabled: SmartCardAppletIdGroupActivationPolicy = SmartCardAppletIdGroupActivationPolicy(2i32);
}
impl ::core::convert::From<i32> for SmartCardAppletIdGroupActivationPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardAppletIdGroupActivationPolicy {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardAppletIdGroupActivationPolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardAppletIdGroupActivationPolicy;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardAppletIdGroupActivationPolicy {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardAppletIdGroupRegistration(pub ::windows::runtime::IInspectable);
impl SmartCardAppletIdGroupRegistration {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn ActivationPolicy(&self) -> ::windows::runtime::Result<SmartCardAppletIdGroupActivationPolicy> {
        let this = self;
        unsafe {
            let mut result__: SmartCardAppletIdGroupActivationPolicy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardAppletIdGroupActivationPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn AppletIdGroup(&self) -> ::windows::runtime::Result<SmartCardAppletIdGroup> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardAppletIdGroup>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RequestActivationPolicyChangeAsync(&self, policy: SmartCardAppletIdGroupActivationPolicy) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardActivationPolicyChangeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), policy, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardActivationPolicyChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetAutomaticResponseApdusAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<SmartCardAutomaticResponseApdu>>>(&self, apdus: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), apdus.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SmartCardReaderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroupRegistration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetPropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, props: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAppletIdGroupRegistration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), props.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardAppletIdGroupRegistration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAppletIdGroupRegistration;{df1208d1-31bb-5596-43b1-6d69a0257b3a})");
}
unsafe impl ::windows::runtime::Interface for SmartCardAppletIdGroupRegistration {
    type Vtable = ISmartCardAppletIdGroupRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdf1208d1_31bb_5596_43b1_6d69a0257b3a);
}
impl ::windows::runtime::RuntimeName for SmartCardAppletIdGroupRegistration {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAppletIdGroupRegistration";
}
impl ::core::convert::From<SmartCardAppletIdGroupRegistration> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardAppletIdGroupRegistration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardAppletIdGroupRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardAppletIdGroupRegistration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardAppletIdGroupRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardAppletIdGroupRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardAppletIdGroupRegistration> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardAppletIdGroupRegistration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardAppletIdGroupRegistration> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardAppletIdGroupRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardAppletIdGroupRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardAppletIdGroupRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardAppletIdGroupRegistration {}
unsafe impl ::core::marker::Sync for SmartCardAppletIdGroupRegistration {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardAutomaticResponseApdu(pub ::windows::runtime::IInspectable);
impl SmartCardAutomaticResponseApdu {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn CommandApdu(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SetCommandApdu<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn CommandApduBitMask(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SetCommandApduBitMask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn ShouldMatchLength(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetShouldMatchLength(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn AppletId(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SetAppletId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn ResponseApdu(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SetResponseApdu<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(commandapdu: Param0, responseapdu: Param1) -> ::windows::runtime::Result<SmartCardAutomaticResponseApdu> {
        Self::ISmartCardAutomaticResponseApduFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), commandapdu.into_param().abi(), responseapdu.into_param().abi(), &mut result__).from_abi::<SmartCardAutomaticResponseApdu>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn InputState(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn SetInputState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn OutputState(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn SetOutputState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn AllowWhenCryptogramGeneratorNotPrepared(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAutomaticResponseApdu3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetAllowWhenCryptogramGeneratorNotPrepared(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardAutomaticResponseApdu3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ISmartCardAutomaticResponseApduFactory<R, F: FnOnce(&ISmartCardAutomaticResponseApduFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardAutomaticResponseApdu, ISmartCardAutomaticResponseApduFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardAutomaticResponseApdu {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAutomaticResponseApdu;{52152bab-c63e-4531-a857-d756d99b986a})");
}
unsafe impl ::windows::runtime::Interface for SmartCardAutomaticResponseApdu {
    type Vtable = ISmartCardAutomaticResponseApdu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x52152bab_c63e_4531_a857_d756d99b986a);
}
impl ::windows::runtime::RuntimeName for SmartCardAutomaticResponseApdu {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAutomaticResponseApdu";
}
impl ::core::convert::From<SmartCardAutomaticResponseApdu> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardAutomaticResponseApdu) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardAutomaticResponseApdu> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardAutomaticResponseApdu) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardAutomaticResponseApdu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardAutomaticResponseApdu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardAutomaticResponseApdu> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardAutomaticResponseApdu) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardAutomaticResponseApdu> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardAutomaticResponseApdu) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardAutomaticResponseApdu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardAutomaticResponseApdu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardAutomaticResponseApdu {}
unsafe impl ::core::marker::Sync for SmartCardAutomaticResponseApdu {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardAutomaticResponseStatus(pub i32);
impl SmartCardAutomaticResponseStatus {
    pub const None: SmartCardAutomaticResponseStatus = SmartCardAutomaticResponseStatus(0i32);
    pub const Success: SmartCardAutomaticResponseStatus = SmartCardAutomaticResponseStatus(1i32);
    pub const UnknownError: SmartCardAutomaticResponseStatus = SmartCardAutomaticResponseStatus(2i32);
}
impl ::core::convert::From<i32> for SmartCardAutomaticResponseStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardAutomaticResponseStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardAutomaticResponseStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardAutomaticResponseStatus;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardAutomaticResponseStatus {
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct SmartCardBackgroundTriggerContract(pub u8);
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardChallengeContext(pub ::windows::runtime::IInspectable);
impl SmartCardChallengeContext {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn Challenge(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn VerifyResponseAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, response: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn ProvisionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, response: Param0, formatcard: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), response.into_param().abi(), formatcard, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn ProvisionAsyncWithNewCardId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, response: Param0, formatcard: bool, newcardid: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), response.into_param().abi(), formatcard, newcardid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn ChangeAdministrativeKeyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, response: Param0, newadministrativekey: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), response.into_param().abi(), newadministrativekey.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardChallengeContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardChallengeContext;{192a5319-c9c4-4947-81cc-44794a61ef91})");
}
unsafe impl ::windows::runtime::Interface for SmartCardChallengeContext {
    type Vtable = ISmartCardChallengeContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x192a5319_c9c4_4947_81cc_44794a61ef91);
}
impl ::windows::runtime::RuntimeName for SmartCardChallengeContext {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardChallengeContext";
}
impl ::core::convert::From<SmartCardChallengeContext> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardChallengeContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardChallengeContext> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardChallengeContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardChallengeContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardChallengeContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardChallengeContext> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardChallengeContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardChallengeContext> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardChallengeContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardChallengeContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardChallengeContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SmartCardChallengeContext> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmartCardChallengeContext) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SmartCardChallengeContext> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmartCardChallengeContext) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for SmartCardChallengeContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &SmartCardChallengeContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmartCardChallengeContext {}
unsafe impl ::core::marker::Sync for SmartCardChallengeContext {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardConnection(pub ::windows::runtime::IInspectable);
impl SmartCardConnection {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn TransmitAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, command: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), command.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardConnection;{7edb991a-a81a-47bc-a649-156be6b7f231})");
}
unsafe impl ::windows::runtime::Interface for SmartCardConnection {
    type Vtable = ISmartCardConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7edb991a_a81a_47bc_a649_156be6b7f231);
}
impl ::windows::runtime::RuntimeName for SmartCardConnection {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardConnection";
}
impl ::core::convert::From<SmartCardConnection> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardConnection> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardConnection> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardConnection> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SmartCardConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmartCardConnection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SmartCardConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmartCardConnection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for SmartCardConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &SmartCardConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmartCardConnection {}
unsafe impl ::core::marker::Sync for SmartCardConnection {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramAlgorithm(pub i32);
impl SmartCardCryptogramAlgorithm {
    pub const None: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(0i32);
    pub const CbcMac: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(1i32);
    pub const Cvc3Umd: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(2i32);
    pub const DecimalizedMsd: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(3i32);
    pub const Cvc3MD: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(4i32);
    pub const Sha1: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(5i32);
    pub const SignedDynamicApplicationData: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(6i32);
    pub const RsaPkcs1: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(7i32);
    pub const Sha256Hmac: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(8i32);
}
impl ::core::convert::From<i32> for SmartCardCryptogramAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramAlgorithm {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramAlgorithm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramAlgorithm;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramAlgorithm {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramGenerator(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramGenerator {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn SupportedCryptogramMaterialTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialType>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn SupportedCryptogramAlgorithms(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn SupportedCryptogramMaterialPackageFormats(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn SupportedCryptogramMaterialPackageConfirmationResponseFormats(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn SupportedSmartCardCryptogramStorageKeyCapabilities(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCapabilities>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCapabilities>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn DeleteCryptogramMaterialStorageKeyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, storagekeyname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), storagekeyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn CreateCryptogramMaterialStorageKeyAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: Param1, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), promptingbehavior, storagekeyname.into_param().abi(), algorithm, capabilities, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Security_Cryptography_Core`*"]
    pub fn RequestCryptogramMaterialStorageKeyInfoAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: Param1, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramStorageKeyInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), promptingbehavior, storagekeyname.into_param().abi(), format, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramStorageKeyInfo>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn ImportCryptogramMaterialPackageAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(
        &self,
        format: SmartCardCryptogramMaterialPackageFormat,
        storagekeyname: Param1,
        materialpackagename: Param2,
        cryptogrammaterialpackage: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), format, storagekeyname.into_param().abi(), materialpackagename.into_param().abi(), cryptogrammaterialpackage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn TryProvePossessionOfCryptogramMaterialPackageAsync<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(
        &self,
        promptingbehavior: SmartCardUnlockPromptingBehavior,
        responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat,
        materialpackagename: Param2,
        materialname: Param3,
        challenge: Param4,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramMaterialPossessionProof>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), promptingbehavior, responseformat, materialpackagename.into_param().abi(), materialname.into_param().abi(), challenge.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramMaterialPossessionProof>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RequestUnlockCryptogramMaterialForUseAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), promptingbehavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn DeleteCryptogramMaterialPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, materialpackagename: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), materialpackagename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetSmartCardCryptogramGeneratorAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGenerator>> {
        Self::ISmartCardCryptogramGeneratorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGenerator>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn ValidateRequestApduAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: Param1, cryptogramplacementsteps: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), promptingbehavior, apdutovalidate.into_param().abi(), cryptogramplacementsteps.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetAllCryptogramStorageKeyCharacteristicsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetAllCryptogramMaterialPackageCharacteristicsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, storagekeyname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), storagekeyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetAllCryptogramMaterialCharacteristicsAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), promptingbehavior, materialpackagename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::ISmartCardCryptogramGeneratorStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ISmartCardCryptogramGeneratorStatics<R, F: FnOnce(&ISmartCardCryptogramGeneratorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramGenerator, ISmartCardCryptogramGeneratorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardCryptogramGeneratorStatics2<R, F: FnOnce(&ISmartCardCryptogramGeneratorStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramGenerator, ISmartCardCryptogramGeneratorStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramGenerator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGenerator;{e39f587b-edd3-4e49-b594-0ff5e4d0c76f})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramGenerator {
    type Vtable = ISmartCardCryptogramGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe39f587b_edd3_4e49_b594_0ff5e4d0c76f);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramGenerator {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGenerator";
}
impl ::core::convert::From<SmartCardCryptogramGenerator> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramGenerator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramGenerator> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramGenerator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramGenerator> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramGenerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramGenerator> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramGenerator {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGenerator {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramGeneratorOperationStatus(pub i32);
impl SmartCardCryptogramGeneratorOperationStatus {
    pub const Success: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(0i32);
    pub const AuthorizationFailed: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(1i32);
    pub const AuthorizationCanceled: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(2i32);
    pub const AuthorizationRequired: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(3i32);
    pub const CryptogramMaterialPackageStorageKeyExists: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(4i32);
    pub const NoCryptogramMaterialPackageStorageKey: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(5i32);
    pub const NoCryptogramMaterialPackage: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(6i32);
    pub const UnsupportedCryptogramMaterialPackage: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(7i32);
    pub const UnknownCryptogramMaterialName: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(8i32);
    pub const InvalidCryptogramMaterialUsage: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(9i32);
    pub const ApduResponseNotSent: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(10i32);
    pub const OtherError: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(11i32);
    pub const ValidationFailed: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(12i32);
    pub const NotSupported: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(13i32);
}
impl ::core::convert::From<i32> for SmartCardCryptogramGeneratorOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramGeneratorOperationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramGeneratorOperationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramGeneratorOperationStatus;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramGeneratorOperationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn OperationStatus(&self) -> ::windows::runtime::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramGeneratorOperationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn Characteristics(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialCharacteristics>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult;{2798e029-d687-4c92-86c6-399e9a0ecb09})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2798e029_d687_4c92_86c6_399e9a0ecb09);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult";
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn OperationStatus(&self) -> ::windows::runtime::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramGeneratorOperationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn Characteristics(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageCharacteristics>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult;{4e6a8a5c-9773-46c4-a32f-b1e543159e04})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e6a8a5c_9773_46c4_a32f_b1e543159e04);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult";
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn OperationStatus(&self) -> ::windows::runtime::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramGeneratorOperationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn Characteristics(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCharacteristics>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult;{8c7ce857-a7e7-489d-b9d6-368061515012})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8c7ce857_a7e7_489d_b9d6_368061515012);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult";
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramMaterialCharacteristics(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramMaterialCharacteristics {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramMaterialCharacteristics, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn MaterialName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn AllowedAlgorithms(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn AllowedProofOfPossessionAlgorithms(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation_Collections`*"]
    pub fn AllowedValidations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn MaterialType(&self) -> ::windows::runtime::Result<SmartCardCryptogramMaterialType> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramMaterialType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramMaterialType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn ProtectionMethod(&self) -> ::windows::runtime::Result<SmartCardCryptogramMaterialProtectionMethod> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramMaterialProtectionMethod = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramMaterialProtectionMethod>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn ProtectionVersion(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn MaterialLength(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramMaterialCharacteristics {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialCharacteristics;{fc9ac5cc-c1d7-4153-923b-a2d43c6c8d49})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramMaterialCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialCharacteristics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfc9ac5cc_c1d7_4153_923b_a2d43c6c8d49);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramMaterialCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialCharacteristics";
}
impl ::core::convert::From<SmartCardCryptogramMaterialCharacteristics> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramMaterialCharacteristics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialCharacteristics> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramMaterialCharacteristics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramMaterialCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramMaterialCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramMaterialCharacteristics> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramMaterialCharacteristics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialCharacteristics> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramMaterialCharacteristics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramMaterialCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramMaterialCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialCharacteristics {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramMaterialPackageCharacteristics(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramMaterialPackageCharacteristics {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramMaterialPackageCharacteristics, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn PackageName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn StorageKeyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn DateImported(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn PackageFormat(&self) -> ::windows::runtime::Result<SmartCardCryptogramMaterialPackageFormat> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramMaterialPackageFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramMaterialPackageFormat>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramMaterialPackageCharacteristics {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageCharacteristics;{ffb58e1f-0692-4c47-93cf-34d91f9dcd00})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramMaterialPackageCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialPackageCharacteristics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xffb58e1f_0692_4c47_93cf_34d91f9dcd00);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramMaterialPackageCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageCharacteristics";
}
impl ::core::convert::From<SmartCardCryptogramMaterialPackageCharacteristics> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramMaterialPackageCharacteristics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialPackageCharacteristics> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramMaterialPackageCharacteristics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramMaterialPackageCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramMaterialPackageCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramMaterialPackageCharacteristics> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramMaterialPackageCharacteristics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialPackageCharacteristics> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramMaterialPackageCharacteristics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramMaterialPackageCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramMaterialPackageCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialPackageCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialPackageCharacteristics {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageConfirmationResponseFormat(pub i32);
impl SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    pub const None: SmartCardCryptogramMaterialPackageConfirmationResponseFormat = SmartCardCryptogramMaterialPackageConfirmationResponseFormat(0i32);
    pub const VisaHmac: SmartCardCryptogramMaterialPackageConfirmationResponseFormat = SmartCardCryptogramMaterialPackageConfirmationResponseFormat(1i32);
}
impl ::core::convert::From<i32> for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageConfirmationResponseFormat;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageFormat(pub i32);
impl SmartCardCryptogramMaterialPackageFormat {
    pub const None: SmartCardCryptogramMaterialPackageFormat = SmartCardCryptogramMaterialPackageFormat(0i32);
    pub const JweRsaPki: SmartCardCryptogramMaterialPackageFormat = SmartCardCryptogramMaterialPackageFormat(1i32);
}
impl ::core::convert::From<i32> for SmartCardCryptogramMaterialPackageFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramMaterialPackageFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramMaterialPackageFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageFormat;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramMaterialPackageFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramMaterialPossessionProof(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramMaterialPossessionProof {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn OperationStatus(&self) -> ::windows::runtime::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramGeneratorOperationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn Proof(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramMaterialPossessionProof {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPossessionProof;{e5b9ab8c-a141-4135-9add-b0d2e3aa1fc9})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramMaterialPossessionProof {
    type Vtable = ISmartCardCryptogramMaterialPossessionProof_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe5b9ab8c_a141_4135_9add_b0d2e3aa1fc9);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramMaterialPossessionProof {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialPossessionProof";
}
impl ::core::convert::From<SmartCardCryptogramMaterialPossessionProof> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramMaterialPossessionProof) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialPossessionProof> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramMaterialPossessionProof) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramMaterialPossessionProof {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramMaterialPossessionProof {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramMaterialPossessionProof> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramMaterialPossessionProof) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialPossessionProof> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramMaterialPossessionProof) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramMaterialPossessionProof {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramMaterialPossessionProof {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialPossessionProof {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialPossessionProof {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialProtectionMethod(pub i32);
impl SmartCardCryptogramMaterialProtectionMethod {
    pub const None: SmartCardCryptogramMaterialProtectionMethod = SmartCardCryptogramMaterialProtectionMethod(0i32);
    pub const WhiteBoxing: SmartCardCryptogramMaterialProtectionMethod = SmartCardCryptogramMaterialProtectionMethod(1i32);
}
impl ::core::convert::From<i32> for SmartCardCryptogramMaterialProtectionMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramMaterialProtectionMethod {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramMaterialProtectionMethod {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialProtectionMethod;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramMaterialProtectionMethod {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialType(pub i32);
impl SmartCardCryptogramMaterialType {
    pub const None: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(0i32);
    pub const StaticDataAuthentication: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(1i32);
    pub const TripleDes112: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(2i32);
    pub const Aes: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(3i32);
    pub const RsaPkcs1: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(4i32);
}
impl ::core::convert::From<i32> for SmartCardCryptogramMaterialType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramMaterialType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramMaterialType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialType;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramMaterialType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementOptions(pub u32);
impl SmartCardCryptogramPlacementOptions {
    pub const None: SmartCardCryptogramPlacementOptions = SmartCardCryptogramPlacementOptions(0u32);
    pub const UnitsAreInNibbles: SmartCardCryptogramPlacementOptions = SmartCardCryptogramPlacementOptions(1u32);
    pub const ChainOutput: SmartCardCryptogramPlacementOptions = SmartCardCryptogramPlacementOptions(2u32);
}
impl ::core::convert::From<u32> for SmartCardCryptogramPlacementOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramPlacementOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramPlacementOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramPlacementOptions;u4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramPlacementOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SmartCardCryptogramPlacementOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SmartCardCryptogramPlacementOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramPlacementStep(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramPlacementStep {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramPlacementStep, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Algorithm(&self) -> ::windows::runtime::Result<SmartCardCryptogramAlgorithm> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramAlgorithm = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetAlgorithm(&self, value: SmartCardCryptogramAlgorithm) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SourceData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SetSourceData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn CryptogramMaterialPackageName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetCryptogramMaterialPackageName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn CryptogramMaterialName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetCryptogramMaterialName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn TemplateOffset(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetTemplateOffset(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn CryptogramOffset(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetCryptogramOffset(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn CryptogramLength(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetCryptogramLength(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn CryptogramPlacementOptions(&self) -> ::windows::runtime::Result<SmartCardCryptogramPlacementOptions> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramPlacementOptions = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramPlacementOptions>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetCryptogramPlacementOptions(&self, value: SmartCardCryptogramPlacementOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn ChainedOutputStep(&self) -> ::windows::runtime::Result<SmartCardCryptogramPlacementStep> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramPlacementStep>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetChainedOutputStep<'a, Param0: ::windows::runtime::IntoParam<'a, SmartCardCryptogramPlacementStep>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramPlacementStep {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramPlacementStep;{947b03eb-8342-4792-a2e5-925636378a53})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramPlacementStep {
    type Vtable = ISmartCardCryptogramPlacementStep_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x947b03eb_8342_4792_a2e5_925636378a53);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramPlacementStep {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramPlacementStep";
}
impl ::core::convert::From<SmartCardCryptogramPlacementStep> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramPlacementStep) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramPlacementStep> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramPlacementStep) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramPlacementStep {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramPlacementStep {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramPlacementStep> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramPlacementStep) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramPlacementStep> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramPlacementStep) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramPlacementStep {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramPlacementStep {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramPlacementStep {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramPlacementStep {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyAlgorithm(pub i32);
impl SmartCardCryptogramStorageKeyAlgorithm {
    pub const None: SmartCardCryptogramStorageKeyAlgorithm = SmartCardCryptogramStorageKeyAlgorithm(0i32);
    pub const Rsa2048: SmartCardCryptogramStorageKeyAlgorithm = SmartCardCryptogramStorageKeyAlgorithm(1i32);
}
impl ::core::convert::From<i32> for SmartCardCryptogramStorageKeyAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramStorageKeyAlgorithm {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramStorageKeyAlgorithm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyAlgorithm;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramStorageKeyAlgorithm {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCapabilities(pub u32);
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const None: SmartCardCryptogramStorageKeyCapabilities = SmartCardCryptogramStorageKeyCapabilities(0u32);
    pub const HardwareProtection: SmartCardCryptogramStorageKeyCapabilities = SmartCardCryptogramStorageKeyCapabilities(1u32);
    pub const UnlockPrompt: SmartCardCryptogramStorageKeyCapabilities = SmartCardCryptogramStorageKeyCapabilities(2u32);
}
impl ::core::convert::From<u32> for SmartCardCryptogramStorageKeyCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptogramStorageKeyCapabilities {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramStorageKeyCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCapabilities;u4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptogramStorageKeyCapabilities {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SmartCardCryptogramStorageKeyCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SmartCardCryptogramStorageKeyCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramStorageKeyCharacteristics(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramStorageKeyCharacteristics {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardCryptogramStorageKeyCharacteristics, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn StorageKeyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Algorithm(&self) -> ::windows::runtime::Result<SmartCardCryptogramStorageKeyAlgorithm> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramStorageKeyAlgorithm = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramStorageKeyAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<SmartCardCryptogramStorageKeyCapabilities> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramStorageKeyCapabilities = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramStorageKeyCapabilities>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramStorageKeyCharacteristics {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCharacteristics;{8552546e-4457-4825-b464-635471a39f5c})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramStorageKeyCharacteristics {
    type Vtable = ISmartCardCryptogramStorageKeyCharacteristics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8552546e_4457_4825_b464_635471a39f5c);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramStorageKeyCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCharacteristics";
}
impl ::core::convert::From<SmartCardCryptogramStorageKeyCharacteristics> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramStorageKeyCharacteristics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramStorageKeyCharacteristics> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramStorageKeyCharacteristics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramStorageKeyCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramStorageKeyCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramStorageKeyCharacteristics> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramStorageKeyCharacteristics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramStorageKeyCharacteristics> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramStorageKeyCharacteristics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramStorageKeyCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramStorageKeyCharacteristics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramStorageKeyCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramStorageKeyCharacteristics {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardCryptogramStorageKeyInfo(pub ::windows::runtime::IInspectable);
impl SmartCardCryptogramStorageKeyInfo {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn OperationStatus(&self) -> ::windows::runtime::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramGeneratorOperationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Security_Cryptography_Core")]
    #[doc = "*Required features: `Devices_SmartCards`, `Security_Cryptography_Core`*"]
    pub fn PublicKeyBlobType(&self) -> ::windows::runtime::Result<super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn PublicKey(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn AttestationStatus(&self) -> ::windows::runtime::Result<SmartCardCryptographicKeyAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptographicKeyAttestationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptographicKeyAttestationStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn Attestation(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn AttestationCertificateChain(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<SmartCardCryptogramStorageKeyCapabilities> {
        let this = self;
        unsafe {
            let mut result__: SmartCardCryptogramStorageKeyCapabilities = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardCryptogramStorageKeyCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn OperationalRequirements(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardCryptogramStorageKeyInfo2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptogramStorageKeyInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyInfo;{77b0f00d-b097-4f61-a26a-9561639c9c3a})");
}
unsafe impl ::windows::runtime::Interface for SmartCardCryptogramStorageKeyInfo {
    type Vtable = ISmartCardCryptogramStorageKeyInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x77b0f00d_b097_4f61_a26a_9561639c9c3a);
}
impl ::windows::runtime::RuntimeName for SmartCardCryptogramStorageKeyInfo {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyInfo";
}
impl ::core::convert::From<SmartCardCryptogramStorageKeyInfo> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardCryptogramStorageKeyInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardCryptogramStorageKeyInfo> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardCryptogramStorageKeyInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardCryptogramStorageKeyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardCryptogramStorageKeyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardCryptogramStorageKeyInfo> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardCryptogramStorageKeyInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardCryptogramStorageKeyInfo> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardCryptogramStorageKeyInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardCryptogramStorageKeyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardCryptogramStorageKeyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramStorageKeyInfo {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramStorageKeyInfo {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardCryptographicKeyAttestationStatus(pub i32);
impl SmartCardCryptographicKeyAttestationStatus {
    pub const NoAttestation: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(0i32);
    pub const SoftwareKeyWithoutTpm: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(1i32);
    pub const SoftwareKeyWithTpm: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(2i32);
    pub const TpmKeyUnknownAttestationStatus: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(3i32);
    pub const TpmKeyWithoutAttestationCapability: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(4i32);
    pub const TpmKeyWithTemporaryAttestationFailure: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(5i32);
    pub const TpmKeyWithLongTermAttestationFailure: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(6i32);
    pub const TpmKeyWithAttestation: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(7i32);
}
impl ::core::convert::From<i32> for SmartCardCryptographicKeyAttestationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardCryptographicKeyAttestationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardCryptographicKeyAttestationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptographicKeyAttestationStatus;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardCryptographicKeyAttestationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardEmulationCategory(pub i32);
impl SmartCardEmulationCategory {
    pub const Other: SmartCardEmulationCategory = SmartCardEmulationCategory(0i32);
    pub const Payment: SmartCardEmulationCategory = SmartCardEmulationCategory(1i32);
}
impl ::core::convert::From<i32> for SmartCardEmulationCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardEmulationCategory {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulationCategory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulationCategory;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardEmulationCategory {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardEmulationType(pub i32);
impl SmartCardEmulationType {
    pub const Host: SmartCardEmulationType = SmartCardEmulationType(0i32);
    pub const Uicc: SmartCardEmulationType = SmartCardEmulationType(1i32);
    pub const EmbeddedSE: SmartCardEmulationType = SmartCardEmulationType(2i32);
}
impl ::core::convert::From<i32> for SmartCardEmulationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardEmulationType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulationType;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardEmulationType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardEmulator(pub ::windows::runtime::IInspectable);
impl SmartCardEmulator {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn EnablementPolicy(&self) -> ::windows::runtime::Result<SmartCardEmulatorEnablementPolicy> {
        let this = self;
        unsafe {
            let mut result__: SmartCardEmulatorEnablementPolicy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardEmulatorEnablementPolicy>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn ApduReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorApduReceivedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RemoveApduReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn ConnectionDeactivated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorConnectionDeactivatedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RemoveConnectionDeactivated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn IsHostCardEmulationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardEmulator>> {
        Self::ISmartCardEmulatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardEmulator>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetAppletIdGroupRegistrationsAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCardAppletIdGroupRegistration>>> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCardAppletIdGroupRegistration>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RegisterAppletIdGroupAsync<'a, Param0: ::windows::runtime::IntoParam<'a, SmartCardAppletIdGroup>>(appletidgroup: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardAppletIdGroupRegistration>> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appletidgroup.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardAppletIdGroupRegistration>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn UnregisterAppletIdGroupAsync<'a, Param0: ::windows::runtime::IntoParam<'a, SmartCardAppletIdGroupRegistration>>(registration: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), registration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn MaxAppletIdGroupRegistrations() -> ::windows::runtime::Result<u16> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::ISmartCardEmulatorStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ISmartCardEmulatorStatics<R, F: FnOnce(&ISmartCardEmulatorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardEmulatorStatics2<R, F: FnOnce(&ISmartCardEmulatorStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardEmulatorStatics3<R, F: FnOnce(&ISmartCardEmulatorStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulator;{dfb906b2-875e-47e5-8077-e8bff1b1c6fb})");
}
unsafe impl ::windows::runtime::Interface for SmartCardEmulator {
    type Vtable = ISmartCardEmulator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdfb906b2_875e_47e5_8077_e8bff1b1c6fb);
}
impl ::windows::runtime::RuntimeName for SmartCardEmulator {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulator";
}
impl ::core::convert::From<SmartCardEmulator> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardEmulator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardEmulator> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardEmulator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardEmulator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardEmulator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardEmulator> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardEmulator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardEmulator> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardEmulator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardEmulator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardEmulator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardEmulator {}
unsafe impl ::core::marker::Sync for SmartCardEmulator {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardEmulatorApduReceivedEventArgs(pub ::windows::runtime::IInspectable);
impl SmartCardEmulatorApduReceivedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn CommandApdu(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn ConnectionProperties(&self) -> ::windows::runtime::Result<SmartCardEmulatorConnectionProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardEmulatorConnectionProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn TryRespondAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, responseapdu: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), responseapdu.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn AutomaticResponseStatus(&self) -> ::windows::runtime::Result<SmartCardAutomaticResponseStatus> {
        let this = self;
        unsafe {
            let mut result__: SmartCardAutomaticResponseStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardAutomaticResponseStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn State(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn TryRespondWithStateAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, responseapdu: Param0, nextstate: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), responseapdu.into_param().abi(), nextstate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn TryRespondWithCryptogramsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>>(&self, responsetemplate: Param0, cryptogramplacementsteps: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), responsetemplate.into_param().abi(), cryptogramplacementsteps.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn TryRespondWithCryptogramsAndStateAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<u32>>>(
        &self,
        responsetemplate: Param0,
        cryptogramplacementsteps: Param1,
        nextstate: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), responsetemplate.into_param().abi(), cryptogramplacementsteps.into_param().abi(), nextstate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulatorApduReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorApduReceivedEventArgs;{d55d1576-69d2-5333-5b5f-f8c0d6e9f09f})");
}
unsafe impl ::windows::runtime::Interface for SmartCardEmulatorApduReceivedEventArgs {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd55d1576_69d2_5333_5b5f_f8c0d6e9f09f);
}
impl ::windows::runtime::RuntimeName for SmartCardEmulatorApduReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorApduReceivedEventArgs";
}
impl ::core::convert::From<SmartCardEmulatorApduReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardEmulatorApduReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardEmulatorApduReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardEmulatorApduReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardEmulatorApduReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardEmulatorApduReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardEmulatorApduReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardEmulatorApduReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardEmulatorApduReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardEmulatorApduReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardEmulatorApduReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardEmulatorApduReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardEmulatorApduReceivedEventArgs {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorApduReceivedEventArgs {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardEmulatorConnectionDeactivatedEventArgs(pub ::windows::runtime::IInspectable);
impl SmartCardEmulatorConnectionDeactivatedEventArgs {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn ConnectionProperties(&self) -> ::windows::runtime::Result<SmartCardEmulatorConnectionProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardEmulatorConnectionProperties>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Reason(&self) -> ::windows::runtime::Result<SmartCardEmulatorConnectionDeactivatedReason> {
        let this = self;
        unsafe {
            let mut result__: SmartCardEmulatorConnectionDeactivatedReason = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardEmulatorConnectionDeactivatedReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedEventArgs;{2186d8d3-c5eb-5262-43df-62a0a1b55557})");
}
unsafe impl ::windows::runtime::Interface for SmartCardEmulatorConnectionDeactivatedEventArgs {
    type Vtable = ISmartCardEmulatorConnectionDeactivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2186d8d3_c5eb_5262_43df_62a0a1b55557);
}
impl ::windows::runtime::RuntimeName for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedEventArgs";
}
impl ::core::convert::From<SmartCardEmulatorConnectionDeactivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardEmulatorConnectionDeactivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardEmulatorConnectionDeactivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardEmulatorConnectionDeactivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardEmulatorConnectionDeactivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardEmulatorConnectionDeactivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardEmulatorConnectionDeactivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardEmulatorConnectionDeactivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardEmulatorConnectionDeactivatedEventArgs {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorConnectionDeactivatedEventArgs {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionDeactivatedReason(pub i32);
impl SmartCardEmulatorConnectionDeactivatedReason {
    pub const ConnectionLost: SmartCardEmulatorConnectionDeactivatedReason = SmartCardEmulatorConnectionDeactivatedReason(0i32);
    pub const ConnectionRedirected: SmartCardEmulatorConnectionDeactivatedReason = SmartCardEmulatorConnectionDeactivatedReason(1i32);
}
impl ::core::convert::From<i32> for SmartCardEmulatorConnectionDeactivatedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardEmulatorConnectionDeactivatedReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulatorConnectionDeactivatedReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedReason;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardEmulatorConnectionDeactivatedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardEmulatorConnectionProperties(pub ::windows::runtime::IInspectable);
impl SmartCardEmulatorConnectionProperties {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<SmartCardEmulatorConnectionSource> {
        let this = self;
        unsafe {
            let mut result__: SmartCardEmulatorConnectionSource = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardEmulatorConnectionSource>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulatorConnectionProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorConnectionProperties;{4e2ca5ee-f969-507d-6cf9-34e2d18df311})");
}
unsafe impl ::windows::runtime::Interface for SmartCardEmulatorConnectionProperties {
    type Vtable = ISmartCardEmulatorConnectionProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e2ca5ee_f969_507d_6cf9_34e2d18df311);
}
impl ::windows::runtime::RuntimeName for SmartCardEmulatorConnectionProperties {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorConnectionProperties";
}
impl ::core::convert::From<SmartCardEmulatorConnectionProperties> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardEmulatorConnectionProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardEmulatorConnectionProperties> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardEmulatorConnectionProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardEmulatorConnectionProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardEmulatorConnectionProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardEmulatorConnectionProperties> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardEmulatorConnectionProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardEmulatorConnectionProperties> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardEmulatorConnectionProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardEmulatorConnectionProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardEmulatorConnectionProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardEmulatorConnectionProperties {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorConnectionProperties {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionSource(pub i32);
impl SmartCardEmulatorConnectionSource {
    pub const Unknown: SmartCardEmulatorConnectionSource = SmartCardEmulatorConnectionSource(0i32);
    pub const NfcReader: SmartCardEmulatorConnectionSource = SmartCardEmulatorConnectionSource(1i32);
}
impl ::core::convert::From<i32> for SmartCardEmulatorConnectionSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardEmulatorConnectionSource {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulatorConnectionSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorConnectionSource;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardEmulatorConnectionSource {
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct SmartCardEmulatorContract(pub u8);
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardEmulatorEnablementPolicy(pub i32);
impl SmartCardEmulatorEnablementPolicy {
    pub const Never: SmartCardEmulatorEnablementPolicy = SmartCardEmulatorEnablementPolicy(0i32);
    pub const Always: SmartCardEmulatorEnablementPolicy = SmartCardEmulatorEnablementPolicy(1i32);
    pub const ScreenOn: SmartCardEmulatorEnablementPolicy = SmartCardEmulatorEnablementPolicy(2i32);
    pub const ScreenUnlocked: SmartCardEmulatorEnablementPolicy = SmartCardEmulatorEnablementPolicy(3i32);
}
impl ::core::convert::From<i32> for SmartCardEmulatorEnablementPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardEmulatorEnablementPolicy {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardEmulatorEnablementPolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorEnablementPolicy;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardEmulatorEnablementPolicy {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardLaunchBehavior(pub i32);
impl SmartCardLaunchBehavior {
    pub const Default: SmartCardLaunchBehavior = SmartCardLaunchBehavior(0i32);
    pub const AboveLock: SmartCardLaunchBehavior = SmartCardLaunchBehavior(1i32);
}
impl ::core::convert::From<i32> for SmartCardLaunchBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardLaunchBehavior {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardLaunchBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardLaunchBehavior;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardLaunchBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardPinCharacterPolicyOption(pub i32);
impl SmartCardPinCharacterPolicyOption {
    pub const Allow: SmartCardPinCharacterPolicyOption = SmartCardPinCharacterPolicyOption(0i32);
    pub const RequireAtLeastOne: SmartCardPinCharacterPolicyOption = SmartCardPinCharacterPolicyOption(1i32);
    pub const Disallow: SmartCardPinCharacterPolicyOption = SmartCardPinCharacterPolicyOption(2i32);
}
impl ::core::convert::From<i32> for SmartCardPinCharacterPolicyOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardPinCharacterPolicyOption {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardPinCharacterPolicyOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardPinCharacterPolicyOption;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardPinCharacterPolicyOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardPinPolicy(pub ::windows::runtime::IInspectable);
impl SmartCardPinPolicy {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardPinPolicy, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn MinLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetMinLength(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn MaxLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetMaxLength(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn UppercaseLetters(&self) -> ::windows::runtime::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__: SmartCardPinCharacterPolicyOption = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardPinCharacterPolicyOption>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetUppercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn LowercaseLetters(&self) -> ::windows::runtime::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__: SmartCardPinCharacterPolicyOption = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardPinCharacterPolicyOption>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetLowercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Digits(&self) -> ::windows::runtime::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__: SmartCardPinCharacterPolicyOption = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardPinCharacterPolicyOption>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetDigits(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SpecialCharacters(&self) -> ::windows::runtime::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__: SmartCardPinCharacterPolicyOption = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardPinCharacterPolicyOption>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SetSpecialCharacters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardPinPolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinPolicy;{183ce184-4db6-4841-ac9e-2ac1f39b7304})");
}
unsafe impl ::windows::runtime::Interface for SmartCardPinPolicy {
    type Vtable = ISmartCardPinPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x183ce184_4db6_4841_ac9e_2ac1f39b7304);
}
impl ::windows::runtime::RuntimeName for SmartCardPinPolicy {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinPolicy";
}
impl ::core::convert::From<SmartCardPinPolicy> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardPinPolicy) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardPinPolicy> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardPinPolicy) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardPinPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardPinPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardPinPolicy> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardPinPolicy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardPinPolicy> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardPinPolicy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardPinPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardPinPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardPinPolicy {}
unsafe impl ::core::marker::Sync for SmartCardPinPolicy {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardPinResetDeferral(pub ::windows::runtime::IInspectable);
impl SmartCardPinResetDeferral {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardPinResetDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinResetDeferral;{18c94aac-7805-4004-85e4-bbefac8f6884})");
}
unsafe impl ::windows::runtime::Interface for SmartCardPinResetDeferral {
    type Vtable = ISmartCardPinResetDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18c94aac_7805_4004_85e4_bbefac8f6884);
}
impl ::windows::runtime::RuntimeName for SmartCardPinResetDeferral {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinResetDeferral";
}
impl ::core::convert::From<SmartCardPinResetDeferral> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardPinResetDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardPinResetDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardPinResetDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardPinResetDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardPinResetDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardPinResetDeferral> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardPinResetDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardPinResetDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardPinResetDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardPinResetDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardPinResetDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardPinResetDeferral {}
unsafe impl ::core::marker::Sync for SmartCardPinResetDeferral {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardPinResetHandler(::windows::runtime::IUnknown);
impl SmartCardPinResetHandler {
    pub fn new<F: FnMut(&::core::option::Option<SmartCardProvisioning>, &::core::option::Option<SmartCardPinResetRequest>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = SmartCardPinResetHandler_box::<F> {
            vtable: &SmartCardPinResetHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, SmartCardProvisioning>, Param1: ::windows::runtime::IntoParam<'a, SmartCardPinResetRequest>>(&self, sender: Param0, request: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), request.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardPinResetHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({138d5e40-f3bc-4a5c-b41d-4b4ef684e237})");
}
unsafe impl ::windows::runtime::Interface for SmartCardPinResetHandler {
    type Vtable = SmartCardPinResetHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x138d5e40_f3bc_4a5c_b41d_4b4ef684e237);
}
#[repr(C)]
#[doc(hidden)]
pub struct SmartCardPinResetHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct SmartCardPinResetHandler_box<F: FnMut(&::core::option::Option<SmartCardProvisioning>, &::core::option::Option<SmartCardPinResetRequest>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const SmartCardPinResetHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<SmartCardProvisioning>, &::core::option::Option<SmartCardPinResetRequest>) -> ::windows::runtime::Result<()> + 'static> SmartCardPinResetHandler_box<F> {
    const VTABLE: SmartCardPinResetHandler_abi = SmartCardPinResetHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<SmartCardPinResetHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <SmartCardProvisioning as ::windows::runtime::Abi>::Abi as *const <SmartCardProvisioning as ::windows::runtime::DefaultType>::DefaultType),
            &*(&request as *const <SmartCardPinResetRequest as ::windows::runtime::Abi>::Abi as *const <SmartCardPinResetRequest as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardPinResetRequest(pub ::windows::runtime::IInspectable);
impl SmartCardPinResetRequest {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn Challenge(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<SmartCardPinResetDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardPinResetDeferral>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SetResponse<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, response: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), response.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardPinResetRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinResetRequest;{12fe3c4d-5fb9-4e8e-9ff6-61f475124fef})");
}
unsafe impl ::windows::runtime::Interface for SmartCardPinResetRequest {
    type Vtable = ISmartCardPinResetRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x12fe3c4d_5fb9_4e8e_9ff6_61f475124fef);
}
impl ::windows::runtime::RuntimeName for SmartCardPinResetRequest {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinResetRequest";
}
impl ::core::convert::From<SmartCardPinResetRequest> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardPinResetRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardPinResetRequest> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardPinResetRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardPinResetRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardPinResetRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardPinResetRequest> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardPinResetRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardPinResetRequest> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardPinResetRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardPinResetRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardPinResetRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardPinResetRequest {}
unsafe impl ::core::marker::Sync for SmartCardPinResetRequest {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardProvisioning(pub ::windows::runtime::IInspectable);
impl SmartCardProvisioning {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SmartCard(&self) -> ::windows::runtime::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCard>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetIdAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::GUID>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::GUID>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetNameAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetChallengeContextAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardChallengeContext>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardChallengeContext>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RequestPinChangeAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RequestPinResetAsync<'a, Param0: ::windows::runtime::IntoParam<'a, SmartCardPinResetHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetAuthorityKeyContainerNameAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardProvisioning2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn FromSmartCardAsync<'a, Param0: ::windows::runtime::IntoParam<'a, SmartCard>>(card: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), card.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn RequestVirtualSmartCardCreationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, SmartCardPinPolicy>>(friendlyname: Param0, administrativekey: Param1, pinpolicy: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), friendlyname.into_param().abi(), administrativekey.into_param().abi(), pinpolicy.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn RequestVirtualSmartCardCreationAsyncWithCardId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, SmartCardPinPolicy>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(
        friendlyname: Param0,
        administrativekey: Param1,
        pinpolicy: Param2,
        cardid: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), friendlyname.into_param().abi(), administrativekey.into_param().abi(), pinpolicy.into_param().abi(), cardid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RequestVirtualSmartCardDeletionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, SmartCard>>(card: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), card.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn RequestAttestedVirtualSmartCardCreationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, SmartCardPinPolicy>>(friendlyname: Param0, administrativekey: Param1, pinpolicy: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), friendlyname.into_param().abi(), administrativekey.into_param().abi(), pinpolicy.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Storage_Streams`*"]
    pub fn RequestAttestedVirtualSmartCardCreationAsyncWithCardId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, SmartCardPinPolicy>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(
        friendlyname: Param0,
        administrativekey: Param1,
        pinpolicy: Param2,
        cardid: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), friendlyname.into_param().abi(), administrativekey.into_param().abi(), pinpolicy.into_param().abi(), cardid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    pub fn ISmartCardProvisioningStatics<R, F: FnOnce(&ISmartCardProvisioningStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardProvisioning, ISmartCardProvisioningStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardProvisioningStatics2<R, F: FnOnce(&ISmartCardProvisioningStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardProvisioning, ISmartCardProvisioningStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardProvisioning {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardProvisioning;{19eeedbd-1fab-477c-b712-1a2c5af1fd6e})");
}
unsafe impl ::windows::runtime::Interface for SmartCardProvisioning {
    type Vtable = ISmartCardProvisioning_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x19eeedbd_1fab_477c_b712_1a2c5af1fd6e);
}
impl ::windows::runtime::RuntimeName for SmartCardProvisioning {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardProvisioning";
}
impl ::core::convert::From<SmartCardProvisioning> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardProvisioning) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardProvisioning> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardProvisioning) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardProvisioning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardProvisioning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardProvisioning> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardProvisioning) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardProvisioning> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardProvisioning) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardProvisioning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardProvisioning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardProvisioning {}
unsafe impl ::core::marker::Sync for SmartCardProvisioning {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardReader(pub ::windows::runtime::IInspectable);
impl SmartCardReader {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<SmartCardReaderKind> {
        let this = self;
        unsafe {
            let mut result__: SmartCardReaderKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardReaderKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn GetStatusAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardReaderStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardReaderStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllCardsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCard>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCard>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn CardAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SmartCardReader, CardAddedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RemoveCardAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn CardRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SmartCardReader, CardRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn RemoveCardRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn GetDeviceSelectorWithKind(kind: SmartCardReaderKind) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmartCardReader>> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmartCardReader>>(result__)
        })
    }
    pub fn ISmartCardReaderStatics<R, F: FnOnce(&ISmartCardReaderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmartCardReader, ISmartCardReaderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardReader;{1074b4e0-54c2-4df0-817a-14c14378f06c})");
}
unsafe impl ::windows::runtime::Interface for SmartCardReader {
    type Vtable = ISmartCardReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1074b4e0_54c2_4df0_817a_14c14378f06c);
}
impl ::windows::runtime::RuntimeName for SmartCardReader {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardReader";
}
impl ::core::convert::From<SmartCardReader> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardReader> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardReader> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardReader> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardReader {}
unsafe impl ::core::marker::Sync for SmartCardReader {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardReaderKind(pub i32);
impl SmartCardReaderKind {
    pub const Any: SmartCardReaderKind = SmartCardReaderKind(0i32);
    pub const Generic: SmartCardReaderKind = SmartCardReaderKind(1i32);
    pub const Tpm: SmartCardReaderKind = SmartCardReaderKind(2i32);
    pub const Nfc: SmartCardReaderKind = SmartCardReaderKind(3i32);
    pub const Uicc: SmartCardReaderKind = SmartCardReaderKind(4i32);
    pub const EmbeddedSE: SmartCardReaderKind = SmartCardReaderKind(5i32);
}
impl ::core::convert::From<i32> for SmartCardReaderKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardReaderKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardReaderKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardReaderKind;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardReaderKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardReaderStatus(pub i32);
impl SmartCardReaderStatus {
    pub const Disconnected: SmartCardReaderStatus = SmartCardReaderStatus(0i32);
    pub const Ready: SmartCardReaderStatus = SmartCardReaderStatus(1i32);
    pub const Exclusive: SmartCardReaderStatus = SmartCardReaderStatus(2i32);
}
impl ::core::convert::From<i32> for SmartCardReaderStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardReaderStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardReaderStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardReaderStatus;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardReaderStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardStatus(pub i32);
impl SmartCardStatus {
    pub const Disconnected: SmartCardStatus = SmartCardStatus(0i32);
    pub const Ready: SmartCardStatus = SmartCardStatus(1i32);
    pub const Shared: SmartCardStatus = SmartCardStatus(2i32);
    pub const Exclusive: SmartCardStatus = SmartCardStatus(3i32);
    pub const Unresponsive: SmartCardStatus = SmartCardStatus(4i32);
}
impl ::core::convert::From<i32> for SmartCardStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardStatus;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SmartCardTriggerDetails(pub ::windows::runtime::IInspectable);
impl SmartCardTriggerDetails {
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn TriggerType(&self) -> ::windows::runtime::Result<SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__: SmartCardTriggerType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardTriggerType>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn SourceAppletId(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_SmartCards`, `Storage_Streams`*"]
    pub fn TriggerData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn Emulator(&self) -> ::windows::runtime::Result<SmartCardEmulator> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCardEmulator>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn TryLaunchCurrentAppAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, arguments: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), arguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_SmartCards`, `Foundation`*"]
    pub fn TryLaunchCurrentAppWithBehaviorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, arguments: Param0, behavior: SmartCardLaunchBehavior) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), arguments.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_SmartCards`*"]
    pub fn SmartCard(&self) -> ::windows::runtime::Result<SmartCard> {
        let this = &::windows::runtime::Interface::cast::<ISmartCardTriggerDetails3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SmartCard>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardTriggerDetails;{5f9bf11e-39ef-4f2b-b44f-0a9155b177bc})");
}
unsafe impl ::windows::runtime::Interface for SmartCardTriggerDetails {
    type Vtable = ISmartCardTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f9bf11e_39ef_4f2b_b44f_0a9155b177bc);
}
impl ::windows::runtime::RuntimeName for SmartCardTriggerDetails {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardTriggerDetails";
}
impl ::core::convert::From<SmartCardTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: SmartCardTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SmartCardTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &SmartCardTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmartCardTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SmartCardTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SmartCardTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: SmartCardTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SmartCardTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &SmartCardTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmartCardTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmartCardTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SmartCardTriggerDetails {}
unsafe impl ::core::marker::Sync for SmartCardTriggerDetails {}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardTriggerType(pub i32);
impl SmartCardTriggerType {
    pub const EmulatorTransaction: SmartCardTriggerType = SmartCardTriggerType(0i32);
    pub const EmulatorNearFieldEntry: SmartCardTriggerType = SmartCardTriggerType(1i32);
    pub const EmulatorNearFieldExit: SmartCardTriggerType = SmartCardTriggerType(2i32);
    pub const EmulatorHostApplicationActivated: SmartCardTriggerType = SmartCardTriggerType(3i32);
    pub const EmulatorAppletIdGroupRegistrationChanged: SmartCardTriggerType = SmartCardTriggerType(4i32);
    pub const ReaderCardAdded: SmartCardTriggerType = SmartCardTriggerType(5i32);
}
impl ::core::convert::From<i32> for SmartCardTriggerType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardTriggerType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardTriggerType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardTriggerType;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardTriggerType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_SmartCards`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmartCardUnlockPromptingBehavior(pub i32);
impl SmartCardUnlockPromptingBehavior {
    pub const AllowUnlockPrompt: SmartCardUnlockPromptingBehavior = SmartCardUnlockPromptingBehavior(0i32);
    pub const RequireUnlockPrompt: SmartCardUnlockPromptingBehavior = SmartCardUnlockPromptingBehavior(1i32);
    pub const PreventUnlockPrompt: SmartCardUnlockPromptingBehavior = SmartCardUnlockPromptingBehavior(2i32);
}
impl ::core::convert::From<i32> for SmartCardUnlockPromptingBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmartCardUnlockPromptingBehavior {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmartCardUnlockPromptingBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardUnlockPromptingBehavior;i4)");
}
impl ::windows::runtime::DefaultType for SmartCardUnlockPromptingBehavior {
    type DefaultType = Self;
}
