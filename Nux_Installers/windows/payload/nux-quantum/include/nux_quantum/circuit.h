#ifndef NUX_QUANTUM_CIRCUIT_H
#define NUX_QUANTUM_CIRCUIT_H

#include <vector>
#include <complex>
#include <string>
#include <memory>

namespace NuxQuantum {

using Complex = std::complex<double>;

// Quantum state representation
class QuantumState {
public:
    QuantumState(int numQubits);
    
    int NumQubits() const { return m_NumQubits; }
    const std::vector<Complex>& Amplitudes() const { return m_Amplitudes; }
    
    // Measurement
    int Measure(int qubit);
    std::vector<int> MeasureAll();
    
    // Probability
    double Probability(int state) const;
    
    void Print() const;
    
private:
    int m_NumQubits;
    std::vector<Complex> m_Amplitudes;
    
    friend class QuantumGate;
};

// Quantum gates
class QuantumGate {
public:
    virtual ~QuantumGate() = default;
    virtual void Apply(QuantumState& state, const std::vector<int>& qubits) = 0;
    virtual std::string Name() const = 0;
};

// Single-qubit gates
class HadamardGate : public QuantumGate {
public:
    void Apply(QuantumState& state, const std::vector<int>& qubits) override;
    std::string Name() const override { return "H"; }
};

class PauliXGate : public QuantumGate {
public:
    void Apply(QuantumState& state, const std::vector<int>& qubits) override;
    std::string Name() const override { return "X"; }
};

class PauliYGate : public QuantumGate {
public:
    void Apply(QuantumState& state, const std::vector<int>& qubits) override;
    std::string Name() const override { return "Y"; }
};

class PauliZGate : public QuantumGate {
public:
    void Apply(QuantumState& state, const std::vector<int>& qubits) override;
    std::string Name() const override { return "Z"; }
};

class PhaseGate : public QuantumGate {
public:
    PhaseGate(double theta);
    void Apply(QuantumState& state, const std::vector<int>& qubits) override;
    std::string Name() const override { return "P"; }
    
private:
    double m_Theta;
};

// Two-qubit gates
class CNOTGate : public QuantumGate {
public:
    void Apply(QuantumState& state, const std::vector<int>& qubits) override;
    std::string Name() const override { return "CNOT"; }
};

class SWAPGate : public QuantumGate {
public:
    void Apply(QuantumState& state, const std::vector<int>& qubits) override;
    std::string Name() const override { return "SWAP"; }
};

// Quantum circuit
class QuantumCircuit {
public:
    QuantumCircuit(int numQubits);
    ~QuantumCircuit();
    
    // Add gates
    void H(int qubit);  // Hadamard
    void X(int qubit);  // Pauli-X (NOT)
    void Y(int qubit);  // Pauli-Y
    void Z(int qubit);  // Pauli-Z
    void P(int qubit, double theta);  // Phase
    void CNOT(int control, int target);
    void SWAP(int qubit1, int qubit2);
    
    // Execute circuit
    QuantumState Execute();
    
    // Circuit info
    int NumQubits() const { return m_NumQubits; }
    int NumGates() const { return m_Gates.size(); }
    void Print() const;
    
private:
    int m_NumQubits;
    std::vector<std::shared_ptr<QuantumGate>> m_Gates;
    std::vector<std::vector<int>> m_GateQubits;
};

// Quantum algorithms
class QuantumAlgorithms {
public:
    // Grover's search
    static QuantumState GroverSearch(int numQubits, 
                                     std::function<bool(int)> oracle);
    
    // Quantum Fourier Transform
    static QuantumCircuit QFT(int numQubits);
    
    // Shor's algorithm (simplified)
    static int Factorize(int N);
};

} // namespace NuxQuantum

#endif // NUX_QUANTUM_CIRCUIT_H
