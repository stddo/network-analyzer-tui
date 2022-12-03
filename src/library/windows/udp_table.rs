use std::ffi::c_void;

use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6};
use windows::Win32::NetworkManagement::IpHelper::{GetExtendedUdpTable, MIB_UDP6ROW_OWNER_PID, MIB_UDP6TABLE_OWNER_PID, MIB_UDPROW_OWNER_PID, MIB_UDPTABLE_OWNER_PID, UDP_TABLE_OWNER_PID};

use crate::library::windows::WinDynStruct;

impl WinDynStruct for MIB_UDPTABLE_OWNER_PID {
    type Item = MIB_UDPROW_OWNER_PID;
    type Buffer = c_void;

    unsafe fn call(buffer: *mut Self::Buffer, size: &mut u32) -> u32 {
        GetExtendedUdpTable(buffer, size, false, AF_INET.0, UDP_TABLE_OWNER_PID, 0)
    }
}

impl WinDynStruct for MIB_UDP6TABLE_OWNER_PID {
    type Item = MIB_UDP6ROW_OWNER_PID;
    type Buffer = c_void;

    unsafe fn call(buffer: *mut Self::Buffer, size: &mut u32) -> u32 {
        GetExtendedUdpTable(buffer, size, false, AF_INET6.0, UDP_TABLE_OWNER_PID, 0)
    }
}