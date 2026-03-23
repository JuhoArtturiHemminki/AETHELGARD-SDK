# AETHELGARD-SDK
> **Adaptive Non-Linear Signal Integrity Engine**

AETHELGARD-SDK is a high-performance, real-time signal processing core written in Rust. It implements a **Volterra-inspired Nonlinear LMS (NLMS)** algorithm designed to clean and predict processor-scale signals where physical saturation and non-linear distortion occur.

---

##  Core Concept
Traditional filters assume linearity. **AETHELGARD-SDK** does not. It is specifically engineered for environments where signal integrity is compromised by physical hardware limits, such as:
*   **Power Rail Transients:** Non-linear voltage droop under CPU load.
*   **RF Digital Pre-Distortion (DPD):** Correcting power amplifier saturation.
*   **Thermal Drift:** Adapting to hardware behavior changes in real-time.

---

##  Mathematical Model
The engine estimates the "clean" signal by combining a linear adaptive FIR structure with a cubic non-linear correction term:

$$y_{linear} = \sum_{i=0}^{N-1} w_i \cdot x_{t-i}$$
$$y_{out} = y_{linear} + \alpha \cdot (y_{linear})^3$$

Where:
*   $w_i$: Adaptive weight coefficients.
*   $\alpha$: Non-linear coefficient (`nl_coeff`) for modeling saturation.
*   The error $e = target - y_{out}$ is used to update weights via **Normalized Least Mean Squares (NLMS)** to ensure stability across varying signal amplitudes.

---

##  Performance Features
*   **Rust Core:** Zero-cost abstractions and memory safety.
*   **Deterministic Latency:** No heap allocations during the processing loop.
*   **Leakage Control:** Prevents coefficient drift during silent or static periods.
*   **C-ABI Compatible:** Seamless integration into C/C++ firmware or high-level drivers.

---

##  Usage

### Rust API
```rust
use aethelgard_sdk::AethelgardV2;

// Initialize: order 64, learning rate 0.05
let mut core = AethelgardV2::new(64, 0.05);

// Process: (dirty_input, clean_reference)
let (estimate, error) = core.process_sample(0.85, 0.70);
