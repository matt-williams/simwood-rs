#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;

use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;
use std::string::ParseError;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AccountType {
    #[serde(rename = "account_type")]
    pub account_type: String,

    #[serde(rename = "number_limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_limit: Option<isize>,

    #[serde(rename = "channel_limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub channel_limit: Option<isize>,

    #[serde(rename = "min_prepay")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_prepay: Option<isize>,

}

impl AccountType {
    pub fn new(account_type: String, ) -> AccountType {
        AccountType {
            account_type: account_type,
            number_limit: None,
            channel_limit: None,
            min_prepay: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AccountTypeResponse {
    #[serde(rename = "success")]
    pub success: bool,

    #[serde(rename = "data")]
    pub data: models::AccountType,

}

impl AccountTypeResponse {
    pub fn new(success: bool, data: models::AccountType, ) -> AccountTypeResponse {
        AccountTypeResponse {
            success: success,
            data: data,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AllocatedNumber999 {
    #[serde(rename = "link")]
    pub link: String,

}

impl AllocatedNumber999 {
    pub fn new(link: String, ) -> AllocatedNumber999 {
        AllocatedNumber999 {
            link: link,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AllocatedNumberBasic {
    #[serde(rename = "link")]
    pub link: String,

    #[serde(rename = "country_code")]
    pub country_code: String,

    #[serde(rename = "number")]
    pub number: String,

    #[serde(rename = "block")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub block: Option<String>,

    #[serde(rename = "bill_class")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bill_class: Option<String>,

    #[serde(rename = "SMS")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms: Option<isize>,

}

impl AllocatedNumberBasic {
    pub fn new(link: String, country_code: String, number: String, ) -> AllocatedNumberBasic {
        AllocatedNumberBasic {
            link: link,
            country_code: country_code,
            number: number,
            block: None,
            bill_class: None,
            sms: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AllocatedNumberFax {
    #[serde(rename = "link")]
    pub link: String,

}

impl AllocatedNumberFax {
    pub fn new(link: String, ) -> AllocatedNumberFax {
        AllocatedNumberFax {
            link: link,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AllocatedNumberResponse {
    #[serde(rename = "basic")]
    pub basic: models::AllocatedNumberBasic,

    #[serde(rename = "voice")]
    pub voice: models::AllocatedNumberVoice,

    #[serde(rename = "fax")]
    pub fax: models::AllocatedNumberFax,

    #[serde(rename = "sms")]
    pub sms: models::AllocatedNumberSms,

    #[serde(rename = "999")]
    pub _999: models::AllocatedNumber999,

}

impl AllocatedNumberResponse {
    pub fn new(basic: models::AllocatedNumberBasic, voice: models::AllocatedNumberVoice, fax: models::AllocatedNumberFax, sms: models::AllocatedNumberSms, _999: models::AllocatedNumber999, ) -> AllocatedNumberResponse {
        AllocatedNumberResponse {
            basic: basic,
            voice: voice,
            fax: fax,
            sms: sms,
            _999: _999,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AllocatedNumberSms {
    #[serde(rename = "link")]
    pub link: String,

}

impl AllocatedNumberSms {
    pub fn new(link: String, ) -> AllocatedNumberSms {
        AllocatedNumberSms {
            link: link,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AllocatedNumberVoice {
    #[serde(rename = "link")]
    pub link: String,

}

impl AllocatedNumberVoice {
    pub fn new(link: String, ) -> AllocatedNumberVoice {
        AllocatedNumberVoice {
            link: link,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AllocatedNumbersResponse {
    #[serde(rename = "type")]
    pub _type: String,

    #[serde(rename = "account")]
    pub account: String,

    #[serde(rename = "format")]
    pub format: String,

    #[serde(rename = "hash")]
    pub hash: String,

    #[serde(rename = "link")]
    pub link: String,

    #[serde(rename = "include_mobile_ott")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub include_mobile_ott: Option<bool>,

}

impl AllocatedNumbersResponse {
    pub fn new(_type: String, account: String, format: String, hash: String, link: String, ) -> AllocatedNumbersResponse {
        AllocatedNumbersResponse {
            _type: _type,
            account: account,
            format: format,
            hash: hash,
            link: link,
            include_mobile_ott: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AvailableNumber {
    #[serde(rename = "country_code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub country_code: Option<String>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<String>,

    #[serde(rename = "block")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub block: Option<String>,

    #[serde(rename = "bill_class")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bill_class: Option<String>,

    #[serde(rename = "SMS")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms: Option<isize>,

    #[serde(rename = "recommended_gold_premium")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub recommended_gold_premium: Option<f64>,

    #[serde(rename = "wholesale_gold_premium")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub wholesale_gold_premium: Option<f64>,

}

impl AvailableNumber {
    pub fn new() -> AvailableNumber {
        AvailableNumber {
            country_code: None,
            number: None,
            block: None,
            bill_class: None,
            sms: None,
            recommended_gold_premium: None,
            wholesale_gold_premium: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AvailableNumbersResponse(Vec<AvailableNumber>);

impl ::std::convert::From<Vec<AvailableNumber>> for AvailableNumbersResponse {
    fn from(x: Vec<AvailableNumber>) -> Self {
        AvailableNumbersResponse(x)
    }
}

impl ::std::convert::From<AvailableNumbersResponse> for Vec<AvailableNumber> {
    fn from(x: AvailableNumbersResponse) -> Self {
        x.0
    }
}

impl ::std::iter::FromIterator<AvailableNumber> for AvailableNumbersResponse {
    fn from_iter<U: IntoIterator<Item=AvailableNumber>>(u: U) -> Self {
        AvailableNumbersResponse(Vec::<AvailableNumber>::from_iter(u))
    }
}

impl ::std::iter::IntoIterator for AvailableNumbersResponse {
    type Item = AvailableNumber;
    type IntoIter = ::std::vec::IntoIter<AvailableNumber>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a AvailableNumbersResponse {
    type Item = &'a AvailableNumber;
    type IntoIter = ::std::slice::Iter<'a, AvailableNumber>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a mut AvailableNumbersResponse {
    type Item = &'a mut AvailableNumber;
    type IntoIter = ::std::slice::IterMut<'a, AvailableNumber>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl ::std::ops::Deref for AvailableNumbersResponse {
    type Target = Vec<AvailableNumber>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for AvailableNumbersResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Errors {
    #[serde(rename = "errors")]
    pub errors: Vec<String>,

}

impl Errors {
    pub fn new(errors: Vec<String>, ) -> Errors {
        Errors {
            errors: errors,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct MyIp {
    #[serde(rename = "ip")]
    pub ip: String,

}

impl MyIp {
    pub fn new(ip: String, ) -> MyIp {
        MyIp {
            ip: ip,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NumberRange {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "prefix")]
    pub prefix: String,

    #[serde(rename = "sabc")]
    pub sabc: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "chargeband")]
    pub chargeband: String,

}

impl NumberRange {
    pub fn new(id: String, prefix: String, sabc: String, description: String, chargeband: String, ) -> NumberRange {
        NumberRange {
            id: id,
            prefix: prefix,
            sabc: sabc,
            description: description,
            chargeband: chargeband,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NumberRangesResponse {
    #[serde(rename = "success")]
    pub success: bool,

    #[serde(rename = "data")]
    pub data: Vec<models::NumberRange>,

}

impl NumberRangesResponse {
    pub fn new(success: bool, data: Vec<models::NumberRange>, ) -> NumberRangesResponse {
        NumberRangesResponse {
            success: success,
            data: data,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OutboundAclIp {
    #[serde(rename = "ip")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip: Option<String>,

    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timestamp: Option<String>,

}

impl OutboundAclIp {
    pub fn new() -> OutboundAclIp {
        OutboundAclIp {
            ip: None,
            timestamp: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OutboundAclIps(Vec<OutboundAclIp>);

impl ::std::convert::From<Vec<OutboundAclIp>> for OutboundAclIps {
    fn from(x: Vec<OutboundAclIp>) -> Self {
        OutboundAclIps(x)
    }
}

impl ::std::convert::From<OutboundAclIps> for Vec<OutboundAclIp> {
    fn from(x: OutboundAclIps) -> Self {
        x.0
    }
}

impl ::std::iter::FromIterator<OutboundAclIp> for OutboundAclIps {
    fn from_iter<U: IntoIterator<Item=OutboundAclIp>>(u: U) -> Self {
        OutboundAclIps(Vec::<OutboundAclIp>::from_iter(u))
    }
}

impl ::std::iter::IntoIterator for OutboundAclIps {
    type Item = OutboundAclIp;
    type IntoIter = ::std::vec::IntoIter<OutboundAclIp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a OutboundAclIps {
    type Item = &'a OutboundAclIp;
    type IntoIter = ::std::slice::Iter<'a, OutboundAclIp>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a mut OutboundAclIps {
    type Item = &'a mut OutboundAclIp;
    type IntoIter = ::std::slice::IterMut<'a, OutboundAclIp>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl ::std::ops::Deref for OutboundAclIps {
    type Target = Vec<OutboundAclIp>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for OutboundAclIps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OutboundTrunk {
    #[serde(rename = "trunk")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trunk: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<String>,

    #[serde(rename = "enabled_in")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled_in: Option<String>,

    #[serde(rename = "enabled_out")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled_out: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,

    #[serde(rename = "quality")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub quality: Option<String>,

    #[serde(rename = "limit_concurrent_out")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit_concurrent_out: Option<String>,

    #[serde(rename = "limit_rate_out")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit_rate_out: Option<String>,

    #[serde(rename = "limit_rate_out_international_hotspot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit_rate_out_international_hotspot: Option<String>,

    #[serde(rename = "cli_force_default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cli_force_default: Option<String>,

}

impl OutboundTrunk {
    pub fn new() -> OutboundTrunk {
        OutboundTrunk {
            trunk: None,
            _type: None,
            enabled: None,
            enabled_in: None,
            enabled_out: None,
            user: None,
            quality: None,
            limit_concurrent_out: None,
            limit_rate_out: None,
            limit_rate_out_international_hotspot: None,
            cli_force_default: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OutboundTrunkSummary {
    #[serde(rename = "trunk")]
    pub trunk: String,

    #[serde(rename = "serviceLevel")]
    pub service_level: String,

    #[serde(rename = "type")]
    pub _type: String,

}

impl OutboundTrunkSummary {
    pub fn new(trunk: String, service_level: String, _type: String, ) -> OutboundTrunkSummary {
        OutboundTrunkSummary {
            trunk: trunk,
            service_level: service_level,
            _type: _type,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OutboundTrunksResponse(Vec<OutboundTrunkSummary>);

impl ::std::convert::From<Vec<OutboundTrunkSummary>> for OutboundTrunksResponse {
    fn from(x: Vec<OutboundTrunkSummary>) -> Self {
        OutboundTrunksResponse(x)
    }
}

impl ::std::convert::From<OutboundTrunksResponse> for Vec<OutboundTrunkSummary> {
    fn from(x: OutboundTrunksResponse) -> Self {
        x.0
    }
}

impl ::std::iter::FromIterator<OutboundTrunkSummary> for OutboundTrunksResponse {
    fn from_iter<U: IntoIterator<Item=OutboundTrunkSummary>>(u: U) -> Self {
        OutboundTrunksResponse(Vec::<OutboundTrunkSummary>::from_iter(u))
    }
}

impl ::std::iter::IntoIterator for OutboundTrunksResponse {
    type Item = OutboundTrunkSummary;
    type IntoIter = ::std::vec::IntoIter<OutboundTrunkSummary>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a OutboundTrunksResponse {
    type Item = &'a OutboundTrunkSummary;
    type IntoIter = ::std::slice::Iter<'a, OutboundTrunkSummary>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a mut OutboundTrunksResponse {
    type Item = &'a mut OutboundTrunkSummary;
    type IntoIter = ::std::slice::IterMut<'a, OutboundTrunkSummary>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl ::std::ops::Deref for OutboundTrunksResponse {
    type Target = Vec<OutboundTrunkSummary>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for OutboundTrunksResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Time {
    #[serde(rename = "api")]
    pub api: String,

    #[serde(rename = "rfc")]
    pub rfc: String,

    #[serde(rename = "timestamp")]
    pub timestamp: i32,

}

impl Time {
    pub fn new(api: String, rfc: String, timestamp: i32, ) -> Time {
        Time {
            api: api,
            rfc: rfc,
            timestamp: timestamp,
        }
    }
}

