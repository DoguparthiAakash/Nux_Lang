import re

with open('src/vm.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# Add num-complex and rand
code = code.replace('use std::collections::{HashMap, BTreeMap};', 'use std::collections::{HashMap, BTreeMap};\nuse rand::Rng;\nuse num_complex::Complex64;')

# Add q_state to struct Vm
code = code.replace('pub shared: Arc<RwLock<SharedVmState>>,\n}', 'pub shared: Arc<RwLock<SharedVmState>>,\n    pub q_state: Vec<Complex64>,\n}')

# Add q_state init to Vm::new
code = code.replace('shared: shared.clone(),\n        }', 'shared: shared.clone(),\n            q_state: Vec::new(),\n        }')

q_ops = """
                    0xE1 => { // OP_Q_ALLOC
                        let num_qubits = self.stack.pop().unwrap_or(1) as usize;
                        let size = 1 << num_qubits;
                        self.q_state = vec![Complex64::new(0.0, 0.0); size];
                        if size > 0 { self.q_state[0] = Complex64::new(1.0, 0.0); }
                    }
                    0xE2 => { // OP_Q_H
                        let target = self.stack.pop().unwrap_or(0) as usize;
                        let size = self.q_state.len();
                        let mut new_state = vec![Complex64::new(0.0, 0.0); size];
                        let inv_sqrt2 = 1.0 / std::f64::consts::SQRT_2;
                        for i in 0..size {
                            let bit = (i >> target) & 1;
                            let partner = i ^ (1 << target);
                            let val = self.q_state[i];
                            if bit == 0 {
                                new_state[i] = new_state[i] + val * inv_sqrt2;
                                new_state[partner] = new_state[partner] + val * inv_sqrt2;
                            } else {
                                new_state[i] = new_state[i] - val * inv_sqrt2;
                                new_state[partner] = new_state[partner] + val * inv_sqrt2;
                            }
                        }
                        self.q_state = new_state;
                    }
                    0xE3 => { // OP_Q_X
                        let target = self.stack.pop().unwrap_or(0) as usize;
                        let size = self.q_state.len();
                        let mut new_state = vec![Complex64::new(0.0, 0.0); size];
                        for i in 0..size {
                            let partner = i ^ (1 << target);
                            new_state[partner] = self.q_state[i];
                        }
                        self.q_state = new_state;
                    }
                    0xE4 => { // OP_Q_Z
                        let target = self.stack.pop().unwrap_or(0) as usize;
                        for i in 0..self.q_state.len() {
                            if ((i >> target) & 1) == 1 {
                                self.q_state[i] = -self.q_state[i];
                            }
                        }
                    }
                    0xE5 => { // OP_Q_CX
                        let target = self.stack.pop().unwrap_or(0) as usize;
                        let control = self.stack.pop().unwrap_or(0) as usize;
                        let size = self.q_state.len();
                        let mut new_state = self.q_state.clone();
                        for i in 0..size {
                            if ((i >> control) & 1) == 1 {
                                let partner = i ^ (1 << target);
                                new_state[partner] = self.q_state[i];
                            }
                        }
                        self.q_state = new_state;
                    }
                    0xE6 => { // OP_Q_MEASURE
                        let target = self.stack.pop().unwrap_or(0) as usize;
                        let size = self.q_state.len();
                        let mut prob_0 = 0.0;
                        for i in 0..size {
                            if ((i >> target) & 1) == 0 {
                                prob_0 += self.q_state[i].norm_sqr();
                            }
                        }
                        let mut rng = rand::thread_rng();
                        let r: f64 = rng.gen();
                        let result = if r < prob_0 { 0 } else { 1 };
                        let norm = if result == 0 { prob_0.sqrt() } else { (1.0 - prob_0).sqrt() };
                        for i in 0..size {
                            if ((i >> target) & 1) == result {
                                self.q_state[i] = self.q_state[i] / norm;
                            } else {
                                self.q_state[i] = Complex64::new(0.0, 0.0);
                            }
                        }
                        self.stack.push(result as i64);
                    }
"""

code = code.replace('0xE0 => { // OP_SPAWN', q_ops + '                    0xE0 => { // OP_SPAWN')

with open('src/vm.rs', 'w', encoding='utf-8') as f:
    f.write(code)
print('VM patched.')
