use crate::vm::VM;
use std::thread;

#[derive(Default, Debug)]
pub struct Scheduler {
    next_pid: u32,
    max_pid: u32,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            next_pid: 0,
            max_pid: std::u32::MAX,
        }
    }

    pub fn get_thread(&mut self, mut vm: VM) -> thread::JoinHandle<VM> {
        self.next_pid = self.next_pid.wrapping_add(1) % self.max_pid;
        thread::spawn(move || {
            vm.run();
            vm
        })
    }
}
