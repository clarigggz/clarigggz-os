// SPDX-License-Identifier: MPL-2.0

use core::arch::global_asm;
use riscv::register::{stvec, scause, stval, sepc};
use crate::println;
use crate::ipc::{IPC, Message};
use crate::task;
use alloc::vec;

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

global_asm!(include_str!("trap.asm"));

unsafe extern "C" {
    fn __alltraps();
}

pub fn init() {
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let cause = scause::read();
    let epc = sepc::read();
    let tval = stval::read();

    match cause.cause() {
        scause::Trap::Exception(scause::Exception::UserEnvCall) => {
            cx.sepc += 4;
            let syscall_id = cx.x[17]; // a7
            let arg0 = cx.x[10]; // a0
            let arg1 = cx.x[11]; // a1
            let arg2 = cx.x[12]; // a2

            match syscall_id {
                1 => { // IPC_SEND
                    let to = arg0 as u64;
                    let data_ptr = arg1 as *const u8;
                    let data_len = arg2;
                    let data = unsafe { core::slice::from_raw_parts(data_ptr, data_len) }.to_vec();
                    IPC.send(Message { from: 0, to, data });
                    cx.x[10] = 0; // return success
                }
                2 => { // IPC_RECV
                    let to = arg0 as u64;
                    if let Some(msg) = IPC.recv(to) {
                        cx.x[10] = msg.data.get(0).cloned().unwrap_or(0) as usize;
                    } else {
                        cx.x[10] = usize::MAX; // no message
                    }
                }
                3 => { // YIELD
                    task::suspend_current_and_run_next();
                    cx.x[10] = 0;
                }
                _ => {
                    println!("Unknown syscall: {}", syscall_id);
                    cx.x[10] = usize::MAX;
                }
            }
        }
        _ => {
            panic!("Unhandled trap: {:?}, epc: {:#x}, tval: {:#x}", cause.cause(), epc, tval);
        }
    }
    cx
}
