# CUDA GPU Support Setup Complete! 🚀

## What We've Done

Your Shimmy fork has been configured for CUDA GPU support. Here's what was set up:

### 1. ✅ Fixed Cargo Configuration
- **Updated `.cargo/config.toml`** to enable CUDA compilation
- **Changed `LLAMA_CUDA`** from "OFF" to "ON"
- **Added `GGML_CUDA = "ON"`** for modern llama.cpp compatibility
- **Set `CMAKE_CUDA_ARCHITECTURES = "all-major"`** for broad GPU compatibility

### 2. ✅ Created Build Scripts
- **`build_cuda.bat`** - Windows Command Prompt build script
- **`build_cuda.ps1`** - PowerShell build script (recommended)
- **`check_cuda.bat`** - System requirements checker
- **`test_cuda.ps1`** - CUDA support verification script

### 3. ✅ Created Documentation
- **`GPU_SETUP.md`** - Comprehensive setup guide
- **This summary document** - Quick reference

## Quick Start Instructions

### Step 1: Check Your System
```cmd
D:\shimmy-DS> .\check_cuda.bat
```
This will verify:
- NVCC compiler is available
- NVIDIA drivers are installed  
- CUDA environment variables are set

### Step 2: Build with CUDA Support
```powershell
D:\shimmy-DS> .\build_cuda.ps1
```
Or if you prefer Command Prompt:
```cmd
D:\shimmy-DS> .\build_cuda.bat
```

### Step 3: Test CUDA Support
```powershell
D:\shimmy-DS> .\test_cuda.ps1
```

### Step 4: Use GPU Acceleration
```cmd
# Start server with CUDA support
D:\shimmy-DS> .\target\release\shimmy.exe serve

# Monitor GPU usage while running
nvidia-smi -l 1
```

## What GPU Support Gives You

With CUDA support enabled, Shimmy can:
- **🚀 Faster inference** - Offload model layers to GPU
- **💾 Larger models** - Use GPU VRAM for bigger models
- **⚡ Real-time generation** - Much faster text generation
- **🔧 Fine control** - Adjust GPU layer count per model

## Key Environment Variables Set

The build scripts automatically configure:
```
CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x
CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x\bin\nvcc.exe
CMAKE_ARGS=-DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES=all-major
FORCE_CMAKE=1
```

## Using GPU Layers

When running models, you can control GPU usage:
```bash
# Use all available GPU memory
.\target\release\shimmy.exe serve --n-gpu-layers -1

# Use specific number of layers (conservative)
.\target\release\shimmy.exe serve --n-gpu-layers 20

# Check what works for your GPU
nvidia-smi  # Monitor VRAM usage
```

## Troubleshooting

### If Build Fails
1. **Install CUDA Toolkit** from NVIDIA website
2. **Install Visual Studio Build Tools** with C++ support
3. **Run as Administrator** if permission issues
4. **Check PATH** includes CUDA bin directory

### If No GPU Acceleration
1. **Check model supports GPU** (GGUF format works best)
2. **Verify layers parameter** (`--n-gpu-layers N`)
3. **Monitor GPU usage** with `nvidia-smi`
4. **Check CUDA initialization** in server logs

### Common Solutions
- **Update NVIDIA drivers** to latest version
- **Use compatible CUDA Toolkit** (12.1+ recommended)
- **Start with fewer layers** and increase gradually
- **Check GPU memory** isn't exhausted

## Performance Tips

1. **Start Conservative**: Use `--n-gpu-layers 10` first
2. **Monitor Memory**: Watch `nvidia-smi` during inference
3. **Adjust Gradually**: Increase layers until VRAM is ~80% full
4. **Use Quantized Models**: Q4_0, Q5_0 formats for efficiency

## Next Steps

1. **Run the build**: `.\build_cuda.ps1`
2. **Test functionality**: `.\test_cuda.ps1`
3. **Start using**: Load a GGUF model and enjoy GPU acceleration!
4. **Monitor performance**: Use `nvidia-smi` to see GPU utilization

## Files Created/Modified

### Created:
- `build_cuda.bat` - Command prompt build script
- `build_cuda.ps1` - PowerShell build script (recommended)
- `check_cuda.bat` - System requirements checker
- `test_cuda.ps1` - CUDA verification script
- `GPU_SETUP.md` - Detailed setup guide
- `CUDA_SETUP_COMPLETE.md` - This summary

### Modified:
- `.cargo/config.toml` - Enabled CUDA compilation

Your Shimmy fork is now ready for GPU acceleration! 🎉

---

**Need Help?** Check `GPU_SETUP.md` for detailed troubleshooting or run the test scripts to verify your setup.
