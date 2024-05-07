use hip_runtime_sys::{hipError_t, hipInit, hipDriverGetVersion, hipRuntimeGetVersion, hipGetDeviceCount, hipDeviceGetName};

fn main() -> () {
    let result = unsafe { hipInit(0) };
    assert_eq!(result, hipError_t::hipSuccess);

    let mut driver_version: i32 = 0;
    let result = unsafe { hipDriverGetVersion(&mut driver_version) };
    assert_eq!(result, hipError_t::hipSuccess);
    println!("Driver Version: {driver_version}");

    let mut runtime_version: i32 = 0;
    let result = unsafe { hipRuntimeGetVersion(&mut runtime_version) };
    assert_eq!(result, hipError_t::hipSuccess);
    println!("Runtime Version: {runtime_version}");

    let mut device_count: i32 = 0;
    let result = unsafe { hipGetDeviceCount(&mut device_count) };
    assert_eq!(result, hipError_t::hipSuccess);
    println!("Device Count: {device_count}");

    for i in 0..device_count {
        let (result, name) = unsafe {
            let mut device_name: [i8; 256] = [0; 256];
            let result = hipDeviceGetName(device_name.as_mut_ptr(), 256, i);
            let device_name_u8: &[u8] = unsafe { std::slice::from_raw_parts(device_name.as_ptr() as *const u8, device_name.len()) };
            let result_str = std::str::from_utf8(&device_name_u8).unwrap();
            (result, result_str)
        };
        assert_eq!(result, hipError_t::hipSuccess);
        println!("Device {}: {}", i, name);
    }
}