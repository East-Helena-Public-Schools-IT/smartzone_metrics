use reqwest::{
    header::{HeaderName, HeaderValue},
    Client, StatusCode,
};
use serde::{Deserialize, Deserializer, Serialize};
use core::hash;
use std::{fs::File, hash::{DefaultHasher, Hash, Hasher}, io::Write, sync::LazyLock};

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
                match serde_json::from_str::<QueryResults<ap::AP>>(&json) {
                    Ok(res) => return res,
                    Err(err) => {
                        // write the error out so it can be debugged
                        println!("{}", err);
                        let mut file = File::create("error.json").unwrap();
                        file.write_all(&json.as_bytes()).unwrap();
                        let mut file = File::create("error_column").unwrap();
                        file.write_all(err.column().to_string().as_bytes()).unwrap();
                    },
                }
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

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}