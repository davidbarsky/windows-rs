#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Storage_FileProperties`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BasicProperties(pub ::windows::runtime::IInspectable);
impl BasicProperties {
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn DateModified(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn ItemDate(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BasicProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.BasicProperties;{d05d55db-785e-4a66-be02-9beec58aea81})");
}
unsafe impl ::windows::runtime::Interface for BasicProperties {
    type Vtable = IBasicProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd05d55db_785e_4a66_be02_9beec58aea81);
}
impl ::windows::runtime::RuntimeName for BasicProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.BasicProperties";
}
impl ::core::convert::From<BasicProperties> for ::windows::runtime::IUnknown {
    fn from(value: BasicProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BasicProperties> for ::windows::runtime::IUnknown {
    fn from(value: &BasicProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BasicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BasicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BasicProperties> for ::windows::runtime::IInspectable {
    fn from(value: BasicProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BasicProperties> for ::windows::runtime::IInspectable {
    fn from(value: &BasicProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BasicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BasicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<BasicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BasicProperties) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BasicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BasicProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for BasicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for &BasicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DocumentProperties(pub ::windows::runtime::IInspectable);
impl DocumentProperties {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Author(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Keywords(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DocumentProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.DocumentProperties;{7eab19bc-1821-4923-b4a9-0aea404d0070})");
}
unsafe impl ::windows::runtime::Interface for DocumentProperties {
    type Vtable = IDocumentProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7eab19bc_1821_4923_b4a9_0aea404d0070);
}
impl ::windows::runtime::RuntimeName for DocumentProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.DocumentProperties";
}
impl ::core::convert::From<DocumentProperties> for ::windows::runtime::IUnknown {
    fn from(value: DocumentProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DocumentProperties> for ::windows::runtime::IUnknown {
    fn from(value: &DocumentProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DocumentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DocumentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DocumentProperties> for ::windows::runtime::IInspectable {
    fn from(value: DocumentProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DocumentProperties> for ::windows::runtime::IInspectable {
    fn from(value: &DocumentProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DocumentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DocumentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<DocumentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DocumentProperties) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DocumentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DocumentProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for DocumentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for &DocumentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Storage_FileProperties`*"]
pub struct GeotagHelper {}
impl GeotagHelper {
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Devices_Geolocation`, `Foundation`*"]
    pub fn GetGeotagAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Geolocation::Geopoint>> {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Devices::Geolocation::Geopoint>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Devices_Geolocation`, `Foundation`*"]
    pub fn SetGeotagFromGeolocatorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geolocator>>(file: Param0, geolocator: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), geolocator.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Devices_Geolocation`, `Foundation`*"]
    pub fn SetGeotagAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(file: Param0, geopoint: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), file.into_param().abi(), geopoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IGeotagHelperStatics<R, F: FnOnce(&IGeotagHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GeotagHelper, IGeotagHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for GeotagHelper {
    const NAME: &'static str = "Windows.Storage.FileProperties.GeotagHelper";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBasicProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBasicProperties {
    type Vtable = IBasicProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd05d55db_785e_4a66_be02_9beec58aea81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDocumentProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDocumentProperties {
    type Vtable = IDocumentProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7eab19bc_1821_4923_b4a9_0aea404d0070);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocumentProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeotagHelperStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeotagHelperStatics {
    type Vtable = IGeotagHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x41493244_2524_4655_86a6_ed16f5fc716b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeotagHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, geolocator: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, geopoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageProperties {
    type Vtable = IImageProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x523c9424_fcff_4275_afee_ecdb9ab47973);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhotoOrientation) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMusicProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMusicProperties {
    type Vtable = IMusicProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbc8aab62_66ec_419a_bc5d_ca65a4cb46da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicProperties_abi(
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
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageItemContentProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorageItemContentProperties {
    type Vtable = IStorageItemContentProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05294bad_bc38_48bf_85d7_770e0e2ae0ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemContentProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage_FileProperties`*"]
pub struct IStorageItemExtraProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorageItemExtraProperties {
    type Vtable = IStorageItemExtraProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc54361b2_54cd_432b_bdbc_4b19c4b470d7);
}
impl IStorageItemExtraProperties {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IStorageItemExtraProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c54361b2-54cd-432b-bdbc-4b19c4b470d7}");
}
impl ::core::convert::From<IStorageItemExtraProperties> for ::windows::runtime::IUnknown {
    fn from(value: IStorageItemExtraProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageItemExtraProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IStorageItemExtraProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStorageItemExtraProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStorageItemExtraProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageItemExtraProperties> for ::windows::runtime::IInspectable {
    fn from(value: IStorageItemExtraProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageItemExtraProperties> for ::windows::runtime::IInspectable {
    fn from(value: &IStorageItemExtraProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IStorageItemExtraProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IStorageItemExtraProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemExtraProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertiestoretrieve: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertiestosave: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IThumbnailProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IThumbnailProperties {
    type Vtable = IThumbnailProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x693dd42f_dbe7_49b5_b3b3_2893ac5d3423);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ThumbnailType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoProperties {
    type Vtable = IVideoProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x719ae507_68de_4db8_97de_49998c059f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VideoOrientation) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Storage_FileProperties`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ImageProperties(pub ::windows::runtime::IInspectable);
impl ImageProperties {
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Rating(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetRating(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Keywords(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn DateTaken(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn SetDateTaken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn Latitude(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn Longitude(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn CameraManufacturer(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetCameraManufacturer<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn CameraModel(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetCameraModel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<PhotoOrientation> {
        let this = self;
        unsafe {
            let mut result__: PhotoOrientation = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoOrientation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn PeopleNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.ImageProperties;{523c9424-fcff-4275-afee-ecdb9ab47973})");
}
unsafe impl ::windows::runtime::Interface for ImageProperties {
    type Vtable = IImageProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x523c9424_fcff_4275_afee_ecdb9ab47973);
}
impl ::windows::runtime::RuntimeName for ImageProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.ImageProperties";
}
impl ::core::convert::From<ImageProperties> for ::windows::runtime::IUnknown {
    fn from(value: ImageProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ImageProperties> for ::windows::runtime::IUnknown {
    fn from(value: &ImageProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ImageProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ImageProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ImageProperties> for ::windows::runtime::IInspectable {
    fn from(value: ImageProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ImageProperties> for ::windows::runtime::IInspectable {
    fn from(value: &ImageProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ImageProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ImageProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ImageProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ImageProperties) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ImageProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for ImageProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for &ImageProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MusicProperties(pub ::windows::runtime::IInspectable);
impl MusicProperties {
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Album(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetAlbum<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Artist(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetArtist<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Genre(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn TrackNumber(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetTrackNumber(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Rating(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetRating(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Bitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn AlbumArtist(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetAlbumArtist<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Composers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Conductors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Subtitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Producers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Publisher(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetPublisher<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Writers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Year(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetYear(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MusicProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.MusicProperties;{bc8aab62-66ec-419a-bc5d-ca65a4cb46da})");
}
unsafe impl ::windows::runtime::Interface for MusicProperties {
    type Vtable = IMusicProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbc8aab62_66ec_419a_bc5d_ca65a4cb46da);
}
impl ::windows::runtime::RuntimeName for MusicProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.MusicProperties";
}
impl ::core::convert::From<MusicProperties> for ::windows::runtime::IUnknown {
    fn from(value: MusicProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MusicProperties> for ::windows::runtime::IUnknown {
    fn from(value: &MusicProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MusicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MusicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MusicProperties> for ::windows::runtime::IInspectable {
    fn from(value: MusicProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MusicProperties> for ::windows::runtime::IInspectable {
    fn from(value: &MusicProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MusicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MusicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MusicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MusicProperties) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MusicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MusicProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for MusicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for &MusicProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoOrientation(pub i32);
impl PhotoOrientation {
    pub const Unspecified: PhotoOrientation = PhotoOrientation(0i32);
    pub const Normal: PhotoOrientation = PhotoOrientation(1i32);
    pub const FlipHorizontal: PhotoOrientation = PhotoOrientation(2i32);
    pub const Rotate180: PhotoOrientation = PhotoOrientation(3i32);
    pub const FlipVertical: PhotoOrientation = PhotoOrientation(4i32);
    pub const Transpose: PhotoOrientation = PhotoOrientation(5i32);
    pub const Rotate270: PhotoOrientation = PhotoOrientation(6i32);
    pub const Transverse: PhotoOrientation = PhotoOrientation(7i32);
    pub const Rotate90: PhotoOrientation = PhotoOrientation(8i32);
}
impl ::core::convert::From<i32> for PhotoOrientation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhotoOrientation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhotoOrientation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.PhotoOrientation;i4)");
}
impl ::windows::runtime::DefaultType for PhotoOrientation {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PropertyPrefetchOptions(pub u32);
impl PropertyPrefetchOptions {
    pub const None: PropertyPrefetchOptions = PropertyPrefetchOptions(0u32);
    pub const MusicProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(1u32);
    pub const VideoProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(2u32);
    pub const ImageProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(4u32);
    pub const DocumentProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(8u32);
    pub const BasicProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(16u32);
}
impl ::core::convert::From<u32> for PropertyPrefetchOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PropertyPrefetchOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PropertyPrefetchOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.PropertyPrefetchOptions;u4)");
}
impl ::windows::runtime::DefaultType for PropertyPrefetchOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PropertyPrefetchOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PropertyPrefetchOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PropertyPrefetchOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PropertyPrefetchOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PropertyPrefetchOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageItemContentProperties(pub ::windows::runtime::IInspectable);
impl StorageItemContentProperties {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn GetMusicPropertiesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MusicProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MusicProperties>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn GetVideoPropertiesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<VideoProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VideoProperties>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn GetImagePropertiesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ImageProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageProperties>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn GetDocumentPropertiesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DocumentProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DocumentProperties>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StorageItemContentProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.StorageItemContentProperties;{05294bad-bc38-48bf-85d7-770e0e2ae0ba})");
}
unsafe impl ::windows::runtime::Interface for StorageItemContentProperties {
    type Vtable = IStorageItemContentProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05294bad_bc38_48bf_85d7_770e0e2ae0ba);
}
impl ::windows::runtime::RuntimeName for StorageItemContentProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.StorageItemContentProperties";
}
impl ::core::convert::From<StorageItemContentProperties> for ::windows::runtime::IUnknown {
    fn from(value: StorageItemContentProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageItemContentProperties> for ::windows::runtime::IUnknown {
    fn from(value: &StorageItemContentProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StorageItemContentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a StorageItemContentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageItemContentProperties> for ::windows::runtime::IInspectable {
    fn from(value: StorageItemContentProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageItemContentProperties> for ::windows::runtime::IInspectable {
    fn from(value: &StorageItemContentProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StorageItemContentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StorageItemContentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<StorageItemContentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StorageItemContentProperties) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageItemContentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StorageItemContentProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for StorageItemContentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for &StorageItemContentProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
#[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageItemThumbnail(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl StorageItemThumbnail {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn OriginalWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn OriginalHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn ReturnedSmallerCachedSize(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<ThumbnailType> {
        let this = &::windows::runtime::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__: ThumbnailType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ThumbnailType>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::Streams::InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Storage_Streams`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::Streams::IInputStream> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::Streams::IOutputStream> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::runtime::Result<super::Streams::IRandomAccessStream> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::runtime::RuntimeType for StorageItemThumbnail {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.StorageItemThumbnail;{cc254827-4b3d-438f-9232-10c76bc7e038})");
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::runtime::Interface for StorageItemThumbnail {
    type Vtable = super::Streams::IRandomAccessStreamWithContentType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcc254827_4b3d_438f_9232_10c76bc7e038);
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::runtime::RuntimeName for StorageItemThumbnail {
    const NAME: &'static str = "Windows.Storage.FileProperties.StorageItemThumbnail";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StorageItemThumbnail> for ::windows::runtime::IUnknown {
    fn from(value: StorageItemThumbnail) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StorageItemThumbnail> for ::windows::runtime::IUnknown {
    fn from(value: &StorageItemThumbnail) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StorageItemThumbnail> for ::windows::runtime::IInspectable {
    fn from(value: StorageItemThumbnail) -> Self {
        value.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StorageItemThumbnail> for ::windows::runtime::IInspectable {
    fn from(value: &StorageItemThumbnail) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StorageItemThumbnail> for super::Streams::IRandomAccessStreamWithContentType {
    fn from(value: StorageItemThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StorageItemThumbnail> for super::Streams::IRandomAccessStreamWithContentType {
    fn from(value: &StorageItemThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IRandomAccessStreamWithContentType> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IRandomAccessStreamWithContentType> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IRandomAccessStreamWithContentType> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IRandomAccessStreamWithContentType> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IContentTypeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IContentTypeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IContentTypeProvider> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IContentTypeProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IContentTypeProvider> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<super::Streams::IContentTypeProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IInputStream> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IInputStream> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IInputStream> {
        ::core::convert::TryInto::<super::Streams::IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IOutputStream> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IOutputStream> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::Streams::IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IRandomAccessStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IRandomAccessStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IRandomAccessStream> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IRandomAccessStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IRandomAccessStream> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<super::Streams::IRandomAccessStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ThumbnailMode(pub i32);
impl ThumbnailMode {
    pub const PicturesView: ThumbnailMode = ThumbnailMode(0i32);
    pub const VideosView: ThumbnailMode = ThumbnailMode(1i32);
    pub const MusicView: ThumbnailMode = ThumbnailMode(2i32);
    pub const DocumentsView: ThumbnailMode = ThumbnailMode(3i32);
    pub const ListView: ThumbnailMode = ThumbnailMode(4i32);
    pub const SingleItem: ThumbnailMode = ThumbnailMode(5i32);
}
impl ::core::convert::From<i32> for ThumbnailMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ThumbnailMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ThumbnailMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailMode;i4)");
}
impl ::windows::runtime::DefaultType for ThumbnailMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ThumbnailOptions(pub u32);
impl ThumbnailOptions {
    pub const None: ThumbnailOptions = ThumbnailOptions(0u32);
    pub const ReturnOnlyIfCached: ThumbnailOptions = ThumbnailOptions(1u32);
    pub const ResizeThumbnail: ThumbnailOptions = ThumbnailOptions(2u32);
    pub const UseCurrentScale: ThumbnailOptions = ThumbnailOptions(4u32);
}
impl ::core::convert::From<u32> for ThumbnailOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ThumbnailOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ThumbnailOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailOptions;u4)");
}
impl ::windows::runtime::DefaultType for ThumbnailOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for ThumbnailOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ThumbnailOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ThumbnailOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ThumbnailOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ThumbnailOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ThumbnailType(pub i32);
impl ThumbnailType {
    pub const Image: ThumbnailType = ThumbnailType(0i32);
    pub const Icon: ThumbnailType = ThumbnailType(1i32);
}
impl ::core::convert::From<i32> for ThumbnailType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ThumbnailType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ThumbnailType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailType;i4)");
}
impl ::windows::runtime::DefaultType for ThumbnailType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoOrientation(pub i32);
impl VideoOrientation {
    pub const Normal: VideoOrientation = VideoOrientation(0i32);
    pub const Rotate90: VideoOrientation = VideoOrientation(90i32);
    pub const Rotate180: VideoOrientation = VideoOrientation(180i32);
    pub const Rotate270: VideoOrientation = VideoOrientation(270i32);
}
impl ::core::convert::From<i32> for VideoOrientation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VideoOrientation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VideoOrientation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.VideoOrientation;i4)");
}
impl ::windows::runtime::DefaultType for VideoOrientation {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_FileProperties`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoProperties(pub ::windows::runtime::IInspectable);
impl VideoProperties {
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Rating(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetRating(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Keywords(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn Latitude(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn Longitude(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Subtitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Producers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Publisher(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetPublisher<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Writers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Year(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn SetYear(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Bitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation_Collections`*"]
    pub fn Directors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_FileProperties`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<VideoOrientation> {
        let this = self;
        unsafe {
            let mut result__: VideoOrientation = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoOrientation>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`, `Foundation_Collections`*"]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_FileProperties`, `Foundation`*"]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.VideoProperties;{719ae507-68de-4db8-97de-49998c059f2f})");
}
unsafe impl ::windows::runtime::Interface for VideoProperties {
    type Vtable = IVideoProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x719ae507_68de_4db8_97de_49998c059f2f);
}
impl ::windows::runtime::RuntimeName for VideoProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.VideoProperties";
}
impl ::core::convert::From<VideoProperties> for ::windows::runtime::IUnknown {
    fn from(value: VideoProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoProperties> for ::windows::runtime::IUnknown {
    fn from(value: &VideoProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VideoProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoProperties> for ::windows::runtime::IInspectable {
    fn from(value: VideoProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoProperties> for ::windows::runtime::IInspectable {
    fn from(value: &VideoProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VideoProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VideoProperties) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoProperties> for IStorageItemExtraProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VideoProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for VideoProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStorageItemExtraProperties> for &VideoProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
