@echo off
REM Build script for Shimmy with CUDA GPU support
echo Building Shimmy with CUDA GPU support...
echo.

REM Check if CUDA is installed
echo Checking CUDA installation...
nvcc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: NVCC not found! Please install CUDA Toolkit first.
    echo Download from: https://developer.nvidia.com/cuda-downloads
    pause
    exit /b 1
)

REM Show CUDA version
echo CUDA Toolkit found:
nvcc --version
echo.

REM Set CUDA environment variables for the build
echo Setting CUDA environment variables...

REM Try to find CUDA installation
if exist "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0" (
    set "CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0"
    set "CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0\bin\nvcc.exe"
) else if exist "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6" (
    set "CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6"
    set "CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6\bin\nvcc.exe"
) else if exist "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5" (
    set "CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5"
    set "CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5\bin\nvcc.exe"
) else if exist "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.4" (
    set "CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.4"
    set "CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.4\bin\nvcc.exe"
) else if exist "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3" (
    set "CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3"
    set "CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3\bin\nvcc.exe"
) else if exist "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.2" (
    set "CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.2"
    set "CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.2\bin\nvcc.exe"
) else if exist "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.1" (
    set "CUDA_PATH=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.1"
    set "CUDACXX=C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.1\bin\nvcc.exe"
) else (
    echo WARNING: Could not find CUDA in standard location.
    echo Please set CUDA_PATH and CUDACXX manually if the build fails.
    echo.
)

echo Using CUDA_PATH: %CUDA_PATH%
echo Using CUDACXX: %CUDACXX%
echo.

REM Set build environment variables for CUDA support
set "CMAKE_ARGS=-DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES=all-major"
set "FORCE_CMAKE=1"

echo Build configuration:
echo CMAKE_ARGS: %CMAKE_ARGS%
echo FORCE_CMAKE: %FORCE_CMAKE%
echo.

REM Clean previous build if requested
if "%1"=="clean" (
    echo Cleaning previous build...
    if exist target rmdir /s /q target
    if exist Cargo.lock del Cargo.lock
    echo.
)

REM Build with CUDA support
echo Starting build with CUDA support...
echo This may take several minutes...
echo.

REM Build the library first
echo Building library...
cargo build --lib --no-default-features --features llama --release
if %errorlevel% neq 0 (
    echo.
    echo ERROR: Library build failed!
    echo.
    echo Common solutions:
    echo 1. Make sure Visual Studio Build Tools are installed
    echo 2. Verify CUDA Toolkit is properly installed
    echo 3. Check that your GPU supports CUDA
    echo 4. Try running as Administrator
    echo.
    pause
    exit /b 1
)

REM Build the executable
echo.
echo Building executable...
cargo build --release --no-default-features --features llama
if %errorlevel% neq 0 (
    echo.
    echo ERROR: Executable build failed!
    pause
    exit /b 1
)

echo.
echo ========================================
echo Build completed successfully!
echo ========================================
echo.
echo Executable location: target\release\shimmy.exe
echo.

REM Test the build
echo Testing CUDA support...
target\release\shimmy.exe --version
echo.

echo To verify CUDA support, run:
echo target\release\shimmy.exe serve
echo.
echo Then check the startup logs for CUDA initialization messages.
echo.

pause
