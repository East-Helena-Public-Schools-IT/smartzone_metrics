use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct AP {
    #[serde(rename = "deviceName")]
    pub device_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "alerts")]
    pub alerts: u64,
    #[serde(rename = "ip")]
    pub ip: String,
    #[serde(rename = "ipv6Address")]
    pub ipv6_address: String,
    #[serde(rename = "txRx")]
    pub tx_rx: u64,
    #[serde(rename = "noise24G")]
    pub noise_24g: i64,
    #[serde(rename = "noise5G")]
    pub noise_5g: i64,
    // "noise6G": null,
    #[serde(rename = "noise6G")]
    pub noise_6g: i64,
    #[serde(rename = "airtime24G")]
    pub airtime_24g: u64,
    #[serde(rename = "airtime5G")]
    pub airtime_5g: u64,
    // "airtime6G": null,
    #[serde(rename = "airtime6G")]
    pub airtime_6g: u64,
    #[serde(rename = "latency24G")]
    pub latency24g: u64,
    // "latency50G": 0,
    #[serde(rename = "latency50G")]
    pub latency50G: u64,
    // "latency6G": null,
    #[serde(rename = "latency6G")]
    pub latency6G: u64,
    #[serde(rename = "capacity")]
    pub capacity: u64,
    #[serde(rename = "capacity24G")]
    pub capacity24G: u64,
    #[serde(rename = "capacity50G")]
    pub capacity50G: u64,
    #[serde(rename = "capacity6G")]
    pub capacity6G: u64,
    #[serde(rename = "connectionFailure")]
    pub connection_failures: u64,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "apMac")]
    pub ap_mac: String,
    #[serde(rename = "channel24G")]
    pub channel_24g: String,
    #[serde(rename = "channel5G")]
    pub channel_5g: String,
    #[serde(rename = "channel6G")]
    pub channel_6g: String,
    #[serde(rename = "channel24gValue")]
    pub channel_24g_value: i64,
    // "channel50gValue": 165,
    #[serde(rename = "channel50gValue")]
    pub channel50gValue: String,
    // "channel6gValue": null,
    #[serde(rename = "channel6gValue")]
    pub channel6gValue: String,
    // "meshRole": "DISABLED",
    #[serde(rename = "meshRole")]
    pub meshRole: String,
    // "meshMode": "Auto",
    #[serde(rename = "meshMode")]
    pub meshMode: String,
    // "zoneName": "R510- PPE",
    #[serde(rename = "zoneName")]
    pub zoneName: String,
    // "zoneAffinityProfileName": null,
    #[serde(rename = "zoneAffinityProfileName")]
    pub zoneAffinityProfileName: String,
    // "apGroupName": "default",
    #[serde(rename = "apGroupName")]
    pub apGroupName: String,
    // "extIp": "10.24.1.202",
    #[serde(rename = "extIp")]
    pub extIp: String,
    // "extPort": "58883",
    #[serde(rename = "extPort")]
    pub extPort: String,
    // "firmwareVersion": "6.1.1.0.1322",
    #[serde(rename = "firmwareVersion")]
    pub firmwareVersion: String,
    // "serial": "121839000189",
    #[serde(rename = "serial")]
    pub serial: String,
    // "retry24G": 0,
    #[serde(rename = "retry24G")]
    pub retry24G: String,
    // "retry5G": 11448,
    #[serde(rename = "retry5G")]
    pub retry5G: String,
    // "retry6G": null,
    #[serde(rename = "retry6G")]
    pub retry6G: String,
    // "configurationStatus": "Up-to-date",
    #[serde(rename = "configurationStatus")]
    pub configurationStatus: String,
    // "lastSeen": 1733327325000,
    #[serde(rename = "lastSeen")]
    pub last_seen: String,
    // "numClients": 37,
    #[serde(rename = "numClients")]
    pub num_clients: u64,
    // "numClients24G": 0,
    #[serde(rename = "numClients24G")]
    pub num_clients_24g: u64,
    // "numClients5G": 37,
    #[serde(rename = "numClients5G")]
    pub num_clients_5g: u64,
    // "numClients6G": null,
    #[serde(rename = "numClients6G")]
    pub num_clients_6g: u64,
    // "tx": 10713562,
    #[serde(rename = "tx")]
    pub tx: u64,
    // "tx24G": 0,
    #[serde(rename = "tx24G")]
    pub tx_24g: u64,
    // "tx50G": 10713562,
    #[serde(rename = "tx50G")]
    pub tx_50g: u64,
    // "tx6G": 0,
    #[serde(rename = "tx6G")]
    pub tx_6g: u64,
    // "rx": 1135785,
    #[serde(rename = "rx")]
    pub rx: u64,
    // "rx24G": 0,
    #[serde(rename = "rx24G")]
    pub rx_24g: u64,
    // "rx50G": 1135785,
    #[serde(rename = "rx50G")]
    pub rx_50g: u64,
    // "rx6G": 0,
    #[serde(rename = "rx6G")]
    pub rx_6g: u64,
    // "txRx24G": 0,
    #[serde(rename = "txRx24G")]
    pub txRx_24g: u64,
    // "txRx50G": 11849347,
    #[serde(rename = "txRx50G")]
    pub tx_rx_50g: u64,
    // "txRx6G": 0,
    #[serde(rename = "txRx6G")]
    pub tx_rx_6g: u64,
    // "location": "",
    #[serde(rename = "location")]
    pub location: String,
    // "wlanGroup24Id": "0749ae52-d756-11ea-96ad-000000000300",
    #[serde(rename = "wlanGroup24Id")]
    pub wlanGroup24Id: String,
    // "wlanGroup50Id": "0749ae52-d756-11ea-96ad-000000000300",
    #[serde(rename = "wlanGroup50Id")]
    pub wlanGroup50Id: String,
    // "wlanGroup6gId": null,
    #[serde(rename = "wlanGroup6gId")]
    pub wlanGroup6gId: String,
    // "wlanGroup24Name": "default",
    #[serde(rename = "wlanGroup24Name")]
    pub wlanGroup24Name: String,
    // "wlanGroup50Name": "default",
    #[serde(rename = "wlanGroup50Name")]
    pub wlanGroup50Name: String,
    // "wlanGroup6gName": null,
    #[serde(rename = "wlanGroup6gName")]
    pub wlanGroup6gName: String,
    // "enabledBonjourGateway": false,
    #[serde(rename = "enabledBonjourGateway")]
    pub enabledBonjourGateway: String,
    // "controlBladeName": "evms",
    #[serde(rename = "controlBladeName")]
    pub controlBladeName: String,
    // "lbsStatus": "Disable",
    #[serde(rename = "lbsStatus")]
    pub lbsStatus: String,
    // "administrativeState": "Unlocked",
    #[serde(rename = "administrativeState")]
    pub administrativeState: String,
    // "registrationState": "Approved",
    #[serde(rename = "registrationState")]
    pub registrationState: String,
    // "provisionMethod": "Discovered",
    #[serde(rename = "provisionMethod")]
    pub provisionMethod: String,
    // "provisionStage": null,
    #[serde(rename = "provisionStage")]
    pub provisionStage: String,
    // "registrationTime": 1532716385871,
    #[serde(rename = "registrationTime")]
    pub registrationTime: String,
    // "managementVlan": 1,
    #[serde(rename = "managementVlan")]
    pub managementVlan: String,
    // "configOverride": false,
    #[serde(rename = "configOverride")]
    pub configOverride: String,
    // "indoorMapId": "402bc1c1-d50e-11eb-b401-e26e4d8d1ad8",
    #[serde(rename = "indoorMapId")]
    pub indoorMapId: String,
    // "apGroupId": "60e88ab5-0449-4531-b7d6-4aa60aaa77d2",
    #[serde(rename = "apGroupId")]
    pub apGroupId: String,
    // "indoorMapName": "Prickly Pear Elementary",
    #[serde(rename = "indoorMapName")]
    pub indoorMapName: String,
    // "indoorMapLocation": "226 E Clinton St, East Helena, MT 59635",
    #[serde(rename = "indoorMapLocation")]
    pub indoorMapLocation: String,
    // "deviceGps": "",
    #[serde(rename = "deviceGps")]
    pub deviceGps: String,
    // "connectionStatus": "Connect",
    #[serde(rename = "connectionStatus")]
    pub connectionStatus: String,
    // "zoneId": "50979e27-2203-420e-b32b-1669af6dcfd6",
    #[serde(rename = "zoneId")]
    pub zoneId: String,
    // "zoneFirmwareVersion": "6.1.1.0.1322",
    #[serde(rename = "zoneFirmwareVersion")]
    pub zoneFirmwareVersion: String,
    // "domainId": "8b2081d5-9662-40d9-a3db-2a3cf4dde3f7",
    #[serde(rename = "domainId")]
    pub domainId: String,
    // "domainName": "Administration Domain",
    #[serde(rename = "domainName")]
    pub domainName: String,
    // "partnerDomainId": "8b2081d5-9662-40d9-a3db-2a3cf4dde3f7",
    #[serde(rename = "partnerDomainId")]
    pub partnerDomainId: String,
    // "dpIp": "",
    #[serde(rename = "dpIp")]
    pub dpIp: String,
    // "controlBladeId": "2c15640e-2ed5-433a-9eb2-d24785d2a9c8",
    #[serde(rename = "controlBladeId")]
    pub controlBladeId: String,
    // "isCriticalAp": false,
    #[serde(rename = "isCriticalAp")]
    pub isCriticalAp: String,
    // "crashDump": 1,
    #[serde(rename = "crashDump")]
    pub crashDump: String,
    // "cableModemSupported": false,
    #[serde(rename = "cableModemSupported")]
    pub cableModemSupported: String,
    // "cableModemResetSupported": false,
    #[serde(rename = "cableModemResetSupported")]
    pub cableModemResetSupported: String,
    // "swapInMac": null,
    #[serde(rename = "swapInMac")]
    pub swapInMac: String,
    // "swapOutMac": null,
    #[serde(rename = "swapOutMac")]
    pub swapOutMac: String,
    #[serde(rename = "isOverallHealthStatusFlagged")]
    pub isOverallHealthStatusFlagged: bool,
    #[serde(rename = "isLatency24GFlagged")]
    pub is_latency_24g_flagged: bool,
    #[serde(rename = "isLatency50GFlagged")]
    pub is_latency_50g_flagged: bool,
    #[serde(rename = "isLatency6GFlagged")]
    pub isLatency6GFlagged: bool,
    #[serde(rename = "isCapacity24GFlagged")]
    pub isCapacity24GFlagged: bool,
    #[serde(rename = "isCapacity50GFlagged")]
    pub isCapacity50GFlagged: bool,
    #[serde(rename = "isCapacity6GFlagged")]
    pub isCapacity6GFlagged: bool,
    // "isConnectionFailure24GFlagged": false,
    #[serde(rename = "isConnectionFailure24GFlagged")]
    pub isConnectionFailure24GFlagged: String,
    // "isConnectionFailure50GFlagged": false,
    #[serde(rename = "isConnectionFailure50GFlagged")]
    pub isConnectionFailure50GFlagged: String,
    // "isConnectionFailure6GFlagged": false,
    #[serde(rename = "isConnectionFailure6GFlagged")]
    pub isConnectionFailure6GFlagged: String,
    // "isConnectionTotalCountFlagged": false,
    #[serde(rename = "isConnectionTotalCountFlagged")]
    pub isConnectionTotalCountFlagged: String,
    // "isConnectionFailureFlagged": false,
    #[serde(rename = "isConnectionFailureFlagged")]
    pub isConnectionFailureFlagged: String,
    // "isAirtimeUtilization24GFlagged": false,
    #[serde(rename = "isAirtimeUtilization24GFlagged")]
    pub isAirtimeUtilization24GFlagged: String,
    // "isAirtimeUtilization50GFlagged": false,
    #[serde(rename = "isAirtimeUtilization50GFlagged")]
    pub isAirtimeUtilization50GFlagged: String,
    // "isAirtimeUtilization6GFlagged": false,
    #[serde(rename = "isAirtimeUtilization6GFlagged")]
    pub isAirtimeUtilization6GFlagged: String,
    // "uptime": 12866158,
    #[serde(rename = "uptime")]
    pub uptime: String,
    // "eirp24G": null,
    #[serde(rename = "eirp24G")]
    pub eirp24G: String,
    // "eirp50G": 24,
    #[serde(rename = "eirp50G")]
    pub eirp50G: String,
    // "eirp6G": null,
    #[serde(rename = "eirp6G")]
    pub eirp6G: String,
    // "supportFips": null,
    #[serde(rename = "supportFips")]
    pub supportFips: String,
    // "ipsecSessionTime": 0,
    #[serde(rename = "ipsecSessionTime")]
    pub ipsecSessionTime: String,
    // "ipsecTxPkts": 0,
    #[serde(rename = "ipsecTxPkts")]
    pub ipsecTxPkts: String,
    // "ipsecRxPkts": 0,
    #[serde(rename = "ipsecRxPkts")]
    pub ipsecRxPkts: String,
    // "ipsecTxBytes": 0,
    #[serde(rename = "ipsecTxBytes")]
    pub ipsecTxBytes: String,
    // "ipsecRxBytes": 0,
    #[serde(rename = "ipsecRxBytes")]
    pub ipsecRxBytes: String,
    // "ipsecTxDropPkts": 0,
    #[serde(rename = "ipsecTxDropPkts")]
    pub ipsecTxDropPkts: String,
    // "ipsecRxDropPkts": 0,
    #[serde(rename = "ipsecRxDropPkts")]
    pub ipsecRxDropPkts: String,
    // "ipsecTxIdleTime": 0,
    #[serde(rename = "ipsecTxIdleTime")]
    pub ipsecTxIdleTime: String,
    // "ipsecRxIdleTime": 0,
    #[serde(rename = "ipsecRxIdleTime")]
    pub ipsecRxIdleTime: String,
    // "ipType": "IPV4",
    #[serde(rename = "ipType")]
    pub ipType: String,
    // "ipv6Type": "Autoconfig",
    #[serde(rename = "ipv6Type")]
    pub ipv6Type: String,
    // "packetCaptureState": "Idle",
    #[serde(rename = "packetCaptureState")]
    pub packetCaptureState: String,
    // "cellularWanInterface": null,
    #[serde(rename = "cellularWanInterface")]
    pub cellularWanInterface: String,
    // "cellularConnectionStatus": null,
    #[serde(rename = "cellularConnectionStatus")]
    pub cellularConnectionStatus: String,
    // "cellularSignalStrength": null,
    #[serde(rename = "cellularSignalStrength")]
    pub cellularSignalStrength: String,
    // "cellularIMSISIM0": null,
    #[serde(rename = "cellularIMSISIM0")]
    pub cellularIMSISIM0: String,
    // "cellularIMSISIM1": null,
    #[serde(rename = "cellularIMSISIM1")]
    pub cellularIMSISIM1: String,
    // "cellularICCIDSIM0": null,
    #[serde(rename = "cellularICCIDSIM0")]
    pub cellularICCIDSIM0: String,
    // "cellularICCIDSIM1": null,
    #[serde(rename = "cellularICCIDSIM1")]
    pub cellularICCIDSIM1: String,
    // "cellularIsSIM0Present": null,
    #[serde(rename = "cellularIsSIM0Present")]
    pub cellularIsSIM0Present: String,
    // "cellularIsSIM1Present": null,
    #[serde(rename = "cellularIsSIM1Present")]
    pub cellularIsSIM1Present: String,
    // "cellularTxBytesSIM0": null,
    #[serde(rename = "cellularTxBytesSIM0")]
    pub cellularTxBytesSIM0: String,
    // "cellularTxBytesSIM1": null,
    #[serde(rename = "cellularTxBytesSIM1")]
    pub cellularTxBytesSIM1: String,
    // "cellularRxBytesSIM0": null,
    #[serde(rename = "cellularRxBytesSIM0")]
    pub cellularRxBytesSIM0: String,
    // "cellularRxBytesSIM1": null,
    #[serde(rename = "cellularRxBytesSIM1")]
    pub cellularRxBytesSIM1: String,
    // "cellularActiveSim": null,
    #[serde(rename = "cellularActiveSim")]
    pub cellularActiveSim: String,
    // "cellularIPaddress": null,
    #[serde(rename = "cellularIPaddress")]
    pub cellularIPaddress: String,
    // "cellularSubnetMask": null,
    #[serde(rename = "cellularSubnetMask")]
    pub cellularSubnetMask: String,
    // "cellularDefaultGateway": null,
    #[serde(rename = "cellularDefaultGateway")]
    pub cellularDefaultGateway: String,
    // "cellularOperator": null,
    #[serde(rename = "cellularOperator")]
    pub cellularOperator: String,
    // "cellular3G4GChannel": null,
    #[serde(rename = "cellular3G4GChannel")]
    pub cellular3G4GChannel: String,
    // "cellularCountry": null,
    #[serde(rename = "cellularCountry")]
    pub cellularCountry: String,
    // "cellularRadioUptime": null,
    #[serde(rename = "cellularRadioUptime")]
    pub cellularRadioUptime: String,
    // "cellularGpsHistory": null,
    #[serde(rename = "cellularGpsHistory")]
    pub cellularGpsHistory: String,
    // "fipsEnabled": null,
    #[serde(rename = "fipsEnabled")]
    pub fipsEnabled: String,
    // "medianTxRadioMCSRate24G": 0,
    #[serde(rename = "medianTxRadioMCSRate24G")]
    pub medianTxRadioMCSRate24G: String,
    // "medianTxRadioMCSRate50G": 156000,
    #[serde(rename = "medianTxRadioMCSRate50G")]
    pub medianTxRadioMCSRate50G: String,
    // "medianTxRadioMCSRate6G": null,
    #[serde(rename = "medianTxRadioMCSRate6G")]
    pub medianTxRadioMCSRate6G: String,
    // "medianRxRadioMCSRate24G": 0,
    #[serde(rename = "medianRxRadioMCSRate24G")]
    pub medianRxRadioMCSRate24G: String,
    // "medianRxRadioMCSRate50G": 0,
    #[serde(rename = "medianRxRadioMCSRate50G")]
    pub medianRxRadioMCSRate50G: String,
    // "medianRxRadioMCSRate6G": null,
    #[serde(rename = "medianRxRadioMCSRate6G")]
    pub medianRxRadioMCSRate6G: String,
    // "monitoringEnabled": false,
    #[serde(rename = "monitoringEnabled")]
    pub monitoringEnabled: String,
    // "txPowerOffset24G": 0,
    #[serde(rename = "txPowerOffset24G")]
    pub txPowerOffset24G: String,
    // "txPowerOffset5G": 0,
    #[serde(rename = "txPowerOffset5G")]
    pub txPowerOffset5G: String,
    // "txPowerOffset6G": null,
    #[serde(rename = "txPowerOffset6G")]
    pub txPowerOffset6G: String,
    // "rxDesense24G": 0,
    #[serde(rename = "rxDesense24G")]
    pub rxDesense24G: String,
    // "rxDesense5G": 0,
    #[serde(rename = "rxDesense5G")]
    pub rxDesense5G: String,
    // "rxDesense6G": null,
    #[serde(rename = "rxDesense6G")]
    pub rxDesense6G: String,
    // "poePortStatus": "1000Mbps",
    #[serde(rename = "poePortStatus")]
    pub poePortStatus: String,
    // "cumulativeTx24G": 0,
    #[serde(rename = "cumulativeTx24G")]
    pub cumulativeTx24G: String,
    // "cumulativeTx5G": 979557632296,
    #[serde(rename = "cumulativeTx5G")]
    pub cumulativeTx5G: String,
    // "cumulativeTx6G": null,
    #[serde(rename = "cumulativeTx6G")]
    pub cumulativeTx6G: String,
    // "cumulativeRx24G": 0,
    #[serde(rename = "cumulativeRx24G")]
    pub cumulativeRx24G: String,
    // "cumulativeRx5G": 22239211175,
    #[serde(rename = "cumulativeRx5G")]
    pub cumulativeRx5G: String,
    // "cumulativeRx6G": null,
    #[serde(rename = "cumulativeRx6G")]
    pub cumulativeRx6G: String,
    // "cumulativeTxRx24G": 0,
    #[serde(rename = "cumulativeTxRx24G")]
    pub cumulativeTxRx24G: String,
    // "cumulativeTxRx5G": 1001796843471,
    #[serde(rename = "cumulativeTxRx5G")]
    pub cumulativeTxRx5G: String,
    // "cumulativeTxRx6G": null,
    #[serde(rename = "cumulativeTxRx6G")]
    pub cumulativeTxRx6G: String,
    // "isDual5gMode": false
    #[serde(rename = "isDual5gMode")]
    pub isDual5gMode: String,
    #[serde(rename = "indoorMapXy")]
    pub indoorMapXy: MapXY,
}

#[derive(Deserialize)]
pub struct MapXY {
    pub x: Option<f32>,
    pub y: Option<f32>,
}
