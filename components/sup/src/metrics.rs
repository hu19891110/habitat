// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


pub enum Counter {
    UTPMessagesReceived,
    UTPMessagesSent,
}

pub struct MetricRegistry {
    pub utp_messages_received: u64,
    pub utp_messages_sent: u64,
}

impl MetricRegistry {
    pub fn new() -> MetricRegistry {
        return MetricRegistry {utp_messages_sent: 0,
                               utp_messages_received: 0}
    }

    pub fn incr(&mut self, name: Counter) {
        match name {
            Counter::UTPMessagesReceived => self.utp_messages_sent += 1,
            Counter::UTPMessagesSent => self.utp_messages_received += 1,
        }
    }


    pub fn decr(&mut self, name: Counter) {
        match name {
            Counter::UTPMessagesReceived => self.utp_messages_sent -= 1,
            Counter::UTPMessagesSent => self.utp_messages_received -= 1,
        }
    }
}
