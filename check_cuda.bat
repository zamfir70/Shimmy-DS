@echo off
echo Checking CUDA installation...
echo.

echo === NVCC Version ===
nvcc --version
echo.

echo === NVIDIA SMI ===
nvidia-smi
echo.

echo === Environment Variables ===
echo CUDA_PATH: %CUDA_PATH%
echo CUDA_HOME: %CUDA_HOME%
echo.

echo === Looking for CUDA in Program Files ===
dir "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\" 2>nul
echo.

pause
