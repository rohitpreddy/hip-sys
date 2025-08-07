use hip_sys::hiprt::{hipError_t, hipInit};

#[test]
fn test_hip_init() {
    let error = unsafe { hipInit(0) };
    assert_eq!(error, hipError_t::hipSuccess);
}

#[cfg(feature = "blas")]
mod blas_tests {
    use hip_sys::hipblas::{hipblasCreate, hipblasDestroy, hipblasHandle_t, hipblasStatus_t};

    #[test]
    fn test_hipblas_create() {
        let mut handle: hipblasHandle_t = std::ptr::null_mut();
        let status = unsafe { hipblasCreate(&mut handle as *mut hipblasHandle_t) };
        assert_eq!(status, hipblasStatus_t::HIPBLAS_STATUS_SUCCESS);
        let status = unsafe { hipblasDestroy(handle) };
        assert_eq!(status, hipblasStatus_t::HIPBLAS_STATUS_SUCCESS);
    }
}

#[cfg(feature = "blaslt")]
mod blaslt_tests {
    use hip_sys::hipblaslt::{hipblasLtCreate, hipblasLtDestroy, hipblasLtHandle_t, hipblasStatus_t};

    #[test]
    fn test_hipblaslt_create() {
        let mut handle: hipblasLtHandle_t = std::ptr::null_mut();
        let status = unsafe { hipblasLtCreate(&mut handle as *mut hipblasLtHandle_t) };
        assert_eq!(status, hipblasStatus_t::HIPBLAS_STATUS_SUCCESS);
        
        // Only destroy if creation was successful
        if !handle.is_null() {
            let status = unsafe { hipblasLtDestroy(handle) };
            assert_eq!(status, hipblasStatus_t::HIPBLAS_STATUS_SUCCESS);
        }
    }
}