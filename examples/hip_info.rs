use hip_runtime_sys::{
    hipDeviceProp_t, hipDriverGetVersion, hipError_t, hipGetDeviceCount, hipGetDeviceProperties,
    hipInit, hipRuntimeGetVersion,
};

fn main() {
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
        let (name, device_prop) = unsafe {
            let mut device_prop: hipDeviceProp_t = std::mem::zeroed();
            let result = hipGetDeviceProperties(&mut device_prop, i);
            assert_eq!(result, hipError_t::hipSuccess);
            let device_name_u8: &[u8] =
                std::slice::from_raw_parts(device_prop.name.as_ptr() as *const u8, 256);
            let result_str = std::str::from_utf8(device_name_u8).unwrap();
            (result_str, device_prop)
        };
        println!(
            "Device {}: {} | multi {}",
            i, name, device_prop.isMultiGpuBoard
        );
        println!(
            " -> mem    | glb: {}GiB, shared/blk: {}KiB, ",
            device_prop.totalGlobalMem / (1024 * 1024 * 1024),
            device_prop.sharedMemPerBlock / (1024)
        );
        println!(
            " -> thread | max/blk: {}, warpSize {}, max [{} {} {}]",
            device_prop.maxThreadsPerBlock,
            device_prop.warpSize,
            device_prop.maxThreadsDim[0],
            device_prop.maxThreadsDim[1],
            device_prop.maxThreadsDim[2]
        );
        println!(
            " -> grid   | max [{} {} {}]",
            device_prop.maxGridSize[0], device_prop.maxGridSize[1], device_prop.maxGridSize[2]
        );
    }
}
