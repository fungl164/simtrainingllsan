use rand;
#[cfg(windows)]
use winapi;
#[cfg(windows)]
use ws2_32;
use rand::Rng;
use simctrl::TONG_YONG_ZAO_SHENG_XIANG_DUI_FU_ZHI;
#[inline]
pub fn gen_range(a:f64, b:f64) -> f64{
    rand::thread_rng().gen_range(a, b)
}
#[inline]
pub fn noise(a:f64) -> f64{
    a * (1.0f64+rand::thread_rng().gen_range(-TONG_YONG_ZAO_SHENG_XIANG_DUI_FU_ZHI,
                            TONG_YONG_ZAO_SHENG_XIANG_DUI_FU_ZHI))
}
#[inline]
pub fn zan_tai_linear(t:f64, x1:f64, x2:f64, t1:f64, t2:f64) -> f64{
    (x2-x1)*(t-t1)/(t2-t1)+x1
}
#[inline]
#[cfg(windows)]
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
#[inline]
#[cfg(unix)]
pub fn ws_init() {
}
