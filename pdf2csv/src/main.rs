use anyhow::Result;
use std::{collections::HashMap, fs};

use nom::{
    bytes::complete::{tag, take_while1},
    sequence::tuple,
    IResult,
};

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

fn parse_print_key_code(input: &str) -> IResult<&str, PrintKeyCode> {
    let (o, code) = tuple((
        take_while1(|c: char| c.is_ascii_digit() || c == '.'),
        tag("-"),
        take_while1(|c: char| c.is_ascii_digit() || c == '.'),
        tag("-"),
        take_while1(|c: char| c.is_ascii_digit() || c == '.'),
    ))(input)?;

    let code = PrintKeyCode::All {
        section: code.0.to_string(),
        block: code.2.to_string(),
        lot: code.4.to_string(),
    };

    Ok((o, code))
}

fn parse_parcels(input: &str) -> Result<HashMap<PrintKeyCode, String>> {
    let first_tag = "*".repeat(103);

    let mut parcels = HashMap::<PrintKeyCode, String>::new();
    let mut lines = input.lines().peekable();
    let mut value: Vec<&str> = vec![];

    while let Some(current_line) = lines.next() {
        if current_line.contains(&first_tag) {
            let replaced = &current_line.replace('*', "");
            if !replaced.is_empty() {
                let (_, sbl) = parse_print_key_code(dbg!(replaced.trim())).unwrap();
                let mut next = lines.next_if(|l| !l.contains(&first_tag));
                while next.is_some() {
                    value.push(next.unwrap());
                    next = lines.next_if(|l| !l.contains(&first_tag));
                }
                parcels.insert(sbl, value.join("\n"));
                value = vec![];
            }
        }
    }

    println!("{:#?}", parcels);

    Ok(parcels)
}

fn main() {
    let parcels = fs::read_to_string("./pdf2csv/txts/vienna_23.txt").unwrap();
    parse_parcels(&parcels).unwrap();
    // let search = vec!["234.008-1-70", "236.015-1-1"];
    // let parcels: Vec<Parcel> = serde_json::from_str(
    //     &fs::read_to_string("./vienna_23.json").expect("Should have been able to read the file"),
    // )
    // .expect("valid parcels");
    //
    // let parcel_map = parcels
    //     .iter()
    //     .fold(HashMap::<String, Parcel>::new(), |mut acc, parcel| {
    //         if let Some(found_parcel) = acc.get(&parcel.print_key_code) {
    //             panic!("should not have duplicates {}", found_parcel.print_key_code);
    //         } else {
    //             acc.insert(parcel.print_key_code.clone(), parcel.clone());
    //         }
    //         acc
    //     });
    //
    // let found = search
    //     .iter()
    //     .filter_map(|s| parcel_map.get(&s.to_string()))
    //     .map(|p| p.clone())
    //     .collect::<Vec<Parcel>>();
    //
    // println!("parcels: {:#?}", found);
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PrintKeyCode {
    All {
        section: String,
        block: String,
        lot: String,
    },
}

// given a text file. Get individual sections separated by
// ******************************************************************************************************* 236.011-1-1 ****************
// So the text between these is the value of the object. And use this to build out the parcels.
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("123.456-1-2", PrintKeyCode::All{section: "123.456".to_string(), block: "1".to_string(), lot: "2".to_string()})]
    #[case("123.456-1-2", PrintKeyCode::All{section: "123.456".to_string(), block: "1".to_string(), lot: "2".to_string()})]
    #[case("186.000-0001-001", PrintKeyCode::All{section: "186.000".to_string(), block: "0001".to_string(), lot: "001".to_string()})]
    fn test_parse_print_key(#[case] input: &str, #[case] expected: PrintKeyCode) -> Result<()> {
        assert_eq!(parse_print_key_code(input).unwrap().1, expected);

        Ok(())
    }
}
