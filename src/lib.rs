#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod hash_ioc;
pub mod network_ioc;
pub mod file_ioc;

mod regex_builder;

use crate::network_ioc::NetworkIOCS;
use crate::network_ioc::parse_network_iocs;
use crate::hash_ioc::HashIOCS;
use crate::hash_ioc::parse_hash_iocs;
use crate::file_ioc::parse_file_iocs;
use crate::file_ioc::FileIOCS;


#[derive(Debug, PartialEq, Eq)]
pub struct IOCS<'a> {
    network_iocs : NetworkIOCS<'a>,
    hash_iocs : HashIOCS<'a>,
    file_iocs : FileIOCS<'a>,
}

pub fn parse_all_iocs(input: &str) -> IOCS {
    return IOCS {
        network_iocs : parse_network_iocs(input),
        hash_iocs : parse_hash_iocs(input),
        file_iocs : parse_file_iocs(input),
    }
}