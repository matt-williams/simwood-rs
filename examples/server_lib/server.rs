//! Server implementation of simwood_rs.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;
use swagger;
use swagger::{Has, XSpanIdString};

use simwood_rs::{Api, ApiError,
                      GetAccountTypeResponse,
                      DeleteAllocatedNumberResponse,
                      GetAllocatedNumberResponse,
                      GetAllocatedNumbersResponse,
                      GetAvailableNumbersResponse,
                      GetNumberRangesResponse,
                      PutAllocatedNumberResponse,
                      DeleteOutboundAclIpResponse,
                      DeleteOutboundTrunkResponse,
                      GetOutboundAclIpsResponse,
                      GetOutboundTrunkResponse,
                      GetOutboundTrunksResponse,
                      PutOutboundAclIpResponse,
                      PutOutboundTrunkResponse,
                      GetMyIpResponse,
                      GetTimeResponse
};
use simwood_rs::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{

    /// Get your current account type, and limitations
    fn get_account_type(&self, account: String, context: &C) -> Box<Future<Item=GetAccountTypeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_account_type(\"{}\") - X-Span-ID: {:?}", account, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// De-configure and irrevocably remove number from account
    fn delete_allocated_number(&self, account: String, number: String, context: &C) -> Box<Future<Item=DeleteAllocatedNumberResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_allocated_number(\"{}\", \"{}\") - X-Span-ID: {:?}", account, number, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Return configuration information on allocated number
    fn get_allocated_number(&self, account: String, number: String, context: &C) -> Box<Future<Item=GetAllocatedNumberResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_allocated_number(\"{}\", \"{}\") - X-Span-ID: {:?}", account, number, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Request a report of all current allocated numbers on account
    fn get_allocated_numbers(&self, account: String, context: &C) -> Box<Future<Item=GetAllocatedNumbersResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_allocated_numbers(\"{}\") - X-Span-ID: {:?}", account, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Returns 1,10 or 100 numbers available for allocation matching the pattern specified
    fn get_available_numbers(&self, account: String, tier: String, number: i32, pattern: Option<String>, context: &C) -> Box<Future<Item=GetAvailableNumbersResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_available_numbers(\"{}\", \"{}\", {}, {:?}) - X-Span-ID: {:?}", account, tier, number, pattern, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Retrieves a list of all available number ranges, including descriptions
    fn get_number_ranges(&self, account: String, context: &C) -> Box<Future<Item=GetNumberRangesResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_number_ranges(\"{}\") - X-Span-ID: {:?}", account, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Allocate an available number to the account
    fn put_allocated_number(&self, account: String, number: String, context: &C) -> Box<Future<Item=PutAllocatedNumberResponse, Error=ApiError>> {
        let context = context.clone();
        println!("put_allocated_number(\"{}\", \"{}\") - X-Span-ID: {:?}", account, number, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Remove IP from ACL-based trunk
    fn delete_outbound_acl_ip(&self, account: String, trunk: String, ip: String, context: &C) -> Box<Future<Item=DeleteOutboundAclIpResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_outbound_acl_ip(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", account, trunk, ip, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Delete the trunk
    fn delete_outbound_trunk(&self, account: String, trunk: String, context: &C) -> Box<Future<Item=DeleteOutboundTrunkResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_outbound_trunk(\"{}\", \"{}\") - X-Span-ID: {:?}", account, trunk, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Request information of specified trunk
    fn get_outbound_acl_ips(&self, account: String, trunk: String, context: &C) -> Box<Future<Item=GetOutboundAclIpsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_outbound_acl_ips(\"{}\", \"{}\") - X-Span-ID: {:?}", account, trunk, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Request information of specified trunk
    fn get_outbound_trunk(&self, account: String, trunk: String, context: &C) -> Box<Future<Item=GetOutboundTrunkResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_outbound_trunk(\"{}\", \"{}\") - X-Span-ID: {:?}", account, trunk, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// List all active outbound trunks
    fn get_outbound_trunks(&self, account: String, context: &C) -> Box<Future<Item=GetOutboundTrunksResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_outbound_trunks(\"{}\") - X-Span-ID: {:?}", account, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Add IP to ACL-based trunk
    fn put_outbound_acl_ip(&self, account: String, trunk: String, ip: String, context: &C) -> Box<Future<Item=PutOutboundAclIpResponse, Error=ApiError>> {
        let context = context.clone();
        println!("put_outbound_acl_ip(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", account, trunk, ip, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create new trunk or update existing trunk
    fn put_outbound_trunk(&self, account: String, trunk: String, outbound_trunk: Option<models::OutboundTrunk>, context: &C) -> Box<Future<Item=PutOutboundTrunkResponse, Error=ApiError>> {
        let context = context.clone();
        println!("put_outbound_trunk(\"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", account, trunk, outbound_trunk, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Return your external IP address, as seen by the Simwood API
    fn get_my_ip(&self, context: &C) -> Box<Future<Item=GetMyIpResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_my_ip() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Returns the current timestamp
    fn get_time(&self, context: &C) -> Box<Future<Item=GetTimeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_time() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
