# Claude Code Setup Guide for Shimmy CUDA 13.0

## Project Overview
This is a Rust-based fork of Shimmy with enhanced narrative intelligence and CUDA 13.0 GPU acceleration support.

## Directory Structure for Claude Code Analysis
```
D:\shimmy-DS\
├── Cargo.toml                 # Main project configuration
├── .cargo\config.toml         # Build configuration (CUDA enabled)
├── build.rs                   # Custom build script for llama.cpp
├── src\                       # Source code
│   ├── main.rs               # Entry point
│   ├── lib.rs                # Library root
│   ├── engine\               # Model inference engines
│   └── shimmy_config.rs      # Runtime configuration
├── build_cuda.ps1            # CUDA 13.0 build script
├── test_cuda.ps1             # GPU verification script
└── GPU_SETUP.md              # Setup documentation
```

## Key Configuration Changes Made
1. **CUDA Support Enabled**: `.cargo\config.toml` has `LLAMA_CUDA = "ON"`
2. **Modern GGML**: Added `GGML_CUDA = "ON"` for latest llama.cpp
3. **CUDA 13.0 Support**: Build scripts detect v13.0 installation
4. **Architecture Targeting**: `CMAKE_CUDA_ARCHITECTURES = "all-major"`

## Build Environment
- **CUDA Version**: 13.0 (latest)
- **Target Platform**: Windows x64
- **Compiler**: MSVC with CUDA nvcc
- **Features**: `llama` feature for GPU acceleration

## Key Files for Understanding

### Primary Config (Read These First)
1. `Cargo.toml` - Project dependencies and features
2. `.cargo\config.toml` - CUDA build configuration
3. `build.rs` - Custom llama.cpp build logic

### Build Scripts (CUDA 13.0 Ready)
4. `build_cuda.ps1` - Main build script with CUDA 13.0 support
5. `test_cuda.ps1` - Verification and testing

### Documentation
6. `README.md` - Project overview and narrative intelligence features
7. `GPU_SETUP.md` - CUDA setup guide
8. `CUDA_SETUP_COMPLETE.md` - Setup completion summary

## Expected Build Process
1. Detect CUDA 13.0 in `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0`
2. Set environment variables (CUDA_PATH, CUDACXX, CMAKE_ARGS)
3. Build llama-cpp-2 dependency with CUDA support
4. Compile Shimmy with GPU acceleration enabled

## GPU Features Available
- Model layer offloading to GPU with `--n-gpu-layers`
- CUDA kernel optimization for inference
- Mixed CPU/GPU memory usage
- Real-time performance monitoring

## Critical Environment Variables
```
CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0
CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0\bin\nvcc.exe
CMAKE_ARGS=-DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES=all-major
LLAMA_CUDA=ON
GGML_CUDA=ON
```

## Success Indicators
- Build completes without CUDA-related errors
- `shimmy.exe --version` shows version info
- GPU memory usage visible in nvidia-smi during inference
- CUDA initialization messages in server logs

## Next Steps for Claude Code
1. Read the configuration files to understand current setup
2. Analyze build scripts for CUDA 13.0 compatibility
3. Examine source code for GPU integration points
4. Help optimize or troubleshoot any build issues
5. Assist with performance tuning for specific hardware

## Common Issues to Watch For
- CUDA_PATH environment variable detection
- Visual Studio Build Tools compatibility
- GPU compute capability detection
- Memory allocation and layer offloading optimization

This setup is optimized for CUDA 13.0 with broad GPU compatibility and should provide significant performance improvements for model inference.
