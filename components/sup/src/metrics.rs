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

use std::collections::VecDeque;

pub enum Counter {
    UTPMessagesReceived,
    UTPMessagesSent,
}

pub enum Window {
    UTPReceiveTime,
}

pub struct NanoWindow {
    pub capacity: usize,
    pub current_size: usize,
    pub dequeue: VecDeque<u64>,
}

impl NanoWindow {
    pub fn new(capacity: usize) -> NanoWindow {
        let q = VecDeque::with_capacity(capacity);
        NanoWindow { capacity: capacity, current_size: 0, dequeue: q}
    }

    pub fn push(&mut self, value: u64) {
        if self.current_size < self.capacity {
            self.dequeue.push_back(value);
            self.current_size += 1;
        } else {
            self.dequeue.pop_front();
            self.dequeue.push_back(value);
        }
    }

    pub fn average(&self) -> u64 {
        if self.current_size == 0 {
            return 0;
        }
        self.dequeue.iter().fold(0, |acc, &x| acc + x) / (self.current_size as u64)
    }
}

pub struct MetricRegistry {
    pub utp_messages_received: u64,
    pub utp_messages_sent: u64,
    pub utp_receive_times: NanoWindow
}

impl MetricRegistry {
    pub fn new() -> MetricRegistry {
        return MetricRegistry {utp_messages_sent: 0,
                               utp_messages_received: 0,
        utp_receive_times: NanoWindow::new(100)}
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

    pub fn window_push(&mut self, name: Window, value: u64) {
        match name {
            Window::UTPReceiveTime => {
                self.utp_receive_times.push(value);
            }
        }
    }

    pub fn window_avg(&self, name: Window) -> u64 {
        match name {
            Window::UTPReceiveTime => {
                self.utp_receive_times.average()
            }
        }
    }

    pub fn bulk_report(&self) {
        unimplemented!();
    }
}
