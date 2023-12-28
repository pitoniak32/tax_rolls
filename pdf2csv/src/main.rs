use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parcel {
    roll_year: String,
    county_name: String,
    primary_owner_first_name: Option<String>,
    primary_owner_last_name: String,
    municipality_name: String,
    print_key_code: String,
    deed_book: Option<String>,
    page: Option<String>,

    #[serde(flatten)]
    parcel_address: ParcelAddress,

    #[serde(flatten)]
    mailing_address: MailingAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParcelAddress {
    #[serde(rename(deserialize = "mailing_address_city"))]
    number: Option<String>,
    #[serde(rename(deserialize = "mailing_address_city"))]
    street: Option<String>,
    #[serde(rename(deserialize = "mailing_address_city"))]
    suff: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailingAddress {
    #[serde(rename(deserialize = "mailing_address_city"))]
    city: Option<String>,
    #[serde(rename(deserialize = "mailing_address_state"))]
    state: Option<String>,
    #[serde(rename(deserialize = "mailing_address_street"))]
    street: Option<String>,
    #[serde(rename(deserialize = "mailing_address_number"))]
    number: Option<String>,
    #[serde(rename(deserialize = "mailing_address_suff"))]
    suff: Option<String>,
    #[serde(rename(deserialize = "mailing_address_zip"))]
    zip: Option<String>,
    #[serde(rename(deserialize = "mailing_address_country"))]
    country: Option<String>,
    #[serde(rename(deserialize = "mailing_address_po_box"))]
    po_box: Option<String>,
}

fn main() {
    let parcels: Vec<Parcel> = serde_json::from_str(
        &fs::read_to_string("/tmp/vienna_23.json").expect("Should have been able to read the file"),
    )
    .expect("valid parcels");

    parcels
        .iter()
        .fold(HashMap::<String, Parcel>::new(), |mut acc, parcel| {
            if let Some(found_parcel) = acc.get(&parcel.print_key_code) {
                panic!("should not have duplicates {}", found_parcel.print_key_code);
            } else {
                acc.insert(parcel.print_key_code.clone(), parcel.clone());
            }
            acc
        });

    println!("parcels: {:#?}", parcels);
}
