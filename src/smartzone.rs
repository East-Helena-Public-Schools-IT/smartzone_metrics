use reqwest::{
    header::{HeaderName, HeaderValue},
    Client, StatusCode,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::LazyLock;

use crate::ap;

static URL: LazyLock<String> =
    LazyLock::new(|| dotenvy::var("RUST_URL").expect("Set RUST_URL env"));

#[derive(Serialize, Debug)]
pub struct Auth {
    username: String,
    password: String,
    #[serde(rename = "timeZoneUtcOffset")]
    time_zone_utc_offset: String,
    #[serde(skip_serializing)]
    client: reqwest::Client,
    #[serde(skip_serializing)]
    session: Option<HeaderValue>,
}

impl Auth {
    pub async fn new(username: String, password: String) -> Self {
        let auth = Self {
            username,
            password,
            time_zone_utc_offset: "-07:00".to_string(),
            client: Client::builder()
                .cookie_store(true)
                .danger_accept_invalid_certs(true)
                .build()
                .expect("Building reqwest client failed."),
            session: None,
        };

        #[derive(Deserialize)]
        struct ApiInfo {
            #[serde(rename = "apiSupportVersions")]
            api_support_versions: Vec<String>
        }

        let res = auth.client.get(format!("{}/wsg/api/public/apiInfo", &*URL)).send().await.expect("Failed to query apiinfo");
        let body = res.text().await.expect("Failed to get apiinfo response");
        let info = serde_json::from_str::<ApiInfo>(&body).expect("Failed to parse apiinfo response");

        let mut api_supported= false;
        for v in info.api_support_versions {
            api_supported = v == "v11_1" || api_supported;
        }
        assert!(api_supported, "Api version not supported by smartzone client {}", &*URL);

        auth
    }

    pub async fn login(&mut self) {
        let login = self
            .client
            .post(format!("{}/wsg/api/public/v11_1/session", &*URL))
            .body(serde_json::to_string(&self).unwrap())
            .send()
            .await
            .unwrap();

        if login.status() != reqwest::StatusCode::OK {
            panic!("Failed to login");
        };

        let (_, session) = login
            .headers()
            .iter()
            .filter(|(name, _)| name.as_str() == "set-cookie")
            .collect::<Vec<(&HeaderName, &HeaderValue)>>()[0];
        self.session = Some(session.clone());
    }

    pub async fn logout(&self) {
        if let Some(s) = &self.session {
            let res = self
                .client
                .delete(format!("{}/wsg/api/public/v11_1/session", &*URL))
                .header("Cookie", s)
                .send()
                .await
                .unwrap();
            if res.status() == StatusCode::OK {
                println!("Logged out");
            }
        }
    }

    pub async fn get_zones(&self) -> Vec<Zone> {
        #[derive(Deserialize)]
        struct Zones {
            list: Vec<Zone>,
        }

        if let Some(s) = &self.session {
            let response = self
                .client
                .get(format!("{}/wsg/api/public/v11_1/rkszones", &*URL))
                .header("Cookie", s)
                .send()
                .await
                .expect("System went offline");
            if let Ok(json) = response.text().await {
                return serde_json::from_str::<Zones>(&json).unwrap().list;
            }
        }
        panic!("Failed to get zones")
    }

    pub async fn get_aps_in_zone(&self, zone: &Zone) -> Vec<MiniAp> {
        #[derive(Deserialize)]
        struct ZoneAPs {
            members: Vec<MiniAp>,
        }

        if let Some(s) = &self.session {
            let response = self
                .client
                .get(format!(
                    "{}/wsg/api/public/v11_1/rkszones/{}/apgroups/default",
                    &*URL, zone.id
                ))
                .header("Cookie", s)
                .send()
                .await
                .unwrap();
            if let Ok(json) = response.text().await {
                return serde_json::from_str::<ZoneAPs>(&json).unwrap().members;
            }
        }
        panic!("Failed to get aps in zones")
    }

    pub async fn query_aps(&self, filter: FilterContainer) -> QueryResults<ap::AP> {
        if let Some(s) = &self.session {
            let response = self
                .client
                .post(format!(
                    "{}/wsg/api/public/v11_1/query/ap",
                    &*URL
                ))
                .header("Cookie", s)
                .body(serde_json::to_string(&filter).unwrap())
                .send()
                .await
                .unwrap();
            if let Ok(json) = response.text().await {
                return serde_json::from_str::<QueryResults<ap::AP>>(&json).unwrap();
            }
        }
        panic!("Failed to query aps")
    }
}

#[derive(Deserialize, Debug)]
pub struct Zone {
    id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MiniAp {
    #[serde(rename = "apMac")]
    ap_mac: String,
    #[serde(rename = "apSerial")]
    ap_serial: String,
}

#[derive(Serialize, Debug)]
pub struct FilterContainer {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "extraFilters")]
    and: Vec<Filter>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "filters")]
    or: Vec<Filter>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "extraNotFilters")]
    nand: Vec<Filter>
}

impl FromIterator<Filter> for FilterContainer {
    fn from_iter<T: IntoIterator<Item = Filter>>(iter: T) -> Self {
        Self { and: Vec::new(), nand: Vec::new(),
            or: iter.into_iter().collect()
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Filter {
    #[serde(rename = "type")]
    ttype: String,
    value: String,
    operator: String
}

impl From<&MiniAp> for Filter {
    fn from(value: &MiniAp) -> Self {
        Self {
            ttype: "AP".to_string(),
            value: value.ap_mac.clone(),
            operator: "eq".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct QueryResults<T> {
    #[serde(rename = "totalCount")]
    total_count: u8,
    #[serde(rename = "hasMore")]
    has_more: bool,
    #[serde(rename = "firstIndex")]
    first_index: u8,
    pub list: Vec<T>
}

#[derive(Deserialize, Debug)]
pub struct FullAp {
    #[serde(rename = "deviceName")]
    pub device_name: String,
    pub description: String,
    pub status: String,
    pub model: String,
    #[serde(deserialize_with="nullable_i64")]
    pub alerts: i64, 
    pub ip: String,
    #[serde(rename = "ipv6Address")]
    pub ipv4_address: String,

    pub serial: String,
    #[serde(rename = "apMac")]
    pub ap_mac: String,
    #[serde(rename = "lastSeen")]
    #[serde(deserialize_with="nullable_u64")]
    pub last_seen: u64,

    // ======================================
    // RX / TX
    // ======================================
    #[serde(deserialize_with="nullable_u64")]
    pub tx: u64,
    #[serde(rename = "tx24G")]
    #[serde(deserialize_with="nullable_u64")]
    pub tx_24g: u64,
    #[serde(rename = "tx50G")]
    #[serde(deserialize_with="nullable_u64")]
    pub tx_5g: u64,
    #[serde(rename = "tx6G")]
    #[serde(deserialize_with="nullable_u64")]
    pub tx_6g: u64,
    #[serde(deserialize_with="nullable_u64")]
    pub rx: u64,
    #[serde(rename = "rx24G")]
    #[serde(deserialize_with="nullable_u64")]
    pub rx_24g: u64,
    #[serde(rename = "rx50G")]
    #[serde(deserialize_with="nullable_u64")]
    pub rx_5g: u64,
    #[serde(rename = "rx6G")]
    #[serde(deserialize_with="nullable_u64")]
    pub rx_6g: u64,

    #[serde(rename = "channel24G")]
    pub channel_24g: String,
    #[serde(rename = "channel5G")]
    pub channel_5g: String,
    #[serde(rename = "channel6G")]
    pub channel_6g: String,


    // ======================================
    // Clients
    // ======================================   
    #[serde(rename = "numClients")]
    #[serde(deserialize_with="nullable_u64")]
    pub num_clients: u64,
    #[serde(rename = "numClients5G")]
    #[serde(deserialize_with="nullable_u64")]
    pub num_clients_5g: u64,
    #[serde(rename = "numClients24G")]
    #[serde(deserialize_with="nullable_u64")]
    pub num_clients_24g: u64,
    #[serde(rename = "numClients6G")]
    #[serde(deserialize_with="nullable_u64")]
    pub num_clients_6g: u64,
    #[serde(rename = "isCapacity24GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_capacity24_gflagged: bool,
    #[serde(rename = "isCapacity50GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_capacity50_gflagged: bool,
    #[serde(rename = "isCapacity6GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_capacity6_gflagged: bool,
 

    #[serde(rename = "isOverallHealthStatusFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_overall_health_status_flagged: bool,

    // ======================================
    // Latency
    // ======================================   
    #[serde(rename = "latency24G")]
    #[serde(deserialize_with="nullable_f32")]
    pub latency24_g: f32,
    #[serde(rename = "latency50G")]
    #[serde(deserialize_with="nullable_f32")]
    pub latency50_g: f32,
    #[serde(rename = "latency6G")]
    #[serde(deserialize_with="nullable_f32")]
    pub latency6_g: f32,
    #[serde(rename = "isLatency24GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_latency24_gflagged: bool,
    #[serde(rename = "isLatency50GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_latency50_gflagged: bool,
    #[serde(rename = "isLatency6GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_latency6_gflagged: bool,

    // ======================================
    // Connection Failures
    // ======================================   
    #[serde(rename = "connectionFailure")]
    #[serde(deserialize_with="nullable_f32")]
    pub connection_failures: f32,
    #[serde(rename = "isConnectionFailure24GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_connection_failure24_gflagged: bool,
    #[serde(rename = "isConnectionFailure50GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_connection_failure50_gflagged: bool,
    #[serde(rename = "isConnectionFailure6GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_connection_failure6_gflagged: bool,
    #[serde(rename = "isConnectionTotalCountFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_connection_total_count_flagged: bool,
    #[serde(rename = "isConnectionFailureFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_connection_failure_flagged: bool,

    // ======================================
    // Airtime Utilization
    // ======================================   
    #[serde(rename = "airtime24G")]
    #[serde(deserialize_with="nullable_u64")]
    pub airtime_24g: u64,
    #[serde(rename = "airtime5G")]
    #[serde(deserialize_with="nullable_u64")]
    pub airtime_5g: u64,
    #[serde(rename = "airtime6G")]
    #[serde(deserialize_with="nullable_u64")]
    pub airtime_6g: u64,
    #[serde(rename = "isAirtimeUtilization24GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_airtime_utilization24_gflagged: bool,
    #[serde(rename = "isAirtimeUtilization50GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_airtime_utilization50_gflagged: bool,
    #[serde(rename = "isAirtimeUtilization6GFlagged")]
    #[serde(deserialize_with="nullable_bool")]
    pub is_airtime_utilization6_gflagged: bool,
    }

fn nullable_u64<'de, D>(d: D) -> Result<u64, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or(0u64)
        })
}

fn nullable_i64<'de, D>(d: D) -> Result<i64, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or(0i64)
        })
}

fn nullable_f32<'de, D>(d: D) -> Result<f32, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or(0f32)
        })
}

fn nullable_bool<'de, D>(d: D) -> Result<bool, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or(false)
        })
}
