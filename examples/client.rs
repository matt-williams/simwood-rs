#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate simwood_rs;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
#[macro_use]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate clap;
extern crate tokio_core;
extern crate uuid;

use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Has, Push, AuthData};

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
use tokio_core::reactor;
#[allow(unused_imports)]
use simwood_rs::{ApiNoContext, ContextWrapperExt,
                      ApiError,
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
use clap::{App, Arg};

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
    "GetAccountType",
    "DeleteAllocatedNumber",
    "GetAllocatedNumber",
    "GetAllocatedNumbers",
    "GetAvailableNumbers",
    "GetNumberRanges",
    "PutAllocatedNumber",
    "DeleteOutboundAclIp",
    "DeleteOutboundTrunk",
    "GetOutboundAclIps",
    "GetOutboundTrunk",
    "GetOutboundTrunks",
    "PutOutboundAclIp",
    "PutOutboundTrunk",
    "GetMyIp",
    "GetTime",
])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("api.simwood.com")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("80")
            .help("Port to contact"))
        .get_matches();

    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        simwood_rs::Client::try_new_https(core.handle(), &base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        simwood_rs::Client::try_new_http(core.handle(), &base_url)
            .expect("Failed to create HTTP client")
    };

    let context: make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
        make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString(self::uuid::Uuid::new_v4().to_string()));
    let client = client.with_context(context);

    match matches.value_of("operation") {

        Some("GetAccountType") => {
            let result = core.run(client.get_account_type("account_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("DeleteAllocatedNumber") => {
            let result = core.run(client.delete_allocated_number("account_example".to_string(), "number_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetAllocatedNumber") => {
            let result = core.run(client.get_allocated_number("account_example".to_string(), "number_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetAllocatedNumbers") => {
            let result = core.run(client.get_allocated_numbers("account_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetAvailableNumbers") => {
            let result = core.run(client.get_available_numbers("account_example".to_string(), "tier_example".to_string(), 56, Some("pattern_example".to_string())));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetNumberRanges") => {
            let result = core.run(client.get_number_ranges("account_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("PutAllocatedNumber") => {
            let result = core.run(client.put_allocated_number("account_example".to_string(), "number_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("DeleteOutboundAclIp") => {
            let result = core.run(client.delete_outbound_acl_ip("account_example".to_string(), "trunk_example".to_string(), "ip_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("DeleteOutboundTrunk") => {
            let result = core.run(client.delete_outbound_trunk("account_example".to_string(), "trunk_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetOutboundAclIps") => {
            let result = core.run(client.get_outbound_acl_ips("account_example".to_string(), "trunk_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetOutboundTrunk") => {
            let result = core.run(client.get_outbound_trunk("account_example".to_string(), "trunk_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetOutboundTrunks") => {
            let result = core.run(client.get_outbound_trunks("account_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("PutOutboundAclIp") => {
            let result = core.run(client.put_outbound_acl_ip("account_example".to_string(), "trunk_example".to_string(), "ip_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("PutOutboundTrunk") => {
            let result = core.run(client.put_outbound_trunk("account_example".to_string(), "trunk_example".to_string(), None));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetMyIp") => {
            let result = core.run(client.get_my_ip());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetTime") => {
            let result = core.run(client.get_time());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

