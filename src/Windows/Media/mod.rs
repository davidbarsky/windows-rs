#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Media_AppBroadcasting")]
pub mod AppBroadcasting;
#[cfg(feature = "Media_AppRecording")]
pub mod AppRecording;
#[cfg(feature = "Media_Audio")]
pub mod Audio;
#[cfg(feature = "Media_Capture")]
pub mod Capture;
#[cfg(feature = "Media_Casting")]
pub mod Casting;
#[cfg(feature = "Media_ClosedCaptioning")]
pub mod ClosedCaptioning;
#[cfg(feature = "Media_ContentRestrictions")]
pub mod ContentRestrictions;
#[cfg(feature = "Media_Control")]
pub mod Control;
#[cfg(feature = "Media_Core")]
pub mod Core;
#[cfg(feature = "Media_Devices")]
pub mod Devices;
#[cfg(feature = "Media_DialProtocol")]
pub mod DialProtocol;
#[cfg(feature = "Media_Editing")]
pub mod Editing;
#[cfg(feature = "Media_Effects")]
pub mod Effects;
#[cfg(feature = "Media_FaceAnalysis")]
pub mod FaceAnalysis;
#[cfg(feature = "Media_Import")]
pub mod Import;
#[cfg(feature = "Media_MediaProperties")]
pub mod MediaProperties;
#[cfg(feature = "Media_Miracast")]
pub mod Miracast;
#[cfg(feature = "Media_Ocr")]
pub mod Ocr;
#[cfg(feature = "Media_PlayTo")]
pub mod PlayTo;
#[cfg(feature = "Media_Playback")]
pub mod Playback;
#[cfg(feature = "Media_Playlists")]
pub mod Playlists;
#[cfg(feature = "Media_Protection")]
pub mod Protection;
#[cfg(feature = "Media_Render")]
pub mod Render;
#[cfg(feature = "Media_SpeechRecognition")]
pub mod SpeechRecognition;
#[cfg(feature = "Media_SpeechSynthesis")]
pub mod SpeechSynthesis;
#[cfg(feature = "Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Media_Transcoding")]
pub mod Transcoding;
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioBuffer(pub ::windows::runtime::IInspectable);
impl AudioBuffer {
    #[doc = "*Required features: `Media`*"]
    pub fn Capacity(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetLength(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IMemoryBufferReference>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioBuffer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AudioBuffer;{35175827-724b-4c6a-b130-f6537f9ae0d0})");
}
unsafe impl ::windows::runtime::Interface for AudioBuffer {
    type Vtable = IAudioBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35175827_724b_4c6a_b130_f6537f9ae0d0);
}
impl ::windows::runtime::RuntimeName for AudioBuffer {
    const NAME: &'static str = "Windows.Media.AudioBuffer";
}
impl ::core::convert::From<AudioBuffer> for ::windows::runtime::IUnknown {
    fn from(value: AudioBuffer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &AudioBuffer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AudioBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioBuffer> for ::windows::runtime::IInspectable {
    fn from(value: AudioBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioBuffer> for ::windows::runtime::IInspectable {
    fn from(value: &AudioBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioBuffer> for super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioBuffer) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioBuffer> for super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioBuffer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IClosable> for AudioBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IClosable> for &AudioBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioBuffer> for super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioBuffer) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioBuffer> for super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioBuffer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IMemoryBuffer> for AudioBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IMemoryBuffer> for &AudioBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioBuffer {}
unsafe impl ::core::marker::Sync for AudioBuffer {}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioBufferAccessMode(pub i32);
impl AudioBufferAccessMode {
    pub const Read: AudioBufferAccessMode = AudioBufferAccessMode(0i32);
    pub const ReadWrite: AudioBufferAccessMode = AudioBufferAccessMode(1i32);
    pub const Write: AudioBufferAccessMode = AudioBufferAccessMode(2i32);
}
impl ::core::convert::From<i32> for AudioBufferAccessMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioBufferAccessMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioBufferAccessMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.AudioBufferAccessMode;i4)");
}
impl ::windows::runtime::DefaultType for AudioBufferAccessMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioFrame(pub ::windows::runtime::IInspectable);
impl AudioFrame {
    #[doc = "*Required features: `Media`*"]
    pub fn LockBuffer(&self, mode: AudioBufferAccessMode) -> ::windows::runtime::Result<AudioBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<AudioBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetRelativeTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RelativeTime(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetSystemRelativeTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SystemRelativeTime(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsDiscontinuous(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn ExtendedProperties(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Create(capacity: u32) -> ::windows::runtime::Result<AudioFrame> {
        Self::IAudioFrameFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), capacity, &mut result__).from_abi::<AudioFrame>(result__)
        })
    }
    pub fn IAudioFrameFactory<R, F: FnOnce(&IAudioFrameFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AudioFrame, IAudioFrameFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AudioFrame;{e36ac304-aab2-4277-9ed0-43cedf8e29c6})");
}
unsafe impl ::windows::runtime::Interface for AudioFrame {
    type Vtable = IAudioFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe36ac304_aab2_4277_9ed0_43cedf8e29c6);
}
impl ::windows::runtime::RuntimeName for AudioFrame {
    const NAME: &'static str = "Windows.Media.AudioFrame";
}
impl ::core::convert::From<AudioFrame> for ::windows::runtime::IUnknown {
    fn from(value: AudioFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioFrame> for ::windows::runtime::IUnknown {
    fn from(value: &AudioFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AudioFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioFrame> for ::windows::runtime::IInspectable {
    fn from(value: AudioFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioFrame> for ::windows::runtime::IInspectable {
    fn from(value: &AudioFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioFrame> for super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFrame) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioFrame> for super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IClosable> for AudioFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IClosable> for &AudioFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrame> for IMediaFrame {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFrame) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrame> for IMediaFrame {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaFrame> for AudioFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaFrame> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaFrame> for &AudioFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaFrame> {
        ::core::convert::TryInto::<IMediaFrame>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFrame {}
unsafe impl ::core::marker::Sync for AudioFrame {}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioProcessing(pub i32);
impl AudioProcessing {
    pub const Default: AudioProcessing = AudioProcessing(0i32);
    pub const Raw: AudioProcessing = AudioProcessing(1i32);
}
impl ::core::convert::From<i32> for AudioProcessing {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioProcessing {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioProcessing {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.AudioProcessing;i4)");
}
impl ::windows::runtime::DefaultType for AudioProcessing {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AutoRepeatModeChangeRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl AutoRepeatModeChangeRequestedEventArgs {
    #[doc = "*Required features: `Media`*"]
    pub fn RequestedAutoRepeatMode(&self) -> ::windows::runtime::Result<MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackAutoRepeatMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackAutoRepeatMode>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutoRepeatModeChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AutoRepeatModeChangeRequestedEventArgs;{ea137efa-d852-438e-882b-c990109a78f4})");
}
unsafe impl ::windows::runtime::Interface for AutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea137efa_d852_438e_882b_c990109a78f4);
}
impl ::windows::runtime::RuntimeName for AutoRepeatModeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.AutoRepeatModeChangeRequestedEventArgs";
}
impl ::core::convert::From<AutoRepeatModeChangeRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AutoRepeatModeChangeRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutoRepeatModeChangeRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AutoRepeatModeChangeRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutoRepeatModeChangeRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AutoRepeatModeChangeRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutoRepeatModeChangeRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AutoRepeatModeChangeRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AutoRepeatModeChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AutoRepeatModeChangeRequestedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioBuffer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioBuffer {
    type Vtable = IAudioBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35175827_724b_4c6a_b130_f6537f9ae0d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioFrame {
    type Vtable = IAudioFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe36ac304_aab2_4277_9ed0_43cedf8e29c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: AudioBufferAccessMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioFrameFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioFrameFactory {
    type Vtable = IAudioFrameFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x91a90ade_2422_40a6_b9ad_30d02404317d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capacity: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutoRepeatModeChangeRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea137efa_d852_438e_882b_c990109a78f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoRepeatModeChangeRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageDisplayProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageDisplayProperties {
    type Vtable = IImageDisplayProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcd0bc7ef_54e7_411f_9933_f0e98b0a96d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDisplayProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaControl(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaControl {
    type Vtable = IMediaControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x98f1fbe1_7a8d_42cb_b6fe_8fe698264f13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SoundLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Media`*"]
pub struct IMediaExtension(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaExtension {
    type Vtable = IMediaExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x07915118_45df_442b_8a3f_f7826a6370ab);
}
impl IMediaExtension {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn SetProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMediaExtension {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{07915118-45df-442b-8a3f-f7826a6370ab}");
}
impl ::core::convert::From<IMediaExtension> for ::windows::runtime::IUnknown {
    fn from(value: IMediaExtension) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMediaExtension> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaExtension) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMediaExtension> for ::windows::runtime::IInspectable {
    fn from(value: IMediaExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaExtension> for ::windows::runtime::IInspectable {
    fn from(value: &IMediaExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMediaExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMediaExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtension_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaExtensionManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaExtensionManager {
    type Vtable = IMediaExtensionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4a25eaf5_242d_4dfb_97f4_69b7c42576ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtensionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputsubtype: ::windows::runtime::GUID, outputsubtype: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputsubtype: ::windows::runtime::GUID, outputsubtype: ::windows::runtime::GUID, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputsubtype: ::windows::runtime::GUID, outputsubtype: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputsubtype: ::windows::runtime::GUID, outputsubtype: ::windows::runtime::GUID, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputsubtype: ::windows::runtime::GUID, outputsubtype: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputsubtype: ::windows::runtime::GUID, outputsubtype: ::windows::runtime::GUID, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputsubtype: ::windows::runtime::GUID, outputsubtype: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputsubtype: ::windows::runtime::GUID, outputsubtype: ::windows::runtime::GUID, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaExtensionManager2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaExtensionManager2 {
    type Vtable = IMediaExtensionManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5bcebf47_4043_4fed_acaf_54ec29dfb1f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtensionManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_AppService")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extension: ::windows::runtime::RawPtr, connection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Media`*"]
pub struct IMediaFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaFrame {
    type Vtable = IMediaFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbfb52f8c_5943_47d8_8e10_05308aa5fbd0);
}
impl IMediaFrame {
    #[doc = "*Required features: `Media`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetRelativeTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RelativeTime(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetSystemRelativeTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SystemRelativeTime(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsDiscontinuous(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn ExtendedProperties(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMediaFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{bfb52f8c-5943-47d8-8e10-05308aa5fbd0}");
}
impl ::core::convert::From<IMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: IMediaFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMediaFrame> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: IMediaFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaFrame> for ::windows::runtime::IInspectable {
    fn from(value: &IMediaFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IMediaFrame> for super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IMediaFrame) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IMediaFrame> for super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IMediaFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IClosable> for IMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IClosable> for &IMediaFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Media`*"]
pub struct IMediaMarker(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaMarker {
    type Vtable = IMediaMarker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1803def8_dca5_4b6f_9c20_e3d3c0643625);
}
impl IMediaMarker {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Time(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn MediaMarkerType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMediaMarker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1803def8-dca5-4b6f-9c20-e3d3c0643625}");
}
impl ::core::convert::From<IMediaMarker> for ::windows::runtime::IUnknown {
    fn from(value: IMediaMarker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMediaMarker> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaMarker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaMarker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaMarker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMediaMarker> for ::windows::runtime::IInspectable {
    fn from(value: IMediaMarker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaMarker> for ::windows::runtime::IInspectable {
    fn from(value: &IMediaMarker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMediaMarker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMediaMarker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaMarkerTypesStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaMarkerTypesStatics {
    type Vtable = IMediaMarkerTypesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbb198040_482f_4743_8832_45853821ece0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarkerTypesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Media`*"]
pub struct IMediaMarkers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaMarkers {
    type Vtable = IMediaMarkers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xafeab189_f8dd_466e_aa10_920b52353fdf);
}
impl IMediaMarkers {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn Markers(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVectorView<IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<IMediaMarker>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMediaMarkers {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{afeab189-f8dd-466e-aa10-920b52353fdf}");
}
impl ::core::convert::From<IMediaMarkers> for ::windows::runtime::IUnknown {
    fn from(value: IMediaMarkers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMediaMarkers> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaMarkers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaMarkers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaMarkers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMediaMarkers> for ::windows::runtime::IInspectable {
    fn from(value: IMediaMarkers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaMarkers> for ::windows::runtime::IInspectable {
    fn from(value: &IMediaMarkers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMediaMarkers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMediaMarkers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarkers_abi(
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
pub struct IMediaProcessingTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeb8564ac_a351_4f4e_b4f0_9bf2408993db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProcessingTriggerDetails_abi(
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
pub struct IMediaTimelineController(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaTimelineController {
    type Vtable = IMediaTimelineController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8ed361f3_0b78_4360_bf71_0c841999ea1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaTimelineControllerState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, positionchangedeventhandler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statechangedeventhandler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaTimelineController2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaTimelineController2 {
    type Vtable = IMediaTimelineController2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xef74ea38_9e72_4df9_8355_6e90c81bbadd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineController2_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaTimelineControllerFailedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8821f81d_3e77_43fb_be26_4fc87a044834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineControllerFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMusicDisplayProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6bbf0c59_d0a0_4d26_92a0_f978e1d18e7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMusicDisplayProperties2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMusicDisplayProperties2 {
    type Vtable = IMusicDisplayProperties2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00368462_97d3_44b9_b00f_008afcefaf18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMusicDisplayProperties3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMusicDisplayProperties3 {
    type Vtable = IMusicDisplayProperties3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4db51ac1_0681_4e8c_9401_b8159d9eefc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaybackPositionChangeRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb4493f88_eb28_4961_9c14_335e44f3e125);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackPositionChangeRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2ce2c41f_3cd6_4f77_9ba7_eb27c26a2140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShuffleEnabledChangeRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x49b593fe_4fd0_4666_a314_c0e01940d302);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShuffleEnabledChangeRequestedEventArgs_abi(
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
pub struct ISystemMediaTransportControls(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x99fa3ff4_1742_42a6_902e_087d41f965ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaPlaybackStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaPlaybackStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SoundLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMediaTransportControls2 {
    type Vtable = ISystemMediaTransportControls2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea98d2f6_7f3c_4af2_a586_72889808efb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaPlaybackAutoRepeatMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timelineproperties: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb7f47116_a56f_4dc8_9e11_92031f4a87c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemMediaTransportControlsButton) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsDisplayUpdater(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8abbc53e_fa55_4ecf_ad8e_c984e5dd1550);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsDisplayUpdater_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaPlaybackType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaPlaybackType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: MediaPlaybackType, source: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd0ca0936_339b_4cb3_8eeb_737607f56e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemMediaTransportControlsProperty) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMediaTransportControlsStatics {
    type Vtable = ISystemMediaTransportControlsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x43ba380a_eca4_4832_91ab_d415fae484c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsStatics_abi(
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
pub struct ISystemMediaTransportControlsTimelineProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5125316a_c3a2_475b_8507_93534dc88f15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsTimelineProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoDisplayProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5609fdb1_5d2d_4872_8170_45dee5bc2f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDisplayProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoDisplayProperties2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoDisplayProperties2 {
    type Vtable = IVideoDisplayProperties2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb410e1ce_ab52_41ab_a486_cc10fab152f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDisplayProperties2_abi(
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
pub struct IVideoEffectsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEffectsStatics {
    type Vtable = IVideoEffectsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1fcda5e8_baf1_4521_980c_3bcebb44cf38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectsStatics_abi(
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
pub struct IVideoFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoFrame {
    type Vtable = IVideoFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0cc06625_90fc_4c92_bd95_7ded21819d1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frame: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoFrame2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoFrame2 {
    type Vtable = IVideoFrame2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3837840d_336c_4366_8d46_060798736c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrame2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frame: ::windows::runtime::RawPtr, sourcebounds: ::windows::runtime::RawPtr, destinationbounds: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoFrameFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoFrameFactory {
    type Vtable = IVideoFrameFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x014b6d69_2228_4c92_92ff_50c380d3e776);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoFrameStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoFrameStatics {
    type Vtable = IVideoFrameStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xab2a556f_6111_4b33_8ec3_2b209a02e17a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, surface: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ImageDisplayProperties(pub ::windows::runtime::IInspectable);
impl ImageDisplayProperties {
    #[doc = "*Required features: `Media`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Subtitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageDisplayProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.ImageDisplayProperties;{cd0bc7ef-54e7-411f-9933-f0e98b0a96d2})");
}
unsafe impl ::windows::runtime::Interface for ImageDisplayProperties {
    type Vtable = IImageDisplayProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcd0bc7ef_54e7_411f_9933_f0e98b0a96d2);
}
impl ::windows::runtime::RuntimeName for ImageDisplayProperties {
    const NAME: &'static str = "Windows.Media.ImageDisplayProperties";
}
impl ::core::convert::From<ImageDisplayProperties> for ::windows::runtime::IUnknown {
    fn from(value: ImageDisplayProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ImageDisplayProperties> for ::windows::runtime::IUnknown {
    fn from(value: &ImageDisplayProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ImageDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ImageDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ImageDisplayProperties> for ::windows::runtime::IInspectable {
    fn from(value: ImageDisplayProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ImageDisplayProperties> for ::windows::runtime::IInspectable {
    fn from(value: &ImageDisplayProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ImageDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ImageDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ImageDisplayProperties {}
unsafe impl ::core::marker::Sync for ImageDisplayProperties {}
#[doc = "*Required features: `Media`*"]
pub struct MediaControl {}
impl MediaControl {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SoundLevelChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveSoundLevelChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn PlayPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemovePlayPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn PausePressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemovePausePressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn StopPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveStopPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn PlayPauseTogglePressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemovePlayPauseTogglePressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RecordPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveRecordPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn NextTrackPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveNextTrackPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn PreviousTrackPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemovePreviousTrackPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn FastForwardPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveFastForwardPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RewindPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveRewindPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn ChannelUpPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveChannelUpPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn ChannelDownPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveChannelDownPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media`*"]
    pub fn SoundLevel() -> ::windows::runtime::Result<SoundLevel> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: SoundLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SoundLevel>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media`*"]
    pub fn SetTrackName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media`*"]
    pub fn TrackName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media`*"]
    pub fn SetArtistName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media`*"]
    pub fn ArtistName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsPlaying(value: bool) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media`*"]
    pub fn IsPlaying() -> ::windows::runtime::Result<bool> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetAlbumArt<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn AlbumArt() -> ::windows::runtime::Result<super::Foundation::Uri> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        })
    }
    pub fn IMediaControl<R, F: FnOnce(&IMediaControl) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaControl, IMediaControl> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MediaControl {
    const NAME: &'static str = "Windows.Media.MediaControl";
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct MediaControlContract(pub u8);
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaExtensionManager(pub ::windows::runtime::IInspectable);
impl MediaExtensionManager {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaExtensionManager, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn RegisterSchemeHandler<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activatableclassid: Param0, scheme: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), scheme.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn RegisterSchemeHandlerWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, scheme: Param1, configuration: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), scheme.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn RegisterByteStreamHandler<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activatableclassid: Param0, fileextension: Param1, mimetype: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), fileextension.into_param().abi(), mimetype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn RegisterByteStreamHandlerWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, fileextension: Param1, mimetype: Param2, configuration: Param3) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), fileextension.into_param().abi(), mimetype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn RegisterAudioDecoder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn RegisterAudioDecoderWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn RegisterAudioEncoder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn RegisterAudioEncoderWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn RegisterVideoDecoder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn RegisterVideoDecoderWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn RegisterVideoEncoder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn RegisterVideoEncoderWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    #[doc = "*Required features: `Media`, `ApplicationModel_AppService`*"]
    pub fn RegisterMediaExtensionForAppService<'a, Param0: ::windows::runtime::IntoParam<'a, IMediaExtension>, Param1: ::windows::runtime::IntoParam<'a, super::ApplicationModel::AppService::AppServiceConnection>>(&self, extension: Param0, connection: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaExtensionManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), extension.into_param().abi(), connection.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaExtensionManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaExtensionManager;{4a25eaf5-242d-4dfb-97f4-69b7c42576ff})");
}
unsafe impl ::windows::runtime::Interface for MediaExtensionManager {
    type Vtable = IMediaExtensionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4a25eaf5_242d_4dfb_97f4_69b7c42576ff);
}
impl ::windows::runtime::RuntimeName for MediaExtensionManager {
    const NAME: &'static str = "Windows.Media.MediaExtensionManager";
}
impl ::core::convert::From<MediaExtensionManager> for ::windows::runtime::IUnknown {
    fn from(value: MediaExtensionManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaExtensionManager> for ::windows::runtime::IUnknown {
    fn from(value: &MediaExtensionManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaExtensionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaExtensionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaExtensionManager> for ::windows::runtime::IInspectable {
    fn from(value: MediaExtensionManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaExtensionManager> for ::windows::runtime::IInspectable {
    fn from(value: &MediaExtensionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaExtensionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaExtensionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaExtensionManager {}
unsafe impl ::core::marker::Sync for MediaExtensionManager {}
#[doc = "*Required features: `Media`*"]
pub struct MediaMarkerTypes {}
impl MediaMarkerTypes {
    #[doc = "*Required features: `Media`*"]
    pub fn Bookmark() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaMarkerTypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IMediaMarkerTypesStatics<R, F: FnOnce(&IMediaMarkerTypesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaMarkerTypes, IMediaMarkerTypesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MediaMarkerTypes {
    const NAME: &'static str = "Windows.Media.MediaMarkerTypes";
}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlaybackAutoRepeatMode(pub i32);
impl MediaPlaybackAutoRepeatMode {
    pub const None: MediaPlaybackAutoRepeatMode = MediaPlaybackAutoRepeatMode(0i32);
    pub const Track: MediaPlaybackAutoRepeatMode = MediaPlaybackAutoRepeatMode(1i32);
    pub const List: MediaPlaybackAutoRepeatMode = MediaPlaybackAutoRepeatMode(2i32);
}
impl ::core::convert::From<i32> for MediaPlaybackAutoRepeatMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaPlaybackAutoRepeatMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaPlaybackAutoRepeatMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackAutoRepeatMode;i4)");
}
impl ::windows::runtime::DefaultType for MediaPlaybackAutoRepeatMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlaybackStatus(pub i32);
impl MediaPlaybackStatus {
    pub const Closed: MediaPlaybackStatus = MediaPlaybackStatus(0i32);
    pub const Changing: MediaPlaybackStatus = MediaPlaybackStatus(1i32);
    pub const Stopped: MediaPlaybackStatus = MediaPlaybackStatus(2i32);
    pub const Playing: MediaPlaybackStatus = MediaPlaybackStatus(3i32);
    pub const Paused: MediaPlaybackStatus = MediaPlaybackStatus(4i32);
}
impl ::core::convert::From<i32> for MediaPlaybackStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaPlaybackStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaPlaybackStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackStatus;i4)");
}
impl ::windows::runtime::DefaultType for MediaPlaybackStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlaybackType(pub i32);
impl MediaPlaybackType {
    pub const Unknown: MediaPlaybackType = MediaPlaybackType(0i32);
    pub const Music: MediaPlaybackType = MediaPlaybackType(1i32);
    pub const Video: MediaPlaybackType = MediaPlaybackType(2i32);
    pub const Image: MediaPlaybackType = MediaPlaybackType(3i32);
}
impl ::core::convert::From<i32> for MediaPlaybackType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaPlaybackType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaPlaybackType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackType;i4)");
}
impl ::windows::runtime::DefaultType for MediaPlaybackType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaProcessingTriggerDetails(pub ::windows::runtime::IInspectable);
impl MediaProcessingTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaProcessingTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProcessingTriggerDetails;{eb8564ac-a351-4f4e-b4f0-9bf2408993db})");
}
unsafe impl ::windows::runtime::Interface for MediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeb8564ac_a351_4f4e_b4f0_9bf2408993db);
}
impl ::windows::runtime::RuntimeName for MediaProcessingTriggerDetails {
    const NAME: &'static str = "Windows.Media.MediaProcessingTriggerDetails";
}
impl ::core::convert::From<MediaProcessingTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: MediaProcessingTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaProcessingTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &MediaProcessingTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaProcessingTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: MediaProcessingTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaProcessingTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &MediaProcessingTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaProcessingTriggerDetails {}
unsafe impl ::core::marker::Sync for MediaProcessingTriggerDetails {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Media`, `Foundation`*"]
pub struct MediaTimeRange {
    pub Start: super::Foundation::TimeSpan,
    pub End: super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl MediaTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for MediaTimeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for MediaTimeRange {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MediaTimeRange").field("Start", &self.Start).field("End", &self.End).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for MediaTimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.Start == other.Start && self.End == other.End
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for MediaTimeRange {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Abi for MediaTimeRange {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for MediaTimeRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Media.MediaTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::DefaultType for MediaTimeRange {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaTimelineController(pub ::windows::runtime::IInspectable);
impl MediaTimelineController {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaTimelineController, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Resume(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Pause(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn ClockRate(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetClockRate(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn State(&self) -> ::windows::runtime::Result<MediaTimelineControllerState> {
        let this = self;
        unsafe {
            let mut result__: MediaTimelineControllerState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaTimelineControllerState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn PositionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::runtime::IInspectable>>>(&self, positionchangedeventhandler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), positionchangedeventhandler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemovePositionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::runtime::IInspectable>>>(&self, statechangedeventhandler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), statechangedeventhandler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsLoopingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsLoopingEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Failed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Ended<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::runtime::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaTimelineController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaTimelineController;{8ed361f3-0b78-4360-bf71-0c841999ea1b})");
}
unsafe impl ::windows::runtime::Interface for MediaTimelineController {
    type Vtable = IMediaTimelineController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8ed361f3_0b78_4360_bf71_0c841999ea1b);
}
impl ::windows::runtime::RuntimeName for MediaTimelineController {
    const NAME: &'static str = "Windows.Media.MediaTimelineController";
}
impl ::core::convert::From<MediaTimelineController> for ::windows::runtime::IUnknown {
    fn from(value: MediaTimelineController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaTimelineController> for ::windows::runtime::IUnknown {
    fn from(value: &MediaTimelineController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaTimelineController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaTimelineController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaTimelineController> for ::windows::runtime::IInspectable {
    fn from(value: MediaTimelineController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaTimelineController> for ::windows::runtime::IInspectable {
    fn from(value: &MediaTimelineController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaTimelineController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaTimelineController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaTimelineController {}
unsafe impl ::core::marker::Sync for MediaTimelineController {}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaTimelineControllerFailedEventArgs(pub ::windows::runtime::IInspectable);
impl MediaTimelineControllerFailedEventArgs {
    #[doc = "*Required features: `Media`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaTimelineControllerFailedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaTimelineControllerFailedEventArgs;{8821f81d-3e77-43fb-be26-4fc87a044834})");
}
unsafe impl ::windows::runtime::Interface for MediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8821f81d_3e77_43fb_be26_4fc87a044834);
}
impl ::windows::runtime::RuntimeName for MediaTimelineControllerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.MediaTimelineControllerFailedEventArgs";
}
impl ::core::convert::From<MediaTimelineControllerFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MediaTimelineControllerFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaTimelineControllerFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MediaTimelineControllerFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaTimelineControllerFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MediaTimelineControllerFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaTimelineControllerFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MediaTimelineControllerFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaTimelineControllerFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaTimelineControllerFailedEventArgs {}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaTimelineControllerState(pub i32);
impl MediaTimelineControllerState {
    pub const Paused: MediaTimelineControllerState = MediaTimelineControllerState(0i32);
    pub const Running: MediaTimelineControllerState = MediaTimelineControllerState(1i32);
    pub const Stalled: MediaTimelineControllerState = MediaTimelineControllerState(2i32);
    pub const Error: MediaTimelineControllerState = MediaTimelineControllerState(3i32);
}
impl ::core::convert::From<i32> for MediaTimelineControllerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaTimelineControllerState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaTimelineControllerState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaTimelineControllerState;i4)");
}
impl ::windows::runtime::DefaultType for MediaTimelineControllerState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MusicDisplayProperties(pub ::windows::runtime::IInspectable);
impl MusicDisplayProperties {
    #[doc = "*Required features: `Media`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn AlbumArtist(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetAlbumArtist<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Artist(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetArtist<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn AlbumTitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetAlbumTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn TrackNumber(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetTrackNumber(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn Genres(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn AlbumTrackCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetAlbumTrackCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MusicDisplayProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MusicDisplayProperties;{6bbf0c59-d0a0-4d26-92a0-f978e1d18e7b})");
}
unsafe impl ::windows::runtime::Interface for MusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6bbf0c59_d0a0_4d26_92a0_f978e1d18e7b);
}
impl ::windows::runtime::RuntimeName for MusicDisplayProperties {
    const NAME: &'static str = "Windows.Media.MusicDisplayProperties";
}
impl ::core::convert::From<MusicDisplayProperties> for ::windows::runtime::IUnknown {
    fn from(value: MusicDisplayProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MusicDisplayProperties> for ::windows::runtime::IUnknown {
    fn from(value: &MusicDisplayProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MusicDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MusicDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MusicDisplayProperties> for ::windows::runtime::IInspectable {
    fn from(value: MusicDisplayProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MusicDisplayProperties> for ::windows::runtime::IInspectable {
    fn from(value: &MusicDisplayProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MusicDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MusicDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MusicDisplayProperties {}
unsafe impl ::core::marker::Sync for MusicDisplayProperties {}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlaybackPositionChangeRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl PlaybackPositionChangeRequestedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RequestedPlaybackPosition(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackPositionChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlaybackPositionChangeRequestedEventArgs;{b4493f88-eb28-4961-9c14-335e44f3e125})");
}
unsafe impl ::windows::runtime::Interface for PlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb4493f88_eb28_4961_9c14_335e44f3e125);
}
impl ::windows::runtime::RuntimeName for PlaybackPositionChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackPositionChangeRequestedEventArgs";
}
impl ::core::convert::From<PlaybackPositionChangeRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PlaybackPositionChangeRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlaybackPositionChangeRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PlaybackPositionChangeRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlaybackPositionChangeRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PlaybackPositionChangeRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlaybackPositionChangeRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PlaybackPositionChangeRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlaybackPositionChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackPositionChangeRequestedEventArgs {}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlaybackRateChangeRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl PlaybackRateChangeRequestedEventArgs {
    #[doc = "*Required features: `Media`*"]
    pub fn RequestedPlaybackRate(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlaybackRateChangeRequestedEventArgs;{2ce2c41f-3cd6-4f77-9ba7-eb27c26a2140})");
}
unsafe impl ::windows::runtime::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2ce2c41f_3cd6_4f77_9ba7_eb27c26a2140);
}
impl ::windows::runtime::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackRateChangeRequestedEventArgs";
}
impl ::core::convert::From<PlaybackRateChangeRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PlaybackRateChangeRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlaybackRateChangeRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PlaybackRateChangeRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlaybackRateChangeRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PlaybackRateChangeRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlaybackRateChangeRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PlaybackRateChangeRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlaybackRateChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackRateChangeRequestedEventArgs {}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ShuffleEnabledChangeRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl ShuffleEnabledChangeRequestedEventArgs {
    #[doc = "*Required features: `Media`*"]
    pub fn RequestedShuffleEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShuffleEnabledChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.ShuffleEnabledChangeRequestedEventArgs;{49b593fe-4fd0-4666-a314-c0e01940d302})");
}
unsafe impl ::windows::runtime::Interface for ShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x49b593fe_4fd0_4666_a314_c0e01940d302);
}
impl ::windows::runtime::RuntimeName for ShuffleEnabledChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.ShuffleEnabledChangeRequestedEventArgs";
}
impl ::core::convert::From<ShuffleEnabledChangeRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ShuffleEnabledChangeRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ShuffleEnabledChangeRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ShuffleEnabledChangeRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ShuffleEnabledChangeRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ShuffleEnabledChangeRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ShuffleEnabledChangeRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ShuffleEnabledChangeRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ShuffleEnabledChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ShuffleEnabledChangeRequestedEventArgs {}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SoundLevel(pub i32);
impl SoundLevel {
    pub const Muted: SoundLevel = SoundLevel(0i32);
    pub const Low: SoundLevel = SoundLevel(1i32);
    pub const Full: SoundLevel = SoundLevel(2i32);
}
impl ::core::convert::From<i32> for SoundLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SoundLevel {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SoundLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SoundLevel;i4)");
}
impl ::windows::runtime::DefaultType for SoundLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMediaTransportControls(pub ::windows::runtime::IInspectable);
impl SystemMediaTransportControls {
    #[doc = "*Required features: `Media`*"]
    pub fn PlaybackStatus(&self) -> ::windows::runtime::Result<MediaPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackStatus>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetPlaybackStatus(&self, value: MediaPlaybackStatus) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn DisplayUpdater(&self) -> ::windows::runtime::Result<SystemMediaTransportControlsDisplayUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaTransportControlsDisplayUpdater>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SoundLevel(&self) -> ::windows::runtime::Result<SoundLevel> {
        let this = self;
        unsafe {
            let mut result__: SoundLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SoundLevel>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsPlayEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsPlayEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsStopEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsStopEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsPauseEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsPauseEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsRecordEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsRecordEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsFastForwardEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsFastForwardEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsRewindEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsRewindEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsPreviousEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsPreviousEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsNextEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsNextEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsChannelUpEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsChannelUpEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsChannelDownEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsChannelDownEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn ButtonPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveButtonPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn PropertyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemovePropertyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn AutoRepeatMode(&self) -> ::windows::runtime::Result<MediaPlaybackAutoRepeatMode> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: MediaPlaybackAutoRepeatMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackAutoRepeatMode>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetAutoRepeatMode(&self, value: MediaPlaybackAutoRepeatMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn ShuffleEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetShuffleEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn PlaybackRate(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn UpdateTimelineProperties<'a, Param0: ::windows::runtime::IntoParam<'a, SystemMediaTransportControlsTimelineProperties>>(&self, timelineproperties: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), timelineproperties.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn PlaybackPositionChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemovePlaybackPositionChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn PlaybackRateChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemovePlaybackRateChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn ShuffleEnabledChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveShuffleEnabledChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn AutoRepeatModeChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RemoveAutoRepeatModeChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<SystemMediaTransportControls> {
        Self::ISystemMediaTransportControlsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaTransportControls>(result__)
        })
    }
    pub fn ISystemMediaTransportControlsStatics<R, F: FnOnce(&ISystemMediaTransportControlsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemMediaTransportControls, ISystemMediaTransportControlsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemMediaTransportControls {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControls;{99fa3ff4-1742-42a6-902e-087d41f965ec})");
}
unsafe impl ::windows::runtime::Interface for SystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x99fa3ff4_1742_42a6_902e_087d41f965ec);
}
impl ::windows::runtime::RuntimeName for SystemMediaTransportControls {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControls";
}
impl ::core::convert::From<SystemMediaTransportControls> for ::windows::runtime::IUnknown {
    fn from(value: SystemMediaTransportControls) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMediaTransportControls> for ::windows::runtime::IUnknown {
    fn from(value: &SystemMediaTransportControls) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemMediaTransportControls {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemMediaTransportControls {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMediaTransportControls> for ::windows::runtime::IInspectable {
    fn from(value: SystemMediaTransportControls) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMediaTransportControls> for ::windows::runtime::IInspectable {
    fn from(value: &SystemMediaTransportControls) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemMediaTransportControls {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemMediaTransportControls {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControls {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControls {}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemMediaTransportControlsButton(pub i32);
impl SystemMediaTransportControlsButton {
    pub const Play: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(0i32);
    pub const Pause: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(1i32);
    pub const Stop: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(2i32);
    pub const Record: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(3i32);
    pub const FastForward: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(4i32);
    pub const Rewind: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(5i32);
    pub const Next: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(6i32);
    pub const Previous: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(7i32);
    pub const ChannelUp: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(8i32);
    pub const ChannelDown: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(9i32);
}
impl ::core::convert::From<i32> for SystemMediaTransportControlsButton {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemMediaTransportControlsButton {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemMediaTransportControlsButton {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsButton;i4)");
}
impl ::windows::runtime::DefaultType for SystemMediaTransportControlsButton {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(pub ::windows::runtime::IInspectable);
impl SystemMediaTransportControlsButtonPressedEventArgs {
    #[doc = "*Required features: `Media`*"]
    pub fn Button(&self) -> ::windows::runtime::Result<SystemMediaTransportControlsButton> {
        let this = self;
        unsafe {
            let mut result__: SystemMediaTransportControlsButton = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaTransportControlsButton>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemMediaTransportControlsButtonPressedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs;{b7f47116-a56f-4dc8-9e11-92031f4a87c2})");
}
unsafe impl ::windows::runtime::Interface for SystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb7f47116_a56f_4dc8_9e11_92031f4a87c2);
}
impl ::windows::runtime::RuntimeName for SystemMediaTransportControlsButtonPressedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs";
}
impl ::core::convert::From<SystemMediaTransportControlsButtonPressedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsButtonPressedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMediaTransportControlsButtonPressedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsButtonPressedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsButtonPressedEventArgs {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsButtonPressedEventArgs {}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMediaTransportControlsDisplayUpdater(pub ::windows::runtime::IInspectable);
impl SystemMediaTransportControlsDisplayUpdater {
    #[doc = "*Required features: `Media`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackType>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetType(&self, value: MediaPlaybackType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn AppMediaId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetAppMediaId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media`, `Storage_Streams`*"]
    pub fn SetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::Storage::Streams::RandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn MusicProperties(&self) -> ::windows::runtime::Result<MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MusicDisplayProperties>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn VideoProperties(&self) -> ::windows::runtime::Result<VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoDisplayProperties>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn ImageProperties(&self) -> ::windows::runtime::Result<ImageDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageDisplayProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media`, `Foundation`, `Storage`*"]
    pub fn CopyFromFileAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::Storage::StorageFile>>(&self, r#type: MediaPlaybackType, source: Param1) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), r#type, source.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn ClearAll(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Update(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemMediaTransportControlsDisplayUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsDisplayUpdater;{8abbc53e-fa55-4ecf-ad8e-c984e5dd1550})");
}
unsafe impl ::windows::runtime::Interface for SystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8abbc53e_fa55_4ecf_ad8e_c984e5dd1550);
}
impl ::windows::runtime::RuntimeName for SystemMediaTransportControlsDisplayUpdater {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsDisplayUpdater";
}
impl ::core::convert::From<SystemMediaTransportControlsDisplayUpdater> for ::windows::runtime::IUnknown {
    fn from(value: SystemMediaTransportControlsDisplayUpdater) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsDisplayUpdater> for ::windows::runtime::IUnknown {
    fn from(value: &SystemMediaTransportControlsDisplayUpdater) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMediaTransportControlsDisplayUpdater> for ::windows::runtime::IInspectable {
    fn from(value: SystemMediaTransportControlsDisplayUpdater) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsDisplayUpdater> for ::windows::runtime::IInspectable {
    fn from(value: &SystemMediaTransportControlsDisplayUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsDisplayUpdater {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsDisplayUpdater {}
#[doc = "*Required features: `Media`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemMediaTransportControlsProperty(pub i32);
impl SystemMediaTransportControlsProperty {
    pub const SoundLevel: SystemMediaTransportControlsProperty = SystemMediaTransportControlsProperty(0i32);
}
impl ::core::convert::From<i32> for SystemMediaTransportControlsProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemMediaTransportControlsProperty {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemMediaTransportControlsProperty {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsProperty;i4)");
}
impl ::windows::runtime::DefaultType for SystemMediaTransportControlsProperty {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(pub ::windows::runtime::IInspectable);
impl SystemMediaTransportControlsPropertyChangedEventArgs {
    #[doc = "*Required features: `Media`*"]
    pub fn Property(&self) -> ::windows::runtime::Result<SystemMediaTransportControlsProperty> {
        let this = self;
        unsafe {
            let mut result__: SystemMediaTransportControlsProperty = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaTransportControlsProperty>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemMediaTransportControlsPropertyChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs;{d0ca0936-339b-4cb3-8eeb-737607f56e08})");
}
unsafe impl ::windows::runtime::Interface for SystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd0ca0936_339b_4cb3_8eeb_737607f56e08);
}
impl ::windows::runtime::RuntimeName for SystemMediaTransportControlsPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs";
}
impl ::core::convert::From<SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsPropertyChangedEventArgs {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsPropertyChangedEventArgs {}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMediaTransportControlsTimelineProperties(pub ::windows::runtime::IInspectable);
impl SystemMediaTransportControlsTimelineProperties {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemMediaTransportControlsTimelineProperties, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetStartTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn EndTime(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetEndTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn MinSeekTime(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetMinSeekTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn MaxSeekTime(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetMaxSeekTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemMediaTransportControlsTimelineProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsTimelineProperties;{5125316a-c3a2-475b-8507-93534dc88f15})");
}
unsafe impl ::windows::runtime::Interface for SystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5125316a_c3a2_475b_8507_93534dc88f15);
}
impl ::windows::runtime::RuntimeName for SystemMediaTransportControlsTimelineProperties {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsTimelineProperties";
}
impl ::core::convert::From<SystemMediaTransportControlsTimelineProperties> for ::windows::runtime::IUnknown {
    fn from(value: SystemMediaTransportControlsTimelineProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsTimelineProperties> for ::windows::runtime::IUnknown {
    fn from(value: &SystemMediaTransportControlsTimelineProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMediaTransportControlsTimelineProperties> for ::windows::runtime::IInspectable {
    fn from(value: SystemMediaTransportControlsTimelineProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsTimelineProperties> for ::windows::runtime::IInspectable {
    fn from(value: &SystemMediaTransportControlsTimelineProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsTimelineProperties {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsTimelineProperties {}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoDisplayProperties(pub ::windows::runtime::IInspectable);
impl VideoDisplayProperties {
    #[doc = "*Required features: `Media`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Subtitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn Genres(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IVideoDisplayProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoDisplayProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.VideoDisplayProperties;{5609fdb1-5d2d-4872-8170-45dee5bc2f5c})");
}
unsafe impl ::windows::runtime::Interface for VideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5609fdb1_5d2d_4872_8170_45dee5bc2f5c);
}
impl ::windows::runtime::RuntimeName for VideoDisplayProperties {
    const NAME: &'static str = "Windows.Media.VideoDisplayProperties";
}
impl ::core::convert::From<VideoDisplayProperties> for ::windows::runtime::IUnknown {
    fn from(value: VideoDisplayProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoDisplayProperties> for ::windows::runtime::IUnknown {
    fn from(value: &VideoDisplayProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VideoDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoDisplayProperties> for ::windows::runtime::IInspectable {
    fn from(value: VideoDisplayProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoDisplayProperties> for ::windows::runtime::IInspectable {
    fn from(value: &VideoDisplayProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoDisplayProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VideoDisplayProperties {}
unsafe impl ::core::marker::Sync for VideoDisplayProperties {}
#[doc = "*Required features: `Media`*"]
pub struct VideoEffects {}
impl VideoEffects {
    #[doc = "*Required features: `Media`*"]
    pub fn VideoStabilization() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IVideoEffectsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IVideoEffectsStatics<R, F: FnOnce(&IVideoEffectsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoEffects, IVideoEffectsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for VideoEffects {
    const NAME: &'static str = "Windows.Media.VideoEffects";
}
#[doc = "*Required features: `Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoFrame(pub ::windows::runtime::IInspectable);
impl VideoFrame {
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media`, `Graphics_Imaging`*"]
    pub fn SoftwareBitmap(&self) -> ::windows::runtime::Result<super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn CopyToAsync<'a, Param0: ::windows::runtime::IntoParam<'a, VideoFrame>>(&self, frame: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), frame.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Media`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Direct3DSurface(&self) -> ::windows::runtime::Result<super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetRelativeTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn RelativeTime(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetSystemRelativeTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SystemRelativeTime(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media`*"]
    pub fn IsDiscontinuous(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media`, `Foundation_Collections`*"]
    pub fn ExtendedProperties(&self) -> ::windows::runtime::Result<super::Foundation::Collections::IPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media`, `Graphics_Imaging`*"]
    pub fn Create(format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> ::windows::runtime::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), format, width, height, &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media`, `Graphics_Imaging`*"]
    pub fn CreateWithAlpha(format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::runtime::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), format, width, height, alpha, &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Media`, `Foundation`, `Graphics_Imaging`*"]
    pub fn CopyToWithBoundsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, VideoFrame>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>>(&self, frame: Param0, sourcebounds: Param1, destinationbounds: Param2) -> ::windows::runtime::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IVideoFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), frame.into_param().abi(), sourcebounds.into_param().abi(), destinationbounds.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `Media`, `Graphics_DirectX`*"]
    pub fn CreateAsDirect3D11SurfaceBacked(format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> ::windows::runtime::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), format, width, height, &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    #[doc = "*Required features: `Media`, `Graphics_DirectX`, `Graphics_DirectX_Direct3D11`*"]
    pub fn CreateAsDirect3D11SurfaceBackedWithDevice<'a, Param3: ::windows::runtime::IntoParam<'a, super::Graphics::DirectX::Direct3D11::IDirect3DDevice>>(format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: Param3) -> ::windows::runtime::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), format, width, height, device.into_param().abi(), &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media`, `Graphics_Imaging`*"]
    pub fn CreateWithSoftwareBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::Graphics::Imaging::SoftwareBitmap>>(bitmap: Param0) -> ::windows::runtime::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Media`, `Graphics_DirectX_Direct3D11`*"]
    pub fn CreateWithDirect3D11Surface<'a, Param0: ::windows::runtime::IntoParam<'a, super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(surface: Param0) -> ::windows::runtime::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), surface.into_param().abi(), &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    pub fn IVideoFrameFactory<R, F: FnOnce(&IVideoFrameFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoFrame, IVideoFrameFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVideoFrameStatics<R, F: FnOnce(&IVideoFrameStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoFrame, IVideoFrameStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.VideoFrame;{0cc06625-90fc-4c92-bd95-7ded21819d1c})");
}
unsafe impl ::windows::runtime::Interface for VideoFrame {
    type Vtable = IVideoFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0cc06625_90fc_4c92_bd95_7ded21819d1c);
}
impl ::windows::runtime::RuntimeName for VideoFrame {
    const NAME: &'static str = "Windows.Media.VideoFrame";
}
impl ::core::convert::From<VideoFrame> for ::windows::runtime::IUnknown {
    fn from(value: VideoFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoFrame> for ::windows::runtime::IUnknown {
    fn from(value: &VideoFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoFrame> for ::windows::runtime::IInspectable {
    fn from(value: VideoFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoFrame> for ::windows::runtime::IInspectable {
    fn from(value: &VideoFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<VideoFrame> for super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VideoFrame) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&VideoFrame> for super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VideoFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IClosable> for VideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IClosable> for &VideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<VideoFrame> for IMediaFrame {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VideoFrame) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoFrame> for IMediaFrame {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VideoFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaFrame> for VideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaFrame> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaFrame> for &VideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaFrame> {
        ::core::convert::TryInto::<IMediaFrame>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoFrame {}
unsafe impl ::core::marker::Sync for VideoFrame {}
