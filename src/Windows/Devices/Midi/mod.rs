#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbe1fa860_62b4_4d52_a37e_92e54d35b909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiChannelPressureMessageFactory {
    type Vtable = IMidiChannelPressureMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6218ed2f_2284_412a_94cf_10fb04842c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, pressure: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiControlChangeMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb7e15f83_780d_405f_b781_3e1598c97f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiControlChangeMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiControlChangeMessageFactory {
    type Vtable = IMidiControlChangeMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2ab14321_956c_46ad_9752_f87f55052fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, controller: u8, controlvalue: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiInPort(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiInPort {
    type Vtable = IMidiInPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd5c1d9db_971a_4eaf_a23d_ea19fe607ff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPort_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiInPortStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiInPortStatics {
    type Vtable = IMidiInPortStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x44c439dc_67ff_4a6e_8bac_fdb6610cf296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPortStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Midi`*"]
pub struct IMidiMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl IMidiMessage {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMidiMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{79767945-1094-4283-9be0-289fc0ee8334}");
}
impl ::core::convert::From<IMidiMessage> for ::windows::runtime::IUnknown {
    fn from(value: IMidiMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMidiMessage> for ::windows::runtime::IUnknown {
    fn from(value: &IMidiMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMidiMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMidiMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMidiMessage> for ::windows::runtime::IInspectable {
    fn from(value: IMidiMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMidiMessage> for ::windows::runtime::IInspectable {
    fn from(value: &IMidiMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMidiMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMidiMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MidiMessageType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiMessageReceivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x76566e56_f328_4b51_907d_b3a8ce96bf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessageReceivedEventArgs_abi(
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
pub struct IMidiNoteOffMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16fd8af4_198e_4d8f_a654_d305a293548f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiNoteOffMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiNoteOffMessageFactory {
    type Vtable = IMidiNoteOffMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa6b240e0_a749_425f_8af4_a4d979cc15b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiNoteOnMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe0224af5_6181_46dd_afa2_410004c057aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiNoteOnMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiNoteOnMessageFactory {
    type Vtable = IMidiNoteOnMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9b4280a0_59c1_420e_b517_15a10aa9606b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Midi`*"]
pub struct IMidiOutPort(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiOutPort {
    type Vtable = IMidiOutPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x931d6d9f_57a2_4a3a_adb8_4640886f6693);
}
impl IMidiOutPort {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn SendMessage<'a, Param0: ::windows::runtime::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn SendBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMidiOutPort {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{931d6d9f-57a2-4a3a-adb8-4640886f6693}");
}
impl ::core::convert::From<IMidiOutPort> for ::windows::runtime::IUnknown {
    fn from(value: IMidiOutPort) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMidiOutPort> for ::windows::runtime::IUnknown {
    fn from(value: &IMidiOutPort) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMidiOutPort> for ::windows::runtime::IInspectable {
    fn from(value: IMidiOutPort) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMidiOutPort> for ::windows::runtime::IInspectable {
    fn from(value: &IMidiOutPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IMidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IMidiOutPort) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IMidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IMidiOutPort) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, midimessage: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mididata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiOutPortStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiOutPortStatics {
    type Vtable = IMidiOutPortStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x065cc3e9_0f88_448b_9b64_a95826c65b8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPortStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x29df4cb1_2e9f_4faf_8c2b_9cb82a9079ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiPitchBendChangeMessageFactory {
    type Vtable = IMidiPitchBendChangeMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf5eedf55_cfc8_4926_b30e_a3622393306c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, bend: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1f7337fe_ace8_48a0_868e_7cdbf20f04d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiPolyphonicKeyPressureMessageFactory {
    type Vtable = IMidiPolyphonicKeyPressureMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe98f483e_c4b3_4dd2_917c_e349815a1b3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, note: u8, pressure: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9cbb3c78_7a3e_4327_aa98_20b8e4485af8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiProgramChangeMessageFactory {
    type Vtable = IMidiProgramChangeMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd6b04387_524b_4104_9c99_6572bfd2e261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, program: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ca50c56_ec5e_4ae4_a115_88dc57cc2b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessage_abi(
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
pub struct IMidiSongPositionPointerMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSongPositionPointerMessageFactory {
    type Vtable = IMidiSongPositionPointerMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9c00e996_f10b_4fea_b395_f5d6cf80f64e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, beats: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiSongSelectMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x49f0f27f_6d83_4741_a5bf_4629f6be974f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiSongSelectMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSongSelectMessageFactory {
    type Vtable = IMidiSongSelectMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x848878e4_8748_4129_a66c_a05493f75daa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, song: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiSynthesizer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSynthesizer {
    type Vtable = IMidiSynthesizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf0da155e_db90_405f_b8ae_21d2e17f2e45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiSynthesizerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSynthesizerStatics {
    type Vtable = IMidiSynthesizerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4224eaa8_6629_4d6b_aa8f_d4521a5a31ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audiodevice: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mididevice: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiSystemExclusiveMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSystemExclusiveMessageFactory {
    type Vtable = IMidiSystemExclusiveMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x083de222_3b74_4320_9b42_0ca8545f8a24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSystemExclusiveMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rawdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0bf7087d_fa63_4a1c_8deb_c0e87796a6d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessageFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiTimeCodeMessageFactory {
    type Vtable = IMidiTimeCodeMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeb3099c5_771c_40de_b961_175a7489a85e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frametype: u8, values: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiActiveSensingMessage(pub ::windows::runtime::IInspectable);
impl MidiActiveSensingMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiActiveSensingMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiActiveSensingMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiActiveSensingMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiActiveSensingMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl ::windows::runtime::RuntimeName for MidiActiveSensingMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiActiveSensingMessage";
}
impl ::core::convert::From<MidiActiveSensingMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiActiveSensingMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiActiveSensingMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiActiveSensingMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiActiveSensingMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiActiveSensingMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiActiveSensingMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiActiveSensingMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiActiveSensingMessage> for IMidiMessage {
    fn from(value: MidiActiveSensingMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiActiveSensingMessage> for IMidiMessage {
    fn from(value: &MidiActiveSensingMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiActiveSensingMessage {}
unsafe impl ::core::marker::Sync for MidiActiveSensingMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiChannelPressureMessage(pub ::windows::runtime::IInspectable);
impl MidiChannelPressureMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Pressure(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiChannelPressureMessage(channel: u8, pressure: u8) -> ::windows::runtime::Result<MidiChannelPressureMessage> {
        Self::IMidiChannelPressureMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channel, pressure, &mut result__).from_abi::<MidiChannelPressureMessage>(result__)
        })
    }
    pub fn IMidiChannelPressureMessageFactory<R, F: FnOnce(&IMidiChannelPressureMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiChannelPressureMessage, IMidiChannelPressureMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiChannelPressureMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiChannelPressureMessage;{be1fa860-62b4-4d52-a37e-92e54d35b909})");
}
unsafe impl ::windows::runtime::Interface for MidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbe1fa860_62b4_4d52_a37e_92e54d35b909);
}
impl ::windows::runtime::RuntimeName for MidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiChannelPressureMessage";
}
impl ::core::convert::From<MidiChannelPressureMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiChannelPressureMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiChannelPressureMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiChannelPressureMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiChannelPressureMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiChannelPressureMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiChannelPressureMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiChannelPressureMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiChannelPressureMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiChannelPressureMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiChannelPressureMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiChannelPressureMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiChannelPressureMessage {}
unsafe impl ::core::marker::Sync for MidiChannelPressureMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiContinueMessage(pub ::windows::runtime::IInspectable);
impl MidiContinueMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiContinueMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiContinueMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiContinueMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiContinueMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl ::windows::runtime::RuntimeName for MidiContinueMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiContinueMessage";
}
impl ::core::convert::From<MidiContinueMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiContinueMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiContinueMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiContinueMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiContinueMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiContinueMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiContinueMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiContinueMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiContinueMessage> for IMidiMessage {
    fn from(value: MidiContinueMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiContinueMessage> for IMidiMessage {
    fn from(value: &MidiContinueMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiContinueMessage {}
unsafe impl ::core::marker::Sync for MidiContinueMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiControlChangeMessage(pub ::windows::runtime::IInspectable);
impl MidiControlChangeMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Controller(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn ControlValue(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiControlChangeMessage(channel: u8, controller: u8, controlvalue: u8) -> ::windows::runtime::Result<MidiControlChangeMessage> {
        Self::IMidiControlChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channel, controller, controlvalue, &mut result__).from_abi::<MidiControlChangeMessage>(result__)
        })
    }
    pub fn IMidiControlChangeMessageFactory<R, F: FnOnce(&IMidiControlChangeMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiControlChangeMessage, IMidiControlChangeMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiControlChangeMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiControlChangeMessage;{b7e15f83-780d-405f-b781-3e1598c97f40})");
}
unsafe impl ::windows::runtime::Interface for MidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb7e15f83_780d_405f_b781_3e1598c97f40);
}
impl ::windows::runtime::RuntimeName for MidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiControlChangeMessage";
}
impl ::core::convert::From<MidiControlChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiControlChangeMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiControlChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiControlChangeMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiControlChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiControlChangeMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiControlChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiControlChangeMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiControlChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiControlChangeMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiControlChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiControlChangeMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiControlChangeMessage {}
unsafe impl ::core::marker::Sync for MidiControlChangeMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiInPort(pub ::windows::runtime::IInspectable);
impl MidiInPort {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn MessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MidiInPort>> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiInPort>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IMidiInPortStatics<R, F: FnOnce(&IMidiInPortStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiInPort, IMidiInPortStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiInPort {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiInPort;{d5c1d9db-971a-4eaf-a23d-ea19fe607ff9})");
}
unsafe impl ::windows::runtime::Interface for MidiInPort {
    type Vtable = IMidiInPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd5c1d9db_971a_4eaf_a23d_ea19fe607ff9);
}
impl ::windows::runtime::RuntimeName for MidiInPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiInPort";
}
impl ::core::convert::From<MidiInPort> for ::windows::runtime::IUnknown {
    fn from(value: MidiInPort) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiInPort> for ::windows::runtime::IUnknown {
    fn from(value: &MidiInPort) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiInPort> for ::windows::runtime::IInspectable {
    fn from(value: MidiInPort) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiInPort> for ::windows::runtime::IInspectable {
    fn from(value: &MidiInPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MidiInPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiInPort) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiInPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiInPort) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiInPort {}
unsafe impl ::core::marker::Sync for MidiInPort {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiMessageReceivedEventArgs(pub ::windows::runtime::IInspectable);
impl MidiMessageReceivedEventArgs {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<IMidiMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMidiMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiMessageReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiMessageReceivedEventArgs;{76566e56-f328-4b51-907d-b3a8ce96bf80})");
}
unsafe impl ::windows::runtime::Interface for MidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x76566e56_f328_4b51_907d_b3a8ce96bf80);
}
impl ::windows::runtime::RuntimeName for MidiMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Midi.MidiMessageReceivedEventArgs";
}
impl ::core::convert::From<MidiMessageReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MidiMessageReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiMessageReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MidiMessageReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiMessageReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MidiMessageReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiMessageReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MidiMessageReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MidiMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MidiMessageReceivedEventArgs {}
#[doc = "*Required features: `Devices_Midi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MidiMessageType(pub i32);
impl MidiMessageType {
    pub const None: MidiMessageType = MidiMessageType(0i32);
    pub const NoteOff: MidiMessageType = MidiMessageType(128i32);
    pub const NoteOn: MidiMessageType = MidiMessageType(144i32);
    pub const PolyphonicKeyPressure: MidiMessageType = MidiMessageType(160i32);
    pub const ControlChange: MidiMessageType = MidiMessageType(176i32);
    pub const ProgramChange: MidiMessageType = MidiMessageType(192i32);
    pub const ChannelPressure: MidiMessageType = MidiMessageType(208i32);
    pub const PitchBendChange: MidiMessageType = MidiMessageType(224i32);
    pub const SystemExclusive: MidiMessageType = MidiMessageType(240i32);
    pub const MidiTimeCode: MidiMessageType = MidiMessageType(241i32);
    pub const SongPositionPointer: MidiMessageType = MidiMessageType(242i32);
    pub const SongSelect: MidiMessageType = MidiMessageType(243i32);
    pub const TuneRequest: MidiMessageType = MidiMessageType(246i32);
    pub const EndSystemExclusive: MidiMessageType = MidiMessageType(247i32);
    pub const TimingClock: MidiMessageType = MidiMessageType(248i32);
    pub const Start: MidiMessageType = MidiMessageType(250i32);
    pub const Continue: MidiMessageType = MidiMessageType(251i32);
    pub const Stop: MidiMessageType = MidiMessageType(252i32);
    pub const ActiveSensing: MidiMessageType = MidiMessageType(254i32);
    pub const SystemReset: MidiMessageType = MidiMessageType(255i32);
}
impl ::core::convert::From<i32> for MidiMessageType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MidiMessageType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MidiMessageType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Midi.MidiMessageType;i4)");
}
impl ::windows::runtime::DefaultType for MidiMessageType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiNoteOffMessage(pub ::windows::runtime::IInspectable);
impl MidiNoteOffMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Note(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Velocity(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiNoteOffMessage(channel: u8, note: u8, velocity: u8) -> ::windows::runtime::Result<MidiNoteOffMessage> {
        Self::IMidiNoteOffMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channel, note, velocity, &mut result__).from_abi::<MidiNoteOffMessage>(result__)
        })
    }
    pub fn IMidiNoteOffMessageFactory<R, F: FnOnce(&IMidiNoteOffMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiNoteOffMessage, IMidiNoteOffMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiNoteOffMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiNoteOffMessage;{16fd8af4-198e-4d8f-a654-d305a293548f})");
}
unsafe impl ::windows::runtime::Interface for MidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16fd8af4_198e_4d8f_a654_d305a293548f);
}
impl ::windows::runtime::RuntimeName for MidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOffMessage";
}
impl ::core::convert::From<MidiNoteOffMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiNoteOffMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiNoteOffMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiNoteOffMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiNoteOffMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiNoteOffMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiNoteOffMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiNoteOffMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiNoteOffMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiNoteOffMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiNoteOffMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiNoteOffMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiNoteOffMessage {}
unsafe impl ::core::marker::Sync for MidiNoteOffMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiNoteOnMessage(pub ::windows::runtime::IInspectable);
impl MidiNoteOnMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Note(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Velocity(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiNoteOnMessage(channel: u8, note: u8, velocity: u8) -> ::windows::runtime::Result<MidiNoteOnMessage> {
        Self::IMidiNoteOnMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channel, note, velocity, &mut result__).from_abi::<MidiNoteOnMessage>(result__)
        })
    }
    pub fn IMidiNoteOnMessageFactory<R, F: FnOnce(&IMidiNoteOnMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiNoteOnMessage, IMidiNoteOnMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiNoteOnMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiNoteOnMessage;{e0224af5-6181-46dd-afa2-410004c057aa})");
}
unsafe impl ::windows::runtime::Interface for MidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe0224af5_6181_46dd_afa2_410004c057aa);
}
impl ::windows::runtime::RuntimeName for MidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOnMessage";
}
impl ::core::convert::From<MidiNoteOnMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiNoteOnMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiNoteOnMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiNoteOnMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiNoteOnMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiNoteOnMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiNoteOnMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiNoteOnMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiNoteOnMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiNoteOnMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiNoteOnMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiNoteOnMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiNoteOnMessage {}
unsafe impl ::core::marker::Sync for MidiNoteOnMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiOutPort(pub ::windows::runtime::IInspectable);
impl MidiOutPort {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn SendMessage<'a, Param0: ::windows::runtime::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn SendBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IMidiOutPort>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IMidiOutPortStatics<R, F: FnOnce(&IMidiOutPortStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiOutPort, IMidiOutPortStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiOutPort {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiOutPort;{931d6d9f-57a2-4a3a-adb8-4640886f6693})");
}
unsafe impl ::windows::runtime::Interface for MidiOutPort {
    type Vtable = IMidiOutPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x931d6d9f_57a2_4a3a_adb8_4640886f6693);
}
impl ::windows::runtime::RuntimeName for MidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiOutPort";
}
impl ::core::convert::From<MidiOutPort> for ::windows::runtime::IUnknown {
    fn from(value: MidiOutPort) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiOutPort> for ::windows::runtime::IUnknown {
    fn from(value: &MidiOutPort) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiOutPort> for ::windows::runtime::IInspectable {
    fn from(value: MidiOutPort) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiOutPort> for ::windows::runtime::IInspectable {
    fn from(value: &MidiOutPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiOutPort> for IMidiOutPort {
    fn from(value: MidiOutPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiOutPort> for IMidiOutPort {
    fn from(value: &MidiOutPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiOutPort> for MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiOutPort> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiOutPort> for &MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiOutPort> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiOutPort) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiOutPort) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiOutPort {}
unsafe impl ::core::marker::Sync for MidiOutPort {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiPitchBendChangeMessage(pub ::windows::runtime::IInspectable);
impl MidiPitchBendChangeMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Bend(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiPitchBendChangeMessage(channel: u8, bend: u16) -> ::windows::runtime::Result<MidiPitchBendChangeMessage> {
        Self::IMidiPitchBendChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channel, bend, &mut result__).from_abi::<MidiPitchBendChangeMessage>(result__)
        })
    }
    pub fn IMidiPitchBendChangeMessageFactory<R, F: FnOnce(&IMidiPitchBendChangeMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiPitchBendChangeMessage, IMidiPitchBendChangeMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiPitchBendChangeMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiPitchBendChangeMessage;{29df4cb1-2e9f-4faf-8c2b-9cb82a9079ca})");
}
unsafe impl ::windows::runtime::Interface for MidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x29df4cb1_2e9f_4faf_8c2b_9cb82a9079ca);
}
impl ::windows::runtime::RuntimeName for MidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPitchBendChangeMessage";
}
impl ::core::convert::From<MidiPitchBendChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiPitchBendChangeMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiPitchBendChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiPitchBendChangeMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiPitchBendChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiPitchBendChangeMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiPitchBendChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiPitchBendChangeMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiPitchBendChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiPitchBendChangeMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiPitchBendChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiPitchBendChangeMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiPitchBendChangeMessage {}
unsafe impl ::core::marker::Sync for MidiPitchBendChangeMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiPolyphonicKeyPressureMessage(pub ::windows::runtime::IInspectable);
impl MidiPolyphonicKeyPressureMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Note(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Pressure(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiPolyphonicKeyPressureMessage(channel: u8, note: u8, pressure: u8) -> ::windows::runtime::Result<MidiPolyphonicKeyPressureMessage> {
        Self::IMidiPolyphonicKeyPressureMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channel, note, pressure, &mut result__).from_abi::<MidiPolyphonicKeyPressureMessage>(result__)
        })
    }
    pub fn IMidiPolyphonicKeyPressureMessageFactory<R, F: FnOnce(&IMidiPolyphonicKeyPressureMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiPolyphonicKeyPressureMessage, IMidiPolyphonicKeyPressureMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiPolyphonicKeyPressureMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage;{1f7337fe-ace8-48a0-868e-7cdbf20f04d6})");
}
unsafe impl ::windows::runtime::Interface for MidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1f7337fe_ace8_48a0_868e_7cdbf20f04d6);
}
impl ::windows::runtime::RuntimeName for MidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage";
}
impl ::core::convert::From<MidiPolyphonicKeyPressureMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiPolyphonicKeyPressureMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiPolyphonicKeyPressureMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiPolyphonicKeyPressureMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiPolyphonicKeyPressureMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiPolyphonicKeyPressureMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiPolyphonicKeyPressureMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiPolyphonicKeyPressureMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiPolyphonicKeyPressureMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiPolyphonicKeyPressureMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiPolyphonicKeyPressureMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiPolyphonicKeyPressureMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiPolyphonicKeyPressureMessage {}
unsafe impl ::core::marker::Sync for MidiPolyphonicKeyPressureMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiProgramChangeMessage(pub ::windows::runtime::IInspectable);
impl MidiProgramChangeMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Program(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiProgramChangeMessage(channel: u8, program: u8) -> ::windows::runtime::Result<MidiProgramChangeMessage> {
        Self::IMidiProgramChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channel, program, &mut result__).from_abi::<MidiProgramChangeMessage>(result__)
        })
    }
    pub fn IMidiProgramChangeMessageFactory<R, F: FnOnce(&IMidiProgramChangeMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiProgramChangeMessage, IMidiProgramChangeMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiProgramChangeMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiProgramChangeMessage;{9cbb3c78-7a3e-4327-aa98-20b8e4485af8})");
}
unsafe impl ::windows::runtime::Interface for MidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9cbb3c78_7a3e_4327_aa98_20b8e4485af8);
}
impl ::windows::runtime::RuntimeName for MidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiProgramChangeMessage";
}
impl ::core::convert::From<MidiProgramChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiProgramChangeMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiProgramChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiProgramChangeMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiProgramChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiProgramChangeMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiProgramChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiProgramChangeMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiProgramChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiProgramChangeMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiProgramChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiProgramChangeMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiProgramChangeMessage {}
unsafe impl ::core::marker::Sync for MidiProgramChangeMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiSongPositionPointerMessage(pub ::windows::runtime::IInspectable);
impl MidiSongPositionPointerMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Beats(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiSongPositionPointerMessage(beats: u16) -> ::windows::runtime::Result<MidiSongPositionPointerMessage> {
        Self::IMidiSongPositionPointerMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), beats, &mut result__).from_abi::<MidiSongPositionPointerMessage>(result__)
        })
    }
    pub fn IMidiSongPositionPointerMessageFactory<R, F: FnOnce(&IMidiSongPositionPointerMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSongPositionPointerMessage, IMidiSongPositionPointerMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSongPositionPointerMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSongPositionPointerMessage;{4ca50c56-ec5e-4ae4-a115-88dc57cc2b79})");
}
unsafe impl ::windows::runtime::Interface for MidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ca50c56_ec5e_4ae4_a115_88dc57cc2b79);
}
impl ::windows::runtime::RuntimeName for MidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongPositionPointerMessage";
}
impl ::core::convert::From<MidiSongPositionPointerMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiSongPositionPointerMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiSongPositionPointerMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSongPositionPointerMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiSongPositionPointerMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiSongPositionPointerMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiSongPositionPointerMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSongPositionPointerMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiSongPositionPointerMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiSongPositionPointerMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiSongPositionPointerMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiSongPositionPointerMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiSongPositionPointerMessage {}
unsafe impl ::core::marker::Sync for MidiSongPositionPointerMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiSongSelectMessage(pub ::windows::runtime::IInspectable);
impl MidiSongSelectMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Song(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiSongSelectMessage(song: u8) -> ::windows::runtime::Result<MidiSongSelectMessage> {
        Self::IMidiSongSelectMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), song, &mut result__).from_abi::<MidiSongSelectMessage>(result__)
        })
    }
    pub fn IMidiSongSelectMessageFactory<R, F: FnOnce(&IMidiSongSelectMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSongSelectMessage, IMidiSongSelectMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSongSelectMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSongSelectMessage;{49f0f27f-6d83-4741-a5bf-4629f6be974f})");
}
unsafe impl ::windows::runtime::Interface for MidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x49f0f27f_6d83_4741_a5bf_4629f6be974f);
}
impl ::windows::runtime::RuntimeName for MidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongSelectMessage";
}
impl ::core::convert::From<MidiSongSelectMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiSongSelectMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiSongSelectMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSongSelectMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiSongSelectMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiSongSelectMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiSongSelectMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSongSelectMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiSongSelectMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiSongSelectMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiSongSelectMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiSongSelectMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiSongSelectMessage {}
unsafe impl ::core::marker::Sync for MidiSongSelectMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiStartMessage(pub ::windows::runtime::IInspectable);
impl MidiStartMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiStartMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiStartMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiStartMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiStartMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl ::windows::runtime::RuntimeName for MidiStartMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStartMessage";
}
impl ::core::convert::From<MidiStartMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiStartMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiStartMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiStartMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiStartMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiStartMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiStartMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiStartMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiStartMessage> for IMidiMessage {
    fn from(value: MidiStartMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiStartMessage> for IMidiMessage {
    fn from(value: &MidiStartMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiStartMessage {}
unsafe impl ::core::marker::Sync for MidiStartMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiStopMessage(pub ::windows::runtime::IInspectable);
impl MidiStopMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiStopMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiStopMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiStopMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiStopMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl ::windows::runtime::RuntimeName for MidiStopMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStopMessage";
}
impl ::core::convert::From<MidiStopMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiStopMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiStopMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiStopMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiStopMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiStopMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiStopMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiStopMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiStopMessage> for IMidiMessage {
    fn from(value: MidiStopMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiStopMessage> for IMidiMessage {
    fn from(value: &MidiStopMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiStopMessage {}
unsafe impl ::core::marker::Sync for MidiStopMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiSynthesizer(pub ::windows::runtime::IInspectable);
impl MidiSynthesizer {
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Midi`, `Devices_Enumeration`*"]
    pub fn AudioDevice(&self) -> ::windows::runtime::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Volume(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn SetVolume(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn SendMessage<'a, Param0: ::windows::runtime::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn SendBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMidiOutPort>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn CreateAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Midi`, `Devices_Enumeration`, `Foundation`*"]
    pub fn CreateFromAudioDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Enumeration::DeviceInformation>>(audiodevice: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), audiodevice.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Midi`, `Devices_Enumeration`*"]
    pub fn IsSynthesizer<'a, Param0: ::windows::runtime::IntoParam<'a, super::Enumeration::DeviceInformation>>(mididevice: Param0) -> ::windows::runtime::Result<bool> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mididevice.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IMidiSynthesizerStatics<R, F: FnOnce(&IMidiSynthesizerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSynthesizer, IMidiSynthesizerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSynthesizer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSynthesizer;{f0da155e-db90-405f-b8ae-21d2e17f2e45})");
}
unsafe impl ::windows::runtime::Interface for MidiSynthesizer {
    type Vtable = IMidiSynthesizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf0da155e_db90_405f_b8ae_21d2e17f2e45);
}
impl ::windows::runtime::RuntimeName for MidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSynthesizer";
}
impl ::core::convert::From<MidiSynthesizer> for ::windows::runtime::IUnknown {
    fn from(value: MidiSynthesizer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiSynthesizer> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSynthesizer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiSynthesizer> for ::windows::runtime::IInspectable {
    fn from(value: MidiSynthesizer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiSynthesizer> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSynthesizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiSynthesizer> for IMidiOutPort {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiSynthesizer) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiSynthesizer> for IMidiOutPort {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiSynthesizer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiOutPort> for MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiOutPort> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiOutPort> for &MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiOutPort> {
        ::core::convert::TryInto::<IMidiOutPort>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MidiSynthesizer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiSynthesizer) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiSynthesizer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiSynthesizer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiSynthesizer {}
unsafe impl ::core::marker::Sync for MidiSynthesizer {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiSystemExclusiveMessage(pub ::windows::runtime::IInspectable);
impl MidiSystemExclusiveMessage {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn CreateMidiSystemExclusiveMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(rawdata: Param0) -> ::windows::runtime::Result<MidiSystemExclusiveMessage> {
        Self::IMidiSystemExclusiveMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), rawdata.into_param().abi(), &mut result__).from_abi::<MidiSystemExclusiveMessage>(result__)
        })
    }
    pub fn IMidiSystemExclusiveMessageFactory<R, F: FnOnce(&IMidiSystemExclusiveMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSystemExclusiveMessage, IMidiSystemExclusiveMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSystemExclusiveMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSystemExclusiveMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiSystemExclusiveMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl ::windows::runtime::RuntimeName for MidiSystemExclusiveMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemExclusiveMessage";
}
impl ::core::convert::From<MidiSystemExclusiveMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiSystemExclusiveMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiSystemExclusiveMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSystemExclusiveMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiSystemExclusiveMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiSystemExclusiveMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiSystemExclusiveMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSystemExclusiveMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiSystemExclusiveMessage> for IMidiMessage {
    fn from(value: MidiSystemExclusiveMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSystemExclusiveMessage> for IMidiMessage {
    fn from(value: &MidiSystemExclusiveMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiSystemExclusiveMessage {}
unsafe impl ::core::marker::Sync for MidiSystemExclusiveMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiSystemResetMessage(pub ::windows::runtime::IInspectable);
impl MidiSystemResetMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSystemResetMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSystemResetMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSystemResetMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiSystemResetMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl ::windows::runtime::RuntimeName for MidiSystemResetMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemResetMessage";
}
impl ::core::convert::From<MidiSystemResetMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiSystemResetMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiSystemResetMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSystemResetMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiSystemResetMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiSystemResetMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiSystemResetMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSystemResetMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiSystemResetMessage> for IMidiMessage {
    fn from(value: MidiSystemResetMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSystemResetMessage> for IMidiMessage {
    fn from(value: &MidiSystemResetMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiSystemResetMessage {}
unsafe impl ::core::marker::Sync for MidiSystemResetMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiTimeCodeMessage(pub ::windows::runtime::IInspectable);
impl MidiTimeCodeMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn FrameType(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Values(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiTimeCodeMessage(frametype: u8, values: u8) -> ::windows::runtime::Result<MidiTimeCodeMessage> {
        Self::IMidiTimeCodeMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), frametype, values, &mut result__).from_abi::<MidiTimeCodeMessage>(result__)
        })
    }
    pub fn IMidiTimeCodeMessageFactory<R, F: FnOnce(&IMidiTimeCodeMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiTimeCodeMessage, IMidiTimeCodeMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiTimeCodeMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTimeCodeMessage;{0bf7087d-fa63-4a1c-8deb-c0e87796a6d7})");
}
unsafe impl ::windows::runtime::Interface for MidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0bf7087d_fa63_4a1c_8deb_c0e87796a6d7);
}
impl ::windows::runtime::RuntimeName for MidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimeCodeMessage";
}
impl ::core::convert::From<MidiTimeCodeMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiTimeCodeMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiTimeCodeMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiTimeCodeMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiTimeCodeMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiTimeCodeMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiTimeCodeMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiTimeCodeMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MidiTimeCodeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiTimeCodeMessage) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiTimeCodeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiTimeCodeMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiTimeCodeMessage {}
unsafe impl ::core::marker::Sync for MidiTimeCodeMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiTimingClockMessage(pub ::windows::runtime::IInspectable);
impl MidiTimingClockMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiTimingClockMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiTimingClockMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTimingClockMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiTimingClockMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl ::windows::runtime::RuntimeName for MidiTimingClockMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimingClockMessage";
}
impl ::core::convert::From<MidiTimingClockMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiTimingClockMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiTimingClockMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiTimingClockMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiTimingClockMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiTimingClockMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiTimingClockMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiTimingClockMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiTimingClockMessage> for IMidiMessage {
    fn from(value: MidiTimingClockMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiTimingClockMessage> for IMidiMessage {
    fn from(value: &MidiTimingClockMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiTimingClockMessage {}
unsafe impl ::core::marker::Sync for MidiTimingClockMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MidiTuneRequestMessage(pub ::windows::runtime::IInspectable);
impl MidiTuneRequestMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiTuneRequestMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiTuneRequestMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTuneRequestMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiTuneRequestMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
impl ::windows::runtime::RuntimeName for MidiTuneRequestMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTuneRequestMessage";
}
impl ::core::convert::From<MidiTuneRequestMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiTuneRequestMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MidiTuneRequestMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiTuneRequestMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MidiTuneRequestMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiTuneRequestMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MidiTuneRequestMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiTuneRequestMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MidiTuneRequestMessage> for IMidiMessage {
    fn from(value: MidiTuneRequestMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiTuneRequestMessage> for IMidiMessage {
    fn from(value: &MidiTuneRequestMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiTuneRequestMessage {}
unsafe impl ::core::marker::Sync for MidiTuneRequestMessage {}
