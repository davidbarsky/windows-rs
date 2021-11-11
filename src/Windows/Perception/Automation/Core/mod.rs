#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Perception_Automation_Core`*"]
pub struct CorePerceptionAutomation {}
impl CorePerceptionAutomation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Perception_Automation_Core`, `Foundation`*"]
    pub fn SetActivationFactoryProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IGetActivationFactory>>(provider: Param0) -> ::windows::runtime::Result<()> {
        Self::ICorePerceptionAutomationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), provider.into_param().abi()).ok() })
    }
    pub fn ICorePerceptionAutomationStatics<R, F: FnOnce(&ICorePerceptionAutomationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CorePerceptionAutomation, ICorePerceptionAutomationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for CorePerceptionAutomation {
    const NAME: &'static str = "Windows.Perception.Automation.Core.CorePerceptionAutomation";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICorePerceptionAutomationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICorePerceptionAutomationStatics {
    type Vtable = ICorePerceptionAutomationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0bb04541_4ce2_4923_9a76_8187ecc59112);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePerceptionAutomationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct PerceptionAutomationCoreContract(pub u8);
