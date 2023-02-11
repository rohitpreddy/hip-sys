# hip-sys
Rust bindings for HIP

# Dependencies

  - A ROCm platform ie a compatible AMD GPU
    * `hipblas` can be accessed with the `blas` feature
    * Specify the path to `hip` with `HIP_PATH`. If not provided,
      `/opt/rocm/hip` is assumed
    * Specify the path to `hipblas` with `HIP_BLAS_PATH`. If not provided,
      `/opt/rocm/hipblas` is assumed
    * Native bindings can be generated with the `bindgen` feature
    * Will compile without an AMD GPU, but device functions will fail
    * Currently CUDA platform support is not available (see cuda-sys https://github.com/rust-cuda/cuda-sys)
  - Install ROCm (see https://rocmdocs.amd.com/en/latest/Installation_Guide/Installation-Guide.html#deploying-rocm)
    * Works with the hcc backend `sudo apt install hip-hcc` (from ROCm package registry) 
  - You may need to install rocsolver as well `sudo apt install rocsolver`

If you have any problems please post an issue.
  
# Tests

Run the tests with:
```
cargo test 
```

# Docs

Open the documentation with:
```
cargo doc --open
```


