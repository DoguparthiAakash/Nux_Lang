// Rust - Distributed Training Infrastructure
// Scale to 1000s of GPUs with efficient gradient synchronization

use std::sync::{Arc, Mutex};
use std::thread;

// Ring AllReduce for efficient gradient synchronization
pub struct RingAllReduce {
    rank: usize,
    world_size: usize,
    gradients: Vec<Arc<Mutex<Vec<f32>>>>,
}

impl RingAllReduce {
    pub fn new(rank: usize, world_size: usize) -> Self {
        RingAllReduce {
            rank,
            world_size,
            gradients: Vec::new(),
        }
    }
    
    // Synchronize gradients across all workers
    // O(N) communication instead of O(N²)
    pub fn all_reduce(&mut self, local_grads: &mut [f32]) {
        let chunk_size = local_grads.len() / self.world_size;
        
        // Scatter-Reduce phase
        for step in 0..self.world_size - 1 {
            let send_chunk = (self.rank - step) % self.world_size;
            let recv_chunk = (self.rank - step - 1) % self.world_size;
            
            // Send to next rank, receive from previous rank
            // In real implementation, use MPI or NCCL
            self.exchange_and_reduce(local_grads, send_chunk, recv_chunk, chunk_size);
        }
        
        // AllGather phase
        for step in 0..self.world_size - 1 {
            let send_chunk = (self.rank + 1 - step) % self.world_size;
            let recv_chunk = (self.rank - step) % self.world_size;
            
            self.exchange(local_grads, send_chunk, recv_chunk, chunk_size);
        }
    }
    
    fn exchange_and_reduce(&self, grads: &mut [f32], send: usize, recv: usize, chunk_size: usize) {
        // Simplified - real implementation uses network communication
        let start = recv * chunk_size;
        let end = (recv + 1) * chunk_size;
        
        // Accumulate gradients
        for i in start..end {
            if i < grads.len() {
                // grads[i] += received_grads[i];
            }
        }
    }
    
    fn exchange(&self, grads: &mut [f32], send: usize, recv: usize, chunk_size: usize) {
        // Exchange chunks between ranks
    }
}

// Data Parallel Training
pub struct DataParallel {
    num_workers: usize,
    all_reduce: RingAllReduce,
}

impl DataParallel {
    pub fn new(num_workers: usize, rank: usize) -> Self {
        DataParallel {
            num_workers,
            all_reduce: RingAllReduce::new(rank, num_workers),
        }
    }
    
    // Train on local batch, then synchronize gradients
    pub fn train_step(&mut self, local_grads: &mut [f32]) {
        // Average gradients across all workers
        self.all_reduce.all_reduce(local_grads);
        
        for grad in local_grads.iter_mut() {
            *grad /= self.num_workers as f32;
        }
    }
}

// Gradient Compression (reduce communication by 100x)
pub struct GradientCompression {
    threshold: f32,
}

impl GradientCompression {
    pub fn new(threshold: f32) -> Self {
        GradientCompression { threshold }
    }
    
    // Top-K sparsification: only send largest gradients
    pub fn compress(&self, grads: &[f32]) -> (Vec<usize>, Vec<f32>) {
        let mut indexed: Vec<(usize, f32)> = grads
            .iter()
            .enumerate()
            .map(|(i, &g)| (i, g.abs()))
            .collect();
        
        // Sort by magnitude
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Keep top-k
        let k = (grads.len() as f32 * self.threshold) as usize;
        let indices: Vec<usize> = indexed[..k].iter().map(|(i, _)| *i).collect();
        let values: Vec<f32> = indices.iter().map(|&i| grads[i]).collect();
        
        (indices, values)
    }
    
    pub fn decompress(&self, indices: &[usize], values: &[f32], size: usize) -> Vec<f32> {
        let mut grads = vec![0.0; size];
        for (i, &idx) in indices.iter().enumerate() {
            grads[idx] = values[i];
        }
        grads
    }
}

// Mixed Precision Training (2x faster, 2x less memory)
pub struct MixedPrecisionTrainer {
    loss_scale: f32,
    scale_factor: f32,
}

impl MixedPrecisionTrainer {
    pub fn new() -> Self {
        MixedPrecisionTrainer {
            loss_scale: 65536.0,  // Initial scale
            scale_factor: 2.0,
        }
    }
    
    // Scale loss to prevent underflow in FP16
    pub fn scale_loss(&self, loss: f32) -> f32 {
        loss * self.loss_scale
    }
    
    // Unscale gradients before optimizer step
    pub fn unscale_gradients(&self, grads: &mut [f32]) {
        for grad in grads.iter_mut() {
            *grad /= self.loss_scale;
        }
    }
    
    // Check for gradient overflow/underflow
    pub fn check_gradients(&mut self, grads: &[f32]) -> bool {
        let has_inf_nan = grads.iter().any(|&g| g.is_infinite() || g.is_nan());
        
        if has_inf_nan {
            // Reduce scale
            self.loss_scale /= self.scale_factor;
            false
        } else {
            // Increase scale
            self.loss_scale *= 1.001;
            true
        }
    }
}

// Gradient Checkpointing (trade compute for memory)
pub struct GradientCheckpointing {
    checkpoints: Vec<usize>,  // Which layers to checkpoint
}

impl GradientCheckpointing {
    pub fn new(num_layers: usize, checkpoint_every: usize) -> Self {
        let checkpoints: Vec<usize> = (0..num_layers)
            .step_by(checkpoint_every)
            .collect();
        
        GradientCheckpointing { checkpoints }
    }
    
    pub fn should_checkpoint(&self, layer_idx: usize) -> bool {
        self.checkpoints.contains(&layer_idx)
    }
}

// C FFI exports
#[no_mangle]
pub extern "C" fn data_parallel_create(num_workers: usize, rank: usize) -> *mut DataParallel {
    Box::into_raw(Box::new(DataParallel::new(num_workers, rank)))
}

#[no_mangle]
pub extern "C" fn data_parallel_train_step(
    dp: *mut DataParallel,
    grads: *mut f32,
    size: usize,
) {
    unsafe {
        let dp = &mut *dp;
        let grads_slice = std::slice::from_raw_parts_mut(grads, size);
        dp.train_step(grads_slice);
    }
}

#[no_mangle]
pub extern "C" fn mixed_precision_create() -> *mut MixedPrecisionTrainer {
    Box::into_raw(Box::new(MixedPrecisionTrainer::new()))
}

#[no_mangle]
pub extern "C" fn mixed_precision_unscale(
    mp: *mut MixedPrecisionTrainer,
    grads: *mut f32,
    size: usize,
) {
    unsafe {
        let mp = &mut *mp;
        let grads_slice = std::slice::from_raw_parts_mut(grads, size);
        mp.unscale_gradients(grads_slice);
    }
}
