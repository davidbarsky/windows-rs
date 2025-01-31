#[doc(hidden)]
#[repr(transparent)]
pub struct IPreviewBuildsManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IPreviewBuildsManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa07dd61_7e4f_59f7_7c9f_def9051c5f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ArePreviewBuildsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetArePreviewBuildsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPreviewBuildsManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPreviewBuildsManagerStatics {
    type Vtable = IPreviewBuildsManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPreviewBuildsManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e422887_b112_5a70_7da1_97d78d32aa29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPreviewBuildsState(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPreviewBuildsState {
    type Vtable = IPreviewBuildsState_Vtbl;
}
unsafe impl ::windows::core::Interface for IPreviewBuildsState {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2f2903e_b223_5f63_7546_3e8eac070a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviewBuildsState_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdate(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdate {
    type Vtable = IWindowsUpdate_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c88dd7_0ef3_52b2_a9ad_66bfc6bd9582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdate_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsFeatureUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsMinorImpact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCritical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsForOS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDriver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsMandatory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUrgent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSeeker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoreInfoUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoreInfoUrl: usize,
    #[cfg(feature = "Foundation")]
    pub SupportUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportUrl: usize,
    pub IsEulaAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub EulaText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub AttentionRequiredInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ActionResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AcceptEula: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateActionCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateActionCompletedEventArgs {
    type Vtable = IWindowsUpdateActionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateActionCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c44b950_a655_5321_aec1_aee762922131);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateActionProgress(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateActionProgress {
    type Vtable = IWindowsUpdateActionProgress_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateActionProgress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83b22d8a_4bb0_549f_ba39_59724882d137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionProgress_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateActionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateActionResult {
    type Vtable = IWindowsUpdateActionResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateActionResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6692c62_f697_51b7_ab7f_e73e5e688f12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateActionResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateAdministrator(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateAdministrator {
    type Vtable = IWindowsUpdateAdministrator_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateAdministrator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a60181c_ba1e_5cf9_aa65_304120b73d72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAdministrator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StartAdministratorScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApproveWindowsUpdateAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateid: *mut ::core::ffi::c_void, action: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeWindowsUpdateActionApproval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateid: *mut ::core::ffi::c_void, action: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApproveWindowsUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateid: *mut ::core::ffi::c_void, approvaldata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeWindowsUpdateApproval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUpdates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateAdministratorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateAdministratorStatics {
    type Vtable = IWindowsUpdateAdministratorStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateAdministratorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x013e6d36_ef69_53bc_8db8_c403bca550ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAdministratorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetRegisteredAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, organizationname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterForAdministration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, organizationname: *mut ::core::ffi::c_void, options: WindowsUpdateAdministratorOptions, result__: *mut WindowsUpdateAdministratorStatus) -> ::windows::core::HRESULT,
    pub UnregisterForAdministration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, organizationname: *mut ::core::ffi::c_void, result__: *mut WindowsUpdateAdministratorStatus) -> ::windows::core::HRESULT,
    pub GetRegisteredAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restartoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelRestartRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestrestarttoken: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateApprovalData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateApprovalData {
    type Vtable = IWindowsUpdateApprovalData_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateApprovalData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaadf5bfd_84db_59bc_85e2_ad4fc1f62f7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateApprovalData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Seeker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seeker: usize,
    #[cfg(feature = "Foundation")]
    pub SetSeeker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSeeker: usize,
    #[cfg(feature = "Foundation")]
    pub AllowDownloadOnMetered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AllowDownloadOnMetered: usize,
    #[cfg(feature = "Foundation")]
    pub SetAllowDownloadOnMetered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAllowDownloadOnMetered: usize,
    #[cfg(feature = "Foundation")]
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComplianceDeadlineInDays: usize,
    #[cfg(feature = "Foundation")]
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetComplianceDeadlineInDays: usize,
    #[cfg(feature = "Foundation")]
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComplianceGracePeriodInDays: usize,
    #[cfg(feature = "Foundation")]
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetComplianceGracePeriodInDays: usize,
    #[cfg(feature = "Foundation")]
    pub OptOutOfAutoReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptOutOfAutoReboot: usize,
    #[cfg(feature = "Foundation")]
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOptOutOfAutoReboot: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateAttentionRequiredInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateAttentionRequiredInfo {
    type Vtable = IWindowsUpdateAttentionRequiredInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateAttentionRequiredInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44df2579_74d3_5ffa_b6ce_09e187e1e0ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAttentionRequiredInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowsUpdateAttentionRequiredReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateAttentionRequiredReasonChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateAttentionRequiredReasonChangedEventArgs {
    type Vtable = IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0627abca_dbb8_524a_b1d2_d9df004eeb31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowsUpdateAttentionRequiredReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateGetAdministratorResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateGetAdministratorResult {
    type Vtable = IWindowsUpdateGetAdministratorResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateGetAdministratorResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb39ffc4_2c42_5b1c_8995_343341c92c50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateGetAdministratorResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Administrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowsUpdateAdministratorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateItem {
    type Vtable = IWindowsUpdateItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb222e44a_49b6_59bf_a033_ef617cd73a98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoreInfoUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoreInfoUrl: usize,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateManager {
    type Vtable = IWindowsUpdateManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5dd966c0_a71a_5602_bbd0_09a70e4573fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ScanningStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScanningStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScanningStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScanningStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub WorkingStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WorkingStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWorkingStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWorkingStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AttentionRequiredReasonChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttentionRequiredReasonChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAttentionRequiredReasonChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAttentionRequiredReasonChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ActionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ScanCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScanCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScanCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScanCompleted: usize,
    pub IsScanning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsWorking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulScanTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulScanTimestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetApplicableUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetApplicableUpdates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMostRecentCompletedUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMostRecentCompletedUpdates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMostRecentCompletedUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMostRecentCompletedUpdatesAsync: usize,
    pub StartScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userinitiated: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateManagerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateManagerFactory {
    type Vtable = IWindowsUpdateManagerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateManagerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b394df8_decb_5f44_b47c_6ccf3bcfdb37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateManagerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateProgressChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateProgressChangedEventArgs {
    type Vtable = IWindowsUpdateProgressChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateProgressChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbfbdeeb_94c8_5aa7_b0fb_66c67c233b0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateProgressChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateRestartRequestOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateRestartRequestOptions {
    type Vtable = IWindowsUpdateRestartRequestOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateRestartRequestOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38cfb7d3_4188_5222_905c_6c4443c951ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateRestartRequestOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoreInfoUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoreInfoUrl: usize,
    #[cfg(feature = "Foundation")]
    pub SetMoreInfoUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMoreInfoUrl: usize,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub OrganizationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOrganizationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OptOutOfAutoReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateRestartRequestOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateRestartRequestOptionsFactory {
    type Vtable = IWindowsUpdateRestartRequestOptionsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateRestartRequestOptionsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75f41d04_0e17_50d0_8c15_6b9d0539b3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateRestartRequestOptionsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::core::ffi::c_void, description: *mut ::core::ffi::c_void, moreinfourl: *mut ::core::ffi::c_void, compliancedeadlineindays: i32, compliancegraceperiodindays: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsUpdateScanCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWindowsUpdateScanCompletedEventArgs {
    type Vtable = IWindowsUpdateScanCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowsUpdateScanCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95b6953e_ba5c_5fe8_b115_12de184a6bb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateScanCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Updates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Updates: usize,
}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct PreviewBuildsManager(::windows::core::IUnknown);
impl PreviewBuildsManager {
    pub fn ArePreviewBuildsAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ArePreviewBuildsAllowed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetArePreviewBuildsAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetArePreviewBuildsAllowed)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetCurrentState(&self) -> ::windows::core::Result<PreviewBuildsState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SyncAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<PreviewBuildsManager> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPreviewBuildsManagerStatics<R, F: FnOnce(&IPreviewBuildsManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PreviewBuildsManager, IPreviewBuildsManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PreviewBuildsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PreviewBuildsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PreviewBuildsManager {}
impl ::core::fmt::Debug for PreviewBuildsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewBuildsManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PreviewBuildsManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsManager;{fa07dd61-7e4f-59f7-7c9f-def9051c5f62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_Vtbl;
}
unsafe impl ::windows::core::Interface for PreviewBuildsManager {
    const IID: ::windows::core::GUID = <IPreviewBuildsManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsManager";
}
::windows::core::interface_hierarchy!(PreviewBuildsManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PreviewBuildsManager {}
unsafe impl ::core::marker::Sync for PreviewBuildsManager {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct PreviewBuildsState(::windows::core::IUnknown);
impl PreviewBuildsState {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PreviewBuildsState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PreviewBuildsState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PreviewBuildsState {}
impl ::core::fmt::Debug for PreviewBuildsState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewBuildsState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PreviewBuildsState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.PreviewBuildsState;{a2f2903e-b223-5f63-7546-3e8eac070a2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PreviewBuildsState {
    type Vtable = IPreviewBuildsState_Vtbl;
}
unsafe impl ::windows::core::Interface for PreviewBuildsState {
    const IID: ::windows::core::GUID = <IPreviewBuildsState as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsState";
}
::windows::core::interface_hierarchy!(PreviewBuildsState, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PreviewBuildsState {}
unsafe impl ::core::marker::Sync for PreviewBuildsState {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdate(::windows::core::IUnknown);
impl WindowsUpdate {
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UpdateId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFeatureUpdate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFeatureUpdate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsMinorImpact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMinorImpact)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSecurity(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSecurity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCritical(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCritical)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsForOS(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsForOS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDriver(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDriver)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsMandatory(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMandatory)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUrgent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUrgent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSeeker(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSeeker)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoreInfoUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoreInfoUrl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportUrl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEulaAccepted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEulaAccepted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EulaText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EulaText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Deadline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AttentionRequiredInfo(&self) -> ::windows::core::Result<WindowsUpdateAttentionRequiredInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttentionRequiredInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ActionResult(&self) -> ::windows::core::Result<WindowsUpdateActionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActionResult)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CurrentAction(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentAction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ActionProgress(&self) -> ::windows::core::Result<WindowsUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActionProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetPropertyValue(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertyValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AcceptEula(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AcceptEula)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for WindowsUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdate {}
impl ::core::fmt::Debug for WindowsUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdate;{c3c88dd7-0ef3-52b2-a9ad-66bfc6bd9582})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdate {
    type Vtable = IWindowsUpdate_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdate {
    const IID: ::windows::core::GUID = <IWindowsUpdate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdate {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdate";
}
::windows::core::interface_hierarchy!(WindowsUpdate, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdate {}
unsafe impl ::core::marker::Sync for WindowsUpdate {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateActionCompletedEventArgs(::windows::core::IUnknown);
impl WindowsUpdateActionCompletedEventArgs {
    pub fn Update(&self) -> ::windows::core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Update)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Action(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Action)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Succeeded)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateActionCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateActionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateActionCompletedEventArgs {}
impl ::core::fmt::Debug for WindowsUpdateActionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateActionCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateActionCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateActionCompletedEventArgs;{2c44b950-a655-5321-aec1-aee762922131})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateActionCompletedEventArgs {
    type Vtable = IWindowsUpdateActionCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateActionCompletedEventArgs {
    const IID: ::windows::core::GUID = <IWindowsUpdateActionCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateActionCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionCompletedEventArgs";
}
::windows::core::interface_hierarchy!(WindowsUpdateActionCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateActionCompletedEventArgs {}
unsafe impl ::core::marker::Sync for WindowsUpdateActionCompletedEventArgs {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateActionProgress(::windows::core::IUnknown);
impl WindowsUpdateActionProgress {
    pub fn Action(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Action)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateActionProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateActionProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateActionProgress {}
impl ::core::fmt::Debug for WindowsUpdateActionProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateActionProgress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateActionProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateActionProgress;{83b22d8a-4bb0-549f-ba39-59724882d137})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateActionProgress {
    type Vtable = IWindowsUpdateActionProgress_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateActionProgress {
    const IID: ::windows::core::GUID = <IWindowsUpdateActionProgress as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateActionProgress {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionProgress";
}
::windows::core::interface_hierarchy!(WindowsUpdateActionProgress, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateActionProgress {}
unsafe impl ::core::marker::Sync for WindowsUpdateActionProgress {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateActionResult(::windows::core::IUnknown);
impl WindowsUpdateActionResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Succeeded)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Action(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Action)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateActionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateActionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateActionResult {}
impl ::core::fmt::Debug for WindowsUpdateActionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateActionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateActionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateActionResult;{e6692c62-f697-51b7-ab7f-e73e5e688f12})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateActionResult {
    type Vtable = IWindowsUpdateActionResult_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateActionResult {
    const IID: ::windows::core::GUID = <IWindowsUpdateActionResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateActionResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionResult";
}
::windows::core::interface_hierarchy!(WindowsUpdateActionResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateActionResult {}
unsafe impl ::core::marker::Sync for WindowsUpdateActionResult {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateAdministrator(::windows::core::IUnknown);
impl WindowsUpdateAdministrator {
    pub fn StartAdministratorScan(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartAdministratorScan)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn ApproveWindowsUpdateAction(&self, updateid: &::windows::core::HSTRING, action: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ApproveWindowsUpdateAction)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(updateid), ::core::mem::transmute_copy(action)).ok() }
    }
    pub fn RevokeWindowsUpdateActionApproval(&self, updateid: &::windows::core::HSTRING, action: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RevokeWindowsUpdateActionApproval)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(updateid), ::core::mem::transmute_copy(action)).ok() }
    }
    pub fn ApproveWindowsUpdate(&self, updateid: &::windows::core::HSTRING, approvaldata: &WindowsUpdateApprovalData) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ApproveWindowsUpdate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(updateid), ::core::mem::transmute_copy(approvaldata)).ok() }
    }
    pub fn RevokeWindowsUpdateApproval(&self, updateid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RevokeWindowsUpdateApproval)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(updateid)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUpdates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUpdates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetRegisteredAdministrator(organizationname: &::windows::core::HSTRING) -> ::windows::core::Result<WindowsUpdateGetAdministratorResult> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRegisteredAdministrator)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(organizationname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RegisterForAdministration(organizationname: &::windows::core::HSTRING, options: WindowsUpdateAdministratorOptions) -> ::windows::core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterForAdministration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(organizationname), options, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UnregisterForAdministration(organizationname: &::windows::core::HSTRING) -> ::windows::core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnregisterForAdministration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(organizationname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetRegisteredAdministratorName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRegisteredAdministratorName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RequestRestart(restartoptions: &WindowsUpdateRestartRequestOptions) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestRestart)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(restartoptions), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CancelRestartRequest(requestrestarttoken: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).CancelRestartRequest)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(requestrestarttoken)).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateAdministratorStatics<R, F: FnOnce(&IWindowsUpdateAdministratorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WindowsUpdateAdministrator, IWindowsUpdateAdministratorStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WindowsUpdateAdministrator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateAdministrator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateAdministrator {}
impl ::core::fmt::Debug for WindowsUpdateAdministrator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministrator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateAdministrator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateAdministrator;{7a60181c-ba1e-5cf9-aa65-304120b73d72})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateAdministrator {
    type Vtable = IWindowsUpdateAdministrator_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateAdministrator {
    const IID: ::windows::core::GUID = <IWindowsUpdateAdministrator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateAdministrator {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAdministrator";
}
::windows::core::interface_hierarchy!(WindowsUpdateAdministrator, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateAdministrator {}
unsafe impl ::core::marker::Sync for WindowsUpdateAdministrator {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateApprovalData(::windows::core::IUnknown);
impl WindowsUpdateApprovalData {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WindowsUpdateApprovalData, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Seeker(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Seeker)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSeeker<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<bool>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSeeker)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AllowDownloadOnMetered(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowDownloadOnMetered)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAllowDownloadOnMetered<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<bool>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowDownloadOnMetered)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ComplianceDeadlineInDays(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComplianceDeadlineInDays)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetComplianceDeadlineInDays<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetComplianceDeadlineInDays)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ComplianceGracePeriodInDays(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComplianceGracePeriodInDays)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetComplianceGracePeriodInDays<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetComplianceGracePeriodInDays)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptOutOfAutoReboot(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OptOutOfAutoReboot)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOptOutOfAutoReboot<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<bool>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOptOutOfAutoReboot)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for WindowsUpdateApprovalData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateApprovalData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateApprovalData {}
impl ::core::fmt::Debug for WindowsUpdateApprovalData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateApprovalData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateApprovalData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateApprovalData;{aadf5bfd-84db-59bc-85e2-ad4fc1f62f7c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateApprovalData {
    type Vtable = IWindowsUpdateApprovalData_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateApprovalData {
    const IID: ::windows::core::GUID = <IWindowsUpdateApprovalData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateApprovalData {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateApprovalData";
}
::windows::core::interface_hierarchy!(WindowsUpdateApprovalData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateApprovalData {}
unsafe impl ::core::marker::Sync for WindowsUpdateApprovalData {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateAttentionRequiredInfo(::windows::core::IUnknown);
impl WindowsUpdateAttentionRequiredInfo {
    pub fn Reason(&self) -> ::windows::core::Result<WindowsUpdateAttentionRequiredReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateAttentionRequiredInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateAttentionRequiredInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateAttentionRequiredInfo {}
impl ::core::fmt::Debug for WindowsUpdateAttentionRequiredInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAttentionRequiredInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateAttentionRequiredInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateAttentionRequiredInfo;{44df2579-74d3-5ffa-b6ce-09e187e1e0ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateAttentionRequiredInfo {
    type Vtable = IWindowsUpdateAttentionRequiredInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateAttentionRequiredInfo {
    const IID: ::windows::core::GUID = <IWindowsUpdateAttentionRequiredInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateAttentionRequiredInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredInfo";
}
::windows::core::interface_hierarchy!(WindowsUpdateAttentionRequiredInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateAttentionRequiredInfo {}
unsafe impl ::core::marker::Sync for WindowsUpdateAttentionRequiredInfo {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateAttentionRequiredReasonChangedEventArgs(::windows::core::IUnknown);
impl WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    pub fn Update(&self) -> ::windows::core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Update)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::core::Result<WindowsUpdateAttentionRequiredReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
impl ::core::fmt::Debug for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAttentionRequiredReasonChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs;{0627abca-dbb8-524a-b1d2-d9df004eeb31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    type Vtable = IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const IID: ::windows::core::GUID = <IWindowsUpdateAttentionRequiredReasonChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs";
}
::windows::core::interface_hierarchy!(WindowsUpdateAttentionRequiredReasonChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
unsafe impl ::core::marker::Sync for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateGetAdministratorResult(::windows::core::IUnknown);
impl WindowsUpdateGetAdministratorResult {
    pub fn Administrator(&self) -> ::windows::core::Result<WindowsUpdateAdministrator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Administrator)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<WindowsUpdateAdministratorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateGetAdministratorResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateGetAdministratorResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateGetAdministratorResult {}
impl ::core::fmt::Debug for WindowsUpdateGetAdministratorResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateGetAdministratorResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateGetAdministratorResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateGetAdministratorResult;{bb39ffc4-2c42-5b1c-8995-343341c92c50})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateGetAdministratorResult {
    type Vtable = IWindowsUpdateGetAdministratorResult_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateGetAdministratorResult {
    const IID: ::windows::core::GUID = <IWindowsUpdateGetAdministratorResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateGetAdministratorResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateGetAdministratorResult";
}
::windows::core::interface_hierarchy!(WindowsUpdateGetAdministratorResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateGetAdministratorResult {}
unsafe impl ::core::marker::Sync for WindowsUpdateGetAdministratorResult {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateItem(::windows::core::IUnknown);
impl WindowsUpdateItem {
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UpdateId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoreInfoUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoreInfoUrl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Category)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Operation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Operation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateItem {}
impl ::core::fmt::Debug for WindowsUpdateItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateItem;{b222e44a-49b6-59bf-a033-ef617cd73a98})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateItem {
    type Vtable = IWindowsUpdateItem_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateItem {
    const IID: ::windows::core::GUID = <IWindowsUpdateItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateItem {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateItem";
}
::windows::core::interface_hierarchy!(WindowsUpdateItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateItem {}
unsafe impl ::core::marker::Sync for WindowsUpdateItem {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateManager(::windows::core::IUnknown);
impl WindowsUpdateManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScanningStateChanged(&self, handler: &super::super::Foundation::TypedEventHandler<WindowsUpdateManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScanningStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScanningStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveScanningStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WorkingStateChanged(&self, handler: &super::super::Foundation::TypedEventHandler<WindowsUpdateManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WorkingStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWorkingStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveWorkingStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProgressChanged(&self, handler: &super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateProgressChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProgressChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgressChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveProgressChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttentionRequiredReasonChanged(&self, handler: &super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateAttentionRequiredReasonChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttentionRequiredReasonChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAttentionRequiredReasonChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAttentionRequiredReasonChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActionCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateActionCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActionCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActionCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveActionCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScanCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateScanCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScanCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScanCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveScanCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn IsScanning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsScanning)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsWorking(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsWorking)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulScanTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastSuccessfulScanTimestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetApplicableUpdates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetApplicableUpdates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMostRecentCompletedUpdates(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdateItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMostRecentCompletedUpdates)(::windows::core::Vtable::as_raw(this), count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMostRecentCompletedUpdatesAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WindowsUpdateItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMostRecentCompletedUpdatesAsync)(::windows::core::Vtable::as_raw(this), count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartScan(&self, userinitiated: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartScan)(::windows::core::Vtable::as_raw(this), userinitiated).ok() }
    }
    pub fn CreateInstance(clientid: &::windows::core::HSTRING) -> ::windows::core::Result<WindowsUpdateManager> {
        Self::IWindowsUpdateManagerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(clientid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateManagerFactory<R, F: FnOnce(&IWindowsUpdateManagerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WindowsUpdateManager, IWindowsUpdateManagerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WindowsUpdateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateManager {}
impl ::core::fmt::Debug for WindowsUpdateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateManager;{5dd966c0-a71a-5602-bbd0-09a70e4573fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateManager {
    type Vtable = IWindowsUpdateManager_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateManager {
    const IID: ::windows::core::GUID = <IWindowsUpdateManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateManager {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateManager";
}
::windows::core::interface_hierarchy!(WindowsUpdateManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateManager {}
unsafe impl ::core::marker::Sync for WindowsUpdateManager {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateProgressChangedEventArgs(::windows::core::IUnknown);
impl WindowsUpdateProgressChangedEventArgs {
    pub fn Update(&self) -> ::windows::core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Update)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ActionProgress(&self) -> ::windows::core::Result<WindowsUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActionProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateProgressChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateProgressChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateProgressChangedEventArgs {}
impl ::core::fmt::Debug for WindowsUpdateProgressChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateProgressChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateProgressChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateProgressChangedEventArgs;{bbfbdeeb-94c8-5aa7-b0fb-66c67c233b0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateProgressChangedEventArgs {
    type Vtable = IWindowsUpdateProgressChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateProgressChangedEventArgs {
    const IID: ::windows::core::GUID = <IWindowsUpdateProgressChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateProgressChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateProgressChangedEventArgs";
}
::windows::core::interface_hierarchy!(WindowsUpdateProgressChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateProgressChangedEventArgs {}
unsafe impl ::core::marker::Sync for WindowsUpdateProgressChangedEventArgs {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateRestartRequestOptions(::windows::core::IUnknown);
impl WindowsUpdateRestartRequestOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WindowsUpdateRestartRequestOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoreInfoUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoreInfoUrl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMoreInfoUrl(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMoreInfoUrl)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ComplianceDeadlineInDays(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComplianceDeadlineInDays)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComplianceDeadlineInDays(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetComplianceDeadlineInDays)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComplianceGracePeriodInDays)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComplianceGracePeriodInDays(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetComplianceGracePeriodInDays)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn OrganizationName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrganizationName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOrganizationName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOrganizationName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn OptOutOfAutoReboot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OptOutOfAutoReboot)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOptOutOfAutoReboot(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOptOutOfAutoReboot)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstance(title: &::windows::core::HSTRING, description: &::windows::core::HSTRING, moreinfourl: &super::super::Foundation::Uri, compliancedeadlineindays: i32, compliancegraceperiodindays: i32) -> ::windows::core::Result<WindowsUpdateRestartRequestOptions> {
        Self::IWindowsUpdateRestartRequestOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(description), ::core::mem::transmute_copy(moreinfourl), compliancedeadlineindays, compliancegraceperiodindays, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateRestartRequestOptionsFactory<R, F: FnOnce(&IWindowsUpdateRestartRequestOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WindowsUpdateRestartRequestOptions, IWindowsUpdateRestartRequestOptionsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WindowsUpdateRestartRequestOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateRestartRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateRestartRequestOptions {}
impl ::core::fmt::Debug for WindowsUpdateRestartRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateRestartRequestOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateRestartRequestOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateRestartRequestOptions;{38cfb7d3-4188-5222-905c-6c4443c951ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateRestartRequestOptions {
    type Vtable = IWindowsUpdateRestartRequestOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateRestartRequestOptions {
    const IID: ::windows::core::GUID = <IWindowsUpdateRestartRequestOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateRestartRequestOptions {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateRestartRequestOptions";
}
::windows::core::interface_hierarchy!(WindowsUpdateRestartRequestOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateRestartRequestOptions {}
unsafe impl ::core::marker::Sync for WindowsUpdateRestartRequestOptions {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
pub struct WindowsUpdateScanCompletedEventArgs(::windows::core::IUnknown);
impl WindowsUpdateScanCompletedEventArgs {
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Succeeded)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Updates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Updates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WindowsUpdateScanCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateScanCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateScanCompletedEventArgs {}
impl ::core::fmt::Debug for WindowsUpdateScanCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateScanCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateScanCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Update.WindowsUpdateScanCompletedEventArgs;{95b6953e-ba5c-5fe8-b115-12de184a6bb0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WindowsUpdateScanCompletedEventArgs {
    type Vtable = IWindowsUpdateScanCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WindowsUpdateScanCompletedEventArgs {
    const IID: ::windows::core::GUID = <IWindowsUpdateScanCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowsUpdateScanCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateScanCompletedEventArgs";
}
::windows::core::interface_hierarchy!(WindowsUpdateScanCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WindowsUpdateScanCompletedEventArgs {}
unsafe impl ::core::marker::Sync for WindowsUpdateScanCompletedEventArgs {}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowsUpdateAdministratorOptions(pub u32);
impl WindowsUpdateAdministratorOptions {
    pub const None: Self = Self(0u32);
    pub const RequireAdministratorApprovalForScans: Self = Self(1u32);
    pub const RequireAdministratorApprovalForUpdates: Self = Self(2u32);
    pub const RequireAdministratorApprovalForActions: Self = Self(4u32);
}
impl ::core::marker::Copy for WindowsUpdateAdministratorOptions {}
impl ::core::clone::Clone for WindowsUpdateAdministratorOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowsUpdateAdministratorOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WindowsUpdateAdministratorOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for WindowsUpdateAdministratorOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministratorOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WindowsUpdateAdministratorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WindowsUpdateAdministratorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateAdministratorOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowsUpdateAdministratorStatus(pub i32);
impl WindowsUpdateAdministratorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoAdministratorRegistered: Self = Self(1i32);
    pub const OtherAdministratorIsRegistered: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowsUpdateAdministratorStatus {}
impl ::core::clone::Clone for WindowsUpdateAdministratorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowsUpdateAdministratorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WindowsUpdateAdministratorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WindowsUpdateAdministratorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministratorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateAdministratorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Management_Update\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowsUpdateAttentionRequiredReason(pub i32);
impl WindowsUpdateAttentionRequiredReason {
    pub const None: Self = Self(0i32);
    pub const SeekerUpdate: Self = Self(1i32);
    pub const ReadyToReboot: Self = Self(2i32);
    pub const NeedNonMeteredNetwork: Self = Self(3i32);
    pub const NeedUserAgreementForMeteredNetwork: Self = Self(4i32);
    pub const NeedNetwork: Self = Self(5i32);
    pub const NeedMoreSpace: Self = Self(6i32);
    pub const BatterySaverEnabled: Self = Self(7i32);
    pub const NeedUserInteraction: Self = Self(8i32);
    pub const NeedUserAgreementForPolicy: Self = Self(9i32);
    pub const CompatibilityError: Self = Self(10i32);
    pub const NeedUserInteractionForEula: Self = Self(11i32);
    pub const NeedUserInteractionForCta: Self = Self(12i32);
    pub const Regulated: Self = Self(13i32);
    pub const ExternalReboot: Self = Self(14i32);
    pub const OtherUpdate: Self = Self(15i32);
    pub const BlockedByProvider: Self = Self(16i32);
    pub const BlockedByPostRebootFailure: Self = Self(17i32);
    pub const UserEngaged: Self = Self(18i32);
    pub const BlockedByBattery: Self = Self(19i32);
    pub const Exclusivity: Self = Self(20i32);
    pub const BlockedBySerialization: Self = Self(21i32);
    pub const ConflictClass: Self = Self(22i32);
    pub const BlockedByAdminApproval: Self = Self(23i32);
    pub const BlockedByTooManyAttempts: Self = Self(24i32);
    pub const BlockedByFailure: Self = Self(25i32);
    pub const Demotion: Self = Self(26i32);
    pub const BlockedByActiveHours: Self = Self(27i32);
    pub const ScheduledForMaintenance: Self = Self(28i32);
    pub const PolicyScheduledInstallTime: Self = Self(29i32);
    pub const BlockedByOobe: Self = Self(30i32);
    pub const DeferredDuringOobe: Self = Self(31i32);
    pub const DeferredForSustainableTime: Self = Self(32i32);
}
impl ::core::marker::Copy for WindowsUpdateAttentionRequiredReason {}
impl ::core::clone::Clone for WindowsUpdateAttentionRequiredReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowsUpdateAttentionRequiredReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WindowsUpdateAttentionRequiredReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for WindowsUpdateAttentionRequiredReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAttentionRequiredReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowsUpdateAttentionRequiredReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAttentionRequiredReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
