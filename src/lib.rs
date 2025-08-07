#[cfg(feature = "blas")]
pub mod hipblas {
    #[doc(inline)]
    pub use hipblas_sys::*;
}

#[cfg(feature = "blaslt")]
pub mod hipblaslt {
    #[doc(inline)]
    pub use hipblaslt_sys::*;
}

pub mod hiprt {
    #[doc(inline)]
    pub use hip_runtime_sys::*;
}