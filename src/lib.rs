#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate url;

// Crates for conversion support
#[cfg(feature = "conversion")]
#[macro_use]
extern crate frunk_derives;
#[cfg(feature = "conversion")]
#[macro_use]
extern crate frunk_enum_derive;
#[cfg(feature = "conversion")]
extern crate frunk_core;

extern crate mime;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate chrono;
extern crate swagger;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

#[deprecated(note = "Import swagger-rs directly")]
pub use swagger::{ApiError, ContextWrapper};
#[deprecated(note = "Import futures directly")]
pub use futures::Future;

pub const BASE_PATH: &'static str = "/v3";
pub const API_VERSION: &'static str = "1.0.0";


#[derive(Debug, PartialEq)]
pub enum GetAccountTypeResponse {
    /// Success
    Success
    (models::AccountTypeResponse)
}

#[derive(Debug, PartialEq)]
pub enum DeleteAllocatedNumberResponse {
    /// Success
    Success
    ,
    /// Number not allocated
    NumberNotAllocated
}

#[derive(Debug, PartialEq)]
pub enum GetAllocatedNumberResponse {
    /// Success
    Success
    (models::AllocatedNumberResponse)
    ,
    /// Number not allocated
    NumberNotAllocated
}

#[derive(Debug, PartialEq)]
pub enum GetAllocatedNumbersResponse {
    /// Success
    Success
    (models::AllocatedNumbersResponse)
}

#[derive(Debug, PartialEq)]
pub enum GetAvailableNumbersResponse {
    /// Success
    Success
    (models::AvailableNumbersResponse)
}

#[derive(Debug, PartialEq)]
pub enum GetNumberRangesResponse {
    /// Success
    Success
    (models::NumberRangesResponse)
}

#[derive(Debug, PartialEq)]
pub enum PutAllocatedNumberResponse {
    /// Success
    Success
    ,
    /// Number not available
    NumberNotAvailable
}

#[derive(Debug, PartialEq)]
pub enum DeleteOutboundAclIpResponse {
    /// Success
    Success
}

#[derive(Debug, PartialEq)]
pub enum DeleteOutboundTrunkResponse {
    /// Success
    Success
}

#[derive(Debug, PartialEq)]
pub enum GetOutboundAclIpsResponse {
    /// Success
    Success
    (models::OutboundAclIps)
}

#[derive(Debug, PartialEq)]
pub enum GetOutboundTrunkResponse {
    /// Success
    Success
    (models::OutboundTrunk)
}

#[derive(Debug, PartialEq)]
pub enum GetOutboundTrunksResponse {
    /// Success
    Success
    (models::OutboundTrunksResponse)
}

#[derive(Debug, PartialEq)]
pub enum PutOutboundAclIpResponse {
    /// Success
    Success
}

#[derive(Debug, PartialEq)]
pub enum PutOutboundTrunkResponse {
    /// Success
    Success
    (models::OutboundTrunk)
    ,
    /// Failure
    Failure
    (models::Errors)
}

#[derive(Debug, PartialEq)]
pub enum GetMyIpResponse {
    /// Success
    Success
    (models::MyIp)
}

#[derive(Debug, PartialEq)]
pub enum GetTimeResponse {
    /// Success
    Success
    (models::Time)
}


/// API
pub trait Api<C> {

    /// Get your current account type, and limitations
    fn get_account_type(&self, account: String, context: &C) -> Box<Future<Item=GetAccountTypeResponse, Error=ApiError>>;

    /// De-configure and irrevocably remove number from account
    fn delete_allocated_number(&self, account: String, number: String, context: &C) -> Box<Future<Item=DeleteAllocatedNumberResponse, Error=ApiError>>;

    /// Return configuration information on allocated number
    fn get_allocated_number(&self, account: String, number: String, context: &C) -> Box<Future<Item=GetAllocatedNumberResponse, Error=ApiError>>;

    /// Request a report of all current allocated numbers on account
    fn get_allocated_numbers(&self, account: String, context: &C) -> Box<Future<Item=GetAllocatedNumbersResponse, Error=ApiError>>;

    /// Returns 1,10 or 100 numbers available for allocation matching the pattern specified
    fn get_available_numbers(&self, account: String, tier: String, number: i32, pattern: Option<String>, context: &C) -> Box<Future<Item=GetAvailableNumbersResponse, Error=ApiError>>;

    /// Retrieves a list of all available number ranges, including descriptions
    fn get_number_ranges(&self, account: String, context: &C) -> Box<Future<Item=GetNumberRangesResponse, Error=ApiError>>;

    /// Allocate an available number to the account
    fn put_allocated_number(&self, account: String, number: String, context: &C) -> Box<Future<Item=PutAllocatedNumberResponse, Error=ApiError>>;

    /// Remove IP from ACL-based trunk
    fn delete_outbound_acl_ip(&self, account: String, trunk: String, ip: String, context: &C) -> Box<Future<Item=DeleteOutboundAclIpResponse, Error=ApiError>>;

    /// Delete the trunk
    fn delete_outbound_trunk(&self, account: String, trunk: String, context: &C) -> Box<Future<Item=DeleteOutboundTrunkResponse, Error=ApiError>>;

    /// Request information of specified trunk
    fn get_outbound_acl_ips(&self, account: String, trunk: String, context: &C) -> Box<Future<Item=GetOutboundAclIpsResponse, Error=ApiError>>;

    /// Request information of specified trunk
    fn get_outbound_trunk(&self, account: String, trunk: String, context: &C) -> Box<Future<Item=GetOutboundTrunkResponse, Error=ApiError>>;

    /// List all active outbound trunks
    fn get_outbound_trunks(&self, account: String, context: &C) -> Box<Future<Item=GetOutboundTrunksResponse, Error=ApiError>>;

    /// Add IP to ACL-based trunk
    fn put_outbound_acl_ip(&self, account: String, trunk: String, ip: String, context: &C) -> Box<Future<Item=PutOutboundAclIpResponse, Error=ApiError>>;

    /// Create new trunk or update existing trunk
    fn put_outbound_trunk(&self, account: String, trunk: String, outbound_trunk: Option<models::OutboundTrunk>, context: &C) -> Box<Future<Item=PutOutboundTrunkResponse, Error=ApiError>>;

    /// Return your external IP address, as seen by the Simwood API
    fn get_my_ip(&self, context: &C) -> Box<Future<Item=GetMyIpResponse, Error=ApiError>>;

    /// Returns the current timestamp
    fn get_time(&self, context: &C) -> Box<Future<Item=GetTimeResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {

    /// Get your current account type, and limitations
    fn get_account_type(&self, account: String) -> Box<Future<Item=GetAccountTypeResponse, Error=ApiError>>;

    /// De-configure and irrevocably remove number from account
    fn delete_allocated_number(&self, account: String, number: String) -> Box<Future<Item=DeleteAllocatedNumberResponse, Error=ApiError>>;

    /// Return configuration information on allocated number
    fn get_allocated_number(&self, account: String, number: String) -> Box<Future<Item=GetAllocatedNumberResponse, Error=ApiError>>;

    /// Request a report of all current allocated numbers on account
    fn get_allocated_numbers(&self, account: String) -> Box<Future<Item=GetAllocatedNumbersResponse, Error=ApiError>>;

    /// Returns 1,10 or 100 numbers available for allocation matching the pattern specified
    fn get_available_numbers(&self, account: String, tier: String, number: i32, pattern: Option<String>) -> Box<Future<Item=GetAvailableNumbersResponse, Error=ApiError>>;

    /// Retrieves a list of all available number ranges, including descriptions
    fn get_number_ranges(&self, account: String) -> Box<Future<Item=GetNumberRangesResponse, Error=ApiError>>;

    /// Allocate an available number to the account
    fn put_allocated_number(&self, account: String, number: String) -> Box<Future<Item=PutAllocatedNumberResponse, Error=ApiError>>;

    /// Remove IP from ACL-based trunk
    fn delete_outbound_acl_ip(&self, account: String, trunk: String, ip: String) -> Box<Future<Item=DeleteOutboundAclIpResponse, Error=ApiError>>;

    /// Delete the trunk
    fn delete_outbound_trunk(&self, account: String, trunk: String) -> Box<Future<Item=DeleteOutboundTrunkResponse, Error=ApiError>>;

    /// Request information of specified trunk
    fn get_outbound_acl_ips(&self, account: String, trunk: String) -> Box<Future<Item=GetOutboundAclIpsResponse, Error=ApiError>>;

    /// Request information of specified trunk
    fn get_outbound_trunk(&self, account: String, trunk: String) -> Box<Future<Item=GetOutboundTrunkResponse, Error=ApiError>>;

    /// List all active outbound trunks
    fn get_outbound_trunks(&self, account: String) -> Box<Future<Item=GetOutboundTrunksResponse, Error=ApiError>>;

    /// Add IP to ACL-based trunk
    fn put_outbound_acl_ip(&self, account: String, trunk: String, ip: String) -> Box<Future<Item=PutOutboundAclIpResponse, Error=ApiError>>;

    /// Create new trunk or update existing trunk
    fn put_outbound_trunk(&self, account: String, trunk: String, outbound_trunk: Option<models::OutboundTrunk>) -> Box<Future<Item=PutOutboundTrunkResponse, Error=ApiError>>;

    /// Return your external IP address, as seen by the Simwood API
    fn get_my_ip(&self) -> Box<Future<Item=GetMyIpResponse, Error=ApiError>>;

    /// Returns the current timestamp
    fn get_time(&self) -> Box<Future<Item=GetTimeResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {

    /// Get your current account type, and limitations
    fn get_account_type(&self, account: String) -> Box<Future<Item=GetAccountTypeResponse, Error=ApiError>> {
        self.api().get_account_type(account, &self.context())
    }

    /// De-configure and irrevocably remove number from account
    fn delete_allocated_number(&self, account: String, number: String) -> Box<Future<Item=DeleteAllocatedNumberResponse, Error=ApiError>> {
        self.api().delete_allocated_number(account, number, &self.context())
    }

    /// Return configuration information on allocated number
    fn get_allocated_number(&self, account: String, number: String) -> Box<Future<Item=GetAllocatedNumberResponse, Error=ApiError>> {
        self.api().get_allocated_number(account, number, &self.context())
    }

    /// Request a report of all current allocated numbers on account
    fn get_allocated_numbers(&self, account: String) -> Box<Future<Item=GetAllocatedNumbersResponse, Error=ApiError>> {
        self.api().get_allocated_numbers(account, &self.context())
    }

    /// Returns 1,10 or 100 numbers available for allocation matching the pattern specified
    fn get_available_numbers(&self, account: String, tier: String, number: i32, pattern: Option<String>) -> Box<Future<Item=GetAvailableNumbersResponse, Error=ApiError>> {
        self.api().get_available_numbers(account, tier, number, pattern, &self.context())
    }

    /// Retrieves a list of all available number ranges, including descriptions
    fn get_number_ranges(&self, account: String) -> Box<Future<Item=GetNumberRangesResponse, Error=ApiError>> {
        self.api().get_number_ranges(account, &self.context())
    }

    /// Allocate an available number to the account
    fn put_allocated_number(&self, account: String, number: String) -> Box<Future<Item=PutAllocatedNumberResponse, Error=ApiError>> {
        self.api().put_allocated_number(account, number, &self.context())
    }

    /// Remove IP from ACL-based trunk
    fn delete_outbound_acl_ip(&self, account: String, trunk: String, ip: String) -> Box<Future<Item=DeleteOutboundAclIpResponse, Error=ApiError>> {
        self.api().delete_outbound_acl_ip(account, trunk, ip, &self.context())
    }

    /// Delete the trunk
    fn delete_outbound_trunk(&self, account: String, trunk: String) -> Box<Future<Item=DeleteOutboundTrunkResponse, Error=ApiError>> {
        self.api().delete_outbound_trunk(account, trunk, &self.context())
    }

    /// Request information of specified trunk
    fn get_outbound_acl_ips(&self, account: String, trunk: String) -> Box<Future<Item=GetOutboundAclIpsResponse, Error=ApiError>> {
        self.api().get_outbound_acl_ips(account, trunk, &self.context())
    }

    /// Request information of specified trunk
    fn get_outbound_trunk(&self, account: String, trunk: String) -> Box<Future<Item=GetOutboundTrunkResponse, Error=ApiError>> {
        self.api().get_outbound_trunk(account, trunk, &self.context())
    }

    /// List all active outbound trunks
    fn get_outbound_trunks(&self, account: String) -> Box<Future<Item=GetOutboundTrunksResponse, Error=ApiError>> {
        self.api().get_outbound_trunks(account, &self.context())
    }

    /// Add IP to ACL-based trunk
    fn put_outbound_acl_ip(&self, account: String, trunk: String, ip: String) -> Box<Future<Item=PutOutboundAclIpResponse, Error=ApiError>> {
        self.api().put_outbound_acl_ip(account, trunk, ip, &self.context())
    }

    /// Create new trunk or update existing trunk
    fn put_outbound_trunk(&self, account: String, trunk: String, outbound_trunk: Option<models::OutboundTrunk>) -> Box<Future<Item=PutOutboundTrunkResponse, Error=ApiError>> {
        self.api().put_outbound_trunk(account, trunk, outbound_trunk, &self.context())
    }

    /// Return your external IP address, as seen by the Simwood API
    fn get_my_ip(&self) -> Box<Future<Item=GetMyIpResponse, Error=ApiError>> {
        self.api().get_my_ip(&self.context())
    }

    /// Returns the current timestamp
    fn get_time(&self) -> Box<Future<Item=GetTimeResponse, Error=ApiError>> {
        self.api().get_time(&self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
