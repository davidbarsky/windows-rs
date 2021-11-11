#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const AppDomainHelper: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xef24f689_14f8_4d92_b4af_d7b1f0e70fd4);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
    pub TimeRecycled: super::super::Foundation::FILETIME,
    pub TimeToTerminate: super::super::Foundation::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: super::super::Foundation::BOOL,
    pub HasAutomaticLifetimeRecycling: super::super::Foundation::BOOL,
    pub TimeForAutomaticRecycling: super::super::Foundation::FILETIME,
    pub MemoryLimitInKB: u32,
    pub MemoryUsageInKBLastCheck: u32,
    pub ActivationLimit: u32,
    pub NumActivationsLastReported: u32,
    pub CallLimit: u32,
    pub NumCallsLastReported: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ApplicationProcessRecycleInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ApplicationProcessRecycleInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ApplicationProcessRecycleInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ApplicationProcessRecycleInfo")
            .field("IsRecyclable", &self.IsRecyclable)
            .field("IsRecycled", &self.IsRecycled)
            .field("TimeRecycled", &self.TimeRecycled)
            .field("TimeToTerminate", &self.TimeToTerminate)
            .field("RecycleReasonCode", &self.RecycleReasonCode)
            .field("IsPendingRecycle", &self.IsPendingRecycle)
            .field("HasAutomaticLifetimeRecycling", &self.HasAutomaticLifetimeRecycling)
            .field("TimeForAutomaticRecycling", &self.TimeForAutomaticRecycling)
            .field("MemoryLimitInKB", &self.MemoryLimitInKB)
            .field("MemoryUsageInKBLastCheck", &self.MemoryUsageInKBLastCheck)
            .field("ActivationLimit", &self.ActivationLimit)
            .field("NumActivationsLastReported", &self.NumActivationsLastReported)
            .field("CallLimit", &self.CallLimit)
            .field("NumCallsLastReported", &self.NumCallsLastReported)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ApplicationProcessRecycleInfo {
    fn eq(&self, other: &Self) -> bool {
        self.IsRecyclable == other.IsRecyclable
            && self.IsRecycled == other.IsRecycled
            && self.TimeRecycled == other.TimeRecycled
            && self.TimeToTerminate == other.TimeToTerminate
            && self.RecycleReasonCode == other.RecycleReasonCode
            && self.IsPendingRecycle == other.IsPendingRecycle
            && self.HasAutomaticLifetimeRecycling == other.HasAutomaticLifetimeRecycling
            && self.TimeForAutomaticRecycling == other.TimeForAutomaticRecycling
            && self.MemoryLimitInKB == other.MemoryLimitInKB
            && self.MemoryUsageInKBLastCheck == other.MemoryUsageInKBLastCheck
            && self.ActivationLimit == other.ActivationLimit
            && self.NumActivationsLastReported == other.NumActivationsLastReported
            && self.CallLimit == other.CallLimit
            && self.NumCallsLastReported == other.NumCallsLastReported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ApplicationProcessRecycleInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ApplicationProcessRecycleInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub struct ApplicationProcessStatistics {
    pub NumCallsOutstanding: u32,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
    pub AvgCallsPerSecond: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl ApplicationProcessStatistics {}
impl ::core::default::Default for ApplicationProcessStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ApplicationProcessStatistics {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ApplicationProcessStatistics")
            .field("NumCallsOutstanding", &self.NumCallsOutstanding)
            .field("NumTrackedComponents", &self.NumTrackedComponents)
            .field("NumComponentInstances", &self.NumComponentInstances)
            .field("AvgCallsPerSecond", &self.AvgCallsPerSecond)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ApplicationProcessStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.NumCallsOutstanding == other.NumCallsOutstanding && self.NumTrackedComponents == other.NumTrackedComponents && self.NumComponentInstances == other.NumComponentInstances && self.AvgCallsPerSecond == other.AvgCallsPerSecond && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3 && self.Reserved4 == other.Reserved4
    }
}
impl ::core::cmp::Eq for ApplicationProcessStatistics {}
unsafe impl ::windows::runtime::Abi for ApplicationProcessStatistics {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: ::windows::runtime::GUID,
    pub ApplicationIdPrimaryApplication: ::windows::runtime::GUID,
    pub ApplicationInstanceId: ::windows::runtime::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: super::super::Foundation::PWSTR,
    pub IsService: super::super::Foundation::BOOL,
    pub IsPaused: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ApplicationProcessSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ApplicationProcessSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ApplicationProcessSummary {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ApplicationProcessSummary")
            .field("PartitionIdPrimaryApplication", &self.PartitionIdPrimaryApplication)
            .field("ApplicationIdPrimaryApplication", &self.ApplicationIdPrimaryApplication)
            .field("ApplicationInstanceId", &self.ApplicationInstanceId)
            .field("ProcessId", &self.ProcessId)
            .field("Type", &self.Type)
            .field("ProcessExeName", &self.ProcessExeName)
            .field("IsService", &self.IsService)
            .field("IsPaused", &self.IsPaused)
            .field("IsRecycled", &self.IsRecycled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ApplicationProcessSummary {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionIdPrimaryApplication == other.PartitionIdPrimaryApplication && self.ApplicationIdPrimaryApplication == other.ApplicationIdPrimaryApplication && self.ApplicationInstanceId == other.ApplicationInstanceId && self.ProcessId == other.ProcessId && self.Type == other.Type && self.ProcessExeName == other.ProcessExeName && self.IsService == other.IsService && self.IsPaused == other.IsPaused && self.IsRecycled == other.IsRecycled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ApplicationProcessSummary {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ApplicationProcessSummary {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: ::windows::runtime::GUID,
    pub PartitionId: ::windows::runtime::GUID,
    pub ApplicationId: ::windows::runtime::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: super::super::Foundation::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ApplicationSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ApplicationSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ApplicationSummary {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ApplicationSummary")
            .field("ApplicationInstanceId", &self.ApplicationInstanceId)
            .field("PartitionId", &self.PartitionId)
            .field("ApplicationId", &self.ApplicationId)
            .field("Type", &self.Type)
            .field("ApplicationName", &self.ApplicationName)
            .field("NumTrackedComponents", &self.NumTrackedComponents)
            .field("NumComponentInstances", &self.NumComponentInstances)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ApplicationSummary {
    fn eq(&self, other: &Self) -> bool {
        self.ApplicationInstanceId == other.ApplicationInstanceId && self.PartitionId == other.PartitionId && self.ApplicationId == other.ApplicationId && self.Type == other.Type && self.ApplicationName == other.ApplicationName && self.NumTrackedComponents == other.NumTrackedComponents && self.NumComponentInstances == other.NumComponentInstances
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ApplicationSummary {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ApplicationSummary {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutoSvcs_Error_Constants(pub u32);
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803138u32);
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803139u32);
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803140u32);
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803141u32);
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803142u32);
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803143u32);
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803148u32);
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803149u32);
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803150u32);
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803151u32);
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599296u32);
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599297u32);
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599298u32);
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599299u32);
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599300u32);
pub const comqcErrOutParam: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599301u32);
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599302u32);
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599303u32);
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599304u32);
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599376u32);
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599377u32);
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599378u32);
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599379u32);
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599380u32);
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599381u32);
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599382u32);
impl ::core::convert::From<u32> for AutoSvcs_Error_Constants {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutoSvcs_Error_Constants {
    type Abi = Self;
}
impl ::core::ops::BitOr for AutoSvcs_Error_Constants {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AutoSvcs_Error_Constants {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AutoSvcs_Error_Constants {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AutoSvcs_Error_Constants {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AutoSvcs_Error_Constants {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ByotServerEx: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0aa_7f19_11d2_978e_0000f8757e2a);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub struct CAppData {
    pub m_idApp: u32,
    pub m_szAppGuid: [u16; 40],
    pub m_dwAppProcessId: u32,
    pub m_AppStatistics: CAppStatistics,
}
impl CAppData {}
impl ::core::default::Default for CAppData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CAppData {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CAppData").field("m_idApp", &self.m_idApp).field("m_szAppGuid", &self.m_szAppGuid).field("m_dwAppProcessId", &self.m_dwAppProcessId).field("m_AppStatistics", &self.m_AppStatistics).finish()
    }
}
impl ::core::cmp::PartialEq for CAppData {
    fn eq(&self, other: &Self) -> bool {
        self.m_idApp == other.m_idApp && self.m_szAppGuid == other.m_szAppGuid && self.m_dwAppProcessId == other.m_dwAppProcessId && self.m_AppStatistics == other.m_AppStatistics
    }
}
impl ::core::cmp::Eq for CAppData {}
unsafe impl ::windows::runtime::Abi for CAppData {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub struct CAppStatistics {
    pub m_cTotalCalls: u32,
    pub m_cTotalInstances: u32,
    pub m_cTotalClasses: u32,
    pub m_cCallsPerSecond: u32,
}
impl CAppStatistics {}
impl ::core::default::Default for CAppStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CAppStatistics {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CAppStatistics").field("m_cTotalCalls", &self.m_cTotalCalls).field("m_cTotalInstances", &self.m_cTotalInstances).field("m_cTotalClasses", &self.m_cTotalClasses).field("m_cCallsPerSecond", &self.m_cCallsPerSecond).finish()
    }
}
impl ::core::cmp::PartialEq for CAppStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.m_cTotalCalls == other.m_cTotalCalls && self.m_cTotalInstances == other.m_cTotalInstances && self.m_cTotalClasses == other.m_cTotalClasses && self.m_cCallsPerSecond == other.m_cCallsPerSecond
    }
}
impl ::core::cmp::Eq for CAppStatistics {}
unsafe impl ::windows::runtime::Abi for CAppStatistics {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub struct CCLSIDData {
    pub m_clsid: ::windows::runtime::GUID,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl CCLSIDData {}
impl ::core::default::Default for CCLSIDData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CCLSIDData {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CCLSIDData")
            .field("m_clsid", &self.m_clsid)
            .field("m_cReferences", &self.m_cReferences)
            .field("m_cBound", &self.m_cBound)
            .field("m_cPooled", &self.m_cPooled)
            .field("m_cInCall", &self.m_cInCall)
            .field("m_dwRespTime", &self.m_dwRespTime)
            .field("m_cCallsCompleted", &self.m_cCallsCompleted)
            .field("m_cCallsFailed", &self.m_cCallsFailed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CCLSIDData {
    fn eq(&self, other: &Self) -> bool {
        self.m_clsid == other.m_clsid && self.m_cReferences == other.m_cReferences && self.m_cBound == other.m_cBound && self.m_cPooled == other.m_cPooled && self.m_cInCall == other.m_cInCall && self.m_dwRespTime == other.m_dwRespTime && self.m_cCallsCompleted == other.m_cCallsCompleted && self.m_cCallsFailed == other.m_cCallsFailed
    }
}
impl ::core::cmp::Eq for CCLSIDData {}
unsafe impl ::windows::runtime::Abi for CCLSIDData {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
pub struct CCLSIDData2 {
    pub m_clsid: ::windows::runtime::GUID,
    pub m_appid: ::windows::runtime::GUID,
    pub m_partid: ::windows::runtime::GUID,
    pub m_pwszAppName: super::super::Foundation::PWSTR,
    pub m_pwszCtxName: super::super::Foundation::PWSTR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CCLSIDData2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CCLSIDData2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CCLSIDData2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CCLSIDData2")
            .field("m_clsid", &self.m_clsid)
            .field("m_appid", &self.m_appid)
            .field("m_partid", &self.m_partid)
            .field("m_pwszAppName", &self.m_pwszAppName)
            .field("m_pwszCtxName", &self.m_pwszCtxName)
            .field("m_eAppType", &self.m_eAppType)
            .field("m_cReferences", &self.m_cReferences)
            .field("m_cBound", &self.m_cBound)
            .field("m_cPooled", &self.m_cPooled)
            .field("m_cInCall", &self.m_cInCall)
            .field("m_dwRespTime", &self.m_dwRespTime)
            .field("m_cCallsCompleted", &self.m_cCallsCompleted)
            .field("m_cCallsFailed", &self.m_cCallsFailed)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CCLSIDData2 {
    fn eq(&self, other: &Self) -> bool {
        self.m_clsid == other.m_clsid
            && self.m_appid == other.m_appid
            && self.m_partid == other.m_partid
            && self.m_pwszAppName == other.m_pwszAppName
            && self.m_pwszCtxName == other.m_pwszCtxName
            && self.m_eAppType == other.m_eAppType
            && self.m_cReferences == other.m_cReferences
            && self.m_cBound == other.m_cBound
            && self.m_cPooled == other.m_cPooled
            && self.m_cInCall == other.m_cInCall
            && self.m_dwRespTime == other.m_dwRespTime
            && self.m_cCallsCompleted == other.m_cCallsCompleted
            && self.m_cCallsFailed == other.m_cCallsFailed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CCLSIDData2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CCLSIDData2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminAccessChecksLevelOptions(pub i32);
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(0i32);
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(1i32);
impl ::core::convert::From<i32> for COMAdminAccessChecksLevelOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminAccessChecksLevelOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminActivationOptions(pub i32);
pub const COMAdminActivationInproc: COMAdminActivationOptions = COMAdminActivationOptions(0i32);
pub const COMAdminActivationLocal: COMAdminActivationOptions = COMAdminActivationOptions(1i32);
impl ::core::convert::From<i32> for COMAdminActivationOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminActivationOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminApplicationExportOptions(pub i32);
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(0i32);
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(1i32);
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(2i32);
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(4i32);
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(16i32);
impl ::core::convert::From<i32> for COMAdminApplicationExportOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminApplicationExportOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminApplicationInstallOptions(pub i32);
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(0i32);
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(1i32);
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(2i32);
impl ::core::convert::From<i32> for COMAdminApplicationInstallOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminApplicationInstallOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminAuthenticationCapabilitiesOptions(pub i32);
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(0i32);
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(2i32);
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(32i32);
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(64i32);
impl ::core::convert::From<i32> for COMAdminAuthenticationCapabilitiesOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminAuthenticationCapabilitiesOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminAuthenticationLevelOptions(pub i32);
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(0i32);
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(1i32);
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(2i32);
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(3i32);
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(4i32);
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(5i32);
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(6i32);
impl ::core::convert::From<i32> for COMAdminAuthenticationLevelOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminAuthenticationLevelOptions {
    type Abi = Self;
}
pub const COMAdminCatalog: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf618c514_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf618c516_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogObject: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf618c515_dfb8_11d1_a2cf_00805fc79235);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminComponentFlags(pub i32);
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = COMAdminComponentFlags(1i32);
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = COMAdminComponentFlags(2i32);
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = COMAdminComponentFlags(4i32);
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = COMAdminComponentFlags(8i32);
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = COMAdminComponentFlags(16i32);
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = COMAdminComponentFlags(32i32);
impl ::core::convert::From<i32> for COMAdminComponentFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminComponentFlags {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminComponentType(pub i32);
pub const COMAdmin32BitComponent: COMAdminComponentType = COMAdminComponentType(1i32);
pub const COMAdmin64BitComponent: COMAdminComponentType = COMAdminComponentType(2i32);
impl ::core::convert::From<i32> for COMAdminComponentType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminComponentType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminErrorCodes(pub i32);
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = COMAdminErrorCodes(-2146368511i32);
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368510i32);
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146368509i32);
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368508i32);
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368505i32);
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368504i32);
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = COMAdminErrorCodes(-2146368503i32);
pub const COMAdminErrBadPath: COMAdminErrorCodes = COMAdminErrorCodes(-2146368502i32);
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368501i32);
pub const COMAdminErrRoleExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368500i32);
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = COMAdminErrorCodes(-2146368499i32);
pub const COMAdminErrNoUser: COMAdminErrorCodes = COMAdminErrorCodes(-2146368497i32);
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = COMAdminErrorCodes(-2146368496i32);
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368495i32);
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368494i32);
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = COMAdminErrorCodes(-2146368493i32);
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368492i32);
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368488i32);
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = COMAdminErrorCodes(-2146368487i32);
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = COMAdminErrorCodes(-2146368486i32);
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = COMAdminErrorCodes(-2146368485i32);
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368483i32);
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368482i32);
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368481i32);
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368477i32);
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368476i32);
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368475i32);
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = COMAdminErrorCodes(-2146368474i32);
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368473i32);
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368472i32);
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368471i32);
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368470i32);
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368469i32);
pub const COMAdminErrSession: COMAdminErrorCodes = COMAdminErrorCodes(-2146368468i32);
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = COMAdminErrorCodes(-2146368467i32);
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146368466i32);
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368464i32);
pub const COMAdminErrSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368461i32);
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = COMAdminErrorCodes(-2146368460i32);
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368459i32);
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368458i32);
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368457i32);
pub const COMAdminErrObjectExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368456i32);
pub const COMAdminErrComponentExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368455i32);
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = COMAdminErrorCodes(-2146368453i32);
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = COMAdminErrorCodes(-2146368452i32);
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = COMAdminErrorCodes(-2146368450i32);
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368449i32);
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368442i32);
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368441i32);
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = COMAdminErrorCodes(-2146368440i32);
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = COMAdminErrorCodes(-2146368439i32);
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = COMAdminErrorCodes(-2146367998i32);
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146367480i32);
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146367479i32);
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = COMAdminErrorCodes(-2146368438i32);
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368437i32);
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368436i32);
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = COMAdminErrorCodes(-2146368435i32);
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146367478i32);
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = COMAdminErrorCodes(-2146368434i32);
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = COMAdminErrorCodes(-2146368433i32);
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146368432i32);
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = COMAdminErrorCodes(-2146368425i32);
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = COMAdminErrorCodes(-2146368423i32);
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368421i32);
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = COMAdminErrorCodes(-2146368398i32);
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = COMAdminErrorCodes(-2146368397i32);
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = COMAdminErrorCodes(-2146368396i32);
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146368395i32);
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368384i32);
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368383i32);
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368382i32);
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368381i32);
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368380i32);
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368379i32);
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = COMAdminErrorCodes(-2146368378i32);
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367473i32);
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367471i32);
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367470i32);
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367469i32);
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = COMAdminErrorCodes(-2146367477i32);
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146367463i32);
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368431i32);
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = COMAdminErrorCodes(-2146367460i32);
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146367459i32);
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = COMAdminErrorCodes(-2146367458i32);
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = COMAdminErrorCodes(-2146367456i32);
impl ::core::convert::From<i32> for COMAdminErrorCodes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminErrorCodes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminFileFlags(pub i32);
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = COMAdminFileFlags(1i32);
pub const COMAdminFileFlagCOM: COMAdminFileFlags = COMAdminFileFlags(2i32);
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = COMAdminFileFlags(4i32);
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = COMAdminFileFlags(8i32);
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = COMAdminFileFlags(16i32);
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = COMAdminFileFlags(32i32);
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = COMAdminFileFlags(64i32);
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = COMAdminFileFlags(128i32);
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = COMAdminFileFlags(256i32);
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = COMAdminFileFlags(512i32);
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = COMAdminFileFlags(1024i32);
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = COMAdminFileFlags(2048i32);
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = COMAdminFileFlags(4096i32);
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = COMAdminFileFlags(8192i32);
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = COMAdminFileFlags(16384i32);
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = COMAdminFileFlags(32768i32);
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = COMAdminFileFlags(65536i32);
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = COMAdminFileFlags(131072i32);
pub const COMAdminFileFlagError: COMAdminFileFlags = COMAdminFileFlags(262144i32);
impl ::core::convert::From<i32> for COMAdminFileFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminFileFlags {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminImpersonationLevelOptions(pub i32);
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(1i32);
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(2i32);
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(3i32);
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(4i32);
impl ::core::convert::From<i32> for COMAdminImpersonationLevelOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminImpersonationLevelOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminInUse(pub i32);
pub const COMAdminNotInUse: COMAdminInUse = COMAdminInUse(0i32);
pub const COMAdminInUseByCatalog: COMAdminInUse = COMAdminInUse(1i32);
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = COMAdminInUse(2i32);
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = COMAdminInUse(3i32);
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = COMAdminInUse(4i32);
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = COMAdminInUse(5i32);
impl ::core::convert::From<i32> for COMAdminInUse {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminInUse {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminOS(pub i32);
pub const COMAdminOSNotInitialized: COMAdminOS = COMAdminOS(0i32);
pub const COMAdminOSWindows3_1: COMAdminOS = COMAdminOS(1i32);
pub const COMAdminOSWindows9x: COMAdminOS = COMAdminOS(2i32);
pub const COMAdminOSWindows2000: COMAdminOS = COMAdminOS(3i32);
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = COMAdminOS(4i32);
pub const COMAdminOSWindows2000Unknown: COMAdminOS = COMAdminOS(5i32);
pub const COMAdminOSUnknown: COMAdminOS = COMAdminOS(6i32);
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = COMAdminOS(11i32);
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = COMAdminOS(12i32);
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = COMAdminOS(13i32);
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = COMAdminOS(14i32);
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = COMAdminOS(15i32);
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = COMAdminOS(16i32);
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = COMAdminOS(17i32);
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = COMAdminOS(18i32);
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = COMAdminOS(19i32);
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = COMAdminOS(20i32);
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = COMAdminOS(21i32);
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = COMAdminOS(22i32);
pub const COMAdminOSWindows7Personal: COMAdminOS = COMAdminOS(23i32);
pub const COMAdminOSWindows7Professional: COMAdminOS = COMAdminOS(24i32);
pub const COMAdminOSWindows7StandardServer: COMAdminOS = COMAdminOS(25i32);
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = COMAdminOS(26i32);
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = COMAdminOS(27i32);
pub const COMAdminOSWindows7WebServer: COMAdminOS = COMAdminOS(28i32);
pub const COMAdminOSWindows8Personal: COMAdminOS = COMAdminOS(29i32);
pub const COMAdminOSWindows8Professional: COMAdminOS = COMAdminOS(30i32);
pub const COMAdminOSWindows8StandardServer: COMAdminOS = COMAdminOS(31i32);
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = COMAdminOS(32i32);
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = COMAdminOS(33i32);
pub const COMAdminOSWindows8WebServer: COMAdminOS = COMAdminOS(34i32);
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = COMAdminOS(35i32);
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = COMAdminOS(36i32);
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = COMAdminOS(37i32);
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = COMAdminOS(38i32);
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = COMAdminOS(39i32);
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = COMAdminOS(40i32);
impl ::core::convert::From<i32> for COMAdminOS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminOS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminQCMessageAuthenticateOptions(pub i32);
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(0i32);
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(1i32);
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(2i32);
impl ::core::convert::From<i32> for COMAdminQCMessageAuthenticateOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminQCMessageAuthenticateOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminServiceOptions(pub i32);
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = COMAdminServiceOptions(1i32);
impl ::core::convert::From<i32> for COMAdminServiceOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminServiceOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminServiceStatusOptions(pub i32);
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(0i32);
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(1i32);
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(2i32);
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(3i32);
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(4i32);
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(5i32);
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(6i32);
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(7i32);
impl ::core::convert::From<i32> for COMAdminServiceStatusOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminServiceStatusOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminSynchronizationOptions(pub i32);
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(0i32);
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(1i32);
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(2i32);
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(3i32);
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(4i32);
impl ::core::convert::From<i32> for COMAdminSynchronizationOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminSynchronizationOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminThreadingModels(pub i32);
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = COMAdminThreadingModels(0i32);
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = COMAdminThreadingModels(1i32);
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = COMAdminThreadingModels(2i32);
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = COMAdminThreadingModels(3i32);
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = COMAdminThreadingModels(4i32);
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = COMAdminThreadingModels(5i32);
impl ::core::convert::From<i32> for COMAdminThreadingModels {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminThreadingModels {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminTransactionOptions(pub i32);
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = COMAdminTransactionOptions(0i32);
pub const COMAdminTransactionNone: COMAdminTransactionOptions = COMAdminTransactionOptions(1i32);
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = COMAdminTransactionOptions(2i32);
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = COMAdminTransactionOptions(3i32);
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = COMAdminTransactionOptions(4i32);
impl ::core::convert::From<i32> for COMAdminTransactionOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminTransactionOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMAdminTxIsolationLevelOptions(pub i32);
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(0i32);
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(1i32);
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(2i32);
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(3i32);
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(4i32);
impl ::core::convert::From<i32> for COMAdminTxIsolationLevelOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMAdminTxIsolationLevelOptions {
    type Abi = Self;
}
pub const COMEvents: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0ab_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMPLUS_APPTYPE(pub i32);
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = COMPLUS_APPTYPE(-1i32);
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = COMPLUS_APPTYPE(1i32);
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = COMPLUS_APPTYPE(0i32);
pub const APPTYPE_SWC: COMPLUS_APPTYPE = COMPLUS_APPTYPE(2i32);
impl ::core::convert::From<i32> for COMPLUS_APPTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMPLUS_APPTYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: ::windows::runtime::GUID,
    pub sMachineName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl COMSVCSEVENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMSVCSEVENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMSVCSEVENTINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COMSVCSEVENTINFO").field("cbSize", &self.cbSize).field("dwPid", &self.dwPid).field("lTime", &self.lTime).field("lMicroTime", &self.lMicroTime).field("perfCount", &self.perfCount).field("guidApp", &self.guidApp).field("sMachineName", &self.sMachineName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMSVCSEVENTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwPid == other.dwPid && self.lTime == other.lTime && self.lMicroTime == other.lMicroTime && self.perfCount == other.perfCount && self.guidApp == other.guidApp && self.sMachineName == other.sMachineName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMSVCSEVENTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COMSVCSEVENTINFO {
    type Abi = Self;
}
pub const CRMClerk: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0bd_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CRMFLAGS(pub i32);
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = CRMFLAGS(1i32);
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = CRMFLAGS(2i32);
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = CRMFLAGS(4i32);
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = CRMFLAGS(8i32);
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = CRMFLAGS(16i32);
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = CRMFLAGS(32i32);
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = CRMFLAGS(64i32);
impl ::core::convert::From<i32> for CRMFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRMFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CRMREGFLAGS(pub i32);
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = CRMREGFLAGS(1i32);
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = CRMREGFLAGS(2i32);
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = CRMREGFLAGS(4i32);
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = CRMREGFLAGS(7i32);
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = CRMREGFLAGS(16i32);
impl ::core::convert::From<i32> for CRMREGFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRMREGFLAGS {
    type Abi = Self;
}
pub const CRMRecoveryClerk: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0be_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_Binding(pub i32);
pub const CSC_NoBinding: CSC_Binding = CSC_Binding(0i32);
pub const CSC_BindToPoolThread: CSC_Binding = CSC_Binding(1i32);
impl ::core::convert::From<i32> for CSC_Binding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_Binding {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_COMTIIntrinsicsConfig(pub i32);
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(0i32);
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(1i32);
impl ::core::convert::From<i32> for CSC_COMTIIntrinsicsConfig {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_COMTIIntrinsicsConfig {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_IISIntrinsicsConfig(pub i32);
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(0i32);
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(1i32);
impl ::core::convert::From<i32> for CSC_IISIntrinsicsConfig {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_IISIntrinsicsConfig {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_InheritanceConfig(pub i32);
pub const CSC_Inherit: CSC_InheritanceConfig = CSC_InheritanceConfig(0i32);
pub const CSC_Ignore: CSC_InheritanceConfig = CSC_InheritanceConfig(1i32);
impl ::core::convert::From<i32> for CSC_InheritanceConfig {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_InheritanceConfig {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_PartitionConfig(pub i32);
pub const CSC_NoPartition: CSC_PartitionConfig = CSC_PartitionConfig(0i32);
pub const CSC_InheritPartition: CSC_PartitionConfig = CSC_PartitionConfig(1i32);
pub const CSC_NewPartition: CSC_PartitionConfig = CSC_PartitionConfig(2i32);
impl ::core::convert::From<i32> for CSC_PartitionConfig {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_PartitionConfig {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_SxsConfig(pub i32);
pub const CSC_NoSxs: CSC_SxsConfig = CSC_SxsConfig(0i32);
pub const CSC_InheritSxs: CSC_SxsConfig = CSC_SxsConfig(1i32);
pub const CSC_NewSxs: CSC_SxsConfig = CSC_SxsConfig(2i32);
impl ::core::convert::From<i32> for CSC_SxsConfig {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_SxsConfig {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_SynchronizationConfig(pub i32);
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(0i32);
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = CSC_SynchronizationConfig(1i32);
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = CSC_SynchronizationConfig(2i32);
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(3i32);
impl ::core::convert::From<i32> for CSC_SynchronizationConfig {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_SynchronizationConfig {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_ThreadPool(pub i32);
pub const CSC_ThreadPoolNone: CSC_ThreadPool = CSC_ThreadPool(0i32);
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = CSC_ThreadPool(1i32);
pub const CSC_STAThreadPool: CSC_ThreadPool = CSC_ThreadPool(2i32);
pub const CSC_MTAThreadPool: CSC_ThreadPool = CSC_ThreadPool(3i32);
impl ::core::convert::From<i32> for CSC_ThreadPool {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_ThreadPool {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_TrackerConfig(pub i32);
pub const CSC_DontUseTracker: CSC_TrackerConfig = CSC_TrackerConfig(0i32);
pub const CSC_UseTracker: CSC_TrackerConfig = CSC_TrackerConfig(1i32);
impl ::core::convert::From<i32> for CSC_TrackerConfig {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_TrackerConfig {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CSC_TransactionConfig(pub i32);
pub const CSC_NoTransaction: CSC_TransactionConfig = CSC_TransactionConfig(0i32);
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = CSC_TransactionConfig(1i32);
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = CSC_TransactionConfig(2i32);
pub const CSC_NewTransaction: CSC_TransactionConfig = CSC_TransactionConfig(3i32);
impl ::core::convert::From<i32> for CSC_TransactionConfig {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CSC_TransactionConfig {
    type Abi = Self;
}
pub const CServiceConfig: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0c8_7f19_11d2_978e_0000f8757e2a);
pub const ClrAssemblyLocator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x458aa3b5_265a_4b75_bc05_9bea4630cf18);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[inline]
pub unsafe fn CoCreateActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(piunknown: Param0, riid: *const ::windows::runtime::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateActivity(piunknown: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoCreateActivity(piunknown.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[inline]
pub unsafe fn CoEnterServiceDomain<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pconfigobject: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoEnterServiceDomain(pconfigobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CoEnterServiceDomain(pconfigobject.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetDefaultContext(::core::mem::transmute(apttype), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[inline]
pub unsafe fn CoLeaveServiceDomain<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkstatus: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLeaveServiceDomain(punkstatus: ::windows::runtime::RawPtr);
        }
        ::core::mem::transmute(CoLeaveServiceDomain(punkstatus.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CoMTSLocator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0ac_7f19_11d2_978e_0000f8757e2a);
pub const ComServiceEvents: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0c3_7f19_11d2_978e_0000f8757e2a);
pub const ComSystemAppEventData: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0c6_7f19_11d2_978e_0000f8757e2a);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: super::super::Foundation::BOOL,
    pub TerminateOnHang: super::super::Foundation::BOOL,
    pub AvgCallThresholdInMs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ComponentHangMonitorInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ComponentHangMonitorInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ComponentHangMonitorInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ComponentHangMonitorInfo").field("IsMonitored", &self.IsMonitored).field("TerminateOnHang", &self.TerminateOnHang).field("AvgCallThresholdInMs", &self.AvgCallThresholdInMs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ComponentHangMonitorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.IsMonitored == other.IsMonitored && self.TerminateOnHang == other.TerminateOnHang && self.AvgCallThresholdInMs == other.AvgCallThresholdInMs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ComponentHangMonitorInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ComponentHangMonitorInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub struct ComponentStatistics {
    pub NumInstances: u32,
    pub NumBoundReferences: u32,
    pub NumPooledObjects: u32,
    pub NumObjectsInCall: u32,
    pub AvgResponseTimeInMs: u32,
    pub NumCallsCompletedRecent: u32,
    pub NumCallsFailedRecent: u32,
    pub NumCallsCompletedTotal: u32,
    pub NumCallsFailedTotal: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl ComponentStatistics {}
impl ::core::default::Default for ComponentStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ComponentStatistics {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ComponentStatistics")
            .field("NumInstances", &self.NumInstances)
            .field("NumBoundReferences", &self.NumBoundReferences)
            .field("NumPooledObjects", &self.NumPooledObjects)
            .field("NumObjectsInCall", &self.NumObjectsInCall)
            .field("AvgResponseTimeInMs", &self.AvgResponseTimeInMs)
            .field("NumCallsCompletedRecent", &self.NumCallsCompletedRecent)
            .field("NumCallsFailedRecent", &self.NumCallsFailedRecent)
            .field("NumCallsCompletedTotal", &self.NumCallsCompletedTotal)
            .field("NumCallsFailedTotal", &self.NumCallsFailedTotal)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ComponentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.NumInstances == other.NumInstances
            && self.NumBoundReferences == other.NumBoundReferences
            && self.NumPooledObjects == other.NumPooledObjects
            && self.NumObjectsInCall == other.NumObjectsInCall
            && self.AvgResponseTimeInMs == other.AvgResponseTimeInMs
            && self.NumCallsCompletedRecent == other.NumCallsCompletedRecent
            && self.NumCallsFailedRecent == other.NumCallsFailedRecent
            && self.NumCallsCompletedTotal == other.NumCallsCompletedTotal
            && self.NumCallsFailedTotal == other.NumCallsFailedTotal
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.Reserved3 == other.Reserved3
            && self.Reserved4 == other.Reserved4
    }
}
impl ::core::cmp::Eq for ComponentStatistics {}
unsafe impl ::windows::runtime::Abi for ComponentStatistics {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
pub struct ComponentSummary {
    pub ApplicationInstanceId: ::windows::runtime::GUID,
    pub PartitionId: ::windows::runtime::GUID,
    pub ApplicationId: ::windows::runtime::GUID,
    pub Clsid: ::windows::runtime::GUID,
    pub ClassName: super::super::Foundation::PWSTR,
    pub ApplicationName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ComponentSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ComponentSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ComponentSummary {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ComponentSummary")
            .field("ApplicationInstanceId", &self.ApplicationInstanceId)
            .field("PartitionId", &self.PartitionId)
            .field("ApplicationId", &self.ApplicationId)
            .field("Clsid", &self.Clsid)
            .field("ClassName", &self.ClassName)
            .field("ApplicationName", &self.ApplicationName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ComponentSummary {
    fn eq(&self, other: &Self) -> bool {
        self.ApplicationInstanceId == other.ApplicationInstanceId && self.PartitionId == other.PartitionId && self.ApplicationId == other.ApplicationId && self.Clsid == other.Clsid && self.ClassName == other.ClassName && self.ApplicationName == other.ApplicationName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ComponentSummary {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ComponentSummary {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContextInfo(pub ::windows::runtime::IUnknown);
impl ContextInfo {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTransaction(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTransactionId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetActivityId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetContextId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ContextInfo {
    type Vtable = ContextInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x19a5a02c_0ac8_11d2_b286_00c04f8ef934);
}
impl ::core::convert::From<ContextInfo> for ::windows::runtime::IUnknown {
    fn from(value: ContextInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContextInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ContextInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContextInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContextInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo> for super::Com::IDispatch {
    fn from(value: ContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo> for super::Com::IDispatch {
    fn from(value: &ContextInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ContextInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ContextInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ContextInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisintx: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptx: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtxid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstractivityid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrctxid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContextInfo2(pub ::windows::runtime::IUnknown);
impl ContextInfo2 {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTransaction(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTransactionId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetActivityId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetContextId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPartitionId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetApplicationId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetApplicationInstanceId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ContextInfo2 {
    type Vtable = ContextInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc99d6e75_2375_11d4_8331_00c04f605588);
}
impl ::core::convert::From<ContextInfo2> for ::windows::runtime::IUnknown {
    fn from(value: ContextInfo2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContextInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &ContextInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContextInfo2> for ContextInfo {
    fn from(value: ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContextInfo2> for ContextInfo {
    fn from(value: &ContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ContextInfo> for ContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ContextInfo> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ContextInfo> for &ContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ContextInfo> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo2> for super::Com::IDispatch {
    fn from(value: ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo2> for super::Com::IDispatch {
    fn from(value: &ContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ContextInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisintx: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptx: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtxid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstractivityid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrctxid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__contextinfo20000: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__contextinfo20001: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__contextinfo20002: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
pub struct CrmLogRecordRead {
    pub dwCrmFlags: u32,
    pub dwSequenceNumber: u32,
    pub blobUserData: super::Com::BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl CrmLogRecordRead {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for CrmLogRecordRead {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for CrmLogRecordRead {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CrmLogRecordRead").field("dwCrmFlags", &self.dwCrmFlags).field("dwSequenceNumber", &self.dwSequenceNumber).field("blobUserData", &self.blobUserData).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for CrmLogRecordRead {
    fn eq(&self, other: &Self) -> bool {
        self.dwCrmFlags == other.dwCrmFlags && self.dwSequenceNumber == other.dwSequenceNumber && self.blobUserData == other.blobUserData
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for CrmLogRecordRead {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for CrmLogRecordRead {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CrmTransactionState(pub i32);
pub const TxState_Active: CrmTransactionState = CrmTransactionState(0i32);
pub const TxState_Committed: CrmTransactionState = CrmTransactionState(1i32);
pub const TxState_Aborted: CrmTransactionState = CrmTransactionState(2i32);
pub const TxState_Indoubt: CrmTransactionState = CrmTransactionState(3i32);
impl ::core::convert::From<i32> for CrmTransactionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CrmTransactionState {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DUMPTYPE(pub i32);
pub const DUMPTYPE_FULL: DUMPTYPE = DUMPTYPE(0i32);
pub const DUMPTYPE_MINI: DUMPTYPE = DUMPTYPE(1i32);
pub const DUMPTYPE_NONE: DUMPTYPE = DUMPTYPE(2i32);
impl ::core::convert::From<i32> for DUMPTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DUMPTYPE {
    type Abi = Self;
}
pub const DispenserManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0c0_7f19_11d2_978e_0000f8757e2a);
pub const Dummy30040732: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0a9_7f19_11d2_978e_0000f8757e2a);
pub const EventServer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabafbc_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const GUID_STRING_SIZE: u32 = 40u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GetAppTrackerDataFlags(pub i32);
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(1i32);
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = GetAppTrackerDataFlags(2i32);
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = GetAppTrackerDataFlags(4i32);
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(8i32);
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(16i32);
impl ::core::convert::From<i32> for GetAppTrackerDataFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GetAppTrackerDataFlags {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[inline]
pub unsafe fn GetDispenserManager() -> ::windows::runtime::Result<IDispenserManager> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDispenserManager(param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IDispenserManager as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        GetDispenserManager(&mut result__).from_abi::<IDispenserManager>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[inline]
pub unsafe fn GetManagedExtensions(dwexts: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagedExtensions(dwexts: *mut u32) -> ::windows::runtime::HRESULT;
        }
        GetManagedExtensions(::core::mem::transmute(dwexts)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GetSecurityCallContextAppObject: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0a8_7f19_11d2_978e_0000f8757e2a);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: super::super::Foundation::BOOL,
    pub fTerminateOnHang: super::super::Foundation::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: u32,
    pub dwDumpCount: u32,
    pub dwInfoMsgCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HANG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HANG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HANG_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HANG_INFO")
            .field("fAppHangMonitorEnabled", &self.fAppHangMonitorEnabled)
            .field("fTerminateOnHang", &self.fTerminateOnHang)
            .field("DumpType", &self.DumpType)
            .field("dwHangTimeout", &self.dwHangTimeout)
            .field("dwDumpCount", &self.dwDumpCount)
            .field("dwInfoMsgCount", &self.dwInfoMsgCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HANG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fAppHangMonitorEnabled == other.fAppHangMonitorEnabled && self.fTerminateOnHang == other.fTerminateOnHang && self.DumpType == other.DumpType && self.dwHangTimeout == other.dwHangTimeout && self.dwDumpCount == other.dwDumpCount && self.dwInfoMsgCount == other.dwInfoMsgCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HANG_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HANG_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppDomainHelper(pub ::windows::runtime::IUnknown);
impl IAppDomainHelper {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkad: Param0, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), punkad.into_param().abi(), ::core::mem::transmute(__midl__iappdomainhelper0000), ::core::mem::transmute(ppool)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn DoCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkad: Param0, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), punkad.into_param().abi(), ::core::mem::transmute(__midl__iappdomainhelper0001), ::core::mem::transmute(ppool)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppDomainHelper {
    type Vtable = IAppDomainHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc7b67079_8255_42c6_9ec0_6994a3548780);
}
impl ::core::convert::From<IAppDomainHelper> for ::windows::runtime::IUnknown {
    fn from(value: IAppDomainHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppDomainHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IAppDomainHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppDomainHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppDomainHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAppDomainHelper> for super::Com::IDispatch {
    fn from(value: IAppDomainHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAppDomainHelper> for super::Com::IDispatch {
    fn from(value: &IAppDomainHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IAppDomainHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IAppDomainHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDomainHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkad: ::windows::runtime::RawPtr, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkad: ::windows::runtime::RawPtr, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAssemblyLocator(pub ::windows::runtime::IUnknown);
impl IAssemblyLocator {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetModules<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, applicationdir: Param0, applicationname: Param1, assemblyname: Param2) -> ::windows::runtime::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), applicationdir.into_param().abi(), applicationname.into_param().abi(), assemblyname.into_param().abi(), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAssemblyLocator {
    type Vtable = IAssemblyLocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x391ffbb9_a8ee_432a_abc8_baa238dab90f);
}
impl ::core::convert::From<IAssemblyLocator> for ::windows::runtime::IUnknown {
    fn from(value: IAssemblyLocator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAssemblyLocator> for ::windows::runtime::IUnknown {
    fn from(value: &IAssemblyLocator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAssemblyLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAssemblyLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAssemblyLocator> for super::Com::IDispatch {
    fn from(value: IAssemblyLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAssemblyLocator> for super::Com::IDispatch {
    fn from(value: &IAssemblyLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IAssemblyLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IAssemblyLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyLocator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applicationdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, assemblyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAsyncErrorNotify(pub ::windows::runtime::IUnknown);
impl IAsyncErrorNotify {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn OnError(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAsyncErrorNotify {
    type Vtable = IAsyncErrorNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfe6777fb_a674_4177_8f32_6d707e113484);
}
impl ::core::convert::From<IAsyncErrorNotify> for ::windows::runtime::IUnknown {
    fn from(value: IAsyncErrorNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAsyncErrorNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IAsyncErrorNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAsyncErrorNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAsyncErrorNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncErrorNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICOMAdminCatalog(pub ::windows::runtime::IUnknown);
impl ICOMAdminCatalog {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetCollection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcollname: Param0) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrcollname.into_param().abi(), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcatalogservername: Param0) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrcatalogservername.into_param().abi(), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MajorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MinorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetCollectionByQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcollname: Param0, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrcollname.into_param().abi(), ::core::mem::transmute(ppsavarquery), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ImportComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrclsidorprogid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), bstrclsidorprogid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn InstallComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrdll: Param1, bstrtlb: Param2, bstrpsdll: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), bstrdll.into_param().abi(), bstrtlb.into_param().abi(), bstrpsdll.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ShutdownApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ExportApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrapplicationfile: Param1, loptions: COMAdminApplicationExportOptions) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn InstallApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrapplicationfile: Param0,
        bstrdestinationdirectory: Param1,
        loptions: COMAdminApplicationInstallOptions,
        bstruserid: Param3,
        bstrpassword: Param4,
        bstrrsn: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), bstrapplicationfile.into_param().abi(), bstrdestinationdirectory.into_param().abi(), ::core::mem::transmute(loptions), bstruserid.into_param().abi(), bstrpassword.into_param().abi(), bstrrsn.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn StopRouter(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn RefreshRouter(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn StartRouter(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved1(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved2(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn InstallMultipleComponents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetMultipleComponentsInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarclassnames), ::core::mem::transmute(ppsavarfileflags), ::core::mem::transmute(ppsavarcomponentflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn RefreshComponents(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn BackupREGDB<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupfilepath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), bstrbackupfilepath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn RestoreREGDB<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupfilepath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), bstrbackupfilepath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn QueryApplicationFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationfile: Param0, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(pbstrapplicationname), ::core::mem::transmute(pbstrapplicationdescription), ::core::mem::transmute(pbhasusers), ::core::mem::transmute(pbisproxy), ::core::mem::transmute(ppsavarfilenames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn StartApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(lservice), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn InstallMultipleEventClasses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn InstallEventClass<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrdll: Param1, bstrtlb: Param2, bstrpsdll: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), bstrdll.into_param().abi(), bstrtlb.into_param().abi(), bstrpsdll.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetEventClassesForIID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstriid: Param0, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), bstriid.into_param().abi(), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarprogids), ::core::mem::transmute(ppsavardescriptions)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICOMAdminCatalog {
    type Vtable = ICOMAdminCatalog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdd662187_dfc2_11d1_a2cf_00805fc79235);
}
impl ::core::convert::From<ICOMAdminCatalog> for ::windows::runtime::IUnknown {
    fn from(value: ICOMAdminCatalog) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICOMAdminCatalog> for ::windows::runtime::IUnknown {
    fn from(value: &ICOMAdminCatalog) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICOMAdminCatalog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICOMAdminCatalog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog> for super::Com::IDispatch {
    fn from(value: ICOMAdminCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog> for super::Com::IDispatch {
    fn from(value: &ICOMAdminCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ICOMAdminCatalog {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ICOMAdminCatalog {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcatalogservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorversion: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdestinationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        loptions: COMAdminApplicationInstallOptions,
        bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lservice: i32, plstatus: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstriid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICOMAdminCatalog2(pub ::windows::runtime::IUnknown);
impl ICOMAdminCatalog2 {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetCollection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcollname: Param0) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrcollname.into_param().abi(), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcatalogservername: Param0) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrcatalogservername.into_param().abi(), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MajorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MinorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetCollectionByQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcollname: Param0, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrcollname.into_param().abi(), ::core::mem::transmute(ppsavarquery), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ImportComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrclsidorprogid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), bstrclsidorprogid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn InstallComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrdll: Param1, bstrtlb: Param2, bstrpsdll: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), bstrdll.into_param().abi(), bstrtlb.into_param().abi(), bstrpsdll.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ShutdownApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ExportApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrapplicationfile: Param1, loptions: COMAdminApplicationExportOptions) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn InstallApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrapplicationfile: Param0,
        bstrdestinationdirectory: Param1,
        loptions: COMAdminApplicationInstallOptions,
        bstruserid: Param3,
        bstrpassword: Param4,
        bstrrsn: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), bstrapplicationfile.into_param().abi(), bstrdestinationdirectory.into_param().abi(), ::core::mem::transmute(loptions), bstruserid.into_param().abi(), bstrpassword.into_param().abi(), bstrrsn.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn StopRouter(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn RefreshRouter(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn StartRouter(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved1(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved2(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn InstallMultipleComponents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetMultipleComponentsInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarclassnames), ::core::mem::transmute(ppsavarfileflags), ::core::mem::transmute(ppsavarcomponentflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn RefreshComponents(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn BackupREGDB<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupfilepath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), bstrbackupfilepath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn RestoreREGDB<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupfilepath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), bstrbackupfilepath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn QueryApplicationFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationfile: Param0, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(pbstrapplicationname), ::core::mem::transmute(pbstrapplicationdescription), ::core::mem::transmute(pbhasusers), ::core::mem::transmute(pbisproxy), ::core::mem::transmute(ppsavarfilenames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn StartApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(lservice), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn InstallMultipleEventClasses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn InstallEventClass<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrdll: Param1, bstrtlb: Param2, bstrpsdll: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), bstrapplidorname.into_param().abi(), bstrdll.into_param().abi(), bstrtlb.into_param().abi(), bstrpsdll.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetEventClassesForIID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstriid: Param0, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), bstriid.into_param().abi(), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarprogids), ::core::mem::transmute(ppsavardescriptions)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetCollectionByQuery2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcollectionname: Param0, pvarquerystrings: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), bstrcollectionname.into_param().abi(), ::core::mem::transmute(pvarquerystrings), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprocessid), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarapplicationinstanceid), ::core::mem::transmute(lreasoncode)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarapplicationinstanceid), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn DumpApplicationInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationinstanceid: Param0, bstrdirectory: Param1, lmaximages: i32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), bstrapplicationinstanceid.into_param().abi(), bstrdirectory.into_param().abi(), ::core::mem::transmute(lmaximages), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsApplicationInstanceDumpSupported(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn CreateServiceForApplication<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrapplicationidorname: Param0,
        bstrservicename: Param1,
        bstrstarttype: Param2,
        bstrerrorcontrol: Param3,
        bstrdependencies: Param4,
        bstrrunas: Param5,
        bstrpassword: Param6,
        bdesktopok: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(
            ::core::mem::transmute_copy(self),
            bstrapplicationidorname.into_param().abi(),
            bstrservicename.into_param().abi(),
            bstrstarttype.into_param().abi(),
            bstrerrorcontrol.into_param().abi(),
            bstrdependencies.into_param().abi(),
            bstrrunas.into_param().abi(),
            bstrpassword.into_param().abi(),
            ::core::mem::transmute(bdesktopok),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn DeleteServiceForApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationidorname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), bstrapplicationidorname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPartitionID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationidorname: Param0) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstrapplicationidorname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPartitionName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationidorname: Param0) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstrapplicationidorname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetCurrentPartition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpartitionidorname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstrpartitionidorname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn CurrentPartitionID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn CurrentPartitionName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GlobalPartitionID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn FlushPartitionCache(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CopyApplications<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsourcepartitionidorname: Param0, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), bstrsourcepartitionidorname.into_param().abi(), ::core::mem::transmute(pvarapplicationid), bstrdestinationpartitionidorname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CopyComponents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsourceapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), bstrsourceapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), bstrdestinationapplicationidorname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveComponents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsourceapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), bstrsourceapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), bstrdestinationapplicationidorname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn AliasComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrsrcapplicationidorname: Param0,
        bstrclsidorprogid: Param1,
        bstrdestapplicationidorname: Param2,
        bstrnewprogid: Param3,
        bstrnewclsid: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), bstrsrcapplicationidorname.into_param().abi(), bstrclsidorprogid.into_param().abi(), bstrdestapplicationidorname.into_param().abi(), bstrnewprogid.into_param().abi(), bstrnewclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsSafeToDelete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdllname: Param0) -> ::windows::runtime::Result<COMAdminInUse> {
        let mut result__: <COMAdminInUse as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), bstrdllname.into_param().abi(), &mut result__).from_abi::<COMAdminInUse>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn ImportUnconfiguredComponents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), bstrapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PromoteUnconfiguredComponents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), bstrapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn ImportComponents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), bstrapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Is64BitCatalogServer(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ExportPartition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpartitionidorname: Param0, bstrpartitionfilename: Param1, loptions: COMAdminApplicationExportOptions) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), bstrpartitionidorname.into_param().abi(), bstrpartitionfilename.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn InstallPartition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrfilename: Param0,
        bstrdestdirectory: Param1,
        loptions: COMAdminApplicationInstallOptions,
        bstruserid: Param3,
        bstrpassword: Param4,
        bstrrsn: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), bstrdestdirectory.into_param().abi(), ::core::mem::transmute(loptions), bstruserid.into_param().abi(), bstrpassword.into_param().abi(), bstrrsn.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn QueryApplicationFile2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrapplicationfile: Param0) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), bstrapplicationfile.into_param().abi(), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetComponentVersionCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrclsidorprogid: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), bstrclsidorprogid.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICOMAdminCatalog2 {
    type Vtable = ICOMAdminCatalog2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x790c6e0b_9194_4cc9_9426_a48a63185696);
}
impl ::core::convert::From<ICOMAdminCatalog2> for ::windows::runtime::IUnknown {
    fn from(value: ICOMAdminCatalog2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICOMAdminCatalog2> for ::windows::runtime::IUnknown {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ICOMAdminCatalog2> for ICOMAdminCatalog {
    fn from(value: ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICOMAdminCatalog2> for ICOMAdminCatalog {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICOMAdminCatalog> for ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICOMAdminCatalog> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICOMAdminCatalog> for &ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICOMAdminCatalog> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog2> for super::Com::IDispatch {
    fn from(value: ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog2> for super::Com::IDispatch {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcatalogservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorversion: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdestinationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        loptions: COMAdminApplicationInstallOptions,
        bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lservice: i32, plstatus: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstriid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcollectionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarquerystrings: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprocessid: i32, pbstrapplicationinstanceid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarapplicationinstanceid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarapplicationinstanceid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarapplicationinstanceid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarapplicationinstanceid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, lreasoncode: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarapplicationinstanceid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarboolpaused: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaximages: i32, pbstrdumpfile: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarbooldumpsupported: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrstarttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrerrorcontrol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdependencies: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrrunas: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bdesktopok: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpartitionid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpartitionname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrglobalpartitionid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsourcepartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarapplicationid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, bstrdestinationpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsrcapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdllname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarcomponenttype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarcomponenttype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarcomponenttype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbis64bit: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpartitionfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdestdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        loptions: COMAdminApplicationInstallOptions,
        bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfilesforimport: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plversioncount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICOMLBArguments(pub ::windows::runtime::IUnknown);
impl ICOMLBArguments {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetCLSID(&self, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclsid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetCLSID(&self, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclsid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetMachineName(&self, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchsvr), ::core::mem::transmute(szservername)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetMachineName<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cchsvr: u32, szservername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchsvr), szservername.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICOMLBArguments {
    type Vtable = ICOMLBArguments_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3a0f150f_8ee5_4b94_b40e_aef2f9e42ed2);
}
impl ::core::convert::From<ICOMLBArguments> for ::windows::runtime::IUnknown {
    fn from(value: ICOMLBArguments) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICOMLBArguments> for ::windows::runtime::IUnknown {
    fn from(value: &ICOMLBArguments) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICOMLBArguments {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICOMLBArguments {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICOMLBArguments_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICatalogCollection(pub ::windows::runtime::IUnknown);
impl ICatalogCollection {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn Add(&self) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Populate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SaveChanges(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetCollection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, bstrcollname: Param0, varobjectkey: Param1) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrcollname.into_param().abi(), varobjectkey.into_param().abi(), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn AddEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn RemoveEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn GetUtilInterface(&self) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn DataStoreMajorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn DataStoreMinorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(psakeys)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn PopulateByQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrquerystring: Param0, lquerytype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), bstrquerystring.into_param().abi(), ::core::mem::transmute(lquerytype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICatalogCollection {
    type Vtable = ICatalogCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6eb22872_8a19_11d0_81b6_00a0c9231c29);
}
impl ::core::convert::From<ICatalogCollection> for ::windows::runtime::IUnknown {
    fn from(value: ICatalogCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICatalogCollection> for ::windows::runtime::IUnknown {
    fn from(value: &ICatalogCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICatalogCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICatalogCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogCollection> for super::Com::IDispatch {
    fn from(value: ICatalogCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogCollection> for super::Com::IDispatch {
    fn from(value: &ICatalogCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ICatalogCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ICatalogCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumvariant: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, ppcatalogobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plobjectcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcatalogobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchanges: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varobjectkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarnamel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppidispatch: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorversionl: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psakeys: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrquerystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lquerytype: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICatalogObject(pub ::windows::runtime::IUnknown);
impl ICatalogObject {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Value<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpropname: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrpropname.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, bstrpropname: Param0, val: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrpropname.into_param().abi(), val.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Key(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsPropertyReadOnly<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpropname: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrpropname.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Valid(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsPropertyWriteOnly<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpropname: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpropname.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICatalogObject {
    type Vtable = ICatalogObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6eb22871_8a19_11d0_81b6_00a0c9231c29);
}
impl ::core::convert::From<ICatalogObject> for ::windows::runtime::IUnknown {
    fn from(value: ICatalogObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICatalogObject> for ::windows::runtime::IUnknown {
    fn from(value: &ICatalogObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICatalogObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICatalogObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogObject> for super::Com::IDispatch {
    fn from(value: ICatalogObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogObject> for super::Com::IDispatch {
    fn from(value: &ICatalogObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ICatalogObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ICatalogObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarretval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarretval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarretval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbretval: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICheckSxsConfig(pub ::windows::runtime::IUnknown);
impl ICheckSxsConfig {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsSameSxsConfig<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszsxsname: Param0, wszsxsdirectory: Param1, wszsxsappname: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszsxsname.into_param().abi(), wszsxsdirectory.into_param().abi(), wszsxsappname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICheckSxsConfig {
    type Vtable = ICheckSxsConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0ff5a96f_11fc_47d1_baa6_25dd347e7242);
}
impl ::core::convert::From<ICheckSxsConfig> for ::windows::runtime::IUnknown {
    fn from(value: ICheckSxsConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICheckSxsConfig> for ::windows::runtime::IUnknown {
    fn from(value: &ICheckSxsConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICheckSxsConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICheckSxsConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICheckSxsConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszsxsname: super::super::Foundation::PWSTR, wszsxsdirectory: super::super::Foundation::PWSTR, wszsxsappname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComActivityEvents(pub ::windows::runtime::IUnknown);
impl IComActivityEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, guidentered: *const ::windows::runtime::GUID, dwthread: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidentered), ::core::mem::transmute(dwthread)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, guidentered: *const ::windows::runtime::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidentered), ::core::mem::transmute(dwthread), ::core::mem::transmute(dwtimeout)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(dwthread), ::core::mem::transmute(dwcalldepth)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, guidleft: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidleft)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, dwcalldepth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(dwcalldepth)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComActivityEvents {
    type Vtable = IComActivityEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130b0_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComActivityEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComActivityEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComActivityEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComActivityEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComActivityEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComActivityEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComActivityEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, guidentered: *const ::windows::runtime::GUID, dwthread: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, guidentered: *const ::windows::runtime::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, guidleft: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::runtime::GUID, dwcalldepth: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComApp2Events(pub ::windows::runtime::IUnknown);
impl IComApp2Events {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAppActivation2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1, guidprocess: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi(), guidprocess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAppShutdown2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAppForceShutdown2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAppPaused2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1, bpaused: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi(), bpaused.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAppRecycle2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1, guidprocess: Param2, lreason: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi(), guidprocess.into_param().abi(), ::core::mem::transmute(lreason)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComApp2Events {
    type Vtable = IComApp2Events_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1290bc1a_b219_418d_b078_5934ded08242);
}
impl ::core::convert::From<IComApp2Events> for ::windows::runtime::IUnknown {
    fn from(value: IComApp2Events) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComApp2Events> for ::windows::runtime::IUnknown {
    fn from(value: &IComApp2Events) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComApp2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComApp2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComApp2Events_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID, guidprocess: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID, guidprocess: ::windows::runtime::GUID, lreason: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComAppEvents(pub ::windows::runtime::IUnknown);
impl IComAppEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAppActivation<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAppShutdown<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAppForceShutdown<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComAppEvents {
    type Vtable = IComAppEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130a6_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComAppEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComAppEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComAppEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComAppEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComAppEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComAppEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComAppEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComCRMEvents(pub ::windows::runtime::IUnknown);
impl IComCRMEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMRecoveryStart<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMRecoveryDone<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMCheckpoint<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMBegin<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pinfo: *const COMSVCSEVENTINFO,
        guidclerkclsid: Param1,
        guidactivity: Param2,
        guidtx: Param3,
        szprogidcompensator: Param4,
        szdescription: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi(), guidactivity.into_param().abi(), guidtx.into_param().abi(), szprogidcompensator.into_param().abi(), szdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMPrepare<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMCommit<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMAbort<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMIndoubt<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMDone<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMRelease<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMAnalyze<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi(), ::core::mem::transmute(dwcrmrecordtype), ::core::mem::transmute(dwrecordsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMWrite<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1, fvariants: Param2, dwrecordsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi(), fvariants.into_param().abi(), ::core::mem::transmute(dwrecordsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMForget<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMForce<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnCRMDeliver<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1, fvariants: Param2, dwrecordsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi(), fvariants.into_param().abi(), ::core::mem::transmute(dwrecordsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComCRMEvents {
    type Vtable = IComCRMEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130b5_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComCRMEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComCRMEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComCRMEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComCRMEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComCRMEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComCRMEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComCRMEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID, guidactivity: ::windows::runtime::GUID, guidtx: ::windows::runtime::GUID, szprogidcompensator: super::super::Foundation::PWSTR, szdescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::runtime::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComExceptionEvents(pub ::windows::runtime::IUnknown);
impl IComExceptionEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnExceptionUser<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(code), ::core::mem::transmute(address), pszstacktrace.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComExceptionEvents {
    type Vtable = IComExceptionEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130b3_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComExceptionEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComExceptionEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComExceptionEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComExceptionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComExceptionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComExceptionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComExceptionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComIdentityEvents(pub ::windows::runtime::IUnknown);
impl IComIdentityEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnIISRequestInfo<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: Param2, pszserverip: Param3, pszurl: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objid), pszclientip.into_param().abi(), pszserverip.into_param().abi(), pszurl.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComIdentityEvents {
    type Vtable = IComIdentityEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130b1_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComIdentityEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComIdentityEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComIdentityEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComIdentityEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComIdentityEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComIdentityEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComIdentityEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: super::super::Foundation::PWSTR, pszserverip: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComInstance2Events(pub ::windows::runtime::IUnknown);
impl IComInstance2Events {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, clsid: *const ::windows::runtime::GUID, tsid: *const ::windows::runtime::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(clsid), ::core::mem::transmute(tsid), ::core::mem::transmute(ctxtid), ::core::mem::transmute(objectid), ::core::mem::transmute(guidpartition)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComInstance2Events {
    type Vtable = IComInstance2Events_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20e3bf07_b506_4ad5_a50c_d2ca5b9c158e);
}
impl ::core::convert::From<IComInstance2Events> for ::windows::runtime::IUnknown {
    fn from(value: IComInstance2Events) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComInstance2Events> for ::windows::runtime::IUnknown {
    fn from(value: &IComInstance2Events) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComInstance2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComInstance2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComInstance2Events_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, clsid: *const ::windows::runtime::GUID, tsid: *const ::windows::runtime::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComInstanceEvents(pub ::windows::runtime::IUnknown);
impl IComInstanceEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, clsid: *const ::windows::runtime::GUID, tsid: *const ::windows::runtime::GUID, ctxtid: u64, objectid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(clsid), ::core::mem::transmute(tsid), ::core::mem::transmute(ctxtid), ::core::mem::transmute(objectid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComInstanceEvents {
    type Vtable = IComInstanceEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130a7_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComInstanceEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComInstanceEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComInstanceEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComInstanceEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComInstanceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComInstanceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComInstanceEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, clsid: *const ::windows::runtime::GUID, tsid: *const ::windows::runtime::GUID, ctxtid: u64, objectid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComLTxEvents(pub ::windows::runtime::IUnknown);
impl IComLTxEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnLtxTransactionStart<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1, tsid: Param2, froot: Param3, nisolationlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi(), tsid.into_param().abi(), froot.into_param().abi(), ::core::mem::transmute(nisolationlevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnLtxTransactionPrepare<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1, fvote: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi(), fvote.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnLtxTransactionAbort<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnLtxTransactionCommit<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnLtxTransactionPromote<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1, txnid: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi(), txnid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComLTxEvents {
    type Vtable = IComLTxEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x605cf82c_578e_4298_975d_82babcd9e053);
}
impl ::core::convert::From<IComLTxEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComLTxEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComLTxEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComLTxEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComLTxEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComLTxEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComLTxEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::runtime::GUID, tsid: ::windows::runtime::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::runtime::GUID, fvote: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::runtime::GUID, txnid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComMethod2Events(pub ::windows::runtime::IUnknown);
impl IComMethod2Events {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, dwthread: u32, imeth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(dwthread), ::core::mem::transmute(imeth)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, dwthread: u32, imeth: u32, hresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(dwthread), ::core::mem::transmute(imeth), ::core::mem::transmute(hresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, dwthread: u32, imeth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(dwthread), ::core::mem::transmute(imeth)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComMethod2Events {
    type Vtable = IComMethod2Events_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb388aaa_567d_4024_af8e_6e93ee748573);
}
impl ::core::convert::From<IComMethod2Events> for ::windows::runtime::IUnknown {
    fn from(value: IComMethod2Events) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComMethod2Events> for ::windows::runtime::IUnknown {
    fn from(value: &IComMethod2Events) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComMethod2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComMethod2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMethod2Events_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, dwthread: u32, imeth: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, dwthread: u32, imeth: u32, hresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, dwthread: u32, imeth: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComMethodEvents(pub ::windows::runtime::IUnknown);
impl IComMethodEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, imeth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(imeth)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, imeth: u32, hresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(imeth), ::core::mem::transmute(hresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, imeth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(imeth)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComMethodEvents {
    type Vtable = IComMethodEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130a9_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComMethodEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComMethodEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComMethodEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComMethodEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComMethodEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComMethodEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMethodEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, imeth: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, imeth: u32, hresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::runtime::GUID, guidrid: *const ::windows::runtime::GUID, imeth: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComMtaThreadPoolKnobs(pub ::windows::runtime::IUnknown);
impl IComMtaThreadPoolKnobs {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MTAGetMaxThreadCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MTASetThrottleValue(&self, dwthrottle: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthrottle)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MTAGetThrottleValue(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IComMtaThreadPoolKnobs {
    type Vtable = IComMtaThreadPoolKnobs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf9a76d2e_76a5_43eb_a0c4_49bec8e48480);
}
impl ::core::convert::From<IComMtaThreadPoolKnobs> for ::windows::runtime::IUnknown {
    fn from(value: IComMtaThreadPoolKnobs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComMtaThreadPoolKnobs> for ::windows::runtime::IUnknown {
    fn from(value: &IComMtaThreadPoolKnobs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComMtaThreadPoolKnobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComMtaThreadPoolKnobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMtaThreadPoolKnobs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmaxthreads: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmaxthreads: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthrottle: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwthrottle: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComObjectConstruction2Events(pub ::windows::runtime::IUnknown);
impl IComObjectConstruction2Events {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjectConstruct2<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, sconstructstring: Param2, oid: u64, guidpartition: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), sconstructstring.into_param().abi(), ::core::mem::transmute(oid), ::core::mem::transmute(guidpartition)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComObjectConstruction2Events {
    type Vtable = IComObjectConstruction2Events_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4b5a7827_8df2_45c0_8f6f_57ea1f856a9f);
}
impl ::core::convert::From<IComObjectConstruction2Events> for ::windows::runtime::IUnknown {
    fn from(value: IComObjectConstruction2Events) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComObjectConstruction2Events> for ::windows::runtime::IUnknown {
    fn from(value: &IComObjectConstruction2Events) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComObjectConstruction2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComObjectConstruction2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectConstruction2Events_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64, guidpartition: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComObjectConstructionEvents(pub ::windows::runtime::IUnknown);
impl IComObjectConstructionEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjectConstruct<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, sconstructstring: Param2, oid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), sconstructstring.into_param().abi(), ::core::mem::transmute(oid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComObjectConstructionEvents {
    type Vtable = IComObjectConstructionEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130af_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComObjectConstructionEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComObjectConstructionEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComObjectConstructionEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComObjectConstructionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComObjectConstructionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComObjectConstructionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectConstructionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComObjectEvents(pub ::windows::runtime::IUnknown);
impl IComObjectEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid), ::core::mem::transmute(objectid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid), ::core::mem::transmute(objectid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComObjectEvents {
    type Vtable = IComObjectEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130aa_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComObjectEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComObjectEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComObjectEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComObjectEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComObjectEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComObjectEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComObjectPool2Events(pub ::windows::runtime::IUnknown);
impl IComObjectPool2Events {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(nreason), ::core::mem::transmute(dwavailable), ::core::mem::transmute(oid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwavailable), ::core::mem::transmute(oid), ::core::mem::transmute(guidpartition)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, guidtx: *const ::windows::runtime::GUID, objid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), ::core::mem::transmute(objid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, guidtx: *const ::windows::runtime::GUID, objid: u64, guidpartition: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), ::core::mem::transmute(objid), ::core::mem::transmute(guidpartition)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComObjectPool2Events {
    type Vtable = IComObjectPool2Events_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x65bf6534_85ea_4f64_8cf4_3d974b2ab1cf);
}
impl ::core::convert::From<IComObjectPool2Events> for ::windows::runtime::IUnknown {
    fn from(value: IComObjectPool2Events) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComObjectPool2Events> for ::windows::runtime::IUnknown {
    fn from(value: &IComObjectPool2Events) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComObjectPool2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComObjectPool2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPool2Events_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, guidtx: *const ::windows::runtime::GUID, objid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, guidtx: *const ::windows::runtime::GUID, objid: u64, guidpartition: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComObjectPoolEvents(pub ::windows::runtime::IUnknown);
impl IComObjectPoolEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(nreason), ::core::mem::transmute(dwavailable), ::core::mem::transmute(oid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, dwavailable: u32, oid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwavailable), ::core::mem::transmute(oid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, guidtx: *const ::windows::runtime::GUID, objid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), ::core::mem::transmute(objid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, guidtx: *const ::windows::runtime::GUID, objid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), ::core::mem::transmute(objid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComObjectPoolEvents {
    type Vtable = IComObjectPoolEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130ad_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComObjectPoolEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComObjectPoolEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComObjectPoolEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComObjectPoolEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComObjectPoolEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComObjectPoolEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPoolEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, dwavailable: u32, oid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, guidtx: *const ::windows::runtime::GUID, objid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, guidobject: *const ::windows::runtime::GUID, guidtx: *const ::windows::runtime::GUID, objid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComObjectPoolEvents2(pub ::windows::runtime::IUnknown);
impl IComObjectPoolEvents2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, dwobjscreated: u32, oid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwobjscreated), ::core::mem::transmute(oid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, dwobjscreated: u32, oid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwobjscreated), ::core::mem::transmute(oid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(dwthreadswaiting), ::core::mem::transmute(dwavail), ::core::mem::transmute(dwcreated), ::core::mem::transmute(dwmin), ::core::mem::transmute(dwmax)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, guidactivity: *const ::windows::runtime::GUID, dwtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidactivity), ::core::mem::transmute(dwtimeout)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwmin), ::core::mem::transmute(dwmax), ::core::mem::transmute(dwtimeout)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComObjectPoolEvents2 {
    type Vtable = IComObjectPoolEvents2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130ae_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComObjectPoolEvents2> for ::windows::runtime::IUnknown {
    fn from(value: IComObjectPoolEvents2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComObjectPoolEvents2> for ::windows::runtime::IUnknown {
    fn from(value: &IComObjectPoolEvents2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComObjectPoolEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComObjectPoolEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPoolEvents2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, dwobjscreated: u32, oid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, dwobjscreated: u32, oid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, guidactivity: *const ::windows::runtime::GUID, dwtimeout: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::runtime::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComQCEvents(pub ::windows::runtime::IUnknown);
impl IComQCEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnQCRecord<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: Param2, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID, msmqhr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objid), szqueue.into_param().abi(), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), ::core::mem::transmute(msmqhr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnQCQueueOpen<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: Param1, queueid: u64, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), szqueue.into_param().abi(), ::core::mem::transmute(queueid), ::core::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(queueid), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), ::core::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(queueid), ::core::mem::transmute(msmqhr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID, retryindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), ::core::mem::transmute(retryindex)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objid), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), ::core::mem::transmute(hr)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComQCEvents {
    type Vtable = IComQCEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130b2_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComQCEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComQCEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComQCEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComQCEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComQCEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComQCEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComQCEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: super::super::Foundation::PWSTR, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID, msmqhr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, szqueue: super::super::Foundation::PWSTR, queueid: u64, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID, retryindex: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::runtime::GUID, guidworkflowid: *const ::windows::runtime::GUID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComResourceEvents(pub ::windows::runtime::IUnknown);
impl IComResourceEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnResourceCreate<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: Param2, resid: u64, enlisted: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), psztype.into_param().abi(), ::core::mem::transmute(resid), enlisted.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnResourceAllocate<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: Param2, resid: u64, enlisted: Param4, numrated: u32, rating: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), psztype.into_param().abi(), ::core::mem::transmute(resid), enlisted.into_param().abi(), ::core::mem::transmute(numrated), ::core::mem::transmute(rating)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnResourceRecycle<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: Param2, resid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), psztype.into_param().abi(), ::core::mem::transmute(resid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnResourceDestroy<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::runtime::HRESULT, psztype: Param3, resid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), ::core::mem::transmute(hr), psztype.into_param().abi(), ::core::mem::transmute(resid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnResourceTrack<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: Param2, resid: u64, enlisted: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), psztype.into_param().abi(), ::core::mem::transmute(resid), enlisted.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComResourceEvents {
    type Vtable = IComResourceEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130ab_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComResourceEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComResourceEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComResourceEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComResourceEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComResourceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComResourceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComResourceEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::runtime::HRESULT, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComSecurityEvents(pub ::windows::runtime::IUnknown);
impl IComSecurityEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAuthenticate<'a, Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, objectid: u64, guidiid: *const ::windows::runtime::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: Param9) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(pinfo),
            ::core::mem::transmute(guidactivity),
            ::core::mem::transmute(objectid),
            ::core::mem::transmute(guidiid),
            ::core::mem::transmute(imeth),
            ::core::mem::transmute(cbbyteorig),
            ::core::mem::transmute(psidoriginaluser),
            ::core::mem::transmute(cbbytecur),
            ::core::mem::transmute(psidcurrentuser),
            bcurrentuserinpersonatinginproc.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnAuthenticateFail<'a, Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, objectid: u64, guidiid: *const ::windows::runtime::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: Param9) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(pinfo),
            ::core::mem::transmute(guidactivity),
            ::core::mem::transmute(objectid),
            ::core::mem::transmute(guidiid),
            ::core::mem::transmute(imeth),
            ::core::mem::transmute(cbbyteorig),
            ::core::mem::transmute(psidoriginaluser),
            ::core::mem::transmute(cbbytecur),
            ::core::mem::transmute(psidcurrentuser),
            bcurrentuserinpersonatinginproc.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComSecurityEvents {
    type Vtable = IComSecurityEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130ac_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComSecurityEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComSecurityEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComSecurityEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComSecurityEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComSecurityEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComSecurityEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComSecurityEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, objectid: u64, guidiid: *const ::windows::runtime::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, objectid: u64, guidiid: *const ::windows::runtime::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComStaThreadPoolKnobs(pub ::windows::runtime::IUnknown);
impl IComStaThreadPoolKnobs {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMinThreadCount(&self, minthreads: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(minthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMinThreadCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMaxThreadCount(&self, maxthreads: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMaxThreadCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetActivityPerThread(&self, activitiesperthread: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(activitiesperthread)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetActivityPerThread(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetActivityRatio(&self, activityratio: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(activityratio)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetActivityRatio(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetThreadCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetQueueDepth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetQueueDepth(&self, dwqdepth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwqdepth)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComStaThreadPoolKnobs {
    type Vtable = IComStaThreadPoolKnobs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x324b64fa_33b6_11d2_98b7_00c04f8ee1c4);
}
impl ::core::convert::From<IComStaThreadPoolKnobs> for ::windows::runtime::IUnknown {
    fn from(value: IComStaThreadPoolKnobs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs> for ::windows::runtime::IUnknown {
    fn from(value: &IComStaThreadPoolKnobs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComStaThreadPoolKnobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComStaThreadPoolKnobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComStaThreadPoolKnobs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minthreads: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minthreads: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxthreads: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxthreads: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activitiesperthread: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activitiesperthread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityratio: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityratio: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwthreads: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwqdepth: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwqdepth: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComStaThreadPoolKnobs2(pub ::windows::runtime::IUnknown);
impl IComStaThreadPoolKnobs2 {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMinThreadCount(&self, minthreads: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(minthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMinThreadCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMaxThreadCount(&self, maxthreads: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMaxThreadCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetActivityPerThread(&self, activitiesperthread: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(activitiesperthread)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetActivityPerThread(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetActivityRatio(&self, activityratio: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(activityratio)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetActivityRatio(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetThreadCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetQueueDepth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetQueueDepth(&self, dwqdepth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwqdepth)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMaxCPULoad(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMaxCPULoad(&self, pdwload: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwload)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetCPUMetricEnabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetCPUMetricEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmetricenabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), bmetricenabled.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetCreateThreadsAggressively(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetCreateThreadsAggressively<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmetricenabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), bmetricenabled.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMaxCSR(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMaxCSR(&self, dwcsr: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcsr)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetWaitTimeForThreadCleanup(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadcleanupwaittime)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComStaThreadPoolKnobs2 {
    type Vtable = IComStaThreadPoolKnobs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x73707523_ff9a_4974_bf84_2108dc213740);
}
impl ::core::convert::From<IComStaThreadPoolKnobs2> for ::windows::runtime::IUnknown {
    fn from(value: IComStaThreadPoolKnobs2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs2> for ::windows::runtime::IUnknown {
    fn from(value: &IComStaThreadPoolKnobs2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComStaThreadPoolKnobs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComStaThreadPoolKnobs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IComStaThreadPoolKnobs2> for IComStaThreadPoolKnobs {
    fn from(value: IComStaThreadPoolKnobs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs2> for IComStaThreadPoolKnobs {
    fn from(value: &IComStaThreadPoolKnobs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IComStaThreadPoolKnobs> for IComStaThreadPoolKnobs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IComStaThreadPoolKnobs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IComStaThreadPoolKnobs> for &IComStaThreadPoolKnobs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IComStaThreadPoolKnobs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComStaThreadPoolKnobs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minthreads: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minthreads: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxthreads: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxthreads: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activitiesperthread: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activitiesperthread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityratio: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityratio: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwthreads: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwqdepth: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwqdepth: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwload: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwload: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcsr: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcsr: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwthreadcleanupwaittime: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadcleanupwaittime: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComThreadEvents(pub ::windows::runtime::IUnknown);
impl IComThreadEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(dwthread), ::core::mem::transmute(dwtheadcnt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(dwthread), ::core::mem::transmute(dwtheadcnt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(aptid), ::core::mem::transmute(dwactcnt), ::core::mem::transmute(dwlowcnt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(aptid), ::core::mem::transmute(dwactcnt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid), ::core::mem::transmute(queuelen)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid), ::core::mem::transmute(queuelen)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid), ::core::mem::transmute(queuelen), ::core::mem::transmute(threadnum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid), ::core::mem::transmute(queuelen)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, aptid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(aptid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(aptid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComThreadEvents {
    type Vtable = IComThreadEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130a5_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComThreadEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComThreadEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComThreadEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComThreadEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComThreadEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComThreadEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::runtime::GUID, aptid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComTrackingInfoCollection(pub ::windows::runtime::IUnknown);
impl IComTrackingInfoCollection {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<TRACKING_COLL_TYPE> {
        let mut result__: <TRACKING_COLL_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TRACKING_COLL_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Item(&self, ulindex: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComTrackingInfoCollection {
    type Vtable = IComTrackingInfoCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc266c677_c9ad_49ab_9fd9_d9661078588a);
}
impl ::core::convert::From<IComTrackingInfoCollection> for ::windows::runtime::IUnknown {
    fn from(value: IComTrackingInfoCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComTrackingInfoCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IComTrackingInfoCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComTrackingInfoCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComTrackingInfoCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut TRACKING_COLL_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComTrackingInfoEvents(pub ::windows::runtime::IUnknown);
impl IComTrackingInfoEvents {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn OnNewTrackingInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, ptoplevelcollection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptoplevelcollection.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComTrackingInfoEvents {
    type Vtable = IComTrackingInfoEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e6cdcc9_fb25_4fd5_9cc5_c9f4b6559cec);
}
impl ::core::convert::From<IComTrackingInfoEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComTrackingInfoEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComTrackingInfoEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComTrackingInfoEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComTrackingInfoEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComTrackingInfoEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptoplevelcollection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComTrackingInfoObject(pub ::windows::runtime::IUnknown);
impl IComTrackingInfoObject {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szpropertyname: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szpropertyname.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IComTrackingInfoObject {
    type Vtable = IComTrackingInfoObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x116e42c5_d8b1_47bf_ab1e_c895ed3e2372);
}
impl ::core::convert::From<IComTrackingInfoObject> for ::windows::runtime::IUnknown {
    fn from(value: IComTrackingInfoObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComTrackingInfoObject> for ::windows::runtime::IUnknown {
    fn from(value: &IComTrackingInfoObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComTrackingInfoObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComTrackingInfoObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szpropertyname: super::super::Foundation::PWSTR, pvarout: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComTrackingInfoProperties(pub ::windows::runtime::IUnknown);
impl IComTrackingInfoProperties {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn PropCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPropName(&self, ulindex: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IComTrackingInfoProperties {
    type Vtable = IComTrackingInfoProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x789b42be_6f6b_443a_898e_67abf390aa14);
}
impl ::core::convert::From<IComTrackingInfoProperties> for ::windows::runtime::IUnknown {
    fn from(value: IComTrackingInfoProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComTrackingInfoProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IComTrackingInfoProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComTrackingInfoProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComTrackingInfoProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, ppszpropname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComTransaction2Events(pub ::windows::runtime::IUnknown);
impl IComTransaction2Events {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransactionStart2<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID, tsid: *const ::windows::runtime::GUID, froot: Param3, nisolationlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), ::core::mem::transmute(tsid), froot.into_param().abi(), ::core::mem::transmute(nisolationlevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransactionPrepare2<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID, fvoteyes: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), fvoteyes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComTransaction2Events {
    type Vtable = IComTransaction2Events_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa136f62a_2f94_4288_86e0_d8a1fa4c0299);
}
impl ::core::convert::From<IComTransaction2Events> for ::windows::runtime::IUnknown {
    fn from(value: IComTransaction2Events) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComTransaction2Events> for ::windows::runtime::IUnknown {
    fn from(value: &IComTransaction2Events) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComTransaction2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComTransaction2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTransaction2Events_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID, tsid: *const ::windows::runtime::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComTransactionEvents(pub ::windows::runtime::IUnknown);
impl IComTransactionEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransactionStart<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID, tsid: *const ::windows::runtime::GUID, froot: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), ::core::mem::transmute(tsid), froot.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransactionPrepare<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID, fvoteyes: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), fvoteyes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComTransactionEvents {
    type Vtable = IComTransactionEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130a8_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComTransactionEvents> for ::windows::runtime::IUnknown {
    fn from(value: IComTransactionEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComTransactionEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IComTransactionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComTransactionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComTransactionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTransactionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID, tsid: *const ::windows::runtime::GUID, froot: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComUserEvent(pub ::windows::runtime::IUnknown);
impl IComUserEvent {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(pvarevent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComUserEvent {
    type Vtable = IComUserEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130a4_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<IComUserEvent> for ::windows::runtime::IUnknown {
    fn from(value: IComUserEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComUserEvent> for ::windows::runtime::IUnknown {
    fn from(value: &IComUserEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComUserEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComUserEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComUserEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContextProperties(pub ::windows::runtime::IUnknown);
impl IContextProperties {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self, plcount: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(plcount)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, pproperty: *mut super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EnumNames(&self) -> ::windows::runtime::Result<IEnumNames> {
        let mut result__: <IEnumNames as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNames>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, name: Param0, property: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), property.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn RemoveProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContextProperties {
    type Vtable = IContextProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd396da85_bf8f_11d1_bbae_00c04fc2fa5f);
}
impl ::core::convert::From<IContextProperties> for ::windows::runtime::IUnknown {
    fn from(value: IContextProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContextProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IContextProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContextProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContextProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContextSecurityPerimeter(pub ::windows::runtime::IUnknown);
impl IContextSecurityPerimeter {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPerimeterFlag(&self, pflag: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pflag)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetPerimeterFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fflag: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), fflag.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContextSecurityPerimeter {
    type Vtable = IContextSecurityPerimeter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa7549a29_a7c4_42e1_8dc1_7e3d748dc24a);
}
impl ::core::convert::From<IContextSecurityPerimeter> for ::windows::runtime::IUnknown {
    fn from(value: IContextSecurityPerimeter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContextSecurityPerimeter> for ::windows::runtime::IUnknown {
    fn from(value: &IContextSecurityPerimeter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContextSecurityPerimeter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContextSecurityPerimeter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextSecurityPerimeter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflag: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fflag: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContextState(pub ::windows::runtime::IUnknown);
impl IContextState {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetDeactivateOnReturn(&self, bdeactivate: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(bdeactivate)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetDeactivateOnReturn(&self, pbdeactivate: *mut i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdeactivate)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMyTransactionVote(&self, txvote: TransactionVote) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(txvote)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptxvote)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContextState {
    type Vtable = IContextState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3c05e54b_a42a_11d2_afc4_00c04f8ee1c4);
}
impl ::core::convert::From<IContextState> for ::windows::runtime::IUnknown {
    fn from(value: IContextState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContextState> for ::windows::runtime::IUnknown {
    fn from(value: &IContextState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContextState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContextState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextState_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bdeactivate: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdeactivate: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, txvote: TransactionVote) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptxvote: *mut TransactionVote) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICreateWithLocalTransaction(pub ::windows::runtime::IUnknown);
impl ICreateWithLocalTransaction {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn CreateInstanceWithSysTx<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, ptransaction: Param0, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptransaction.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(pobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICreateWithLocalTransaction {
    type Vtable = ICreateWithLocalTransaction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x227ac7a8_8423_42ce_b7cf_03061ec9aaa3);
}
impl ::core::convert::From<ICreateWithLocalTransaction> for ::windows::runtime::IUnknown {
    fn from(value: ICreateWithLocalTransaction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICreateWithLocalTransaction> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateWithLocalTransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateWithLocalTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICreateWithLocalTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithLocalTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICreateWithTipTransactionEx(pub ::windows::runtime::IUnknown);
impl ICreateWithTipTransactionEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtipurl: Param0, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrtipurl.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(pobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICreateWithTipTransactionEx {
    type Vtable = ICreateWithTipTransactionEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x455acf59_5345_11d2_99cf_00c04f797bc9);
}
impl ::core::convert::From<ICreateWithTipTransactionEx> for ::windows::runtime::IUnknown {
    fn from(value: ICreateWithTipTransactionEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICreateWithTipTransactionEx> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateWithTipTransactionEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateWithTipTransactionEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICreateWithTipTransactionEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithTipTransactionEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtipurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICreateWithTransactionEx(pub ::windows::runtime::IUnknown);
impl ICreateWithTransactionEx {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_DistributedTransactionCoordinator`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::DistributedTransactionCoordinator::ITransaction>>(&self, ptransaction: Param0, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptransaction.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(pobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICreateWithTransactionEx {
    type Vtable = ICreateWithTransactionEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x455acf57_5345_11d2_99cf_00c04f797bc9);
}
impl ::core::convert::From<ICreateWithTransactionEx> for ::windows::runtime::IUnknown {
    fn from(value: ICreateWithTransactionEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICreateWithTransactionEx> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateWithTransactionEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateWithTransactionEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICreateWithTransactionEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithTransactionEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICrmCompensator(pub ::windows::runtime::IUnknown);
impl ICrmCompensator {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetLogControl<'a, Param0: ::windows::runtime::IntoParam<'a, ICrmLogControl>>(&self, plogcontrol: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), plogcontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn BeginPrepare(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn PrepareRecord<'a, Param0: ::windows::runtime::IntoParam<'a, CrmLogRecordRead>>(&self, crmlogrec: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), crmlogrec.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn EndPrepare(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn BeginCommit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, frecovery: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), frecovery.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CommitRecord<'a, Param0: ::windows::runtime::IntoParam<'a, CrmLogRecordRead>>(&self, crmlogrec: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), crmlogrec.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EndCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn BeginAbort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, frecovery: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), frecovery.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AbortRecord<'a, Param0: ::windows::runtime::IntoParam<'a, CrmLogRecordRead>>(&self, crmlogrec: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), crmlogrec.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EndAbort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICrmCompensator {
    type Vtable = ICrmCompensator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbbc01830_8d3b_11d1_82ec_00a0c91eede9);
}
impl ::core::convert::From<ICrmCompensator> for ::windows::runtime::IUnknown {
    fn from(value: ICrmCompensator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICrmCompensator> for ::windows::runtime::IUnknown {
    fn from(value: &ICrmCompensator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICrmCompensator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICrmCompensator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmCompensator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plogcontrol: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frecovery: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frecovery: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICrmCompensatorVariants(pub ::windows::runtime::IUnknown);
impl ICrmCompensatorVariants {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetLogControlVariants<'a, Param0: ::windows::runtime::IntoParam<'a, ICrmLogControl>>(&self, plogcontrol: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), plogcontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn BeginPrepareVariants(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PrepareRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(plogrecord), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EndPrepareVariants(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn BeginCommitVariants(&self, brecovery: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(brecovery)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CommitRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(plogrecord), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EndCommitVariants(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn BeginAbortVariants(&self, brecovery: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(brecovery)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn AbortRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(plogrecord), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EndAbortVariants(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICrmCompensatorVariants {
    type Vtable = ICrmCompensatorVariants_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf0baf8e4_7804_11d1_82e9_00a0c91eede9);
}
impl ::core::convert::From<ICrmCompensatorVariants> for ::windows::runtime::IUnknown {
    fn from(value: ICrmCompensatorVariants) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICrmCompensatorVariants> for ::windows::runtime::IUnknown {
    fn from(value: &ICrmCompensatorVariants) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICrmCompensatorVariants {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICrmCompensatorVariants {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmCompensatorVariants_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plogcontrol: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plogrecord: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pbforget: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboktoprepare: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brecovery: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plogrecord: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pbforget: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brecovery: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plogrecord: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pbforget: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICrmFormatLogRecords(pub ::windows::runtime::IUnknown);
impl ICrmFormatLogRecords {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetColumnCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetColumnHeaders(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetColumn<'a, Param0: ::windows::runtime::IntoParam<'a, CrmLogRecordRead>>(&self, crmlogrec: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), crmlogrec.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetColumnVariants<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, logrecord: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), logrecord.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICrmFormatLogRecords {
    type Vtable = ICrmFormatLogRecords_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9c51d821_c98b_11d1_82fb_00a0c91eede9);
}
impl ::core::convert::From<ICrmFormatLogRecords> for ::windows::runtime::IUnknown {
    fn from(value: ICrmFormatLogRecords) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICrmFormatLogRecords> for ::windows::runtime::IUnknown {
    fn from(value: &ICrmFormatLogRecords) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICrmFormatLogRecords {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICrmFormatLogRecords {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmFormatLogRecords_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcolumncount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheaders: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logrecord: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pformattedlogrecord: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICrmLogControl(pub ::windows::runtime::IUnknown);
impl ICrmLogControl {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn TransactionUOW(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn RegisterCompensator<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpcwstrprogidcompensator: Param0, lpcwstrdescription: Param1, lcrmregflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), lpcwstrprogidcompensator.into_param().abi(), lpcwstrdescription.into_param().abi(), ::core::mem::transmute(lcrmregflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn WriteLogRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(plogrecord)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ForceLog(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ForgetLogRecord(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ForceTransactionToAbort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn WriteLogRecord(&self, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(rgblob), ::core::mem::transmute(cblob)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICrmLogControl {
    type Vtable = ICrmLogControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa0e174b3_d26e_11d2_8f84_00805fc7bcd9);
}
impl ::core::convert::From<ICrmLogControl> for ::windows::runtime::IUnknown {
    fn from(value: ICrmLogControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICrmLogControl> for ::windows::runtime::IUnknown {
    fn from(value: &ICrmLogControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICrmLogControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICrmLogControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmLogControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpcwstrprogidcompensator: super::super::Foundation::PWSTR, lpcwstrdescription: super::super::Foundation::PWSTR, lcrmregflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plogrecord: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICrmMonitor(pub ::windows::runtime::IUnknown);
impl ICrmMonitor {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetClerks(&self) -> ::windows::runtime::Result<ICrmMonitorClerks> {
        let mut result__: <ICrmMonitorClerks as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ICrmMonitorClerks>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn HoldClerk<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICrmMonitor {
    type Vtable = ICrmMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70c8e443_c7ed_11d1_82fb_00a0c91eede9);
}
impl ::core::convert::From<ICrmMonitor> for ::windows::runtime::IUnknown {
    fn from(value: ICrmMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICrmMonitor> for ::windows::runtime::IUnknown {
    fn from(value: &ICrmMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICrmMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICrmMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclerks: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICrmMonitorClerks(pub ::windows::runtime::IUnknown);
impl ICrmMonitorClerks {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn ProgIdCompensator<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Description<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn TransactionUOW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn ActivityId<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICrmMonitorClerks {
    type Vtable = ICrmMonitorClerks_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70c8e442_c7ed_11d1_82fb_00a0c91eede9);
}
impl ::core::convert::From<ICrmMonitorClerks> for ::windows::runtime::IUnknown {
    fn from(value: ICrmMonitorClerks) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICrmMonitorClerks> for ::windows::runtime::IUnknown {
    fn from(value: &ICrmMonitorClerks) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICrmMonitorClerks {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICrmMonitorClerks {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICrmMonitorClerks> for super::Com::IDispatch {
    fn from(value: ICrmMonitorClerks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICrmMonitorClerks> for super::Com::IDispatch {
    fn from(value: &ICrmMonitorClerks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ICrmMonitorClerks {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ICrmMonitorClerks {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitorClerks_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICrmMonitorLogRecords(pub ::windows::runtime::IUnknown);
impl ICrmMonitorLogRecords {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn TransactionState(&self) -> ::windows::runtime::Result<CrmTransactionState> {
        let mut result__: <CrmTransactionState as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<CrmTransactionState>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn StructuredRecords(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcrmlogrec)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetLogRecordVariants<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, indexnumber: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), indexnumber.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICrmMonitorLogRecords {
    type Vtable = ICrmMonitorLogRecords_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70c8e441_c7ed_11d1_82fb_00a0c91eede9);
}
impl ::core::convert::From<ICrmMonitorLogRecords> for ::windows::runtime::IUnknown {
    fn from(value: ICrmMonitorLogRecords) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICrmMonitorLogRecords> for ::windows::runtime::IUnknown {
    fn from(value: &ICrmMonitorLogRecords) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICrmMonitorLogRecords {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICrmMonitorLogRecords {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitorLogRecords_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut CrmTransactionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, indexnumber: ::core::mem::ManuallyDrop<super::Com::VARIANT>, plogrecord: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDispenserDriver(pub ::windows::runtime::IUnknown);
impl IDispenserDriver {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(restypid), ::core::mem::transmute(presid), ::core::mem::transmute(psecsfreebeforedestroy)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn RateResource<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, restypid: usize, resid: usize, frequirestransactionenlistment: Param2, prating: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(restypid), ::core::mem::transmute(resid), frequirestransactionenlistment.into_param().abi(), ::core::mem::transmute(prating)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EnlistResource(&self, resid: usize, transid: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(resid), ::core::mem::transmute(transid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ResetResource(&self, resid: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(resid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn DestroyResource(&self, resid: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(resid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn DestroyResourceS(&self, resid: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(resid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDispenserDriver {
    type Vtable = IDispenserDriver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x208b3651_2b48_11cf_be10_00aa00a2fa25);
}
impl ::core::convert::From<IDispenserDriver> for ::windows::runtime::IUnknown {
    fn from(value: IDispenserDriver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDispenserDriver> for ::windows::runtime::IUnknown {
    fn from(value: &IDispenserDriver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDispenserDriver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDispenserDriver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispenserDriver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resid: usize, transid: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resid: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resid: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resid: *mut u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDispenserManager(pub ::windows::runtime::IUnknown);
impl IDispenserManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn RegisterDispenser<'a, Param0: ::windows::runtime::IntoParam<'a, IDispenserDriver>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, __midl__idispensermanager0000: Param0, szdispensername: Param1) -> ::windows::runtime::Result<IHolder> {
        let mut result__: <IHolder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), __midl__idispensermanager0000.into_param().abi(), szdispensername.into_param().abi(), &mut result__).from_abi::<IHolder>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__idispensermanager0002), ::core::mem::transmute(__midl__idispensermanager0003)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDispenserManager {
    type Vtable = IDispenserManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5cb31e10_2b5f_11cf_be10_00aa00a2fa25);
}
impl ::core::convert::From<IDispenserManager> for ::windows::runtime::IUnknown {
    fn from(value: IDispenserManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDispenserManager> for ::windows::runtime::IUnknown {
    fn from(value: &IDispenserManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDispenserManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDispenserManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispenserManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__idispensermanager0000: ::windows::runtime::RawPtr, szdispensername: super::super::Foundation::PWSTR, __midl__idispensermanager0001: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumNames(pub ::windows::runtime::IUnknown);
impl IEnumNames {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgname), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumNames> {
        let mut result__: <IEnumNames as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNames>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumNames {
    type Vtable = IEnumNames_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372af2_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IEnumNames> for ::windows::runtime::IUnknown {
    fn from(value: IEnumNames) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumNames> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumNames) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumNames {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumNames {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNames_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEventServerTrace(pub ::windows::runtime::IUnknown);
impl IEventServerTrace {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn StartTraceGuid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidevent: Param0, bstrguidfilter: Param1, lpidfilter: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrguidevent.into_param().abi(), bstrguidfilter.into_param().abi(), ::core::mem::transmute(lpidfilter)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn StopTraceGuid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidevent: Param0, bstrguidfilter: Param1, lpidfilter: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrguidevent.into_param().abi(), bstrguidfilter.into_param().abi(), ::core::mem::transmute(lpidfilter)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(plcntguids), ::core::mem::transmute(pbstrguidlist)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEventServerTrace {
    type Vtable = IEventServerTrace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9a9f12b8_80af_47ab_a579_35ea57725370);
}
impl ::core::convert::From<IEventServerTrace> for ::windows::runtime::IUnknown {
    fn from(value: IEventServerTrace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEventServerTrace> for ::windows::runtime::IUnknown {
    fn from(value: &IEventServerTrace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEventServerTrace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEventServerTrace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEventServerTrace> for super::Com::IDispatch {
    fn from(value: IEventServerTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEventServerTrace> for super::Com::IDispatch {
    fn from(value: &IEventServerTrace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IEventServerTrace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IEventServerTrace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventServerTrace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcntguids: *mut i32, pbstrguidlist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGetAppTrackerData(pub ::windows::runtime::IUnknown);
impl IGetAppTrackerData {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetApplicationProcesses(&self, partitionid: *const ::windows::runtime::GUID, applicationid: *const ::windows::runtime::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(partitionid), ::core::mem::transmute(applicationid), ::core::mem::transmute(flags), ::core::mem::transmute(numapplicationprocesses), ::core::mem::transmute(applicationprocesses)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetApplicationProcessDetails(&self, applicationinstanceid: *const ::windows::runtime::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(applicationinstanceid),
            ::core::mem::transmute(processid),
            ::core::mem::transmute(flags),
            ::core::mem::transmute(summary),
            ::core::mem::transmute(statistics),
            ::core::mem::transmute(recycleinfo),
            ::core::mem::transmute(anycomponentshangmonitored),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetApplicationsInProcess(&self, applicationinstanceid: *const ::windows::runtime::GUID, processid: u32, partitionid: *const ::windows::runtime::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(applicationinstanceid), ::core::mem::transmute(processid), ::core::mem::transmute(partitionid), ::core::mem::transmute(flags), ::core::mem::transmute(numapplicationsinprocess), ::core::mem::transmute(applications)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetComponentsInProcess(&self, applicationinstanceid: *const ::windows::runtime::GUID, processid: u32, partitionid: *const ::windows::runtime::GUID, applicationid: *const ::windows::runtime::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(applicationinstanceid),
            ::core::mem::transmute(processid),
            ::core::mem::transmute(partitionid),
            ::core::mem::transmute(applicationid),
            ::core::mem::transmute(flags),
            ::core::mem::transmute(numcomponentsinprocess),
            ::core::mem::transmute(components),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetComponentDetails(&self, applicationinstanceid: *const ::windows::runtime::GUID, processid: u32, clsid: *const ::windows::runtime::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(applicationinstanceid), ::core::mem::transmute(processid), ::core::mem::transmute(clsid), ::core::mem::transmute(flags), ::core::mem::transmute(summary), ::core::mem::transmute(statistics), ::core::mem::transmute(hangmonitorinfo)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTrackerDataAsCollectionObject(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetSuggestedPollingInterval(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGetAppTrackerData {
    type Vtable = IGetAppTrackerData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x507c3ac8_3e12_4cb0_9366_653d3e050638);
}
impl ::core::convert::From<IGetAppTrackerData> for ::windows::runtime::IUnknown {
    fn from(value: IGetAppTrackerData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGetAppTrackerData> for ::windows::runtime::IUnknown {
    fn from(value: &IGetAppTrackerData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGetAppTrackerData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGetAppTrackerData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetAppTrackerData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partitionid: *const ::windows::runtime::GUID, applicationid: *const ::windows::runtime::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applicationinstanceid: *const ::windows::runtime::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applicationinstanceid: *const ::windows::runtime::GUID, processid: u32, partitionid: *const ::windows::runtime::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applicationinstanceid: *const ::windows::runtime::GUID, processid: u32, partitionid: *const ::windows::runtime::GUID, applicationid: *const ::windows::runtime::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applicationinstanceid: *const ::windows::runtime::GUID, processid: u32, clsid: *const ::windows::runtime::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, toplevelcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollingintervalinseconds: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGetContextProperties(pub ::windows::runtime::IUnknown);
impl IGetContextProperties {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self, plcount: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(plcount)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, pproperty: *mut super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EnumNames(&self) -> ::windows::runtime::Result<IEnumNames> {
        let mut result__: <IEnumNames as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNames>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGetContextProperties {
    type Vtable = IGetContextProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372af4_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IGetContextProperties> for ::windows::runtime::IUnknown {
    fn from(value: IGetContextProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGetContextProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IGetContextProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGetContextProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGetContextProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetContextProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGetSecurityCallContext(pub ::windows::runtime::IUnknown);
impl IGetSecurityCallContext {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetSecurityCallContext(&self) -> ::windows::runtime::Result<ISecurityCallContext> {
        let mut result__: <ISecurityCallContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISecurityCallContext>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGetSecurityCallContext {
    type Vtable = IGetSecurityCallContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcafc823f_b441_11d1_b82b_0000f8757e2a);
}
impl ::core::convert::From<IGetSecurityCallContext> for ::windows::runtime::IUnknown {
    fn from(value: IGetSecurityCallContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGetSecurityCallContext> for ::windows::runtime::IUnknown {
    fn from(value: &IGetSecurityCallContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGetSecurityCallContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGetSecurityCallContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGetSecurityCallContext> for super::Com::IDispatch {
    fn from(value: IGetSecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGetSecurityCallContext> for super::Com::IDispatch {
    fn from(value: &IGetSecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGetSecurityCallContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGetSecurityCallContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetSecurityCallContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHolder(pub ::windows::runtime::IUnknown);
impl IHolder {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iholder0000), ::core::mem::transmute(__midl__iholder0001)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn FreeResource(&self, __midl__iholder0002: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iholder0002)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn TrackResource(&self, __midl__iholder0003: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iholder0003)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iholder0004)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn UntrackResource<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, __midl__iholder0005: usize, __midl__iholder0006: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iholder0005), __midl__iholder0006.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn UntrackResourceS<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iholder0007), __midl__iholder0008.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iholder0009)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHolder {
    type Vtable = IHolder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbf6a1850_2b45_11cf_be10_00aa00a2fa25);
}
impl ::core::convert::From<IHolder> for ::windows::runtime::IUnknown {
    fn from(value: IHolder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHolder> for ::windows::runtime::IUnknown {
    fn from(value: &IHolder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__iholder0002: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__iholder0003: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__iholder0004: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__iholder0009: usize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILBEvents(pub ::windows::runtime::IUnknown);
impl ILBEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn TargetUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrservername: Param0, bstrclsideng: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrservername.into_param().abi(), bstrclsideng.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn TargetDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrservername: Param0, bstrclsideng: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrservername.into_param().abi(), bstrclsideng.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn EngineDefined<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpropname: Param0, varpropvalue: *const super::Com::VARIANT, bstrclsideng: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrpropname.into_param().abi(), ::core::mem::transmute(varpropvalue), bstrclsideng.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILBEvents {
    type Vtable = ILBEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x683130b4_2e50_11d2_98a5_00c04f8ee1c4);
}
impl ::core::convert::From<ILBEvents> for ::windows::runtime::IUnknown {
    fn from(value: ILBEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILBEvents> for ::windows::runtime::IUnknown {
    fn from(value: &ILBEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILBEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILBEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILBEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varpropvalue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMTSActivity(pub ::windows::runtime::IUnknown);
impl IMTSActivity {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SynchronousCall<'a, Param0: ::windows::runtime::IntoParam<'a, IMTSCall>>(&self, pcall: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcall.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn AsyncCall<'a, Param0: ::windows::runtime::IntoParam<'a, IMTSCall>>(&self, pcall: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcall.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved1(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn BindToCurrentThread(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn UnbindFromThread(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMTSActivity {
    type Vtable = IMTSActivity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372af0_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IMTSActivity> for ::windows::runtime::IUnknown {
    fn from(value: IMTSActivity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMTSActivity> for ::windows::runtime::IUnknown {
    fn from(value: &IMTSActivity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMTSActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMTSActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSActivity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcall: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcall: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMTSCall(pub ::windows::runtime::IUnknown);
impl IMTSCall {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn OnCall(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMTSCall {
    type Vtable = IMTSCall_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372aef_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IMTSCall> for ::windows::runtime::IUnknown {
    fn from(value: IMTSCall) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMTSCall> for ::windows::runtime::IUnknown {
    fn from(value: &IMTSCall) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMTSCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMTSCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSCall_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMTSLocator(pub ::windows::runtime::IUnknown);
impl IMTSLocator {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetEventDispatcher(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMTSLocator {
    type Vtable = IMTSLocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd19b8bfd_7f88_11d0_b16e_00aa00ba3258);
}
impl ::core::convert::From<IMTSLocator> for ::windows::runtime::IUnknown {
    fn from(value: IMTSLocator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMTSLocator> for ::windows::runtime::IUnknown {
    fn from(value: &IMTSLocator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMTSLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMTSLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMTSLocator> for super::Com::IDispatch {
    fn from(value: IMTSLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMTSLocator> for super::Com::IDispatch {
    fn from(value: &IMTSLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IMTSLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IMTSLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSLocator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IManagedActivationEvents(pub ::windows::runtime::IUnknown);
impl IManagedActivationEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn CreateManagedStub<'a, Param0: ::windows::runtime::IntoParam<'a, IManagedObjectInfo>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pinfo: Param0, fdist: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pinfo.into_param().abi(), fdist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn DestroyManagedStub<'a, Param0: ::windows::runtime::IntoParam<'a, IManagedObjectInfo>>(&self, pinfo: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pinfo.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IManagedActivationEvents {
    type Vtable = IManagedActivationEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa5f325af_572f_46da_b8ab_827c3d95d99e);
}
impl ::core::convert::From<IManagedActivationEvents> for ::windows::runtime::IUnknown {
    fn from(value: IManagedActivationEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IManagedActivationEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IManagedActivationEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IManagedActivationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IManagedActivationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedActivationEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: ::windows::runtime::RawPtr, fdist: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IManagedObjectInfo(pub ::windows::runtime::IUnknown);
impl IManagedObjectInfo {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetIUnknown(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetIObjectControl(&self) -> ::windows::runtime::Result<IObjectControl> {
        let mut result__: <IObjectControl as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IObjectControl>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetInPool<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, IManagedPooledObj>>(&self, binpool: Param0, ppooledobj: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), binpool.into_param().abi(), ppooledobj.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetWrapperStrength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrong: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrong.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IManagedObjectInfo {
    type Vtable = IManagedObjectInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1427c51a_4584_49d8_90a0_c50d8086cbe9);
}
impl ::core::convert::From<IManagedObjectInfo> for ::windows::runtime::IUnknown {
    fn from(value: IManagedObjectInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IManagedObjectInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IManagedObjectInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IManagedObjectInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IManagedObjectInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedObjectInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctrl: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binpool: super::super::Foundation::BOOL, ppooledobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrong: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IManagedPoolAction(pub ::windows::runtime::IUnknown);
impl IManagedPoolAction {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn LastRelease(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IManagedPoolAction {
    type Vtable = IManagedPoolAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xda91b74e_5388_4783_949d_c1cd5fb00506);
}
impl ::core::convert::From<IManagedPoolAction> for ::windows::runtime::IUnknown {
    fn from(value: IManagedPoolAction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IManagedPoolAction> for ::windows::runtime::IUnknown {
    fn from(value: &IManagedPoolAction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IManagedPoolAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IManagedPoolAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedPoolAction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IManagedPooledObj(pub ::windows::runtime::IUnknown);
impl IManagedPooledObj {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetHeld<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, m_bheld: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), m_bheld.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IManagedPooledObj {
    type Vtable = IManagedPooledObj_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc5da4bea_1b42_4437_8926_b6a38860a770);
}
impl ::core::convert::From<IManagedPooledObj> for ::windows::runtime::IUnknown {
    fn from(value: IManagedPooledObj) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IManagedPooledObj> for ::windows::runtime::IUnknown {
    fn from(value: &IManagedPooledObj) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IManagedPooledObj {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IManagedPooledObj {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedPooledObj_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, m_bheld: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMessageMover(pub ::windows::runtime::IUnknown);
impl IMessageMover {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SourcePath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetSourcePath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn DestPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetDestPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn CommitBatchSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetCommitBatchSize(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MoveMessages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMessageMover {
    type Vtable = IMessageMover_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x588a085a_b795_11d1_8054_00c04fc340ee);
}
impl ::core::convert::From<IMessageMover> for ::windows::runtime::IUnknown {
    fn from(value: IMessageMover) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMessageMover> for ::windows::runtime::IUnknown {
    fn from(value: &IMessageMover) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMessageMover {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMessageMover {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMessageMover> for super::Com::IDispatch {
    fn from(value: IMessageMover) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMessageMover> for super::Com::IDispatch {
    fn from(value: &IMessageMover) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IMessageMover {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IMessageMover {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageMover_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmessagesmoved: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMtsEventInfo(pub ::windows::runtime::IUnknown);
impl IMtsEventInfo {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Names(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn EventID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Value<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, skey: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), skey.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMtsEventInfo {
    type Vtable = IMtsEventInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd56c3dc1_8482_11d0_b170_00aa00ba3258);
}
impl ::core::convert::From<IMtsEventInfo> for ::windows::runtime::IUnknown {
    fn from(value: IMtsEventInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMtsEventInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IMtsEventInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMtsEventInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMtsEventInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEventInfo> for super::Com::IDispatch {
    fn from(value: IMtsEventInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEventInfo> for super::Com::IDispatch {
    fn from(value: &IMtsEventInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IMtsEventInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IMtsEventInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMtsEventInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sdisplayname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sguideventid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, skey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMtsEvents(pub ::windows::runtime::IUnknown);
impl IMtsEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn PackageName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn PackageGuid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PostEvent(&self, vevent: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(vevent)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn FireEvents(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetProcessID(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMtsEvents {
    type Vtable = IMtsEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbacedf4d_74ab_11d0_b162_00aa00ba3258);
}
impl ::core::convert::From<IMtsEvents> for ::windows::runtime::IUnknown {
    fn from(value: IMtsEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMtsEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IMtsEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMtsEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMtsEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEvents> for super::Com::IDispatch {
    fn from(value: IMtsEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEvents> for super::Com::IDispatch {
    fn from(value: &IMtsEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IMtsEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IMtsEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMtsEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vevent: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMtsGrp(pub ::windows::runtime::IUnknown);
impl IMtsGrp {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMtsGrp {
    type Vtable = IMtsGrp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4b2e958c_0393_11d1_b1ab_00aa00ba3258);
}
impl ::core::convert::From<IMtsGrp> for ::windows::runtime::IUnknown {
    fn from(value: IMtsGrp) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMtsGrp> for ::windows::runtime::IUnknown {
    fn from(value: &IMtsGrp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMtsGrp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMtsGrp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsGrp> for super::Com::IDispatch {
    fn from(value: IMtsGrp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsGrp> for super::Com::IDispatch {
    fn from(value: &IMtsGrp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IMtsGrp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IMtsGrp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMtsGrp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, ppunkdispatcher: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjPool(pub ::windows::runtime::IUnknown);
impl IObjPool {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved1(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved2(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved3(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved4(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn PutEndTx<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pobj: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pobj.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved5(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved6(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IObjPool {
    type Vtable = IObjPool_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7d8805a0_2ea7_11d1_b1cc_00aa00ba3258);
}
impl ::core::convert::From<IObjPool> for ::windows::runtime::IUnknown {
    fn from(value: IObjPool) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjPool> for ::windows::runtime::IUnknown {
    fn from(value: &IObjPool) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjPool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjPool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjPool_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobj: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectConstruct(pub ::windows::runtime::IUnknown);
impl IObjectConstruct {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn Construct<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDispatch>>(&self, pctorobj: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pctorobj.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectConstruct {
    type Vtable = IObjectConstruct_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x41c4f8b3_7439_11d2_98cb_00c04f8ee1c4);
}
impl ::core::convert::From<IObjectConstruct> for ::windows::runtime::IUnknown {
    fn from(value: IObjectConstruct) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectConstruct> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectConstruct) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectConstruct {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectConstruct {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectConstruct_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctorobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectConstructString(pub ::windows::runtime::IUnknown);
impl IObjectConstructString {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ConstructString(&self, pval: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pval)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectConstructString {
    type Vtable = IObjectConstructString_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x41c4f8b2_7439_11d2_98cb_00c04f8ee1c4);
}
impl ::core::convert::From<IObjectConstructString> for ::windows::runtime::IUnknown {
    fn from(value: IObjectConstructString) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectConstructString> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectConstructString) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectConstructString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectConstructString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IObjectConstructString> for super::Com::IDispatch {
    fn from(value: IObjectConstructString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IObjectConstructString> for super::Com::IDispatch {
    fn from(value: &IObjectConstructString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IObjectConstructString {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IObjectConstructString {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectConstructString_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectContext(pub ::windows::runtime::IUnknown);
impl IObjectContext {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn CreateInstance(&self, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetAbort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EnableCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn DisableCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsInTransaction(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsSecurityEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsCallerInRole<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrole: Param0, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrrole.into_param().abi(), ::core::mem::transmute(pfisinrole)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectContext {
    type Vtable = IObjectContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372ae0_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IObjectContext> for ::windows::runtime::IUnknown {
    fn from(value: IObjectContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectContext> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectContextActivity(pub ::windows::runtime::IUnknown);
impl IObjectContextActivity {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectContextActivity {
    type Vtable = IObjectContextActivity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372afc_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IObjectContextActivity> for ::windows::runtime::IUnknown {
    fn from(value: IObjectContextActivity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectContextActivity> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectContextActivity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectContextActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectContextActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextActivity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectContextInfo(pub ::windows::runtime::IUnknown);
impl IObjectContextInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsInTransaction(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTransaction(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTransactionId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetContextId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectContextInfo {
    type Vtable = IObjectContextInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x75b52ddb_e8ed_11d1_93ad_00aa00ba3258);
}
impl ::core::convert::From<IObjectContextInfo> for ::windows::runtime::IUnknown {
    fn from(value: IObjectContextInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectContextInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectContextInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectContextInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectContextInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptrans: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectContextInfo2(pub ::windows::runtime::IUnknown);
impl IObjectContextInfo2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsInTransaction(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTransaction(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTransactionId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetContextId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetPartitionId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetApplicationId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetApplicationInstanceId(&self, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectContextInfo2 {
    type Vtable = IObjectContextInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x594be71a_4bc4_438b_9197_cfd176248b09);
}
impl ::core::convert::From<IObjectContextInfo2> for ::windows::runtime::IUnknown {
    fn from(value: IObjectContextInfo2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectContextInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectContextInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IObjectContextInfo2> for IObjectContextInfo {
    fn from(value: IObjectContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextInfo2> for IObjectContextInfo {
    fn from(value: &IObjectContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObjectContextInfo> for IObjectContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObjectContextInfo> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObjectContextInfo> for &IObjectContextInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObjectContextInfo> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptrans: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectContextTip(pub ::windows::runtime::IUnknown);
impl IObjectContextTip {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTipUrl(&self, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptipurl)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectContextTip {
    type Vtable = IObjectContextTip_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92fd41ca_bad9_11d2_9a2d_00c04f797bc9);
}
impl ::core::convert::From<IObjectContextTip> for ::windows::runtime::IUnknown {
    fn from(value: IObjectContextTip) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectContextTip> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectContextTip) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectContextTip {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectContextTip {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextTip_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptipurl: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IObjectControl(pub ::windows::runtime::IUnknown);
impl IObjectControl {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Activate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Deactivate(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn CanBePooled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IObjectControl {
    type Vtable = IObjectControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372aec_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IObjectControl> for ::windows::runtime::IUnknown {
    fn from(value: IObjectControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IObjectControl> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPlaybackControl(pub ::windows::runtime::IUnknown);
impl IPlaybackControl {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn FinalClientRetry(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn FinalServerRetry(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPlaybackControl {
    type Vtable = IPlaybackControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372afd_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IPlaybackControl> for ::windows::runtime::IUnknown {
    fn from(value: IPlaybackControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPlaybackControl> for ::windows::runtime::IUnknown {
    fn from(value: &IPlaybackControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPlaybackControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPlaybackControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPoolManager(pub ::windows::runtime::IUnknown);
impl IPoolManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ShutdownPool<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, clsidorprogid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), clsidorprogid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPoolManager {
    type Vtable = IPoolManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0a469861_5a91_43a0_99b6_d5e179bb0631);
}
impl ::core::convert::From<IPoolManager> for ::windows::runtime::IUnknown {
    fn from(value: IPoolManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPoolManager> for ::windows::runtime::IUnknown {
    fn from(value: &IPoolManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPoolManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPoolManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPoolManager> for super::Com::IDispatch {
    fn from(value: IPoolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPoolManager> for super::Com::IDispatch {
    fn from(value: &IPoolManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IPoolManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IPoolManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPoolManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProcessInitializer(pub ::windows::runtime::IUnknown);
impl IProcessInitializer {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Startup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkprocesscontrol: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkprocesscontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Shutdown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProcessInitializer {
    type Vtable = IProcessInitializer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1113f52d_dc7f_4943_aed6_88d04027e32a);
}
impl ::core::convert::From<IProcessInitializer> for ::windows::runtime::IUnknown {
    fn from(value: IProcessInitializer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProcessInitializer> for ::windows::runtime::IUnknown {
    fn from(value: &IProcessInitializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProcessInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProcessInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitializer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkprocesscontrol: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityCallContext(pub ::windows::runtime::IUnknown);
impl ISecurityCallContext {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsCallerInRole<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrole: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrrole.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsSecurityEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn IsUserInRole<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, puser: *const super::Com::VARIANT, bstrrole: Param1) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(puser), bstrrole.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityCallContext {
    type Vtable = ISecurityCallContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcafc823e_b441_11d1_b82b_0000f8757e2a);
}
impl ::core::convert::From<ISecurityCallContext> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityCallContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityCallContext> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityCallContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityCallContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityCallContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallContext> for super::Com::IDispatch {
    fn from(value: ISecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallContext> for super::Com::IDispatch {
    fn from(value: &ISecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ISecurityCallContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ISecurityCallContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityCallContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfisenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puser: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityCallersColl(pub ::windows::runtime::IUnknown);
impl ISecurityCallersColl {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<ISecurityIdentityColl> {
        let mut result__: <ISecurityIdentityColl as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<ISecurityIdentityColl>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityCallersColl {
    type Vtable = ISecurityCallersColl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcafc823d_b441_11d1_b82b_0000f8757e2a);
}
impl ::core::convert::From<ISecurityCallersColl> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityCallersColl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityCallersColl> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityCallersColl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityCallersColl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityCallersColl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallersColl> for super::Com::IDispatch {
    fn from(value: ISecurityCallersColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallersColl> for super::Com::IDispatch {
    fn from(value: &ISecurityCallersColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ISecurityCallersColl {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ISecurityCallersColl {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityCallersColl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pobj: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityIdentityColl(pub ::windows::runtime::IUnknown);
impl ISecurityIdentityColl {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityIdentityColl {
    type Vtable = ISecurityIdentityColl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcafc823c_b441_11d1_b82b_0000f8757e2a);
}
impl ::core::convert::From<ISecurityIdentityColl> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityIdentityColl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityIdentityColl> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityIdentityColl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityIdentityColl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityIdentityColl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityIdentityColl> for super::Com::IDispatch {
    fn from(value: ISecurityIdentityColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityIdentityColl> for super::Com::IDispatch {
    fn from(value: &ISecurityIdentityColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ISecurityIdentityColl {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ISecurityIdentityColl {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityIdentityColl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityProperty(pub ::windows::runtime::IUnknown);
impl ISecurityProperty {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDirectCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(psid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetOriginalCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(psid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDirectCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(psid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetOriginalCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(psid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn ReleaseSID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>>(&self, psid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), psid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityProperty {
    type Vtable = ISecurityProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372aea_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<ISecurityProperty> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityProperty> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psid: *mut super::super::Foundation::PSID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psid: *mut super::super::Foundation::PSID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psid: *mut super::super::Foundation::PSID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psid: *mut super::super::Foundation::PSID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psid: super::super::Foundation::PSID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISelectCOMLBServer(pub ::windows::runtime::IUnknown);
impl ISelectCOMLBServer {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Init(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetLBServer<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISelectCOMLBServer {
    type Vtable = ISelectCOMLBServer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdcf443f4_3f8a_4872_b9f0_369a796d12d6);
}
impl ::core::convert::From<ISelectCOMLBServer> for ::windows::runtime::IUnknown {
    fn from(value: ISelectCOMLBServer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISelectCOMLBServer> for ::windows::runtime::IUnknown {
    fn from(value: &ISelectCOMLBServer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISelectCOMLBServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISelectCOMLBServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectCOMLBServer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISendMethodEvents(pub ::windows::runtime::IUnknown);
impl ISendMethodEvents {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SendMethodCall(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::runtime::GUID, dwmeth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pidentity), ::core::mem::transmute(riid), ::core::mem::transmute(dwmeth)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SendMethodReturn(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::runtime::GUID, dwmeth: u32, hrcall: ::windows::runtime::HRESULT, hrserver: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pidentity), ::core::mem::transmute(riid), ::core::mem::transmute(dwmeth), ::core::mem::transmute(hrcall), ::core::mem::transmute(hrserver)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISendMethodEvents {
    type Vtable = ISendMethodEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2732fd59_b2b4_4d44_878c_8b8f09626008);
}
impl ::core::convert::From<ISendMethodEvents> for ::windows::runtime::IUnknown {
    fn from(value: ISendMethodEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISendMethodEvents> for ::windows::runtime::IUnknown {
    fn from(value: &ISendMethodEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISendMethodEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISendMethodEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISendMethodEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::runtime::GUID, dwmeth: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::runtime::GUID, dwmeth: u32, hrcall: ::windows::runtime::HRESULT, hrserver: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceActivity(pub ::windows::runtime::IUnknown);
impl IServiceActivity {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SynchronousCall<'a, Param0: ::windows::runtime::IntoParam<'a, IServiceCall>>(&self, piservicecall: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), piservicecall.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn AsynchronousCall<'a, Param0: ::windows::runtime::IntoParam<'a, IServiceCall>>(&self, piservicecall: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), piservicecall.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn BindToCurrentThread(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn UnbindFromThread(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceActivity {
    type Vtable = IServiceActivity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x67532e0c_9e2f_4450_a354_035633944e17);
}
impl ::core::convert::From<IServiceActivity> for ::windows::runtime::IUnknown {
    fn from(value: IServiceActivity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceActivity> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceActivity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceActivity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piservicecall: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piservicecall: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceCall(pub ::windows::runtime::IUnknown);
impl IServiceCall {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn OnCall(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceCall {
    type Vtable = IServiceCall_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbd3e2e12_42dd_40f4_a09a_95a50c58304b);
}
impl ::core::convert::From<IServiceCall> for ::windows::runtime::IUnknown {
    fn from(value: IServiceCall) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceCall> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceCall) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceCall_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceComTIIntrinsicsConfig(pub ::windows::runtime::IUnknown);
impl IServiceComTIIntrinsicsConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(comtiintrinsicsconfig)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceComTIIntrinsicsConfig {
    type Vtable = IServiceComTIIntrinsicsConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x09e6831e_04e1_4ed4_9d0f_e8b168bafeaf);
}
impl ::core::convert::From<IServiceComTIIntrinsicsConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceComTIIntrinsicsConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceComTIIntrinsicsConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceComTIIntrinsicsConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceComTIIntrinsicsConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceComTIIntrinsicsConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceComTIIntrinsicsConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceIISIntrinsicsConfig(pub ::windows::runtime::IUnknown);
impl IServiceIISIntrinsicsConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iisintrinsicsconfig)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceIISIntrinsicsConfig {
    type Vtable = IServiceIISIntrinsicsConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1a0cf920_d452_46f4_bc36_48118d54ea52);
}
impl ::core::convert::From<IServiceIISIntrinsicsConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceIISIntrinsicsConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceIISIntrinsicsConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceIISIntrinsicsConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceIISIntrinsicsConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceIISIntrinsicsConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceIISIntrinsicsConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceInheritanceConfig(pub ::windows::runtime::IUnknown);
impl IServiceInheritanceConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(inheritanceconfig)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceInheritanceConfig {
    type Vtable = IServiceInheritanceConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92186771_d3b4_4d77_a8ea_ee842d586f35);
}
impl ::core::convert::From<IServiceInheritanceConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceInheritanceConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceInheritanceConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceInheritanceConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceInheritanceConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceInheritanceConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceInheritanceConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServicePartitionConfig(pub ::windows::runtime::IUnknown);
impl IServicePartitionConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(partitionconfig)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn PartitionID(&self, guidpartitionid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidpartitionid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServicePartitionConfig {
    type Vtable = IServicePartitionConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80182d03_5ea4_4831_ae97_55beffc2e590);
}
impl ::core::convert::From<IServicePartitionConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServicePartitionConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServicePartitionConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServicePartitionConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServicePartitionConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServicePartitionConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePartitionConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partitionconfig: CSC_PartitionConfig) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidpartitionid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServicePool(pub ::windows::runtime::IUnknown);
impl IServicePool {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, ppoolconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ppoolconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetObject(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Shutdown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServicePool {
    type Vtable = IServicePool_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb302df81_ea45_451e_99a2_09f9fd1b1e13);
}
impl ::core::convert::From<IServicePool> for ::windows::runtime::IUnknown {
    fn from(value: IServicePool) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServicePool> for ::windows::runtime::IUnknown {
    fn from(value: &IServicePool) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServicePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServicePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePool_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppoolconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServicePoolConfig(pub ::windows::runtime::IUnknown);
impl IServicePoolConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMaxPoolSize(&self, dwmaxpool: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxpool)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwmaxpool)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMinPoolSize(&self, dwminpool: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwminpool)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn MinPoolSize(&self, pdwminpool: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwminpool)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreationtimeout)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcreationtimeout)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SetTransactionAffinity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ftxaffinity: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ftxaffinity.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn TransactionAffinity(&self, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pftxaffinity)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn SetClassFactory<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IClassFactory>>(&self, pfactory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pfactory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    pub unsafe fn ClassFactory(&self) -> ::windows::runtime::Result<super::Com::IClassFactory> {
        let mut result__: <super::Com::IClassFactory as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IClassFactory>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IServicePoolConfig {
    type Vtable = IServicePoolConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa9690656_5bca_470c_8451_250c1f43a33e);
}
impl ::core::convert::From<IServicePoolConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServicePoolConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServicePoolConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServicePoolConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServicePoolConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServicePoolConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePoolConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmaxpool: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmaxpool: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwminpool: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwminpool: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcreationtimeout: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcreationtimeout: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ftxaffinity: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfactory: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfactory: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceSxsConfig(pub ::windows::runtime::IUnknown);
impl IServiceSxsConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(scsconfig)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SxsName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szsxsname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), szsxsname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn SxsDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szsxsdirectory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), szsxsdirectory.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceSxsConfig {
    type Vtable = IServiceSxsConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc7cd7379_f3f2_4634_811b_703281d73e08);
}
impl ::core::convert::From<IServiceSxsConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceSxsConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceSxsConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceSxsConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceSxsConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceSxsConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSxsConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scsconfig: CSC_SxsConfig) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szsxsname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szsxsdirectory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceSynchronizationConfig(pub ::windows::runtime::IUnknown);
impl IServiceSynchronizationConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(synchconfig)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceSynchronizationConfig {
    type Vtable = IServiceSynchronizationConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfd880e81_6dce_4c58_af83_a208846c0030);
}
impl ::core::convert::From<IServiceSynchronizationConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceSynchronizationConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceSynchronizationConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceSynchronizationConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceSynchronizationConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceSynchronizationConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSynchronizationConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, synchconfig: CSC_SynchronizationConfig) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceSysTxnConfig(pub ::windows::runtime::IUnknown);
impl IServiceSysTxnConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(transactionconfig)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(option)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ultimeoutsec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn BringYourOwnTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sztipurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sztipurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn NewTransactionDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sztxdesc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sztxdesc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_DistributedTransactionCoordinator`*"]
    pub unsafe fn ConfigureBYOT<'a, Param0: ::windows::runtime::IntoParam<'a, super::DistributedTransactionCoordinator::ITransaction>>(&self, pitxbyot: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pitxbyot.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ConfigureBYOTSysTxn<'a, Param0: ::windows::runtime::IntoParam<'a, ITransactionProxy>>(&self, ptxproxy: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ptxproxy.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceSysTxnConfig {
    type Vtable = IServiceSysTxnConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x33caf1a1_fcb8_472b_b45e_967448ded6d8);
}
impl ::core::convert::From<IServiceSysTxnConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceSysTxnConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceSysTxnConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceSysTxnConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IServiceSysTxnConfig> for IServiceTransactionConfig {
    fn from(value: IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for IServiceTransactionConfig {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IServiceTransactionConfig> for IServiceSysTxnConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, IServiceTransactionConfig> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IServiceTransactionConfig> for &IServiceSysTxnConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, IServiceTransactionConfig> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IServiceSysTxnConfig> for IServiceTransactionConfigBase {
    fn from(value: IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for IServiceTransactionConfigBase {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IServiceTransactionConfigBase> for IServiceSysTxnConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, IServiceTransactionConfigBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IServiceTransactionConfigBase> for &IServiceSysTxnConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, IServiceTransactionConfigBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSysTxnConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transactionconfig: CSC_TransactionConfig) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, option: COMAdminTxIsolationLevelOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ultimeoutsec: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sztipurl: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sztxdesc: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitxbyot: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptxproxy: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceThreadPoolConfig(pub ::windows::runtime::IUnknown);
impl IServiceThreadPoolConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(threadpool)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetBindingInfo(&self, binding: CSC_Binding) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(binding)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceThreadPoolConfig {
    type Vtable = IServiceThreadPoolConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x186d89bc_f277_4bcc_80d5_4df7b836ef4a);
}
impl ::core::convert::From<IServiceThreadPoolConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceThreadPoolConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceThreadPoolConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceThreadPoolConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceThreadPoolConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceThreadPoolConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceThreadPoolConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threadpool: CSC_ThreadPool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binding: CSC_Binding) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceTrackerConfig(pub ::windows::runtime::IUnknown);
impl IServiceTrackerConfig {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn TrackerConfig<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: Param1, sztrackerctxname: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(trackerconfig), sztrackerappname.into_param().abi(), sztrackerctxname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceTrackerConfig {
    type Vtable = IServiceTrackerConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6c3a3e1d_0ba6_4036_b76f_d0404db816c9);
}
impl ::core::convert::From<IServiceTrackerConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceTrackerConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceTrackerConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceTrackerConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceTrackerConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceTrackerConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTrackerConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, trackerconfig: CSC_TrackerConfig, sztrackerappname: super::super::Foundation::PWSTR, sztrackerctxname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceTransactionConfig(pub ::windows::runtime::IUnknown);
impl IServiceTransactionConfig {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(transactionconfig)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(option)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ultimeoutsec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn BringYourOwnTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sztipurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sztipurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn NewTransactionDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sztxdesc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sztxdesc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_DistributedTransactionCoordinator`*"]
    pub unsafe fn ConfigureBYOT<'a, Param0: ::windows::runtime::IntoParam<'a, super::DistributedTransactionCoordinator::ITransaction>>(&self, pitxbyot: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pitxbyot.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceTransactionConfig {
    type Vtable = IServiceTransactionConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x59f4c2a3_d3d7_4a31_b6e4_6ab3177c50b9);
}
impl ::core::convert::From<IServiceTransactionConfig> for ::windows::runtime::IUnknown {
    fn from(value: IServiceTransactionConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceTransactionConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceTransactionConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceTransactionConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceTransactionConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IServiceTransactionConfig> for IServiceTransactionConfigBase {
    fn from(value: IServiceTransactionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTransactionConfig> for IServiceTransactionConfigBase {
    fn from(value: &IServiceTransactionConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IServiceTransactionConfigBase> for IServiceTransactionConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, IServiceTransactionConfigBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IServiceTransactionConfigBase> for &IServiceTransactionConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, IServiceTransactionConfigBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTransactionConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transactionconfig: CSC_TransactionConfig) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, option: COMAdminTxIsolationLevelOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ultimeoutsec: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sztipurl: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sztxdesc: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitxbyot: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceTransactionConfigBase(pub ::windows::runtime::IUnknown);
impl IServiceTransactionConfigBase {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(transactionconfig)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(option)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ultimeoutsec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn BringYourOwnTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sztipurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sztipurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn NewTransactionDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sztxdesc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sztxdesc.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceTransactionConfigBase {
    type Vtable = IServiceTransactionConfigBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x772b3fbe_6ffd_42fb_b5f8_8f9b260f3810);
}
impl ::core::convert::From<IServiceTransactionConfigBase> for ::windows::runtime::IUnknown {
    fn from(value: IServiceTransactionConfigBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceTransactionConfigBase> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceTransactionConfigBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceTransactionConfigBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceTransactionConfigBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTransactionConfigBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transactionconfig: CSC_TransactionConfig) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, option: COMAdminTxIsolationLevelOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ultimeoutsec: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sztipurl: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sztxdesc: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISharedProperty(pub ::windows::runtime::IUnknown);
impl ISharedProperty {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Value(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, val: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISharedProperty {
    type Vtable = ISharedProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a005c01_a5de_11cf_9e66_00aa00a3f464);
}
impl ::core::convert::From<ISharedProperty> for ::windows::runtime::IUnknown {
    fn from(value: ISharedProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISharedProperty> for ::windows::runtime::IUnknown {
    fn from(value: &ISharedProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISharedProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISharedProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedProperty> for super::Com::IDispatch {
    fn from(value: ISharedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedProperty> for super::Com::IDispatch {
    fn from(value: &ISharedProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ISharedProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ISharedProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISharedPropertyGroup(pub ::windows::runtime::IUnknown);
impl ISharedPropertyGroup {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn CreatePropertyByPosition(&self, index: i32, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(fexists), ::core::mem::transmute(ppprop)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn PropertyByPosition(&self, index: i32) -> ::windows::runtime::Result<ISharedProperty> {
        let mut result__: <ISharedProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<ISharedProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn CreateProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(fexists), ::core::mem::transmute(ppprop)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn Property<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<ISharedProperty> {
        let mut result__: <ISharedProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<ISharedProperty>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISharedPropertyGroup {
    type Vtable = ISharedPropertyGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a005c07_a5de_11cf_9e66_00aa00a3f464);
}
impl ::core::convert::From<ISharedPropertyGroup> for ::windows::runtime::IUnknown {
    fn from(value: ISharedPropertyGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISharedPropertyGroup> for ::windows::runtime::IUnknown {
    fn from(value: &ISharedPropertyGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISharedPropertyGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISharedPropertyGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroup> for super::Com::IDispatch {
    fn from(value: ISharedPropertyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroup> for super::Com::IDispatch {
    fn from(value: &ISharedPropertyGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ISharedPropertyGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ISharedPropertyGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPropertyGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, fexists: *mut i16, ppprop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, ppproperty: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fexists: *mut i16, ppprop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppproperty: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISharedPropertyGroupManager(pub ::windows::runtime::IUnknown);
impl ISharedPropertyGroupManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn CreatePropertyGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::core::option::Option<ISharedPropertyGroup>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(dwisomode), ::core::mem::transmute(dwrelmode), ::core::mem::transmute(fexists), ::core::mem::transmute(ppgroup)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn Group<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<ISharedPropertyGroup> {
        let mut result__: <ISharedPropertyGroup as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<ISharedPropertyGroup>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISharedPropertyGroupManager {
    type Vtable = ISharedPropertyGroupManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a005c0d_a5de_11cf_9e66_00aa00a3f464);
}
impl ::core::convert::From<ISharedPropertyGroupManager> for ::windows::runtime::IUnknown {
    fn from(value: ISharedPropertyGroupManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISharedPropertyGroupManager> for ::windows::runtime::IUnknown {
    fn from(value: &ISharedPropertyGroupManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISharedPropertyGroupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISharedPropertyGroupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroupManager> for super::Com::IDispatch {
    fn from(value: ISharedPropertyGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroupManager> for super::Com::IDispatch {
    fn from(value: &ISharedPropertyGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ISharedPropertyGroupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ISharedPropertyGroupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPropertyGroupManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISystemAppEventData(pub ::windows::runtime::IUnknown);
impl ISystemAppEventData {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Startup(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn OnDataChanged<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: Param3, dwreason: u32, u64tracehandle: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpid), ::core::mem::transmute(dwmask), ::core::mem::transmute(dwnumbersinks), bstrdwmethodmask.into_param().abi(), ::core::mem::transmute(dwreason), ::core::mem::transmute(u64tracehandle)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISystemAppEventData {
    type Vtable = ISystemAppEventData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd6d48a3c_d5c5_49e7_8c74_99e4889ed52f);
}
impl ::core::convert::From<ISystemAppEventData> for ::windows::runtime::IUnknown {
    fn from(value: ISystemAppEventData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISystemAppEventData> for ::windows::runtime::IUnknown {
    fn from(value: &ISystemAppEventData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISystemAppEventData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISystemAppEventData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemAppEventData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IThreadPoolKnobs(pub ::windows::runtime::IUnknown);
impl IThreadPoolKnobs {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(plcmaxthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(plccurrentthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMaxThreads(&self, lcmaxthreads: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcmaxthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsecdeletedelay)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetDeleteDelay(&self, msecdeletedelay: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(msecdeletedelay)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(plcmaxqueuedrequests)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(plccurrentqueuedrequests)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcmaxqueuedrequests)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetMinThreads(&self, lcminthreads: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcminthreads)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetQueueDepth(&self, lcqueuedepth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcqueuedepth)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IThreadPoolKnobs {
    type Vtable = IThreadPoolKnobs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51372af7_cae7_11cf_be81_00aa00a2fa25);
}
impl ::core::convert::From<IThreadPoolKnobs> for ::windows::runtime::IUnknown {
    fn from(value: IThreadPoolKnobs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IThreadPoolKnobs> for ::windows::runtime::IUnknown {
    fn from(value: &IThreadPoolKnobs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IThreadPoolKnobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IThreadPoolKnobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolKnobs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcmaxthreads: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plccurrentthreads: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcmaxthreads: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsecdeletedelay: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msecdeletedelay: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcmaxqueuedrequests: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plccurrentqueuedrequests: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcmaxqueuedrequests: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcminthreads: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcqueuedepth: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionContext(pub ::windows::runtime::IUnknown);
impl ITransactionContext {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pszprogid: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszprogid.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionContext {
    type Vtable = ITransactionContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7999fc21_d3c6_11cf_acab_00a024a55aef);
}
impl ::core::convert::From<ITransactionContext> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionContext> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITransactionContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITransactionContext> for super::Com::IDispatch {
    fn from(value: ITransactionContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITransactionContext> for super::Com::IDispatch {
    fn from(value: &ITransactionContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ITransactionContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ITransactionContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionContextEx(pub ::windows::runtime::IUnknown);
impl ITransactionContextEx {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn CreateInstance(&self, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(pobject)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionContextEx {
    type Vtable = ITransactionContextEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7999fc22_d3c6_11cf_acab_00a024a55aef);
}
impl ::core::convert::From<ITransactionContextEx> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionContextEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionContextEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionContextEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionContextEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITransactionContextEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionContextEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionProperty(pub ::windows::runtime::IUnknown);
impl ITransactionProperty {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved1(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved2(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved3(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved4(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved5(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved6(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved7(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved8(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved9(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTransactionResourcePool(&self) -> ::windows::runtime::Result<ITransactionResourcePool> {
        let mut result__: <ITransactionResourcePool as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITransactionResourcePool>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved10(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved11(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved12(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved13(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved14(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved15(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved16(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Reserved17(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionProperty {
    type Vtable = ITransactionProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x788ea814_87b1_11d1_bba6_00c04fc2fa5f);
}
impl ::core::convert::From<ITransactionProperty> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionProperty> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITransactionProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptxpool: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionProxy(pub ::windows::runtime::IUnknown);
impl ITransactionProxy {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Commit<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), guid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_DistributedTransactionCoordinator`*"]
    pub unsafe fn Promote(&self) -> ::windows::runtime::Result<super::DistributedTransactionCoordinator::ITransaction> {
        let mut result__: <super::DistributedTransactionCoordinator::ITransaction as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::DistributedTransactionCoordinator::ITransaction>(result__)
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_DistributedTransactionCoordinator`*"]
    pub unsafe fn CreateVoter<'a, Param0: ::windows::runtime::IntoParam<'a, super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>>(&self, ptxasync: Param0) -> ::windows::runtime::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2> {
        let mut result__: <super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ptxasync.into_param().abi(), &mut result__).from_abi::<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__itransactionproxy0000)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetIdentifier(&self, pbstridentifier: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstridentifier)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsReusable(&self, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfisreusable)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionProxy {
    type Vtable = ITransactionProxy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x02558374_df2e_4dae_bd6b_1d5c994f9bdc);
}
impl ::core::convert::From<ITransactionProxy> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionProxy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionProxy> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionProxy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITransactionProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionProxy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))] usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptxasync: ::windows::runtime::RawPtr, ppballot: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, __midl__itransactionproxy0000: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstridentifier: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionResourcePool(pub ::windows::runtime::IUnknown);
impl ITransactionResourcePool {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn PutResource<'a, Param0: ::windows::runtime::IntoParam<'a, IObjPool>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, ppool: Param0, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ppool.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetResource<'a, Param0: ::windows::runtime::IntoParam<'a, IObjPool>>(&self, ppool: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ppool.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionResourcePool {
    type Vtable = ITransactionResourcePool_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc5feb7c1_346a_11d1_b1cc_00aa00ba3258);
}
impl ::core::convert::From<ITransactionResourcePool> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionResourcePool) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionResourcePool> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionResourcePool) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionResourcePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITransactionResourcePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourcePool_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppool: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppool: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionStatus(pub ::windows::runtime::IUnknown);
impl ITransactionStatus {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetTransactionStatus(&self, hrstatus: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetTransactionStatus(&self, phrstatus: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(phrstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionStatus {
    type Vtable = ITransactionStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x61f589e8_3724_4898_a0a4_664ae9e1d1b4);
}
impl ::core::convert::From<ITransactionStatus> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionStatus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionStatus> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITransactionStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrstatus: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrstatus: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITxProxyHolder(pub ::windows::runtime::IUnknown);
impl ITxProxyHolder {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn GetIdentifier(&self, pguidltx: *mut ::windows::runtime::GUID) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidltx)))
    }
}
unsafe impl ::windows::runtime::Interface for ITxProxyHolder {
    type Vtable = ITxProxyHolder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x13d86f31_0139_41af_bcad_c7d50435fe9f);
}
impl ::core::convert::From<ITxProxyHolder> for ::windows::runtime::IUnknown {
    fn from(value: ITxProxyHolder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITxProxyHolder> for ::windows::runtime::IUnknown {
    fn from(value: &ITxProxyHolder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITxProxyHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITxProxyHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITxProxyHolder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidltx: *mut ::windows::runtime::GUID),
);
pub const LBEvents: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0c1_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LockModes(pub i32);
pub const LockSetGet: LockModes = LockModes(0i32);
pub const LockMethod: LockModes = LockModes(1i32);
impl ::core::convert::From<i32> for LockModes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LockModes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[inline]
pub unsafe fn MTSCreateActivity(riid: *const ::windows::runtime::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MTSCreateActivity(riid: *const ::windows::runtime::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        MTSCreateActivity(::core::mem::transmute(riid), ::core::mem::transmute(ppobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub const MessageMover: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0bf_7f19_11d2_978e_0000f8757e2a);
pub const MtsGrp: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4b2e958d_0393_11d1_b1ab_00aa00ba3258);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ObjectContext(pub ::windows::runtime::IUnknown);
impl ObjectContext {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprogid: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrprogid.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn SetAbort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn EnableCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn DisableCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn IsSecurityEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn IsCallerInRole<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrole: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrrole.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Security(&self) -> ::windows::runtime::Result<SecurityProperty> {
        let mut result__: <SecurityProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<SecurityProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn ContextInfo(&self) -> ::windows::runtime::Result<ContextInfo> {
        let mut result__: <ContextInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ContextInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ObjectContext {
    type Vtable = ObjectContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74c08646_cedb_11cf_8b49_00aa00b8a790);
}
impl ::core::convert::From<ObjectContext> for ::windows::runtime::IUnknown {
    fn from(value: ObjectContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ObjectContext> for ::windows::runtime::IUnknown {
    fn from(value: &ObjectContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ObjectContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ObjectContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ObjectContext> for super::Com::IDispatch {
    fn from(value: ObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ObjectContext> for super::Com::IDispatch {
    fn from(value: &ObjectContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for ObjectContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &ObjectContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ObjectContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisintx: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbinrole: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurityproperty: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontextinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ObjectControl(pub ::windows::runtime::IUnknown);
impl ObjectControl {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Activate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub unsafe fn CanBePooled(&self, pbpoolable: *mut i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbpoolable)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ObjectControl {
    type Vtable = ObjectControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7dc41850_0c31_11d0_8b79_00aa00b8a790);
}
impl ::core::convert::From<ObjectControl> for ::windows::runtime::IUnknown {
    fn from(value: ObjectControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ObjectControl> for ::windows::runtime::IUnknown {
    fn from(value: &ObjectControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ObjectControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ObjectControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ObjectControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpoolable: *mut i16) -> ::windows::runtime::HRESULT,
);
pub const PoolMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabafb5_7f19_11d2_978e_0000f8757e2a);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: ::windows::runtime::GUID,
    pub ProcessStartTime: i64,
    pub dwRecycleLifetimeLimit: u32,
    pub dwRecycleMemoryLimit: u32,
    pub dwRecycleExpirationTimeout: u32,
}
impl RECYCLE_INFO {}
impl ::core::default::Default for RECYCLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RECYCLE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RECYCLE_INFO")
            .field("guidCombaseProcessIdentifier", &self.guidCombaseProcessIdentifier)
            .field("ProcessStartTime", &self.ProcessStartTime)
            .field("dwRecycleLifetimeLimit", &self.dwRecycleLifetimeLimit)
            .field("dwRecycleMemoryLimit", &self.dwRecycleMemoryLimit)
            .field("dwRecycleExpirationTimeout", &self.dwRecycleExpirationTimeout)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RECYCLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.guidCombaseProcessIdentifier == other.guidCombaseProcessIdentifier && self.ProcessStartTime == other.ProcessStartTime && self.dwRecycleLifetimeLimit == other.dwRecycleLifetimeLimit && self.dwRecycleMemoryLimit == other.dwRecycleMemoryLimit && self.dwRecycleExpirationTimeout == other.dwRecycleExpirationTimeout
    }
}
impl ::core::cmp::Eq for RECYCLE_INFO {}
unsafe impl ::windows::runtime::Abi for RECYCLE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[inline]
pub unsafe fn RecycleSurrogate(lreasoncode: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecycleSurrogate(lreasoncode: i32) -> ::windows::runtime::HRESULT;
        }
        RecycleSurrogate(::core::mem::transmute(lreasoncode)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ReleaseModes(pub i32);
pub const Standard: ReleaseModes = ReleaseModes(0i32);
pub const Process: ReleaseModes = ReleaseModes(1i32);
impl ::core::convert::From<i32> for ReleaseModes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ReleaseModes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[inline]
pub unsafe fn SafeRef<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(rid: *const ::windows::runtime::GUID, punk: Param1) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeRef(rid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SafeRef(::core::mem::transmute(rid), punk.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SecurityCallContext: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0a7_7f19_11d2_978e_0000f8757e2a);
pub const SecurityCallers: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0a6_7f19_11d2_978e_0000f8757e2a);
pub const SecurityIdentity: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0a5_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecurityProperty(pub ::windows::runtime::IUnknown);
impl SecurityProperty {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDirectCallerName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDirectCreatorName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetOriginalCallerName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_Foundation`*"]
    pub unsafe fn GetOriginalCreatorName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for SecurityProperty {
    type Vtable = SecurityProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe74a7215_014d_11d1_a63c_00a0c911b4e0);
}
impl ::core::convert::From<SecurityProperty> for ::windows::runtime::IUnknown {
    fn from(value: SecurityProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecurityProperty> for ::windows::runtime::IUnknown {
    fn from(value: &SecurityProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecurityProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SecurityProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<SecurityProperty> for super::Com::IDispatch {
    fn from(value: SecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&SecurityProperty> for super::Com::IDispatch {
    fn from(value: &SecurityProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for SecurityProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &SecurityProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct SecurityProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const ServicePool: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0c9_7f19_11d2_978e_0000f8757e2a);
pub const ServicePoolConfig: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabb0ca_7f19_11d2_978e_0000f8757e2a);
pub const SharedProperty: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a005c05_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroup: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a005c0b_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroupManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a005c11_a5de_11cf_9e66_00aa00a3f464);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRACKING_COLL_TYPE(pub i32);
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(0i32);
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(1i32);
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(2i32);
impl ::core::convert::From<i32> for TRACKING_COLL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRACKING_COLL_TYPE {
    type Abi = Self;
}
pub const TrackerServer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecabafb9_7f19_11d2_978e_0000f8757e2a);
pub const TransactionContext: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7999fc25_d3c6_11cf_acab_00a024a55aef);
pub const TransactionContextEx: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5cb66670_d3d4_11cf_acab_00a024a55aef);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TransactionVote(pub i32);
pub const TxCommit: TransactionVote = TransactionVote(0i32);
pub const TxAbort: TransactionVote = TransactionVote(1i32);
impl ::core::convert::From<i32> for TransactionVote {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TransactionVote {
    type Abi = Self;
}
