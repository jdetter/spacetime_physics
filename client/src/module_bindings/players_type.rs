// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit ).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::quat_type::Quat;
use super::vec_3_type::Vec3;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct Players {
    pub id: __sdk::Identity,
    pub position: Vec3,
    pub rotation: Quat,
    pub rigid_body_id: u64,
}

impl __sdk::InModule for Players {
    type Module = super::RemoteModule;
}
