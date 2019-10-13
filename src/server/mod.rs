#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate chrono;
extern crate percent_encoding;
extern crate url;

use std::sync::Arc;
use std::marker::PhantomData;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use hyper::header::{Headers, ContentType};
use self::url::form_urlencoded;
use mimetypes;
use serde_json;

#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::{ApiError, XSpanId, XSpanIdString, Has, RequestParser};
use swagger::auth::Scopes;

use {Api,
     GetAccountTypeResponse,
     DeleteAllocatedNumberResponse,
     DeleteNumberConfigResponse,
     GetAllocatedNumberResponse,
     GetAllocatedNumbersResponse,
     GetAvailableNumbersResponse,
     GetNumberConfigResponse,
     GetNumberRangesResponse,
     PutAllocatedNumberResponse,
     PutNumberConfigResponse,
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
#[allow(unused_imports)]
use models;

pub mod context;

header! { (Warning, "Warning") => [String] }

mod paths {
    extern crate regex;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/v3/accounts/(?P<account>[^/?#]*)/type$",
            r"^/v3/numbers/(?P<account>[^/?#]*)/allocated/all$",
            r"^/v3/numbers/(?P<account>[^/?#]*)/allocated/(?P<number>[^/?#]*)$",
            r"^/v3/numbers/(?P<account>[^/?#]*)/allocated/(?P<number>[^/?#]*)/config$",
            r"^/v3/numbers/(?P<account>[^/?#]*)/available/(?P<tier>[^/?#]*)/(?P<number>[^/?#]*)$",
            r"^/v3/numbers/(?P<account>[^/?#]*)/ranges$",
            r"^/v3/tools/myip$",
            r"^/v3/tools/time$",
            r"^/v3/voice/(?P<account>[^/?#]*)/outbound$",
            r"^/v3/voice/(?P<account>[^/?#]*)/outbound/(?P<trunk>[^/?#]*)$",
            r"^/v3/voice/(?P<account>[^/?#]*)/outbound/(?P<trunk>[^/?#]*)/acl$",
            r"^/v3/voice/(?P<account>[^/?#]*)/outbound/(?P<trunk>[^/?#]*)/acl/(?P<ip>[^/?#]*)$"
        ]).unwrap();
    }
    pub static ID_ACCOUNTS_ACCOUNT_TYPE: usize = 0;
    lazy_static! {
        pub static ref REGEX_ACCOUNTS_ACCOUNT_TYPE: regex::Regex = regex::Regex::new(r"^/v3/accounts/(?P<account>[^/?#]*)/type$").unwrap();
    }
    pub static ID_NUMBERS_ACCOUNT_ALLOCATED_ALL: usize = 1;
    lazy_static! {
        pub static ref REGEX_NUMBERS_ACCOUNT_ALLOCATED_ALL: regex::Regex = regex::Regex::new(r"^/v3/numbers/(?P<account>[^/?#]*)/allocated/all$").unwrap();
    }
    pub static ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER: usize = 2;
    lazy_static! {
        pub static ref REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER: regex::Regex = regex::Regex::new(r"^/v3/numbers/(?P<account>[^/?#]*)/allocated/(?P<number>[^/?#]*)$").unwrap();
    }
    pub static ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG: usize = 3;
    lazy_static! {
        pub static ref REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG: regex::Regex = regex::Regex::new(r"^/v3/numbers/(?P<account>[^/?#]*)/allocated/(?P<number>[^/?#]*)/config$").unwrap();
    }
    pub static ID_NUMBERS_ACCOUNT_AVAILABLE_TIER_NUMBER: usize = 4;
    lazy_static! {
        pub static ref REGEX_NUMBERS_ACCOUNT_AVAILABLE_TIER_NUMBER: regex::Regex = regex::Regex::new(r"^/v3/numbers/(?P<account>[^/?#]*)/available/(?P<tier>[^/?#]*)/(?P<number>[^/?#]*)$").unwrap();
    }
    pub static ID_NUMBERS_ACCOUNT_RANGES: usize = 5;
    lazy_static! {
        pub static ref REGEX_NUMBERS_ACCOUNT_RANGES: regex::Regex = regex::Regex::new(r"^/v3/numbers/(?P<account>[^/?#]*)/ranges$").unwrap();
    }
    pub static ID_TOOLS_MYIP: usize = 6;
    pub static ID_TOOLS_TIME: usize = 7;
    pub static ID_VOICE_ACCOUNT_OUTBOUND: usize = 8;
    lazy_static! {
        pub static ref REGEX_VOICE_ACCOUNT_OUTBOUND: regex::Regex = regex::Regex::new(r"^/v3/voice/(?P<account>[^/?#]*)/outbound$").unwrap();
    }
    pub static ID_VOICE_ACCOUNT_OUTBOUND_TRUNK: usize = 9;
    lazy_static! {
        pub static ref REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK: regex::Regex = regex::Regex::new(r"^/v3/voice/(?P<account>[^/?#]*)/outbound/(?P<trunk>[^/?#]*)$").unwrap();
    }
    pub static ID_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL: usize = 10;
    lazy_static! {
        pub static ref REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL: regex::Regex = regex::Regex::new(r"^/v3/voice/(?P<account>[^/?#]*)/outbound/(?P<trunk>[^/?#]*)/acl$").unwrap();
    }
    pub static ID_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP: usize = 11;
    lazy_static! {
        pub static ref REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP: regex::Regex = regex::Regex::new(r"^/v3/voice/(?P<account>[^/?#]*)/outbound/(?P<trunk>[^/?#]*)/acl/(?P<ip>[^/?#]*)$").unwrap();
    }
}

pub struct NewService<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
{
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> NewService<T, C> {
        NewService{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::NewService for NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Instance = Service<T, C>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(Service::new(self.api_impl.clone()))
    }
}

pub struct Service<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> Service<T, C> {
        Service{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::Service for Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Response, Error=Error>>;

    fn call(&self, (req, mut context): Self::Request) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (method, uri, _, headers, body) = req.deconstruct();
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        // This match statement is duplicated below in `parse_operation_id()`.
        // Please update both places if changing how this code is autogenerated.
        match &method {

            // GetAccountType - GET /accounts/{account}/type
            &hyper::Method::Get if path.matched(paths::ID_ACCOUNTS_ACCOUNT_TYPE) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_ACCOUNTS_ACCOUNT_TYPE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ACCOUNTS_ACCOUNT_TYPE in set but failed match against \"{}\"", path, paths::REGEX_ACCOUNTS_ACCOUNT_TYPE.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.get_account_type(param_account, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAccountTypeResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ACCOUNT_TYPE_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // DeleteAllocatedNumber - DELETE /numbers/{account}/allocated/{number}
            &hyper::Method::Delete if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_ALLOCATED_NUMBER in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_number = match percent_encoding::percent_decode(path_params["number"].as_bytes()).decode_utf8() {
                    Ok(param_number) => match param_number.parse::<String>() {
                        Ok(param_number) => param_number,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter number: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["number"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.delete_allocated_number(param_account, param_number, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteAllocatedNumberResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                DeleteAllocatedNumberResponse::NumberNotAllocated


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // DeleteNumberConfig - DELETE /numbers/{account}/allocated/{number}/config
            &hyper::Method::Delete if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_number = match percent_encoding::percent_decode(path_params["number"].as_bytes()).decode_utf8() {
                    Ok(param_number) => match param_number.parse::<String>() {
                        Ok(param_number) => param_number,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter number: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["number"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.delete_number_config(param_account, param_number, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteNumberConfigResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                DeleteNumberConfigResponse::NumberNotAllocated


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetAllocatedNumber - GET /numbers/{account}/allocated/{number}
            &hyper::Method::Get if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_ALLOCATED_NUMBER in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_number = match percent_encoding::percent_decode(path_params["number"].as_bytes()).decode_utf8() {
                    Ok(param_number) => match param_number.parse::<String>() {
                        Ok(param_number) => param_number,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter number: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["number"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.get_allocated_number(param_account, param_number, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllocatedNumberResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ALLOCATED_NUMBER_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetAllocatedNumberResponse::NumberNotAllocated


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetAllocatedNumbers - POST /numbers/{account}/allocated/all
            &hyper::Method::Post if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_ALL) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_ALL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_ALLOCATED_ALL in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_ALL.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.get_allocated_numbers(param_account, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllocatedNumbersResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ALLOCATED_NUMBERS_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetAvailableNumbers - GET /numbers/{account}/available/{tier}/{number}
            &hyper::Method::Get if path.matched(paths::ID_NUMBERS_ACCOUNT_AVAILABLE_TIER_NUMBER) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_AVAILABLE_TIER_NUMBER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_AVAILABLE_TIER_NUMBER in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_AVAILABLE_TIER_NUMBER.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_tier = match percent_encoding::percent_decode(path_params["tier"].as_bytes()).decode_utf8() {
                    Ok(param_tier) => match param_tier.parse::<String>() {
                        Ok(param_tier) => param_tier,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter tier: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["tier"]))))
                };
                let param_number = match percent_encoding::percent_decode(path_params["number"].as_bytes()).decode_utf8() {
                    Ok(param_number) => match param_number.parse::<i32>() {
                        Ok(param_number) => param_number,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter number: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["number"]))))
                };
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_pattern = query_params.iter().filter(|e| e.0 == "pattern").map(|e| e.1.to_owned())
                    .nth(0);

                let param_pattern = param_pattern.and_then(|param_pattern| param_pattern.parse::<>().ok());
                Box::new({
                        {{
                                Box::new(api_impl.get_available_numbers(param_account, param_tier, param_number, param_pattern, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAvailableNumbersResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_AVAILABLE_NUMBERS_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetNumberConfig - GET /numbers/{account}/allocated/{number}/config
            &hyper::Method::Get if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_number = match percent_encoding::percent_decode(path_params["number"].as_bytes()).decode_utf8() {
                    Ok(param_number) => match param_number.parse::<String>() {
                        Ok(param_number) => param_number,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter number: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["number"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.get_number_config(param_account, param_number, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetNumberConfigResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_NUMBER_CONFIG_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetNumberConfigResponse::NumberNotAllocated


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetNumberRanges - GET /numbers/{account}/ranges
            &hyper::Method::Get if path.matched(paths::ID_NUMBERS_ACCOUNT_RANGES) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_RANGES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_RANGES in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_RANGES.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.get_number_ranges(param_account, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetNumberRangesResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_NUMBER_RANGES_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // PutAllocatedNumber - PUT /numbers/{account}/allocated/{number}
            &hyper::Method::Put if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_ALLOCATED_NUMBER in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_number = match percent_encoding::percent_decode(path_params["number"].as_bytes()).decode_utf8() {
                    Ok(param_number) => match param_number.parse::<String>() {
                        Ok(param_number) => param_number,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter number: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["number"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.put_allocated_number(param_account, param_number, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutAllocatedNumberResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                PutAllocatedNumberResponse::NumberNotAvailable


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // PutNumberConfig - PUT /numbers/{account}/allocated/{number}/config
            &hyper::Method::Put if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG in set but failed match against \"{}\"", path, paths::REGEX_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_number = match percent_encoding::percent_decode(path_params["number"].as_bytes()).decode_utf8() {
                    Ok(param_number) => match param_number.parse::<String>() {
                        Ok(param_number) => param_number,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter number: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["number"]))))
                };
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_number_config: Option<models::NumberConfig> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_number_config) => param_number_config,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };
                                Box::new(api_impl.put_number_config(param_account, param_number, param_number_config, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutNumberConfigResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::PUT_NUMBER_CONFIG_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                PutNumberConfigResponse::NumberNotAllocated


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter NumberConfig: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>
            },

            // DeleteOutboundAclIp - DELETE /voice/{account}/outbound/{trunk}/acl/{ip}
            &hyper::Method::Delete if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP in set but failed match against \"{}\"", path, paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_trunk = match percent_encoding::percent_decode(path_params["trunk"].as_bytes()).decode_utf8() {
                    Ok(param_trunk) => match param_trunk.parse::<String>() {
                        Ok(param_trunk) => param_trunk,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter trunk: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["trunk"]))))
                };
                let param_ip = match percent_encoding::percent_decode(path_params["ip"].as_bytes()).decode_utf8() {
                    Ok(param_ip) => match param_ip.parse::<String>() {
                        Ok(param_ip) => param_ip,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter ip: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["ip"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.delete_outbound_acl_ip(param_account, param_trunk, param_ip, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOutboundAclIpResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // DeleteOutboundTrunk - DELETE /voice/{account}/outbound/{trunk}
            &hyper::Method::Delete if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VOICE_ACCOUNT_OUTBOUND_TRUNK in set but failed match against \"{}\"", path, paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_trunk = match percent_encoding::percent_decode(path_params["trunk"].as_bytes()).decode_utf8() {
                    Ok(param_trunk) => match param_trunk.parse::<String>() {
                        Ok(param_trunk) => param_trunk,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter trunk: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["trunk"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.delete_outbound_trunk(param_account, param_trunk, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOutboundTrunkResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetOutboundAclIps - GET /voice/{account}/outbound/{trunk}/acl
            &hyper::Method::Get if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL in set but failed match against \"{}\"", path, paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_trunk = match percent_encoding::percent_decode(path_params["trunk"].as_bytes()).decode_utf8() {
                    Ok(param_trunk) => match param_trunk.parse::<String>() {
                        Ok(param_trunk) => param_trunk,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter trunk: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["trunk"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.get_outbound_acl_ips(param_account, param_trunk, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOutboundAclIpsResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OUTBOUND_ACL_IPS_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetOutboundTrunk - GET /voice/{account}/outbound/{trunk}
            &hyper::Method::Get if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VOICE_ACCOUNT_OUTBOUND_TRUNK in set but failed match against \"{}\"", path, paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_trunk = match percent_encoding::percent_decode(path_params["trunk"].as_bytes()).decode_utf8() {
                    Ok(param_trunk) => match param_trunk.parse::<String>() {
                        Ok(param_trunk) => param_trunk,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter trunk: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["trunk"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.get_outbound_trunk(param_account, param_trunk, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOutboundTrunkResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OUTBOUND_TRUNK_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetOutboundTrunks - GET /voice/{account}/outbound
            &hyper::Method::Get if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_VOICE_ACCOUNT_OUTBOUND
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VOICE_ACCOUNT_OUTBOUND in set but failed match against \"{}\"", path, paths::REGEX_VOICE_ACCOUNT_OUTBOUND.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.get_outbound_trunks(param_account, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOutboundTrunksResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OUTBOUND_TRUNKS_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // PutOutboundAclIp - PUT /voice/{account}/outbound/{trunk}/acl/{ip}
            &hyper::Method::Put if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP in set but failed match against \"{}\"", path, paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_trunk = match percent_encoding::percent_decode(path_params["trunk"].as_bytes()).decode_utf8() {
                    Ok(param_trunk) => match param_trunk.parse::<String>() {
                        Ok(param_trunk) => param_trunk,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter trunk: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["trunk"]))))
                };
                let param_ip = match percent_encoding::percent_decode(path_params["ip"].as_bytes()).decode_utf8() {
                    Ok(param_ip) => match param_ip.parse::<String>() {
                        Ok(param_ip) => param_ip,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter ip: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["ip"]))))
                };
                Box::new({
                        {{
                                Box::new(api_impl.put_outbound_acl_ip(param_account, param_trunk, param_ip, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutOutboundAclIpResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // PutOutboundTrunk - PUT /voice/{account}/outbound/{trunk}
            &hyper::Method::Put if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }
                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VOICE_ACCOUNT_OUTBOUND_TRUNK in set but failed match against \"{}\"", path, paths::REGEX_VOICE_ACCOUNT_OUTBOUND_TRUNK.as_str())
                    );
                let param_account = match percent_encoding::percent_decode(path_params["account"].as_bytes()).decode_utf8() {
                    Ok(param_account) => match param_account.parse::<String>() {
                        Ok(param_account) => param_account,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter account: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["account"]))))
                };
                let param_trunk = match percent_encoding::percent_decode(path_params["trunk"].as_bytes()).decode_utf8() {
                    Ok(param_trunk) => match param_trunk.parse::<String>() {
                        Ok(param_trunk) => param_trunk,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter trunk: {:?}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["trunk"]))))
                };
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_outbound_trunk: Option<models::OutboundTrunk> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_outbound_trunk) => param_outbound_trunk,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter OutboundTrunk - doesn't match schema: {}", e)))),
                                    }
                                } else {
                                    None
                                };
                                let param_outbound_trunk = match param_outbound_trunk {
                                    Some(param_outbound_trunk) => param_outbound_trunk,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter OutboundTrunk"))),
                                };
                                Box::new(api_impl.put_outbound_trunk(param_account, param_trunk, param_outbound_trunk, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutOutboundTrunkResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::PUT_OUTBOUND_TRUNK_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                PutOutboundTrunkResponse::Failure

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::PUT_OUTBOUND_TRUNK_FAILURE.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter OutboundTrunk: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>
            },

            // GetMyIp - GET /tools/myip
            &hyper::Method::Get if path.matched(paths::ID_TOOLS_MYIP) => {
                Box::new({
                        {{
                                Box::new(api_impl.get_my_ip(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetMyIpResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_MY_IP_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            // GetTime - GET /tools/time
            &hyper::Method::Get if path.matched(paths::ID_TOOLS_TIME) => {
                Box::new({
                        {{
                                Box::new(api_impl.get_time(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetTimeResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_TIME_SUCCESS.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Box<Future<Item=Response, Error=Error>>
            },

            _ => Box::new(future::ok(Response::new().with_status(StatusCode::NotFound))) as Box<Future<Item=Response, Error=Error>>,
        }
    }
}

impl<T, C> Clone for Service<T, C>
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}


/// Request parser for `Api`.
pub struct ApiRequestParser;
impl RequestParser for ApiRequestParser {
    fn parse_operation_id(request: &Request) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {

            // GetAccountType - GET /accounts/{account}/type
            &hyper::Method::Get if path.matched(paths::ID_ACCOUNTS_ACCOUNT_TYPE) => Ok("GetAccountType"),

            // DeleteAllocatedNumber - DELETE /numbers/{account}/allocated/{number}
            &hyper::Method::Delete if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER) => Ok("DeleteAllocatedNumber"),

            // DeleteNumberConfig - DELETE /numbers/{account}/allocated/{number}/config
            &hyper::Method::Delete if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG) => Ok("DeleteNumberConfig"),

            // GetAllocatedNumber - GET /numbers/{account}/allocated/{number}
            &hyper::Method::Get if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER) => Ok("GetAllocatedNumber"),

            // GetAllocatedNumbers - POST /numbers/{account}/allocated/all
            &hyper::Method::Post if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_ALL) => Ok("GetAllocatedNumbers"),

            // GetAvailableNumbers - GET /numbers/{account}/available/{tier}/{number}
            &hyper::Method::Get if path.matched(paths::ID_NUMBERS_ACCOUNT_AVAILABLE_TIER_NUMBER) => Ok("GetAvailableNumbers"),

            // GetNumberConfig - GET /numbers/{account}/allocated/{number}/config
            &hyper::Method::Get if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG) => Ok("GetNumberConfig"),

            // GetNumberRanges - GET /numbers/{account}/ranges
            &hyper::Method::Get if path.matched(paths::ID_NUMBERS_ACCOUNT_RANGES) => Ok("GetNumberRanges"),

            // PutAllocatedNumber - PUT /numbers/{account}/allocated/{number}
            &hyper::Method::Put if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER) => Ok("PutAllocatedNumber"),

            // PutNumberConfig - PUT /numbers/{account}/allocated/{number}/config
            &hyper::Method::Put if path.matched(paths::ID_NUMBERS_ACCOUNT_ALLOCATED_NUMBER_CONFIG) => Ok("PutNumberConfig"),

            // DeleteOutboundAclIp - DELETE /voice/{account}/outbound/{trunk}/acl/{ip}
            &hyper::Method::Delete if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP) => Ok("DeleteOutboundAclIp"),

            // DeleteOutboundTrunk - DELETE /voice/{account}/outbound/{trunk}
            &hyper::Method::Delete if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK) => Ok("DeleteOutboundTrunk"),

            // GetOutboundAclIps - GET /voice/{account}/outbound/{trunk}/acl
            &hyper::Method::Get if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL) => Ok("GetOutboundAclIps"),

            // GetOutboundTrunk - GET /voice/{account}/outbound/{trunk}
            &hyper::Method::Get if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK) => Ok("GetOutboundTrunk"),

            // GetOutboundTrunks - GET /voice/{account}/outbound
            &hyper::Method::Get if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND) => Ok("GetOutboundTrunks"),

            // PutOutboundAclIp - PUT /voice/{account}/outbound/{trunk}/acl/{ip}
            &hyper::Method::Put if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK_ACL_IP) => Ok("PutOutboundAclIp"),

            // PutOutboundTrunk - PUT /voice/{account}/outbound/{trunk}
            &hyper::Method::Put if path.matched(paths::ID_VOICE_ACCOUNT_OUTBOUND_TRUNK) => Ok("PutOutboundTrunk"),

            // GetMyIp - GET /tools/myip
            &hyper::Method::Get if path.matched(paths::ID_TOOLS_MYIP) => Ok("GetMyIp"),

            // GetTime - GET /tools/time
            &hyper::Method::Get if path.matched(paths::ID_TOOLS_TIME) => Ok("GetTime"),
            _ => Err(()),
        }
    }
}
