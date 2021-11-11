#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPreviewBuildsManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfa07dd61_7e4f_59f7_7c9f_def9051c5f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPreviewBuildsManagerStatics {
    type Vtable = IPreviewBuildsManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3e422887_b112_5a70_7da1_97d78d32aa29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPreviewBuildsState(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPreviewBuildsState {
    type Vtable = IPreviewBuildsState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa2f2903e_b223_5f63_7546_3e8eac070a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsState_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Management_Update`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PreviewBuildsManager(pub ::windows::runtime::IInspectable);
impl PreviewBuildsManager {
    #[doc = "*Required features: `Management_Update`*"]
    pub fn ArePreviewBuildsAllowed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Management_Update`*"]
    pub fn SetArePreviewBuildsAllowed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Management_Update`*"]
    pub fn GetCurrentState(&self) -> ::windows::runtime::Result<PreviewBuildsState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PreviewBuildsState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Management_Update`, `Foundation`*"]
    pub fn SyncAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Management_Update`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<PreviewBuildsManager> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PreviewBuildsManager>(result__)
        })
    }
    #[doc = "*Required features: `Management_Update`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IPreviewBuildsManagerStatics<R, F: FnOnce(&IPreviewBuildsManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PreviewBuildsManager, IPreviewBuildsManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PreviewBuildsManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsManager;{fa07dd61-7e4f-59f7-7c9f-def9051c5f62})");
}
unsafe impl ::windows::runtime::Interface for PreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfa07dd61_7e4f_59f7_7c9f_def9051c5f62);
}
impl ::windows::runtime::RuntimeName for PreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsManager";
}
impl ::core::convert::From<PreviewBuildsManager> for ::windows::runtime::IUnknown {
    fn from(value: PreviewBuildsManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PreviewBuildsManager> for ::windows::runtime::IUnknown {
    fn from(value: &PreviewBuildsManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PreviewBuildsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PreviewBuildsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PreviewBuildsManager> for ::windows::runtime::IInspectable {
    fn from(value: PreviewBuildsManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PreviewBuildsManager> for ::windows::runtime::IInspectable {
    fn from(value: &PreviewBuildsManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PreviewBuildsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PreviewBuildsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PreviewBuildsManager {}
unsafe impl ::core::marker::Sync for PreviewBuildsManager {}
#[doc = "*Required features: `Management_Update`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PreviewBuildsState(pub ::windows::runtime::IInspectable);
impl PreviewBuildsState {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Management_Update`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PreviewBuildsState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsState;{a2f2903e-b223-5f63-7546-3e8eac070a2e})");
}
unsafe impl ::windows::runtime::Interface for PreviewBuildsState {
    type Vtable = IPreviewBuildsState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa2f2903e_b223_5f63_7546_3e8eac070a2e);
}
impl ::windows::runtime::RuntimeName for PreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsState";
}
impl ::core::convert::From<PreviewBuildsState> for ::windows::runtime::IUnknown {
    fn from(value: PreviewBuildsState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PreviewBuildsState> for ::windows::runtime::IUnknown {
    fn from(value: &PreviewBuildsState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PreviewBuildsState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PreviewBuildsState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PreviewBuildsState> for ::windows::runtime::IInspectable {
    fn from(value: PreviewBuildsState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PreviewBuildsState> for ::windows::runtime::IInspectable {
    fn from(value: &PreviewBuildsState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PreviewBuildsState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PreviewBuildsState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PreviewBuildsState {}
unsafe impl ::core::marker::Sync for PreviewBuildsState {}
