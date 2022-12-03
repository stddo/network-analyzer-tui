use std::ffi::c_void;

use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6};
use windows::Win32::NetworkManagement::IpHelper::{GetExtendedTcpTable, MIB_TCP6ROW_OWNER_PID, MIB_TCP6TABLE_OWNER_PID, MIB_TCPROW2, MIB_TCPROW_OWNER_PID, TCP_TABLE_OWNER_PID_ALL};

use crate::library::windows::WinDynStruct;

impl WinDynStruct for MIB_TCPROW_OWNER_PID {
    type Item = MIB_TCPROW2;
    type Buffer = c_void;

    unsafe fn call(buffer: *mut Self::Buffer, size: &mut u32) -> u32 {
        GetExtendedTcpTable(buffer, size, false, AF_INET.0, TCP_TABLE_OWNER_PID_ALL, 0)
    }
}

impl WinDynStruct for MIB_TCP6TABLE_OWNER_PID {
    type Item = MIB_TCP6ROW_OWNER_PID;
    type Buffer = c_void;

    unsafe fn call(buffer: *mut Self::Buffer, size: &mut u32) -> u32 {
        GetExtendedTcpTable(buffer, size, false, AF_INET6.0, TCP_TABLE_OWNER_PID_ALL, 0)
    }
}