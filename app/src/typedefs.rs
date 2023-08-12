use serde::{Deserialize, Serialize};
use serde_with::{de, skip_serializing_none};
use std::{collections::HashMap, fmt, ops::Add, sync};

#[derive(Serialize, Deserialize, Clone)]
pub struct GenericResponse {
    pub code: i32,
    pub data: serde_json::Value, // type - any!
}
// GenericResponse stringer
impl fmt::Display for GenericResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "generic-response: code: {}, data: {}",
            self.code, self.data
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ThisEM {
    pub host_id: u32,
    pub hosts: HashMap<u32, String>,
}
//
// ThisEM stringer
impl fmt::Display for ThisEM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = String::new();
        for (k, v) in self.hosts.iter() {
            let s0 = format!("{}:{}\n", k, v);
            s = s.add(&s0.as_str());
        }
        write!(f, "this-EM: id {}, hosts: {}", self.host_id, s)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    #[serde(skip)]
    pub app_name: String, // we want serde to not fill this, we fill instead
    #[serde(skip)]
    pub app_id: u32, // we want serde to not fill this, we fill instead

    // depart if true is telling of graceful stop of app
    // other ways to invent it should be possible within Zero
    pub depart: bool,
    pub app_type: String,

    // app advertize a uri:port to let Zero know where to connect back
    pub app_uri: String,
    pub app_port: u32,

    // app can let the Zero know of all apps that it has dependencies on
    pub app_dependencies: Vec<String>, // if optional Option<Vec<String>>

    // app can have dependencies based on type, and on name of apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_dependencies: Option<Vec<String>>, // if !optional Vec<String>

    // app can rightaway also send data to its peer and dependencies
    // note: this is only used if depart=false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_data: Option<HashMap<String, serde_json::Value>>,
}

// RegisterRequest stringer
impl fmt::Display for RegisterRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = &self.app_data {
            for (key, val) in v.into_iter() {
                let _ = write!(f, "{}: {}\n", key, val);
            }
        }
        if let Some(tdep) = &self.type_dependencies {
            let _ = write!(f, "type depenedencies: {}", tdep.join(""));
        }
        write!(
            f,
            r#"RegisterRequest:
                (app_name: "{}",
                 app_id: "{}",
                 is_depart: "{}",
                 app_type: "{}",
                 app_dependencies: "[{}]",
                 )"#,
            self.app_name,
            self.app_id,
            self.depart,
            self.app_type,
            self.app_dependencies.join(", "),
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InfoToOtherMS {
    #[serde(skip)]
    pub app_name: String, // we want serde to not fill this, we fill instead
    #[serde(skip)]
    pub app_id: u32, // we want serde to not fill this, we fill instead

    pub app_data: HashMap<String, serde_json::Value>,
}

impl fmt::Display for InfoToOtherMS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (k, v) in self.app_data.iter() {
            s = s.add(format!("{}:{}", k, v).as_str());
        }
        write!(f, "{}, {}, {}", self.app_name, self.app_id, s)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterResponse {
    pub dependency_info: HashMap<String, serde_json::Value>,
    pub peer_info: HashMap<String, serde_json::Value>,
}

impl fmt::Display for RegisterResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dep_info = String::new();
        let mut peer_info = String::new();
        for (k, v) in self.dependency_info.iter() {
            dep_info = dep_info.add(format!("{}:{}", k, v).as_str());
        }
        for (k, v) in self.peer_info.iter() {
            peer_info = peer_info.add(format!("{}:{}", k, v).as_str());
        }
        write!(f, "{}, {}", dep_info, peer_info)
    }
}

#[derive(Serialize, Deserialize)]
pub struct InMemModDB {
    pub regitered_modules: sync::Mutex<Vec<RegisterRequest>>,
}
impl fmt::Display for InMemModDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let locked_inner_object = self.regitered_modules.lock().unwrap();
        let len = locked_inner_object.len();
        let s = format!("inner vector<RegisterRequest> is of size={}", len);
        write!(f, "{}", s)
        // lock will be released here
    }
}

// good q/a:
// https://stackoverflow.com/questions/75121484/shared-state-doesnt-work-because-of-lifetimes
