#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveBlobGetResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveBlobGetResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x917281e0_7201_4953_aa2c_4008f03aef45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobGetResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveBlobInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveBlobInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadd38034_baf0_4645_b6d0_46edaffb3c2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveBlobInfoGetResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveBlobInfoGetResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7578582_3697_42bf_989c_665d923b5231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoGetResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveBlobInfoQuery(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveBlobInfoQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fdd74b2_eeee_447b_a9d2_7f96c0f83208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoQuery_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetBlobInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBlobInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetBlobInfoWithIndexAndMaxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBlobInfoWithIndexAndMaxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveContainer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveContainer {
    type Vtable = IGameSaveContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c08f89_563f_4ecd_9c6f_33fd0e323d10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Provider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SubmitUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobstowrite: *mut ::core::ffi::c_void, blobstodelete: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SubmitUpdatesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobstoread: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ReadAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobstoread: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubmitPropertySetUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobstowrite: *mut ::core::ffi::c_void, blobstodelete: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubmitPropertySetUpdatesAsync: usize,
    pub CreateBlobInfoQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobnameprefix: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveContainerInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveContainerInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7e27300_155d_4bb4_b2ba_930306f391b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastModifiedTime: usize,
    pub NeedsSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveContainerInfoGetResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveContainerInfoGetResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffc50d74_c581_4f9d_9e39_30a10c1e4c50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoGetResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveContainerInfoQuery(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveContainerInfoQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c94e863_6f80_4327_9327_ffc11afd42b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoQuery_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetContainerInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContainerInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContainerInfoWithIndexAndMaxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContainerInfoWithIndexAndMaxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveOperationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf0f1a05_24a0_4582_9a55_b1bbbb9388d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveOperationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveProvider {
    type Vtable = IGameSaveProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90a60394_80fe_4211_97f8_a5de14dd95d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub CreateContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContainerAsync: usize,
    pub CreateContainerInfoQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateContainerInfoQueryWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containernameprefix: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetRemainingBytesInQuotaAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRemainingBytesInQuotaAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ContainersChangedSinceLastSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContainersChangedSinceLastSync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveProviderGetResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveProviderGetResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ab90816_d393_4d65_ac16_41c3e67ab945);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProviderGetResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameSaveProviderStatics {
    type Vtable = IGameSaveProviderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameSaveProviderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd01d3ed0_7b03_449d_8cbd_3402842a1048);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, serviceconfigid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetSyncOnDemandForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, serviceconfigid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetSyncOnDemandForUserAsync: usize,
}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveBlobGetResult(::windows::core::IUnknown);
impl GameSaveBlobGetResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveBlobGetResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveBlobGetResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveBlobGetResult {}
impl ::core::fmt::Debug for GameSaveBlobGetResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveBlobGetResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveBlobGetResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult;{917281e0-7201-4953-aa2c-4008f03aef45})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveBlobGetResult {
    const IID: ::windows::core::GUID = <IGameSaveBlobGetResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveBlobGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult";
}
::windows::core::interface_hierarchy!(GameSaveBlobGetResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveBlobGetResult {}
unsafe impl ::core::marker::Sync for GameSaveBlobGetResult {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveBlobInfo(::windows::core::IUnknown);
impl GameSaveBlobInfo {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveBlobInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveBlobInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveBlobInfo {}
impl ::core::fmt::Debug for GameSaveBlobInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveBlobInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveBlobInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo;{add38034-baf0-4645-b6d0-46edaffb3c2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveBlobInfo {
    const IID: ::windows::core::GUID = <IGameSaveBlobInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveBlobInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo";
}
::windows::core::interface_hierarchy!(GameSaveBlobInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveBlobInfo {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfo {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveBlobInfoGetResult(::windows::core::IUnknown);
impl GameSaveBlobInfoGetResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveBlobInfoGetResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveBlobInfoGetResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveBlobInfoGetResult {}
impl ::core::fmt::Debug for GameSaveBlobInfoGetResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveBlobInfoGetResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveBlobInfoGetResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult;{c7578582-3697-42bf-989c-665d923b5231})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveBlobInfoGetResult {
    const IID: ::windows::core::GUID = <IGameSaveBlobInfoGetResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveBlobInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult";
}
::windows::core::interface_hierarchy!(GameSaveBlobInfoGetResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveBlobInfoGetResult {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfoGetResult {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveBlobInfoQuery(::windows::core::IUnknown);
impl GameSaveBlobInfoQuery {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetBlobInfoAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBlobInfoAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetBlobInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBlobInfoWithIndexAndMaxAsync)(::windows::core::Vtable::as_raw(this), startindex, maxnumberofitems, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetItemCountAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveBlobInfoQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveBlobInfoQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveBlobInfoQuery {}
impl ::core::fmt::Debug for GameSaveBlobInfoQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveBlobInfoQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveBlobInfoQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery;{9fdd74b2-eeee-447b-a9d2-7f96c0f83208})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveBlobInfoQuery {
    const IID: ::windows::core::GUID = <IGameSaveBlobInfoQuery as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveBlobInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery";
}
::windows::core::interface_hierarchy!(GameSaveBlobInfoQuery, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveBlobInfoQuery {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfoQuery {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveContainer(::windows::core::IUnknown);
impl GameSaveContainer {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Provider(&self) -> ::windows::core::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Provider)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SubmitUpdatesAsync<P0, E0, P1, E1>(&self, blobstowrite: P0, blobstodelete: P1, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubmitUpdatesAsync)(::windows::core::Vtable::as_raw(this), blobstowrite.try_into().map_err(|e| e.into())?.abi(), blobstodelete.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0, E0>(&self, blobstoread: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), blobstoread.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsync<P0, E0>(&self, blobstoread: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAsync)(::windows::core::Vtable::as_raw(this), blobstoread.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SubmitPropertySetUpdatesAsync<P0, E0, P1, E1>(&self, blobstowrite: P0, blobstodelete: P1, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubmitPropertySetUpdatesAsync)(::windows::core::Vtable::as_raw(this), blobstowrite.try_into().map_err(|e| e.into())?.abi(), blobstodelete.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateBlobInfoQuery(&self, blobnameprefix: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveBlobInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBlobInfoQuery)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(blobnameprefix), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveContainer {}
impl ::core::fmt::Debug for GameSaveContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainer;{c3c08f89-563f-4ecd-9c6f-33fd0e323d10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveContainer {
    type Vtable = IGameSaveContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveContainer {
    const IID: ::windows::core::GUID = <IGameSaveContainer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveContainer {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainer";
}
::windows::core::interface_hierarchy!(GameSaveContainer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveContainer {}
unsafe impl ::core::marker::Sync for GameSaveContainer {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveContainerInfo(::windows::core::IUnknown);
impl GameSaveContainerInfo {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TotalSize(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TotalSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastModifiedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastModifiedTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NeedsSync(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NeedsSync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveContainerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveContainerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveContainerInfo {}
impl ::core::fmt::Debug for GameSaveContainerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveContainerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveContainerInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo;{b7e27300-155d-4bb4-b2ba-930306f391b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveContainerInfo {
    const IID: ::windows::core::GUID = <IGameSaveContainerInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveContainerInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo";
}
::windows::core::interface_hierarchy!(GameSaveContainerInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveContainerInfo {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfo {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveContainerInfoGetResult(::windows::core::IUnknown);
impl GameSaveContainerInfoGetResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveContainerInfoGetResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveContainerInfoGetResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveContainerInfoGetResult {}
impl ::core::fmt::Debug for GameSaveContainerInfoGetResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveContainerInfoGetResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveContainerInfoGetResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult;{ffc50d74-c581-4f9d-9e39-30a10c1e4c50})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveContainerInfoGetResult {
    const IID: ::windows::core::GUID = <IGameSaveContainerInfoGetResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveContainerInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult";
}
::windows::core::interface_hierarchy!(GameSaveContainerInfoGetResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveContainerInfoGetResult {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfoGetResult {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveContainerInfoQuery(::windows::core::IUnknown);
impl GameSaveContainerInfoQuery {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContainerInfoAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContainerInfoAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContainerInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContainerInfoWithIndexAndMaxAsync)(::windows::core::Vtable::as_raw(this), startindex, maxnumberofitems, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetItemCountAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveContainerInfoQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveContainerInfoQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveContainerInfoQuery {}
impl ::core::fmt::Debug for GameSaveContainerInfoQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveContainerInfoQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveContainerInfoQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery;{3c94e863-6f80-4327-9327-ffc11afd42b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveContainerInfoQuery {
    const IID: ::windows::core::GUID = <IGameSaveContainerInfoQuery as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveContainerInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery";
}
::windows::core::interface_hierarchy!(GameSaveContainerInfoQuery, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveContainerInfoQuery {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfoQuery {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveOperationResult(::windows::core::IUnknown);
impl GameSaveOperationResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveOperationResult {}
impl ::core::fmt::Debug for GameSaveOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveOperationResult;{cf0f1a05-24a0-4582-9a55-b1bbbb9388d8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveOperationResult {
    const IID: ::windows::core::GUID = <IGameSaveOperationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveOperationResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveOperationResult";
}
::windows::core::interface_hierarchy!(GameSaveOperationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveOperationResult {}
unsafe impl ::core::marker::Sync for GameSaveOperationResult {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveProvider(::windows::core::IUnknown);
impl GameSaveProvider {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateContainer(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveContainer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateContainer)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteContainerAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteContainerAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateContainerInfoQuery(&self) -> ::windows::core::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateContainerInfoQuery)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateContainerInfoQueryWithName(&self, containernameprefix: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateContainerInfoQueryWithName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(containernameprefix), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRemainingBytesInQuotaAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRemainingBytesInQuotaAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContainersChangedSinceLastSync(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContainersChangedSinceLastSync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetForUserAsync(user: &super::super::super::System::User, serviceconfigid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>> {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(serviceconfigid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetSyncOnDemandForUserAsync(user: &super::super::super::System::User, serviceconfigid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>> {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSyncOnDemandForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(serviceconfigid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameSaveProviderStatics<R, F: FnOnce(&IGameSaveProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameSaveProvider, IGameSaveProviderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GameSaveProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveProvider {}
impl ::core::fmt::Debug for GameSaveProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveProvider;{90a60394-80fe-4211-97f8-a5de14dd95d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveProvider {
    type Vtable = IGameSaveProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveProvider {
    const IID: ::windows::core::GUID = <IGameSaveProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveProvider {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProvider";
}
::windows::core::interface_hierarchy!(GameSaveProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveProvider {}
unsafe impl ::core::marker::Sync for GameSaveProvider {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveProviderGetResult(::windows::core::IUnknown);
impl GameSaveProviderGetResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveProviderGetResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveProviderGetResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveProviderGetResult {}
impl ::core::fmt::Debug for GameSaveProviderGetResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveProviderGetResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveProviderGetResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult;{3ab90816-d393-4d65-ac16-41c3e67ab945})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_Vtbl;
}
unsafe impl ::windows::core::Interface for GameSaveProviderGetResult {
    const IID: ::windows::core::GUID = <IGameSaveProviderGetResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameSaveProviderGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult";
}
::windows::core::interface_hierarchy!(GameSaveProviderGetResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameSaveProviderGetResult {}
unsafe impl ::core::marker::Sync for GameSaveProviderGetResult {}
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameSaveErrorStatus(pub i32);
impl GameSaveErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Abort: Self = Self(-2147467260i32);
    pub const InvalidContainerName: Self = Self(-2138898431i32);
    pub const NoAccess: Self = Self(-2138898430i32);
    pub const OutOfLocalStorage: Self = Self(-2138898429i32);
    pub const UserCanceled: Self = Self(-2138898428i32);
    pub const UpdateTooBig: Self = Self(-2138898427i32);
    pub const QuotaExceeded: Self = Self(-2138898426i32);
    pub const ProvidedBufferTooSmall: Self = Self(-2138898425i32);
    pub const BlobNotFound: Self = Self(-2138898424i32);
    pub const NoXboxLiveInfo: Self = Self(-2138898423i32);
    pub const ContainerNotInSync: Self = Self(-2138898422i32);
    pub const ContainerSyncFailed: Self = Self(-2138898421i32);
    pub const UserHasNoXboxLiveInfo: Self = Self(-2138898420i32);
    pub const ObjectExpired: Self = Self(-2138898419i32);
}
impl ::core::marker::Copy for GameSaveErrorStatus {}
impl ::core::clone::Clone for GameSaveErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameSaveErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GameSaveErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameSaveErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.XboxLive.Storage.GameSaveErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
