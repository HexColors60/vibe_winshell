//! # ProcessManagerApp - close_network_connection_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn close_network_connection(&mut self, connection_id: &str) -> bool {
        #[cfg(target_os = "linux")]
        {
            if let Some(conn) = self
                .network_connections
                .iter()
                .find(|c| c.connection_id == connection_id)
            {
                self.add_log(
                    format!(
                        "Linux: Closing connection: {} -> {}", conn.local_addr, conn
                        .remote_addr
                    ),
                );
                self.refresh_network_connections();
                return true;
            }
            false
        }
        #[cfg(windows)]
        {
            use windows::Win32::NetworkManagement::IpHelper::{
                SetTcpEntry, MIB_TCPROW_LH, MIB_TCP_STATE_DELETE_TCB,
            };
            if let Some(conn) = self
                .network_connections
                .iter()
                .find(|c| c.connection_id == connection_id)
            {
                let parts: Vec<&str> = connection_id.split('-').collect();
                if parts.len() < 3 {
                    self.add_log(
                        format!("Invalid connection ID format: {}", connection_id),
                    );
                    return false;
                }
                let local_parts: Vec<&str> = conn.local_addr.split(':').collect();
                let remote_parts: Vec<&str> = conn.remote_addr.split(':').collect();
                if local_parts.len() < 2 || remote_parts.len() < 2 {
                    self.add_log(
                        format!(
                            "Cannot parse IPv6 connections yet: {} -> {}", conn
                            .local_addr, conn.remote_addr
                        ),
                    );
                    return false;
                }
                let local_ip_parts: Vec<u8> = local_parts[0]
                    .split('.')
                    .filter_map(|s| s.parse().ok())
                    .collect();
                let remote_ip_parts: Vec<u8> = remote_parts[0]
                    .split('.')
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if local_ip_parts.len() != 4 || remote_ip_parts.len() != 4 {
                    self.add_log(
                        format!(
                            "Cannot parse IP addresses: {} -> {}", conn.local_addr, conn
                            .remote_addr
                        ),
                    );
                    return false;
                }
                let local_port: u16 = local_parts[1].parse().unwrap_or(0);
                let remote_port: u16 = remote_parts[1].parse().unwrap_or(0);
                let local_addr: u32 = (local_ip_parts[0] as u32)
                    | ((local_ip_parts[1] as u32) << 8)
                    | ((local_ip_parts[2] as u32) << 16)
                    | ((local_ip_parts[3] as u32) << 24);
                let remote_addr: u32 = (remote_ip_parts[0] as u32)
                    | ((remote_ip_parts[1] as u32) << 8)
                    | ((remote_ip_parts[2] as u32) << 16)
                    | ((remote_ip_parts[3] as u32) << 24);
                let tcp_row = MIB_TCPROW_LH {
                    Anonymous: windows::Win32::NetworkManagement::IpHelper::MIB_TCPROW_LH_0 {
                        dwState: MIB_TCP_STATE_DELETE_TCB.0 as u32,
                    },
                    dwLocalAddr: local_addr,
                    dwLocalPort: local_port.to_be() as u32,
                    dwRemoteAddr: remote_addr,
                    dwRemotePort: remote_port.to_be() as u32,
                };
                unsafe {
                    let result = SetTcpEntry(&tcp_row as *const _ as *const _);
                    if result == 0 {
                        self.add_log(
                            format!(
                                "✅ Closed TCP connection: {} -> {}", conn.local_addr, conn
                                .remote_addr
                            ),
                        );
                        self.refresh_network_connections();
                        return true;
                    } else {
                        self.add_log(
                            format!(
                                "❌ Failed to close connection (error {}): {} -> {}. May need admin rights.",
                                result, conn.local_addr, conn.remote_addr
                            ),
                        );
                        return false;
                    }
                }
            } else {
                self.add_log(format!("Connection not found: {}", connection_id));
            }
            false
        }
        #[cfg(not(any(target_os = "linux", windows)))]
        {
            self.add_log(
                "Connection closing not supported on this platform".to_string(),
            );
            false
        }
    }
}
