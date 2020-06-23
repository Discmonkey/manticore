// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// NOTE: This file is autogenerated by:
// $ util/new_protocol_target.py request_counter::RequestCounterResponse

#![no_main]

use libfuzzer_sys::fuzz_target;

use manticore::protocol::Deserialize;
use manticore::protocol::request_counter::RequestCounterResponse;

fuzz_target!(|data: &[u8]| {
    let mut data = data;
    let _ = RequestCounterResponse::deserialize(&mut data);
});

