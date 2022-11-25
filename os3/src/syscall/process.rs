//! Process management syscalls
use alloc::task;
use alloc::vec::Vec;

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{
    exit_current_and_run_next, get_task_info, suspend_current_and_run_next, TaskStatus,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: Vec<u32>,
    pub time: usize,
}

impl TaskInfo {
    pub fn new() -> Self {
        let mut syscall_times: Vec<u32> = Vec::new();
        for _ in 0..MAX_SYSCALL_NUM {
            syscall_times.push(0);
        }
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times,
            time: 0,
        }
    }
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    unsafe {
        (*ti) = get_task_info();
        (*ti).time = (get_time_us() - (*ti).time) / 1000;
    }
    0
}
