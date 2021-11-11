#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConditionForceEffect(pub ::windows::runtime::IInspectable);
impl ConditionForceEffect {
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Gain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn State(&self) -> ::windows::runtime::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ConditionForceEffectKind> {
        let this = &::windows::runtime::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe {
            let mut result__: ConditionForceEffectKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConditionForceEffectKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation_Numerics`*"]
    pub fn SetParameters<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, direction: Param0, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), direction.into_param().abi(), positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn CreateInstance(effectkind: ConditionForceEffectKind) -> ::windows::runtime::Result<ConditionForceEffect> {
        Self::IConditionForceEffectFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), effectkind, &mut result__).from_abi::<ConditionForceEffect>(result__)
        })
    }
    pub fn IConditionForceEffectFactory<R, F: FnOnce(&IConditionForceEffectFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ConditionForceEffect, IConditionForceEffectFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConditionForceEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConditionForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows::runtime::Interface for ConditionForceEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl ::windows::runtime::RuntimeName for ConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
}
impl ::core::convert::From<ConditionForceEffect> for ::windows::runtime::IUnknown {
    fn from(value: ConditionForceEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConditionForceEffect> for ::windows::runtime::IUnknown {
    fn from(value: &ConditionForceEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConditionForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConditionForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConditionForceEffect> for ::windows::runtime::IInspectable {
    fn from(value: ConditionForceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConditionForceEffect> for ::windows::runtime::IInspectable {
    fn from(value: &ConditionForceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConditionForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConditionForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ConditionForceEffect> for IForceFeedbackEffect {
    fn from(value: ConditionForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConditionForceEffect> for IForceFeedbackEffect {
    fn from(value: &ConditionForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IForceFeedbackEffect> for ConditionForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IForceFeedbackEffect> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IForceFeedbackEffect> for &ConditionForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IForceFeedbackEffect> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConditionForceEffect {}
unsafe impl ::core::marker::Sync for ConditionForceEffect {}
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: ConditionForceEffectKind = ConditionForceEffectKind(0i32);
    pub const Damper: ConditionForceEffectKind = ConditionForceEffectKind(1i32);
    pub const Inertia: ConditionForceEffectKind = ConditionForceEffectKind(2i32);
    pub const Friction: ConditionForceEffectKind = ConditionForceEffectKind(3i32);
}
impl ::core::convert::From<i32> for ConditionForceEffectKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ConditionForceEffectKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ConditionForceEffectKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind;i4)");
}
impl ::windows::runtime::DefaultType for ConditionForceEffectKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConstantForceEffect(pub ::windows::runtime::IInspectable);
impl ConstantForceEffect {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ConstantForceEffect, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Gain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn State(&self) -> ::windows::runtime::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`, `Foundation_Numerics`*"]
    pub fn SetParameters<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, duration: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vector.into_param().abi(), duration.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`, `Foundation_Numerics`*"]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param6: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param7: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(
        &self,
        vector: Param0,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: Param4,
        attackduration: Param5,
        sustainduration: Param6,
        releaseduration: Param7,
        repeatcount: u32,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), vector.into_param().abi(), attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConstantForceEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConstantForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows::runtime::Interface for ConstantForceEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl ::windows::runtime::RuntimeName for ConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
}
impl ::core::convert::From<ConstantForceEffect> for ::windows::runtime::IUnknown {
    fn from(value: ConstantForceEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConstantForceEffect> for ::windows::runtime::IUnknown {
    fn from(value: &ConstantForceEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConstantForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConstantForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConstantForceEffect> for ::windows::runtime::IInspectable {
    fn from(value: ConstantForceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConstantForceEffect> for ::windows::runtime::IInspectable {
    fn from(value: &ConstantForceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConstantForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConstantForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ConstantForceEffect> for IForceFeedbackEffect {
    fn from(value: ConstantForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConstantForceEffect> for IForceFeedbackEffect {
    fn from(value: &ConstantForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IForceFeedbackEffect> for ConstantForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IForceFeedbackEffect> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IForceFeedbackEffect> for &ConstantForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IForceFeedbackEffect> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConstantForceEffect {}
unsafe impl ::core::marker::Sync for ConstantForceEffect {}
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(0u32);
    pub const X: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(1u32);
    pub const Y: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(2u32);
    pub const Z: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(4u32);
}
impl ::core::convert::From<u32> for ForceFeedbackEffectAxes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ForceFeedbackEffectAxes {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ForceFeedbackEffectAxes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes;u4)");
}
impl ::windows::runtime::DefaultType for ForceFeedbackEffectAxes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ForceFeedbackEffectAxes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ForceFeedbackEffectAxes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ForceFeedbackEffectAxes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: ForceFeedbackEffectState = ForceFeedbackEffectState(0i32);
    pub const Running: ForceFeedbackEffectState = ForceFeedbackEffectState(1i32);
    pub const Paused: ForceFeedbackEffectState = ForceFeedbackEffectState(2i32);
    pub const Faulted: ForceFeedbackEffectState = ForceFeedbackEffectState(3i32);
}
impl ::core::convert::From<i32> for ForceFeedbackEffectState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ForceFeedbackEffectState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ForceFeedbackEffectState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState;i4)");
}
impl ::windows::runtime::DefaultType for ForceFeedbackEffectState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(0i32);
    pub const EffectStorageFull: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(1i32);
    pub const EffectNotSupported: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(2i32);
}
impl ::core::convert::From<i32> for ForceFeedbackLoadEffectResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ForceFeedbackLoadEffectResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ForceFeedbackLoadEffectResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult;i4)");
}
impl ::windows::runtime::DefaultType for ForceFeedbackLoadEffectResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ForceFeedbackMotor(pub ::windows::runtime::IInspectable);
impl ForceFeedbackMotor {
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn AreEffectsPaused(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn MasterGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn SetMasterGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn SupportedAxes(&self) -> ::windows::runtime::Result<ForceFeedbackEffectAxes> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectAxes = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectAxes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`*"]
    pub fn LoadEffectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IForceFeedbackEffect>>(&self, effect: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn PauseAllEffects(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn ResumeAllEffects(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn StopAllEffects(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`*"]
    pub fn TryDisableAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`*"]
    pub fn TryEnableAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`*"]
    pub fn TryResetAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`*"]
    pub fn TryUnloadEffectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IForceFeedbackEffect>>(&self, effect: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ForceFeedbackMotor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor;{8d3d417c-a5ea-4516-8026-2b00f74ef6e5})");
}
unsafe impl ::windows::runtime::Interface for ForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
}
impl ::windows::runtime::RuntimeName for ForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
}
impl ::core::convert::From<ForceFeedbackMotor> for ::windows::runtime::IUnknown {
    fn from(value: ForceFeedbackMotor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ForceFeedbackMotor> for ::windows::runtime::IUnknown {
    fn from(value: &ForceFeedbackMotor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ForceFeedbackMotor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ForceFeedbackMotor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ForceFeedbackMotor> for ::windows::runtime::IInspectable {
    fn from(value: ForceFeedbackMotor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ForceFeedbackMotor> for ::windows::runtime::IInspectable {
    fn from(value: &ForceFeedbackMotor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ForceFeedbackMotor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ForceFeedbackMotor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ForceFeedbackMotor {}
unsafe impl ::core::marker::Sync for ForceFeedbackMotor {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IConditionForceEffect(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConditionForceEffect {
    type Vtable = IConditionForceEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x32d1ea68_3695_4e69_85c0_cd1944189140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ConditionForceEffectKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConditionForceEffectFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConditionForceEffectFactory {
    type Vtable = IConditionForceEffectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x91a99264_1810_4eb6_a773_bfd3b8cddbab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effectkind: ConditionForceEffectKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConstantForceEffect(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConstantForceEffect {
    type Vtable = IConstantForceEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9bfa0140_f3c7_415c_b068_0f068734bce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstantForceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
pub struct IForceFeedbackEffect(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IForceFeedbackEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl IForceFeedbackEffect {
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Gain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn State(&self) -> ::windows::runtime::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IForceFeedbackEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a17fba0c-2ae4-48c2-8063-eabd0777cb89}");
}
impl ::core::convert::From<IForceFeedbackEffect> for ::windows::runtime::IUnknown {
    fn from(value: IForceFeedbackEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IForceFeedbackEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IForceFeedbackEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IForceFeedbackEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IForceFeedbackEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IForceFeedbackEffect> for ::windows::runtime::IInspectable {
    fn from(value: IForceFeedbackEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IForceFeedbackEffect> for ::windows::runtime::IInspectable {
    fn from(value: &IForceFeedbackEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IForceFeedbackEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IForceFeedbackEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ForceFeedbackEffectState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IForceFeedbackMotor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackMotor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ForceFeedbackEffectAxes) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effect: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effect: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeriodicForceEffect(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPeriodicForceEffect {
    type Vtable = IPeriodicForceEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c5138d7_fc75_4d52_9a0a_efe4cab5fe64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PeriodicForceEffectKind) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        vector: super::super::super::Foundation::Numerics::Vector3,
        frequency: f32,
        phase: f32,
        bias: f32,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: super::super::super::Foundation::TimeSpan,
        attackduration: super::super::super::Foundation::TimeSpan,
        sustainduration: super::super::super::Foundation::TimeSpan,
        releaseduration: super::super::super::Foundation::TimeSpan,
        repeatcount: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeriodicForceEffectFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPeriodicForceEffectFactory {
    type Vtable = IPeriodicForceEffectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6f62eb1a_9851_477b_b318_35ecaa15070f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effectkind: PeriodicForceEffectKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRampForceEffect(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRampForceEffect {
    type Vtable = IRampForceEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf1f81259_1ca6_4080_b56d_b43f3354d052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRampForceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        startvector: super::super::super::Foundation::Numerics::Vector3,
        endvector: super::super::super::Foundation::Numerics::Vector3,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: super::super::super::Foundation::TimeSpan,
        attackduration: super::super::super::Foundation::TimeSpan,
        sustainduration: super::super::super::Foundation::TimeSpan,
        releaseduration: super::super::super::Foundation::TimeSpan,
        repeatcount: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PeriodicForceEffect(pub ::windows::runtime::IInspectable);
impl PeriodicForceEffect {
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Gain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn State(&self) -> ::windows::runtime::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<PeriodicForceEffectKind> {
        let this = &::windows::runtime::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe {
            let mut result__: PeriodicForceEffectKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeriodicForceEffectKind>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`, `Foundation_Numerics`*"]
    pub fn SetParameters<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, frequency: f32, phase: f32, bias: f32, duration: Param4) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), vector.into_param().abi(), frequency, phase, bias, duration.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`, `Foundation_Numerics`*"]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param7: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param8: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param9: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param10: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(
        &self,
        vector: Param0,
        frequency: f32,
        phase: f32,
        bias: f32,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: Param7,
        attackduration: Param8,
        sustainduration: Param9,
        releaseduration: Param10,
        repeatcount: u32,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), vector.into_param().abi(), frequency, phase, bias, attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn CreateInstance(effectkind: PeriodicForceEffectKind) -> ::windows::runtime::Result<PeriodicForceEffect> {
        Self::IPeriodicForceEffectFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), effectkind, &mut result__).from_abi::<PeriodicForceEffect>(result__)
        })
    }
    pub fn IPeriodicForceEffectFactory<R, F: FnOnce(&IPeriodicForceEffectFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PeriodicForceEffect, IPeriodicForceEffectFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PeriodicForceEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows::runtime::Interface for PeriodicForceEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl ::windows::runtime::RuntimeName for PeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
}
impl ::core::convert::From<PeriodicForceEffect> for ::windows::runtime::IUnknown {
    fn from(value: PeriodicForceEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for ::windows::runtime::IUnknown {
    fn from(value: &PeriodicForceEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PeriodicForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PeriodicForceEffect> for ::windows::runtime::IInspectable {
    fn from(value: PeriodicForceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for ::windows::runtime::IInspectable {
    fn from(value: &PeriodicForceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PeriodicForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PeriodicForceEffect> for IForceFeedbackEffect {
    fn from(value: PeriodicForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for IForceFeedbackEffect {
    fn from(value: &PeriodicForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IForceFeedbackEffect> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IForceFeedbackEffect> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IForceFeedbackEffect> for &PeriodicForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IForceFeedbackEffect> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PeriodicForceEffect {}
unsafe impl ::core::marker::Sync for PeriodicForceEffect {}
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: PeriodicForceEffectKind = PeriodicForceEffectKind(0i32);
    pub const SineWave: PeriodicForceEffectKind = PeriodicForceEffectKind(1i32);
    pub const TriangleWave: PeriodicForceEffectKind = PeriodicForceEffectKind(2i32);
    pub const SawtoothWaveUp: PeriodicForceEffectKind = PeriodicForceEffectKind(3i32);
    pub const SawtoothWaveDown: PeriodicForceEffectKind = PeriodicForceEffectKind(4i32);
}
impl ::core::convert::From<i32> for PeriodicForceEffectKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PeriodicForceEffectKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PeriodicForceEffectKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind;i4)");
}
impl ::windows::runtime::DefaultType for PeriodicForceEffectKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RampForceEffect(pub ::windows::runtime::IInspectable);
impl RampForceEffect {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RampForceEffect, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Gain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn State(&self) -> ::windows::runtime::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`, `Foundation_Numerics`*"]
    pub fn SetParameters<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, startvector: Param0, endvector: Param1, duration: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), startvector.into_param().abi(), endvector.into_param().abi(), duration.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `Gaming_Input_ForceFeedback`, `Foundation`, `Foundation_Numerics`*"]
    pub fn SetParametersWithEnvelope<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>,
        Param8: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>,
    >(
        &self,
        startvector: Param0,
        endvector: Param1,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: Param5,
        attackduration: Param6,
        sustainduration: Param7,
        releaseduration: Param8,
        repeatcount: u32,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), startvector.into_param().abi(), endvector.into_param().abi(), attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RampForceEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.RampForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows::runtime::Interface for RampForceEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl ::windows::runtime::RuntimeName for RampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.RampForceEffect";
}
impl ::core::convert::From<RampForceEffect> for ::windows::runtime::IUnknown {
    fn from(value: RampForceEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RampForceEffect> for ::windows::runtime::IUnknown {
    fn from(value: &RampForceEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RampForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RampForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RampForceEffect> for ::windows::runtime::IInspectable {
    fn from(value: RampForceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RampForceEffect> for ::windows::runtime::IInspectable {
    fn from(value: &RampForceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RampForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RampForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RampForceEffect> for IForceFeedbackEffect {
    fn from(value: RampForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RampForceEffect> for IForceFeedbackEffect {
    fn from(value: &RampForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IForceFeedbackEffect> for RampForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IForceFeedbackEffect> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IForceFeedbackEffect> for &RampForceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IForceFeedbackEffect> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RampForceEffect {}
unsafe impl ::core::marker::Sync for RampForceEffect {}
