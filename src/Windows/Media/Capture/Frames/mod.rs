#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioMediaFrame(pub ::windows::runtime::IInspectable);
impl AudioMediaFrame {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn FrameReference(&self) -> ::windows::runtime::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Media_MediaProperties`*"]
    pub fn AudioEncodingProperties(&self) -> ::windows::runtime::Result<super::super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn GetAudioFrame(&self) -> ::windows::runtime::Result<super::super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::AudioFrame>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioMediaFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.AudioMediaFrame;{a3a9feff-8021-441b-9a46-e7f0137b7981})");
}
unsafe impl ::windows::runtime::Interface for AudioMediaFrame {
    type Vtable = IAudioMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa3a9feff_8021_441b_9a46_e7f0137b7981);
}
impl ::windows::runtime::RuntimeName for AudioMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.AudioMediaFrame";
}
impl ::core::convert::From<AudioMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: AudioMediaFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: &AudioMediaFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AudioMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: AudioMediaFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: &AudioMediaFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioMediaFrame {}
unsafe impl ::core::marker::Sync for AudioMediaFrame {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BufferMediaFrame(pub ::windows::runtime::IInspectable);
impl BufferMediaFrame {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn FrameReference(&self) -> ::windows::runtime::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Storage_Streams`*"]
    pub fn Buffer(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BufferMediaFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.BufferMediaFrame;{b5b153c7-9b84-4062-b79c-a365b2596854})");
}
unsafe impl ::windows::runtime::Interface for BufferMediaFrame {
    type Vtable = IBufferMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb5b153c7_9b84_4062_b79c_a365b2596854);
}
impl ::windows::runtime::RuntimeName for BufferMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.BufferMediaFrame";
}
impl ::core::convert::From<BufferMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: BufferMediaFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BufferMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: &BufferMediaFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BufferMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BufferMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BufferMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: BufferMediaFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BufferMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: &BufferMediaFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BufferMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BufferMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BufferMediaFrame {}
unsafe impl ::core::marker::Sync for BufferMediaFrame {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DepthMediaFrame(pub ::windows::runtime::IInspectable);
impl DepthMediaFrame {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn FrameReference(&self) -> ::windows::runtime::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn VideoMediaFrame(&self) -> ::windows::runtime::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn DepthFormat(&self) -> ::windows::runtime::Result<DepthMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DepthMediaFrameFormat>(result__)
        }
    }
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Media_Capture_Frames`, `Media_Devices_Core`, `Perception_Spatial`*"]
    pub fn TryCreateCoordinateMapper<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Core::CameraIntrinsics>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, cameraintrinsics: Param0, coordinatesystem: Param1) -> ::windows::runtime::Result<super::super::Devices::Core::DepthCorrelatedCoordinateMapper> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cameraintrinsics.into_param().abi(), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::Devices::Core::DepthCorrelatedCoordinateMapper>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn MaxReliableDepth(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IDepthMediaFrame2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn MinReliableDepth(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IDepthMediaFrame2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DepthMediaFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.DepthMediaFrame;{47135e4f-8549-45c0-925b-80d35efdb10a})");
}
unsafe impl ::windows::runtime::Interface for DepthMediaFrame {
    type Vtable = IDepthMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x47135e4f_8549_45c0_925b_80d35efdb10a);
}
impl ::windows::runtime::RuntimeName for DepthMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.DepthMediaFrame";
}
impl ::core::convert::From<DepthMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: DepthMediaFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DepthMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: &DepthMediaFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DepthMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DepthMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DepthMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: DepthMediaFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DepthMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: &DepthMediaFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DepthMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DepthMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DepthMediaFrame {}
unsafe impl ::core::marker::Sync for DepthMediaFrame {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DepthMediaFrameFormat(pub ::windows::runtime::IInspectable);
impl DepthMediaFrameFormat {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn VideoFormat(&self) -> ::windows::runtime::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn DepthScaleInMeters(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DepthMediaFrameFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.DepthMediaFrameFormat;{c312cf40-d729-453e-8780-2e04f140d28e})");
}
unsafe impl ::windows::runtime::Interface for DepthMediaFrameFormat {
    type Vtable = IDepthMediaFrameFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc312cf40_d729_453e_8780_2e04f140d28e);
}
impl ::windows::runtime::RuntimeName for DepthMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.DepthMediaFrameFormat";
}
impl ::core::convert::From<DepthMediaFrameFormat> for ::windows::runtime::IUnknown {
    fn from(value: DepthMediaFrameFormat) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DepthMediaFrameFormat> for ::windows::runtime::IUnknown {
    fn from(value: &DepthMediaFrameFormat) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DepthMediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DepthMediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DepthMediaFrameFormat> for ::windows::runtime::IInspectable {
    fn from(value: DepthMediaFrameFormat) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DepthMediaFrameFormat> for ::windows::runtime::IInspectable {
    fn from(value: &DepthMediaFrameFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DepthMediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DepthMediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DepthMediaFrameFormat {}
unsafe impl ::core::marker::Sync for DepthMediaFrameFormat {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioMediaFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioMediaFrame {
    type Vtable = IAudioMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa3a9feff_8021_441b_9a46_e7f0137b7981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMediaFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBufferMediaFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBufferMediaFrame {
    type Vtable = IBufferMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb5b153c7_9b84_4062_b79c_a365b2596854);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferMediaFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDepthMediaFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDepthMediaFrame {
    type Vtable = IDepthMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x47135e4f_8549_45c0_925b_80d35efdb10a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cameraintrinsics: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "Perception_Spatial")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDepthMediaFrame2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDepthMediaFrame2 {
    type Vtable = IDepthMediaFrame2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6cca473d_c4a4_4176_b0cd_33eae3b35aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrame2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDepthMediaFrameFormat(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDepthMediaFrameFormat {
    type Vtable = IDepthMediaFrameFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc312cf40_d729_453e_8780_2e04f140d28e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrameFormat_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInfraredMediaFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInfraredMediaFrame {
    type Vtable = IInfraredMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3fd13503_004b_4f0e_91ac_465299b41658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfraredMediaFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameArrivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameArrivedEventArgs {
    type Vtable = IMediaFrameArrivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0b430add_a490_4435_ada1_9affd55239f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameArrivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameFormat(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameFormat {
    type Vtable = IMediaFrameFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x71902b4e_b279_4a97_a9db_bd5a2fb78f39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameFormat_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameFormat2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameFormat2 {
    type Vtable = IMediaFrameFormat2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x63856340_5e87_4c10_86d1_6df097a6c6a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameFormat2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameReader(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameReader {
    type Vtable = IMediaFrameReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe4c94395_2028_48ed_90b0_d1c1b162e24c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameReader2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameReader2 {
    type Vtable = IMediaFrameReader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x871127b3_8531_4050_87cc_a13733cf3e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaFrameReaderAcquisitionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameReference(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameReference {
    type Vtable = IMediaFrameReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf6b88641_f0dc_4044_8dc9_961cedd05bad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaFrameSourceKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameReference2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameReference2 {
    type Vtable = IMediaFrameReference2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xddbc3ecc_d5b2_49ef_836a_947d989b80c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReference2_abi(
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
pub struct IMediaFrameSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSource {
    type Vtable = IMediaFrameSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd6782953_90db_46a8_8add_2aa884a8d253);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Devices_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceController(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceController {
    type Vtable = IMediaFrameSourceController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6d076635_316d_4b8f_b7b6_eeb04a8c6525);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertyvalue: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceController2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceController2 {
    type Vtable = IMediaFrameSourceController2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xefc49fd4_fcf2_4a03_b4e4_ac9628739bee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceController2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceController3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceController3 {
    type Vtable = IMediaFrameSourceController3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1f0cf815_2464_4651_b1e8_4a82dbdb54de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceController3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceGetPropertyResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceGetPropertyResult {
    type Vtable = IMediaFrameSourceGetPropertyResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x088616c2_3a64_4bd5_bd2b_e7c898d2f37a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGetPropertyResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaFrameSourceGetPropertyStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceGroup(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceGroup {
    type Vtable = IMediaFrameSourceGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7f605b87_4832_4b5f_ae3d_412faab37d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceGroupStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceGroupStatics {
    type Vtable = IMediaFrameSourceGroupStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1c48bfc5_436f_4508_94cf_d5d8b7326445);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGroupStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceInfo {
    type Vtable = IMediaFrameSourceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x87bdc9cd_4601_408f_91cf_038318cd0af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::MediaStreamType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaFrameSourceKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceInfo2 {
    type Vtable = IMediaFrameSourceInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x195a7855_6457_42c6_a769_19b65bd32e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrameSourceInfo3 {
    type Vtable = IMediaFrameSourceInfo3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca824ab6_66ea_5885_a2b6_26c0eeec3c7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayregion: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Enumeration::Panel) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameArrivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMultiSourceMediaFrameArrivedEventArgs {
    type Vtable = IMultiSourceMediaFrameArrivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x63115e01_cf51_48fd_aab0_6d693eb48127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameArrivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReader(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMultiSourceMediaFrameReader {
    type Vtable = IMultiSourceMediaFrameReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8d144402_f763_488d_98f2_b437bcf075e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReader2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMultiSourceMediaFrameReader2 {
    type Vtable = IMultiSourceMediaFrameReader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xef5c8abd_fc5c_4c6b_9d81_3cb9cc637c26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaFrameReaderAcquisitionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReference(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMultiSourceMediaFrameReference {
    type Vtable = IMultiSourceMediaFrameReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x21964b1a_7fe2_44d6_92e5_298e6d2810e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoMediaFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoMediaFrame {
    type Vtable = IVideoMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00dd4ccb_32bd_4fe1_a013_7cc13cf5dbcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoMediaFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(feature = "Media_Devices_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoMediaFrameFormat(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoMediaFrameFormat {
    type Vtable = IVideoMediaFrameFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x46027fc0_d71b_45c7_8f14_6d9a0ae604e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoMediaFrameFormat_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InfraredMediaFrame(pub ::windows::runtime::IInspectable);
impl InfraredMediaFrame {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn FrameReference(&self) -> ::windows::runtime::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn VideoMediaFrame(&self) -> ::windows::runtime::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn IsIlluminated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InfraredMediaFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.InfraredMediaFrame;{3fd13503-004b-4f0e-91ac-465299b41658})");
}
unsafe impl ::windows::runtime::Interface for InfraredMediaFrame {
    type Vtable = IInfraredMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3fd13503_004b_4f0e_91ac_465299b41658);
}
impl ::windows::runtime::RuntimeName for InfraredMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.InfraredMediaFrame";
}
impl ::core::convert::From<InfraredMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: InfraredMediaFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InfraredMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: &InfraredMediaFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InfraredMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InfraredMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InfraredMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: InfraredMediaFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InfraredMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: &InfraredMediaFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InfraredMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InfraredMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InfraredMediaFrame {}
unsafe impl ::core::marker::Sync for InfraredMediaFrame {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameArrivedEventArgs(pub ::windows::runtime::IInspectable);
impl MediaFrameArrivedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameArrivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs;{0b430add-a490-4435-ada1-9affd55239f7})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameArrivedEventArgs {
    type Vtable = IMediaFrameArrivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0b430add_a490_4435_ada1_9affd55239f7);
}
impl ::windows::runtime::RuntimeName for MediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs";
}
impl ::core::convert::From<MediaFrameArrivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameArrivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameArrivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameArrivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameArrivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameArrivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameArrivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameArrivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaFrameArrivedEventArgs {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameFormat(pub ::windows::runtime::IInspectable);
impl MediaFrameFormat {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn MajorType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Subtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Media_MediaProperties`*"]
    pub fn FrameRate(&self) -> ::windows::runtime::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn VideoFormat(&self) -> ::windows::runtime::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Media_MediaProperties`*"]
    pub fn AudioEncodingProperties(&self) -> ::windows::runtime::Result<super::super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameFormat2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameFormat;{71902b4e-b279-4a97-a9db-bd5a2fb78f39})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameFormat {
    type Vtable = IMediaFrameFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x71902b4e_b279_4a97_a9db_bd5a2fb78f39);
}
impl ::windows::runtime::RuntimeName for MediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameFormat";
}
impl ::core::convert::From<MediaFrameFormat> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameFormat) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameFormat> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameFormat) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameFormat> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameFormat) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameFormat> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaFrameFormat {}
unsafe impl ::core::marker::Sync for MediaFrameFormat {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameReader(pub ::windows::runtime::IInspectable);
impl MediaFrameReader {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn FrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn RemoveFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn TryAcquireLatestFrame(&self) -> ::windows::runtime::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameReaderStartStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameReaderStartStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn StopAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameReader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn AcquisitionMode(&self) -> ::windows::runtime::Result<MediaFrameReaderAcquisitionMode> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameReader2>(self)?;
        unsafe {
            let mut result__: MediaFrameReaderAcquisitionMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReaderAcquisitionMode>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameReader;{e4c94395-2028-48ed-90b0-d1c1b162e24c})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameReader {
    type Vtable = IMediaFrameReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe4c94395_2028_48ed_90b0_d1c1b162e24c);
}
impl ::windows::runtime::RuntimeName for MediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameReader";
}
impl ::core::convert::From<MediaFrameReader> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameReader> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameReader> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameReader> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaFrameReader) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaFrameReader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for MediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &MediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaFrameReader {}
unsafe impl ::core::marker::Sync for MediaFrameReader {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaFrameReaderAcquisitionMode(pub i32);
impl MediaFrameReaderAcquisitionMode {
    pub const Realtime: MediaFrameReaderAcquisitionMode = MediaFrameReaderAcquisitionMode(0i32);
    pub const Buffered: MediaFrameReaderAcquisitionMode = MediaFrameReaderAcquisitionMode(1i32);
}
impl ::core::convert::From<i32> for MediaFrameReaderAcquisitionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaFrameReaderAcquisitionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameReaderAcquisitionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameReaderAcquisitionMode;i4)");
}
impl ::windows::runtime::DefaultType for MediaFrameReaderAcquisitionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaFrameReaderStartStatus(pub i32);
impl MediaFrameReaderStartStatus {
    pub const Success: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(0i32);
    pub const UnknownFailure: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(1i32);
    pub const DeviceNotAvailable: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(2i32);
    pub const OutputFormatNotSupported: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(3i32);
    pub const ExclusiveControlNotAvailable: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(4i32);
}
impl ::core::convert::From<i32> for MediaFrameReaderStartStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaFrameReaderStartStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameReaderStartStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameReaderStartStatus;i4)");
}
impl ::windows::runtime::DefaultType for MediaFrameReaderStartStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameReference(pub ::windows::runtime::IInspectable);
impl MediaFrameReference {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn SourceKind(&self) -> ::windows::runtime::Result<MediaFrameSourceKind> {
        let this = self;
        unsafe {
            let mut result__: MediaFrameSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceKind>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn SystemRelativeTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn BufferMediaFrame(&self) -> ::windows::runtime::Result<BufferMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BufferMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn VideoMediaFrame(&self) -> ::windows::runtime::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrame>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Perception_Spatial`*"]
    pub fn CoordinateSystem(&self) -> ::windows::runtime::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::Spatial::SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn AudioMediaFrame(&self) -> ::windows::runtime::Result<AudioMediaFrame> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameReference2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioMediaFrame>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameReference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameReference;{f6b88641-f0dc-4044-8dc9-961cedd05bad})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameReference {
    type Vtable = IMediaFrameReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf6b88641_f0dc_4044_8dc9_961cedd05bad);
}
impl ::windows::runtime::RuntimeName for MediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameReference";
}
impl ::core::convert::From<MediaFrameReference> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameReference> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameReference> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameReference> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaFrameReference> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaFrameReference) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaFrameReference> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaFrameReference) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for MediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &MediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaFrameReference {}
unsafe impl ::core::marker::Sync for MediaFrameReference {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameSource(pub ::windows::runtime::IInspectable);
impl MediaFrameSource {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Info(&self) -> ::windows::runtime::Result<MediaFrameSourceInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceInfo>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Controller(&self) -> ::windows::runtime::Result<MediaFrameSourceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceController>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation_Collections`*"]
    pub fn SupportedFormats(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<MediaFrameFormat>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn CurrentFormat(&self) -> ::windows::runtime::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn SetFormatAsync<'a, Param0: ::windows::runtime::IntoParam<'a, MediaFrameFormat>>(&self, format: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), format.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn FormatChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn RemoveFormatChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Devices_Core")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Media_Devices_Core`*"]
    pub fn TryGetCameraIntrinsics<'a, Param0: ::windows::runtime::IntoParam<'a, MediaFrameFormat>>(&self, format: Param0) -> ::windows::runtime::Result<super::super::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), format.into_param().abi(), &mut result__).from_abi::<super::super::Devices::Core::CameraIntrinsics>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSource;{d6782953-90db-46a8-8add-2aa884a8d253})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameSource {
    type Vtable = IMediaFrameSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd6782953_90db_46a8_8add_2aa884a8d253);
}
impl ::windows::runtime::RuntimeName for MediaFrameSource {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSource";
}
impl ::core::convert::From<MediaFrameSource> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameSource> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameSource> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameSource> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaFrameSource {}
unsafe impl ::core::marker::Sync for MediaFrameSource {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameSourceController(pub ::windows::runtime::IInspectable);
impl MediaFrameSourceController {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn GetPropertyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn SetPropertyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, propertyid: Param0, propertyvalue: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertyid.into_param().abi(), propertyvalue.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Media_Devices`*"]
    pub fn VideoDeviceController(&self) -> ::windows::runtime::Result<super::super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::VideoDeviceController>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn GetPropertyByExtendedIdAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, extendedpropertyid: &[<u8 as ::windows::runtime::DefaultType>::DefaultType], maxpropertyvaluesize: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameSourceController2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), extendedpropertyid.len() as u32, ::core::mem::transmute(extendedpropertyid.as_ptr()), maxpropertyvaluesize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn SetPropertyByExtendedIdAsync(&self, extendedpropertyid: &[<u8 as ::windows::runtime::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameSourceController2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), extendedpropertyid.len() as u32, ::core::mem::transmute(extendedpropertyid.as_ptr()), propertyvalue.len() as u32, ::core::mem::transmute(propertyvalue.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Media_Devices`*"]
    pub fn AudioDeviceController(&self) -> ::windows::runtime::Result<super::super::Devices::AudioDeviceController> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameSourceController3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::AudioDeviceController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameSourceController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceController;{6d076635-316d-4b8f-b7b6-eeb04a8c6525})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameSourceController {
    type Vtable = IMediaFrameSourceController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6d076635_316d_4b8f_b7b6_eeb04a8c6525);
}
impl ::windows::runtime::RuntimeName for MediaFrameSourceController {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceController";
}
impl ::core::convert::From<MediaFrameSourceController> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameSourceController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameSourceController> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameSourceController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameSourceController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameSourceController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameSourceController> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameSourceController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameSourceController> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameSourceController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameSourceController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameSourceController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceController {}
unsafe impl ::core::marker::Sync for MediaFrameSourceController {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameSourceGetPropertyResult(pub ::windows::runtime::IInspectable);
impl MediaFrameSourceGetPropertyResult {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<MediaFrameSourceGetPropertyStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaFrameSourceGetPropertyStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceGetPropertyStatus>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameSourceGetPropertyResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult;{088616c2-3a64-4bd5-bd2b-e7c898d2f37a})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameSourceGetPropertyResult {
    type Vtable = IMediaFrameSourceGetPropertyResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x088616c2_3a64_4bd5_bd2b_e7c898d2f37a);
}
impl ::windows::runtime::RuntimeName for MediaFrameSourceGetPropertyResult {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult";
}
impl ::core::convert::From<MediaFrameSourceGetPropertyResult> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameSourceGetPropertyResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameSourceGetPropertyResult> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameSourceGetPropertyResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameSourceGetPropertyResult> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameSourceGetPropertyResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameSourceGetPropertyResult> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameSourceGetPropertyResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceGetPropertyResult {}
unsafe impl ::core::marker::Sync for MediaFrameSourceGetPropertyResult {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyStatus(pub i32);
impl MediaFrameSourceGetPropertyStatus {
    pub const Success: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(0i32);
    pub const UnknownFailure: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(1i32);
    pub const NotSupported: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(2i32);
    pub const DeviceNotAvailable: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(3i32);
    pub const MaxPropertyValueSizeTooSmall: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(4i32);
    pub const MaxPropertyValueSizeRequired: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(5i32);
}
impl ::core::convert::From<i32> for MediaFrameSourceGetPropertyStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaFrameSourceGetPropertyStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameSourceGetPropertyStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyStatus;i4)");
}
impl ::windows::runtime::DefaultType for MediaFrameSourceGetPropertyStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameSourceGroup(pub ::windows::runtime::IInspectable);
impl MediaFrameSourceGroup {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation_Collections`*"]
    pub fn SourceInfos(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceInfo>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsync() -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceGroup>>> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceGroup>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(id: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGroup>> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGroup>>(result__)
        })
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IMediaFrameSourceGroupStatics<R, F: FnOnce(&IMediaFrameSourceGroupStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaFrameSourceGroup, IMediaFrameSourceGroupStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameSourceGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceGroup;{7f605b87-4832-4b5f-ae3d-412faab37d34})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameSourceGroup {
    type Vtable = IMediaFrameSourceGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7f605b87_4832_4b5f_ae3d_412faab37d34);
}
impl ::windows::runtime::RuntimeName for MediaFrameSourceGroup {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceGroup";
}
impl ::core::convert::From<MediaFrameSourceGroup> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameSourceGroup) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameSourceGroup> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameSourceGroup) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameSourceGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameSourceGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameSourceGroup> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameSourceGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameSourceGroup> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameSourceGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameSourceGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameSourceGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceGroup {}
unsafe impl ::core::marker::Sync for MediaFrameSourceGroup {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaFrameSourceInfo(pub ::windows::runtime::IInspectable);
impl MediaFrameSourceInfo {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn MediaStreamType(&self) -> ::windows::runtime::Result<super::MediaStreamType> {
        let this = self;
        unsafe {
            let mut result__: super::MediaStreamType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaStreamType>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn SourceKind(&self) -> ::windows::runtime::Result<MediaFrameSourceKind> {
        let this = self;
        unsafe {
            let mut result__: MediaFrameSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceKind>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn SourceGroup(&self) -> ::windows::runtime::Result<MediaFrameSourceGroup> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameSourceGroup>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<super::super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Perception_Spatial`*"]
    pub fn CoordinateSystem(&self) -> ::windows::runtime::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::Spatial::SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn ProfileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameSourceInfo2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation_Collections`*"]
    pub fn VideoProfileMediaDescription(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameSourceInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))]
    #[doc = "*Required features: `Media_Capture_Frames`, `Devices_Enumeration`, `UI_WindowManagement`*"]
    pub fn GetRelativePanel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::WindowManagement::DisplayRegion>>(&self, displayregion: Param0) -> ::windows::runtime::Result<super::super::super::Devices::Enumeration::Panel> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrameSourceInfo3>(self)?;
        unsafe {
            let mut result__: super::super::super::Devices::Enumeration::Panel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), displayregion.into_param().abi(), &mut result__).from_abi::<super::super::super::Devices::Enumeration::Panel>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameSourceInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceInfo;{87bdc9cd-4601-408f-91cf-038318cd0af3})");
}
unsafe impl ::windows::runtime::Interface for MediaFrameSourceInfo {
    type Vtable = IMediaFrameSourceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x87bdc9cd_4601_408f_91cf_038318cd0af3);
}
impl ::windows::runtime::RuntimeName for MediaFrameSourceInfo {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceInfo";
}
impl ::core::convert::From<MediaFrameSourceInfo> for ::windows::runtime::IUnknown {
    fn from(value: MediaFrameSourceInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaFrameSourceInfo> for ::windows::runtime::IUnknown {
    fn from(value: &MediaFrameSourceInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaFrameSourceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaFrameSourceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaFrameSourceInfo> for ::windows::runtime::IInspectable {
    fn from(value: MediaFrameSourceInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaFrameSourceInfo> for ::windows::runtime::IInspectable {
    fn from(value: &MediaFrameSourceInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaFrameSourceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaFrameSourceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceInfo {}
unsafe impl ::core::marker::Sync for MediaFrameSourceInfo {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaFrameSourceKind(pub i32);
impl MediaFrameSourceKind {
    pub const Custom: MediaFrameSourceKind = MediaFrameSourceKind(0i32);
    pub const Color: MediaFrameSourceKind = MediaFrameSourceKind(1i32);
    pub const Infrared: MediaFrameSourceKind = MediaFrameSourceKind(2i32);
    pub const Depth: MediaFrameSourceKind = MediaFrameSourceKind(3i32);
    pub const Audio: MediaFrameSourceKind = MediaFrameSourceKind(4i32);
    pub const Image: MediaFrameSourceKind = MediaFrameSourceKind(5i32);
    pub const Metadata: MediaFrameSourceKind = MediaFrameSourceKind(6i32);
}
impl ::core::convert::From<i32> for MediaFrameSourceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaFrameSourceKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameSourceKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceKind;i4)");
}
impl ::windows::runtime::DefaultType for MediaFrameSourceKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaFrameSourceSetPropertyStatus(pub i32);
impl MediaFrameSourceSetPropertyStatus {
    pub const Success: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(0i32);
    pub const UnknownFailure: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(1i32);
    pub const NotSupported: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(2i32);
    pub const InvalidValue: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(3i32);
    pub const DeviceNotAvailable: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(4i32);
    pub const NotInControl: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(5i32);
}
impl ::core::convert::From<i32> for MediaFrameSourceSetPropertyStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaFrameSourceSetPropertyStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaFrameSourceSetPropertyStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceSetPropertyStatus;i4)");
}
impl ::windows::runtime::DefaultType for MediaFrameSourceSetPropertyStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MultiSourceMediaFrameArrivedEventArgs(pub ::windows::runtime::IInspectable);
impl MultiSourceMediaFrameArrivedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for MultiSourceMediaFrameArrivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs;{63115e01-cf51-48fd-aab0-6d693eb48127})");
}
unsafe impl ::windows::runtime::Interface for MultiSourceMediaFrameArrivedEventArgs {
    type Vtable = IMultiSourceMediaFrameArrivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x63115e01_cf51_48fd_aab0_6d693eb48127);
}
impl ::windows::runtime::RuntimeName for MultiSourceMediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs";
}
impl ::core::convert::From<MultiSourceMediaFrameArrivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MultiSourceMediaFrameArrivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameArrivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MultiSourceMediaFrameArrivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MultiSourceMediaFrameArrivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MultiSourceMediaFrameArrivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameArrivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MultiSourceMediaFrameArrivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameArrivedEventArgs {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MultiSourceMediaFrameReader(pub ::windows::runtime::IInspectable);
impl MultiSourceMediaFrameReader {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn FrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn RemoveFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn TryAcquireLatestFrame(&self) -> ::windows::runtime::Result<MultiSourceMediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MultiSourceMediaFrameReference>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn StopAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMultiSourceMediaFrameReader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn AcquisitionMode(&self) -> ::windows::runtime::Result<MediaFrameReaderAcquisitionMode> {
        let this = &::windows::runtime::Interface::cast::<IMultiSourceMediaFrameReader2>(self)?;
        unsafe {
            let mut result__: MediaFrameReaderAcquisitionMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReaderAcquisitionMode>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MultiSourceMediaFrameReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameReader;{8d144402-f763-488d-98f2-b437bcf075e7})");
}
unsafe impl ::windows::runtime::Interface for MultiSourceMediaFrameReader {
    type Vtable = IMultiSourceMediaFrameReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8d144402_f763_488d_98f2_b437bcf075e7);
}
impl ::windows::runtime::RuntimeName for MultiSourceMediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameReader";
}
impl ::core::convert::From<MultiSourceMediaFrameReader> for ::windows::runtime::IUnknown {
    fn from(value: MultiSourceMediaFrameReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReader> for ::windows::runtime::IUnknown {
    fn from(value: &MultiSourceMediaFrameReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MultiSourceMediaFrameReader> for ::windows::runtime::IInspectable {
    fn from(value: MultiSourceMediaFrameReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReader> for ::windows::runtime::IInspectable {
    fn from(value: &MultiSourceMediaFrameReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MultiSourceMediaFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MultiSourceMediaFrameReader) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MultiSourceMediaFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MultiSourceMediaFrameReader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameReader {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameReader {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MultiSourceMediaFrameReaderStartStatus(pub i32);
impl MultiSourceMediaFrameReaderStartStatus {
    pub const Success: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(0i32);
    pub const NotSupported: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(1i32);
    pub const InsufficientResources: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(2i32);
    pub const DeviceNotAvailable: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(3i32);
    pub const UnknownFailure: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(4i32);
}
impl ::core::convert::From<i32> for MultiSourceMediaFrameReaderStartStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MultiSourceMediaFrameReaderStartStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MultiSourceMediaFrameReaderStartStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MultiSourceMediaFrameReaderStartStatus;i4)");
}
impl ::windows::runtime::DefaultType for MultiSourceMediaFrameReaderStartStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MultiSourceMediaFrameReference(pub ::windows::runtime::IInspectable);
impl MultiSourceMediaFrameReference {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn TryGetFrameReferenceBySourceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, sourceid: Param0) -> ::windows::runtime::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sourceid.into_param().abi(), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MultiSourceMediaFrameReference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameReference;{21964b1a-7fe2-44d6-92e5-298e6d2810e9})");
}
unsafe impl ::windows::runtime::Interface for MultiSourceMediaFrameReference {
    type Vtable = IMultiSourceMediaFrameReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x21964b1a_7fe2_44d6_92e5_298e6d2810e9);
}
impl ::windows::runtime::RuntimeName for MultiSourceMediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameReference";
}
impl ::core::convert::From<MultiSourceMediaFrameReference> for ::windows::runtime::IUnknown {
    fn from(value: MultiSourceMediaFrameReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReference> for ::windows::runtime::IUnknown {
    fn from(value: &MultiSourceMediaFrameReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MultiSourceMediaFrameReference> for ::windows::runtime::IInspectable {
    fn from(value: MultiSourceMediaFrameReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReference> for ::windows::runtime::IInspectable {
    fn from(value: &MultiSourceMediaFrameReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MultiSourceMediaFrameReference> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MultiSourceMediaFrameReference) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MultiSourceMediaFrameReference> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MultiSourceMediaFrameReference) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameReference {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameReference {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoMediaFrame(pub ::windows::runtime::IInspectable);
impl VideoMediaFrame {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn FrameReference(&self) -> ::windows::runtime::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn VideoFormat(&self) -> ::windows::runtime::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoMediaFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Graphics_Imaging`*"]
    pub fn SoftwareBitmap(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Direct3DSurface(&self) -> ::windows::runtime::Result<super::super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[cfg(feature = "Media_Devices_Core")]
    #[doc = "*Required features: `Media_Capture_Frames`, `Media_Devices_Core`*"]
    pub fn CameraIntrinsics(&self) -> ::windows::runtime::Result<super::super::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn InfraredMediaFrame(&self) -> ::windows::runtime::Result<InfraredMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InfraredMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn DepthMediaFrame(&self) -> ::windows::runtime::Result<DepthMediaFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DepthMediaFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn GetVideoFrame(&self) -> ::windows::runtime::Result<super::super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::VideoFrame>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoMediaFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.VideoMediaFrame;{00dd4ccb-32bd-4fe1-a013-7cc13cf5dbcf})");
}
unsafe impl ::windows::runtime::Interface for VideoMediaFrame {
    type Vtable = IVideoMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00dd4ccb_32bd_4fe1_a013_7cc13cf5dbcf);
}
impl ::windows::runtime::RuntimeName for VideoMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.VideoMediaFrame";
}
impl ::core::convert::From<VideoMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: VideoMediaFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: &VideoMediaFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VideoMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: VideoMediaFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: &VideoMediaFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VideoMediaFrame {}
unsafe impl ::core::marker::Sync for VideoMediaFrame {}
#[doc = "*Required features: `Media_Capture_Frames`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoMediaFrameFormat(pub ::windows::runtime::IInspectable);
impl VideoMediaFrameFormat {
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn MediaFrameFormat(&self) -> ::windows::runtime::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn DepthFormat(&self) -> ::windows::runtime::Result<DepthMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DepthMediaFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture_Frames`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoMediaFrameFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.VideoMediaFrameFormat;{46027fc0-d71b-45c7-8f14-6d9a0ae604e4})");
}
unsafe impl ::windows::runtime::Interface for VideoMediaFrameFormat {
    type Vtable = IVideoMediaFrameFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x46027fc0_d71b_45c7_8f14_6d9a0ae604e4);
}
impl ::windows::runtime::RuntimeName for VideoMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.VideoMediaFrameFormat";
}
impl ::core::convert::From<VideoMediaFrameFormat> for ::windows::runtime::IUnknown {
    fn from(value: VideoMediaFrameFormat) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoMediaFrameFormat> for ::windows::runtime::IUnknown {
    fn from(value: &VideoMediaFrameFormat) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoMediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VideoMediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoMediaFrameFormat> for ::windows::runtime::IInspectable {
    fn from(value: VideoMediaFrameFormat) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoMediaFrameFormat> for ::windows::runtime::IInspectable {
    fn from(value: &VideoMediaFrameFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoMediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoMediaFrameFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VideoMediaFrameFormat {}
unsafe impl ::core::marker::Sync for VideoMediaFrameFormat {}
