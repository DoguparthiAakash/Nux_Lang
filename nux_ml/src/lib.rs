#[no_mangle]
pub extern "C" fn tensor_new(size: i64) -> i64 {
    let vec: Vec<f32> = vec![0.0; size as usize];
    Box::into_raw(Box::new(vec)) as i64
}

#[no_mangle]
pub extern "C" fn tensor_free(ptr: i64) -> i64 {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut Vec<f32>);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn tensor_set(ptr: i64, index: i64, val: i64) -> i64 {
    // Note: Since Nux currently only sends i64 via FFI easily,
    // we assume val is actually the bit pattern of an f32, or just an integer
    // which we cast to f32. Let's just cast for now to test it.
    if ptr != 0 {
        let vec = unsafe { &mut *(ptr as *mut Vec<f32>) };
        if index >= 0 && index < vec.len() as i64 {
            vec[index as usize] = val as f32;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn tensor_get(ptr: i64, index: i64) -> i64 {
    if ptr != 0 {
        let vec = unsafe { &*(ptr as *mut Vec<f32>) };
        if index >= 0 && index < vec.len() as i64 {
            return vec[index as usize] as i64;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn matmul(a_ptr: i64, b_ptr: i64, c_ptr: i64, m: i64, k: i64, n: i64) -> i64 {
    if a_ptr == 0 || b_ptr == 0 || c_ptr == 0 { return -1; }
    
    let a = unsafe { &*(a_ptr as *mut Vec<f32>) };
    let b = unsafe { &*(b_ptr as *mut Vec<f32>) };
    let c = unsafe { &mut *(c_ptr as *mut Vec<f32>) };
    
    // Multi-threaded AMD/Intel CPU matmul (A: m x k, B: k x n, C: m x n)
    // We use std::thread::scope to spawn workers and compute rows in parallel chunks.
    let num_threads = 4;
    std::thread::scope(|s| {
        let chunk_size = (m as usize / num_threads).max(1);
        for chunk_idx in 0..num_threads {
            let start = chunk_idx * chunk_size;
            let end = if chunk_idx == num_threads - 1 { m as usize } else { start + chunk_size };
            if start >= m as usize { break; }
            
            // Unsafe pointer sharing for threads
            let c_ptr_raw = c.as_mut_ptr() as usize;
            let a_ptr_raw = a.as_ptr() as usize;
            let b_ptr_raw = b.as_ptr() as usize;
            
            s.spawn(move || {
                let local_a = unsafe { std::slice::from_raw_parts(a_ptr_raw as *const f32, (m * k) as usize) };
                let local_b = unsafe { std::slice::from_raw_parts(b_ptr_raw as *const f32, (k * n) as usize) };
                let local_c = unsafe { std::slice::from_raw_parts_mut(c_ptr_raw as *mut f32, (m * n) as usize) };
                
                for i in start..end {
                    for j in 0..(n as usize) {
                        let mut sum = 0.0;
                        for p in 0..(k as usize) {
                            sum += local_a[i * (k as usize) + p] * local_b[p * (n as usize) + j];
                        }
                        local_c[i * (n as usize) + j] = sum;
                    }
                }
            });
        }
    });
    
    0
}

// Basic ReLU activation
#[no_mangle]
pub extern "C" fn relu(ptr: i64) -> i64 {
    if ptr != 0 {
        let vec = unsafe { &mut *(ptr as *mut Vec<f32>) };
        for val in vec.iter_mut() {
            if *val < 0.0 {
                *val = 0.0;
            }
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn rmsnorm(ptr: i64, eps: f32) -> i64 {
    if ptr != 0 {
        let vec = unsafe { &mut *(ptr as *mut Vec<f32>) };
        let size = vec.len() as f32;
        let mut sum = 0.0;
        for &val in vec.iter() {
            sum += val * val;
        }
        let rms = ((sum / size) + eps).sqrt();
        for val in vec.iter_mut() {
            *val = *val / rms;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn embedding(input_ptr: i64, weight_ptr: i64, output_ptr: i64, embedding_dim: i64, seq_len: i64) -> i64 {
    if input_ptr == 0 || weight_ptr == 0 || output_ptr == 0 { return -1; }
    
    // input is a vector of tokens (indices), lengths = seq_len
    // weight is a flat vector (num_embeddings * embedding_dim)
    // output is a flat vector (seq_len * embedding_dim)
    
    let input = unsafe { &*(input_ptr as *mut Vec<f32>) };
    let weight = unsafe { &*(weight_ptr as *mut Vec<f32>) };
    let output = unsafe { &mut *(output_ptr as *mut Vec<f32>) };
    
    for i in 0..(seq_len as usize) {
        let token_idx = input[i] as usize;
        for d in 0..(embedding_dim as usize) {
            output[i * (embedding_dim as usize) + d] = weight[token_idx * (embedding_dim as usize) + d];
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn softmax(ptr: i64) -> i64 {
    if ptr != 0 {
        let vec = unsafe { &mut *(ptr as *mut Vec<f32>) };
        if vec.is_empty() { return 0; }
        
        // Find max for numerical stability
        let mut max_val = vec[0];
        for &val in vec.iter() {
            if val > max_val { max_val = val; }
        }
        
        let mut sum = 0.0;
        for val in vec.iter_mut() {
            *val = (*val - max_val).exp();
            sum += *val;
        }
        
        for val in vec.iter_mut() {
            *val = *val / sum;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn ml_get_device_info() -> i64 {
    // Return an integer representing the available hardware backends
    // 0: CPU only
    // 1: CPU + CUDA
    // 2: CPU + Metal
    // 3: CPU + TPU
    
    #[cfg(feature = "cuda")]
    return 1;
    
    #[cfg(feature = "metal")]
    return 2;
    
    #[cfg(feature = "tpu")]
    return 3;
    
    #[cfg(not(any(feature = "cuda", feature = "metal", feature = "tpu")))]
    return 0;
}

#[no_mangle]
pub extern "C" fn ml_set_device(device_type: i64) -> i64 {
    // Return 1 if success, 0 if unsupported
    let info = ml_get_device_info();
    if device_type == 0 {
        return 1; // CPU always supported
    }
    if device_type == info {
        return 1; // Requested device is the compiled hardware backend
    }
    0
}

#[no_mangle]
pub extern "C" fn tensor_add(a_ptr: i64, b_ptr: i64, c_ptr: i64) -> i64 {
    if a_ptr == 0 || b_ptr == 0 || c_ptr == 0 { return -1; }
    
    let a = unsafe { &*(a_ptr as *mut Vec<f32>) };
    let b = unsafe { &*(b_ptr as *mut Vec<f32>) };
    let c = unsafe { &mut *(c_ptr as *mut Vec<f32>) };
    
    let len = a.len().min(b.len()).min(c.len());
    for i in 0..len {
        c[i] = a[i] + b[i];
    }
    0
}

#[no_mangle]
pub extern "C" fn tensor_scale(ptr: i64, factor_bits: i64) -> i64 {
    if ptr != 0 {
        // Nux passes integers, so we assume factor_bits is an f32 bitcast to i64 (or we just accept an integer and cast to f32)
        // Let's assume the user passes a scaled integer (e.g. factor * 1000) or we just bitcast.
        // For simplicity, let's just cast the i64 to f32. Wait, Nux FFI passes `i64`.
        // If Nux passes an integer, it'll just be `factor_bits as f32`.
        // Wait, for scaling like 1/sqrt(dk) we need float. 
        // In Nux, variables are stored as `i64`. If it's a float, the bit pattern is an `f64`.
        // Let's interpret it as `f64` using `f64::from_bits(factor_bits as u64)`.
        let factor = f64::from_bits(factor_bits as u64) as f32;
        let vec = unsafe { &mut *(ptr as *mut Vec<f32>) };
        for val in vec.iter_mut() {
            *val *= factor;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn tensor_copy(src_ptr: i64, dest_ptr: i64) -> i64 {
    if src_ptr == 0 || dest_ptr == 0 { return -1; }
    let src = unsafe { &*(src_ptr as *mut Vec<f32>) };
    let dest = unsafe { &mut *(dest_ptr as *mut Vec<f32>) };
    
    let len = src.len().min(dest.len());
    dest[..len].copy_from_slice(&src[..len]);
    0
}

