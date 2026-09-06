import os

path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\vm.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

# 1. Add quantum_state to SharedVmState
if "pub quantum_state: RwLock<Vec<Complex64>>" not in content:
    content = content.replace(
        "pub heap_arrays: RwLock<Vec<Vec<i64>>>,",
        "pub heap_arrays: RwLock<Vec<Vec<i64>>>,\n    pub quantum_state: RwLock<Vec<Complex64>>,"
    )

# 2. Add quantum_state initialization in NuxVm::new
if "quantum_state: RwLock::new(Vec::new())," not in content:
    content = content.replace(
        "heap_arrays: RwLock::new(Vec::new()),",
        "heap_arrays: RwLock::new(Vec::new()),\n                quantum_state: RwLock::new(Vec::new()),"
    )

# 3. Add OP_Q_* execution in run() loop
quantum_ops = """
                0xEA => { // OP_Q_ALLOC
                    let size = self.stack.pop().unwrap() as usize;
                    let num_amplitudes = 1 << size;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    q_state.clear();
                    q_state.resize(num_amplitudes, Complex64::new(0.0, 0.0));
                    q_state[0] = Complex64::new(1.0, 0.0);
                },
                0xEB => { // OP_Q_H
                    let target = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let bit = 1 << target;
                    let inv_sqrt2 = 1.0 / std::f64::consts::SQRT_2;
                    for i in 0..n {
                        if (i & bit) == 0 {
                            let a = q_state[i];
                            let b = q_state[i | bit];
                            q_state[i] = (a + b) * inv_sqrt2;
                            q_state[i | bit] = (a - b) * inv_sqrt2;
                        }
                    }
                },
                0xEC => { // OP_Q_X
                    let target = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let bit = 1 << target;
                    for i in 0..n {
                        if (i & bit) == 0 {
                            let temp = q_state[i];
                            q_state[i] = q_state[i | bit];
                            q_state[i | bit] = temp;
                        }
                    }
                },
                0xED => { // OP_Q_Z
                    let target = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let bit = 1 << target;
                    for i in 0..n {
                        if (i & bit) != 0 {
                            q_state[i] = q_state[i] * -1.0;
                        }
                    }
                },
                0xEE => { // OP_Q_CX
                    let target = self.stack.pop().unwrap() as usize;
                    let control = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let cbit = 1 << control;
                    let tbit = 1 << target;
                    for i in 0..n {
                        if (i & cbit) != 0 && (i & tbit) == 0 {
                            let temp = q_state[i];
                            q_state[i] = q_state[i | tbit];
                            q_state[i | tbit] = temp;
                        }
                    }
                },
                0xEF => { // OP_Q_MEASURE
                    let target = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let bit = 1 << target;
                    let mut prob_zero = 0.0;
                    for i in 0..n {
                        if (i & bit) == 0 {
                            prob_zero += q_state[i].norm_sqr();
                        }
                    }
                    
                    let mut rng = rand::thread_rng();
                    let random_val: f64 = rng.gen();
                    let outcome = if random_val < prob_zero { 0 } else { 1 };
                    
                    let norm = if outcome == 0 { prob_zero.sqrt() } else { (1.0 - prob_zero).sqrt() };
                    
                    for i in 0..n {
                        if ((i & bit) == 0 && outcome == 1) || ((i & bit) != 0 && outcome == 0) {
                            q_state[i] = Complex64::new(0.0, 0.0);
                        } else {
                            q_state[i] = q_state[i] / norm;
                        }
                    }
                    self.stack.push(outcome as i64);
                },
"""

if "0xEA => { // OP_Q_ALLOC" not in content:
    content = content.replace("0xE7 => { // OP_FFI_PYTHON", quantum_ops + "\n                0xE7 => { // OP_FFI_PYTHON")

with open(path, "w", encoding="utf-8") as f:
    f.write(content)

print("Patched vm.rs with Quantum operations!")
