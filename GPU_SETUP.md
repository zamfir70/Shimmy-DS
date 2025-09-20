# Shimmy GPU Setup Guide

This guide will help you enable CUDA GPU support for your Shimmy fork.

## Prerequisites

### 1. NVIDIA GPU
- CUDA-compatible NVIDIA GPU
- At least 4GB VRAM (8GB+ recommended)
- Compute Capability 3.5 or higher

### 2. NVIDIA Drivers
- Latest NVIDIA drivers (minimum 530.30.02 for CUDA 12.x)
- Download from: https://www.nvidia.com/drivers/

### 3. CUDA Toolkit
- CUDA Toolkit 12.1 or higher (12.4+ recommended)
- Download from: https://developer.nvidia.com/cuda-downloads
- **Important**: Install with Visual Studio Integration enabled

### 4. Visual Studio Build Tools
- Visual Studio 2019 or 2022 Build Tools
- C++ build tools with CMake support
- Download from: https://visualstudio.microsoft.com/downloads/

## Quick Setup

### Step 1: Check Requirements
Run the CUDA check script:
```powershell
.\check_cuda.bat
```

### Step 2: Build with CUDA Support
Run the CUDA build script:
```powershell
# PowerShell (Recommended)
.\build_cuda.ps1

# Or Command Prompt
.\build_cuda.bat
```

### Step 3: Test CUDA Support
```powershell
.\test_cuda.ps1
```

## Troubleshooting

### Build Issues

**Error: "nvcc not found"**
- Install CUDA Toolkit
- Add CUDA bin directory to PATH
- Set CUDACXX environment variable

**Error: "Visual Studio not found"**
- Install Visual Studio Build Tools
- Ensure C++ build tools are installed

**Error: "CUDA compilation failed"**
- Check GPU compute capability
- Update NVIDIA drivers

### Runtime Issues

**No GPU acceleration**
- Check if model supports GPU offloading
- Verify CUDA layers parameter: `--n-gpu-layers N`
- Monitor with `nvidia-smi` during inference

## Verification

### Check CUDA Support
1. Start server: `.\target\release\shimmy.exe serve`
2. Look for CUDA initialization messages in logs
3. Load a model and check GPU memory usage with `nvidia-smi`
