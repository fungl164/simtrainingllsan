#[cfg(windows)]
use winapi;
#[cfg(windows)]
use ws2_32;

#[inline] #[cfg(windows)]
pub fn ws_init() {
    let w_version_requested = winapi::MAKEWORD(2, 2);
    let mut wsa_data = winapi::winsock2::WSADATA {
        wVersion : w_version_requested,
        wHighVersion : w_version_requested,
        iMaxSockets : 1000,
        iMaxUdpDg : 10000,
        lpVendorInfo : &mut 0,
        szDescription : [0; winapi::winsock2::WSADESCRIPTION_LEN + 1],
        szSystemStatus : [0; winapi::winsock2::WSASYS_STATUS_LEN + 1],
    };
    unsafe{
        ws2_32::WSAStartup(w_version_requested, &mut wsa_data);
    }
}
#[inline] #[cfg(unix)]
pub fn ws_init() {
}
