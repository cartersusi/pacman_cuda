import sys
import os
os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'  

if len(sys.argv) != 2:
    print("Usage: script.py <integer>")
    sys.exit(1)
try:
    integer_arg = int(sys.argv[1])
except ValueError:
    print(f"'{sys.argv[1]}' is not a valid integer.")
    sys.exit(1)

def get_cudnn_version():
    cudnn_header = "/usr/include/cudnn_version.h"
    
    major_cmd = f"cat {cudnn_header} | grep CUDNN_MAJOR"
    minor_cmd = f"cat {cudnn_header} | grep CUDNN_MINOR"
    patch_cmd = f"cat {cudnn_header} | grep CUDNN_PATCHLEVEL"
    
    major = subprocess.check_output(major_cmd, shell=True).decode().split()[2]
    minor = subprocess.check_output(minor_cmd, shell=True).decode().split()[2]
    patch = subprocess.check_output(patch_cmd, shell=True).decode().split()[2]
    
    return f"{major}.{minor}.{patch}"


if integer_arg == 0:
    import subprocess
    print("GPU:")
    try:
        subprocess.run("nvidia-smi", shell=True)
    except Exception:
        print("nvidia-smi not found.")
        sys.exit(1)
    print("\nCUDA Version:")
    try:
        subprocess.run("nvcc --version", shell=True)
    except Exception:
        print("nvcc not found.")
        sys.exit(1)
    print("\ncuDNN Version:")
    try:
        print(get_cudnn_version())
    except Exception:
        print("cuDNN not found.")
        sys.exit(1)
    print("\ngcc Version:")
    try:
        subprocess.run("gcc --version", shell=True)
    except Exception:
        print("gcc not found.")
        sys.exit(1)
    sys.exit(0)
elif integer_arg == 1:
    try:
        import torch
    except ImportError:
        print("PyTorch not installed.")
        sys.exit(1)
    print(f'Torch Version: {torch.__version__}')
    if torch.cuda.is_available():
        print(f'GPU: Available\nGPU: {torch.cuda.get_device_name(0)}')
        sys.exit(0)
    print("No GPU available")
    sys.exit(1)
elif integer_arg == 2:
    try:
        import tensorflow as tf
    except ImportError:
        print("TensorFlow not installed.")
        sys.exit(1)
    print("TensorFlow version:", tf.__version__)
    print("CUDA version:", tf.sysconfig.get_build_info()['cuda_version'])
    print("cuDNN version:", tf.sysconfig.get_build_info()['cudnn_version'])
    print("GPU: ",tf.config.list_physical_devices('GPU'))
    