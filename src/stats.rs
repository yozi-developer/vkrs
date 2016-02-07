use std::borrow::Borrow;
use api::{Id, Collection, Bool, OwnerId};
use serde::de;
use std::fmt::Debug;
use std::str::FromStr;
use chrono::naive::date::NaiveDate;
use chrono::offset::local::Local;

#[cfg(feature = "unstable")]
include!("stats.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/stats.rs"));

request! {
    #[derive(Copy, Eq)]
    struct Get for ["stats.get"](v => 5.44) -> Collection<Period> {
        group_id: Option<Id> = () => {Option},
        app_id: Option<Id> = () => {Option},
        date_from: NaiveDate = (Local::today().naive_local()) => {},
        date_to: NaiveDate = (Local::today().succ().naive_local()) => {},
    }
}


