#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x62b496cb_539d_5343_a65f_41f5300ec70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPalmRejectionDelayZonePreviewStatics {
    type Vtable = IPalmRejectionDelayZonePreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcdef5ee0_93d0_53a9_8f0e_9a379f8f7530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputpanelvisual: ::windows::runtime::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputpanelvisual: ::windows::runtime::RawPtr, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: ::windows::runtime::RawPtr, viewportrect: super::super::super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))] usize,
);
#[doc = "*Required features: `UI_Input_Inking_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PalmRejectionDelayZonePreview(pub ::windows::runtime::IInspectable);
impl PalmRejectionDelayZonePreview {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`, `UI_Composition`*"]
    pub fn CreateForVisual<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Rect>>(inputpanelvisual: Param0, inputpanelrect: Param1) -> ::windows::runtime::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), &mut result__).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    #[doc = "*Required features: `UI_Input_Inking_Preview`, `Foundation`, `UI_Composition`*"]
    pub fn CreateForVisualWithViewportClip<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Rect>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Visual>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Rect>>(
        inputpanelvisual: Param0,
        inputpanelrect: Param1,
        viewportvisual: Param2,
        viewportrect: Param3,
    ) -> ::windows::runtime::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), viewportvisual.into_param().abi(), viewportrect.into_param().abi(), &mut result__).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    pub fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview;{62b496cb-539d-5343-a65f-41f5300ec70c})");
}
unsafe impl ::windows::runtime::Interface for PalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x62b496cb_539d_5343_a65f_41f5300ec70c);
}
impl ::windows::runtime::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
impl ::core::convert::From<PalmRejectionDelayZonePreview> for ::windows::runtime::IUnknown {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PalmRejectionDelayZonePreview> for ::windows::runtime::IUnknown {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PalmRejectionDelayZonePreview> for ::windows::runtime::IInspectable {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PalmRejectionDelayZonePreview> for ::windows::runtime::IInspectable {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PalmRejectionDelayZonePreview> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PalmRejectionDelayZonePreview) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PalmRejectionDelayZonePreview> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PalmRejectionDelayZonePreview) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::IClosable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::IClosable> for &PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for PalmRejectionDelayZonePreview {}
unsafe impl ::core::marker::Sync for PalmRejectionDelayZonePreview {}
