# ICPE: Infinite Context Paging Engine 🧠

An ultra-low latency, zero-copy context virtual memory paging engine written in Rust, designed to break physical VRAM limitations for LLMs and Long-Lived Autonomous Agents.

## 🔬 The Core Innovation

ICPE treats LLM token context layers exactly like operating system virtual memory (Paging/Swap). Instead of holding massive, low-activation histories in expensive GPU VRAM, ICPE utilizes an **Attention-Driven Predictive Eviction** algorithm to page out cold contexts to disk via memory-mapped files (mmap), prefetching hot slices back into high-speed memory nanoseconds before the next inference step.

------------------------------------------------------------------------

## 🛡️ Binary Security, Architecture & Compliance

This project is written **100% in Rust**. To protect our core intellectual property and proprietary algorithms during public evaluation, the high-performance predictive engine and thread synchronization heuristics are distributed as pre-compiled, highly optimized Rust binary blobs (`.a` / `.so`) located in the `/lib` directory. 

* **Target Architectures Provided:** `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` (ARM64/Graviton compliant).
* **Compliance:** All binaries are cryptographically signed and built via public, isolated GitHub Actions workflows. SHA-256 hashes are verified at runtime. The core contains 0% external networking, 0% telemetry, and operates strictly within local system memory bounds.
* **What is Up for Acquisition:** Full clean-room source code of the core engine, mathematical specifications, compilation toolchains, and global IP ownership are strictly reserved for total acquisition.

------------------------------------------------------------------------

## 📊 Verifiable Benchmarks (Criterion Release Mode)

ICPE eliminates standard I/O syscall overhead by mapping the execution engine directly into the kernel page cache space using `memmap2` and `zerocopy`.

* **Prefetch & Eviction Latency:** ~419.34 µs (Microseconds) under continuous concurrent thread stress, crossing the FFI boundary safely into the protected core.
* **Memory Copy Overhead:** 0% (True Zero-Copy byte casting).
* **RAM Footprint:** Deterministic, fixed, and completely bounded.

------------------------------------------------------------------------

## ⚖️ Evaluation License

This public repository operates under a strict Open-Core Evaluation License. The architecture, Python wrappers, and benchmarking test-suites are fully open and verifiable. You are free to natively compile, benchmark, and run integration tests locally. Commercial use, production deployment, or cloud infrastructure embedding of the pre-compiled core without an Enterprise License or total IP Acquisition is strictly prohibited.

------------------------------------------------------------------------

## 🚀 How to Run and Verify Performance

You can natively compile the project and audit the benchmarking claims directly on your local infrastructure.

### 1. Requirements (Linux)

Ensure you have the Rust toolchain, Python 3.12 development headers, and the native linker installed on your machine:

```bash
sudo apt update
sudo apt install build-essential python3-dev python3-config lld
```

### 2. Verify Local Benchmarks (Criterion)

The micro-benchmarking suite is isolated within the core source files to prevent Python runtime context symbol collisions. To run the statistical hardware latency reports, execute:

```bash
# Clear any stale linker metadata
cargo clean

# Run the target context manager benchmark suite
cargo bench --bench context_manager_bench
```

The detailed statistical distribution curves will be generated under target/criterion/report/index.html.

### 3. Test the Python Extension Module

Build the native extension into your local Python environment and execute the integration pipeline test:

```bash
# Activate your local virtual environment
source .venv/bin/activate

# Install the compilation wrapper
pip install maturin

# Compile the project using the pre-compiled high-performance core
maturin develop --release

# Run the live agent context swapping simulation
python3 test_engine.py
```

### 💼 Corporate Development & M&A (Mergers and Acquisitions)

This technology is architected by senior systems engineers and is optimized for direct cloud infrastructure integration (e.g., AWS Bedrock, Google Vertex AI, Meta GenAI cluster nodes).

To protect deep engineering focus, we operate a strictly asynchronous, documentation-first M&A pipeline. We do not participate in introductory discovery calls, introductory Zoom sessions, or loose technical alignment meetings.

The software speaks for itself. If your engineering team has executed cargo bench, audited the micro-benchmarks, and verified that ICPE solves your VRAM/latency bottleneck, the acquisition workflow is as follows:

1. Submission: Your Corporate Development / M&A team submits a formal, written Letter of Intent (LOI) or a firm acquisition proposal from an official corporate domain directly to our secure channel.

2. Due Diligence: Upon receiving a valid LOI, we will instantly grant access to a secure data room containing clean-room source code, mathematical specifications, comprehensive fuzzing reports, and automated hardware stress tests under a standard NDA.

3. Closing: Fully automated IP transfer and legal closing.

📩 Official Channel for LOIs & Firm Proposals: corpdev@matheusdelgado.com

