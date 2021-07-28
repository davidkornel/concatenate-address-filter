/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use quilkin::filters::{prelude::*, DynFilterFactory};
use quilkin::runner::run;
use std::str;


/// The `ConcatenateBytes` filter's job is to add a byte packet to either the beginning or end of each UDP packet that passes
/// through. This is commonly used to provide an auth token to each packet, so they can be routed appropriately.
#[quilkin::filter("ConcatenateAddress.v1")]
struct ConcatenateAddress;

pub struct ConcatAddressFactory;

impl Default for ConcatAddressFactory {
    fn default() -> Self {
        Self {}
    }
}

impl FilterFactory for ConcatAddressFactory {
    fn name(&self) -> &'static str {
        ConcatenateAddress::FILTER_NAME
    }

    fn create_filter(&self, _args: CreateFilterArgs) -> Result<Box<dyn Filter>, Error> {
        Ok(Box::new(ConcatenateAddress))
    }
}

impl Filter for ConcatenateAddress {
    fn read(&self, mut ctx: ReadContext) -> Option<ReadResponse> {
        ctx.contents.extend(ctx.from.to_string().into_bytes().iter());
        Some(ctx.into())
    }

    fn write(&self, mut ctx: WriteContext) -> Option<WriteResponse> {
        Some(ctx.into())
    }
}

#[tokio::main]
async fn main() {
    run(vec![DynFilterFactory::from(Box::new(ConcatAddressFactory))].into_iter()).await.unwrap();
}