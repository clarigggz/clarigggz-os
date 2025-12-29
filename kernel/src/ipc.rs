// SPDX-License-Identifier: MPL-2.0

use spin::Mutex;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Message {
    pub from: u64,
    pub to: u64,
    pub data: Vec<u8>,
}

pub struct IpcManager {
    messages: Mutex<VecDeque<Message>>,
}

impl IpcManager {
    pub const fn new() -> Self {
        Self {
            messages: Mutex::new(VecDeque::new()),
        }
    }

    pub fn send(&self, msg: Message) {
        self.messages.lock().push_back(msg);
    }

    pub fn recv(&self, to: u64) -> Option<Message> {
        let mut msgs = self.messages.lock();
        if let Some(pos) = msgs.iter().position(|m| m.to == to) {
            return msgs.remove(pos);
        }
        None
    }
}

pub static IPC: IpcManager = IpcManager::new();
