use cupti_binding::{cuInit, cudaDeviceSynchronize, cudaGetDeviceCount, cudaError_t, cudaError_cudaSuccess, CUresult, CUdevice};

fn assert_cu_success(result_code: CUresult) {
    assert!(result_code == cudaError_cudaSuccess);
}

fn assert_cuda_success(return_code: cudaError_t) {
    assert!(return_code == cudaError_cudaSuccess);
}

fn main() {
    unsafe {
        assert_cu_success(cuInit(0));

        let mut count:i32 = -1;
        assert_cuda_success(cudaGetDeviceCount(&mut count));
        println!("device count {}", count);

        cudaDeviceSynchronize();
    }
}
