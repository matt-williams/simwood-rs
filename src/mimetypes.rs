/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit

    lazy_static! {
        /// Create Mime objects for the response content types for GetAccountType
        pub static ref GET_ACCOUNT_TYPE_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllocatedNumber
        pub static ref GET_ALLOCATED_NUMBER_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllocatedNumbers
        pub static ref GET_ALLOCATED_NUMBERS_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAvailableNumbers
        pub static ref GET_AVAILABLE_NUMBERS_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNumberRanges
        pub static ref GET_NUMBER_RANGES_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetOutboundAclIps
        pub static ref GET_OUTBOUND_ACL_IPS_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetOutboundTrunk
        pub static ref GET_OUTBOUND_TRUNK_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetOutboundTrunks
        pub static ref GET_OUTBOUND_TRUNKS_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for PutOutboundTrunk
        pub static ref PUT_OUTBOUND_TRUNK_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for PutOutboundTrunk
        pub static ref PUT_OUTBOUND_TRUNK_FAILURE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMyIp
        pub static ref GET_MY_IP_SUCCESS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetTime
        pub static ref GET_TIME_SUCCESS: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

    lazy_static! {
        /// Create Mime objects for the request content types for PutOutboundTrunk
        pub static ref PUT_OUTBOUND_TRUNK: Mime = "application/json".parse().unwrap();
    }

}
