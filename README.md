# Cuda builder for Tensorflow and Pytorch.

## LAST UPDATED: (09/01/2024)
- There is currently no way for both Tensorflow and Torch to use the same cuda version and Python 3.12
- If you would like to use the same cuda version for both, use Python 3.8-3.11 (Links for this install will be at the bottom)

### Changes
**Python 3.12 support**
- New Script:
    - Installs cuda 12.3, the only cuda version supported for Tensorflow in Python 3.12.
    - For Torch, cuda has to be installed within the virtual environment. 
- Old Script:
    - Installs cuda 11.8, this is the last time Tensorflow and Torch shared a cuda version.

---

### ❗❗❗DO NOT USE YAY OR GIT❗❗❗

##### My experiences from the first 10+ times using yay and git
- 30+ minute gcc compile times ✅
- Linker Errors ✅
- Auto-updates & Version Mismatches ✅
- Nvidia doesn't like you ✅
- They actually hate you ✅

---

### Current Compatability
https://www.tensorflow.org/install/source#gpu \
https://pytorch.org/get-started/locally/

Version	            | Python version	| Compiler	    | Build tools	| cuDNN	 | CUDA
| :----:            |    :----:         |        :----: |  :----:       |:----:  |:----:
tensorflow-2.17.0	| 3.9-3.12	        | Clang 17.0.6	| Bazel 6.5.0	| 8.9	 | 12.3
tensorflow-2.13.0	| 3.8-3.11	        |  Clang 16.0.0	| Bazel 5.3.0	| 8.6	 | 11.8
Pytorch(Stable)	    |  3.8+             |  	            | 	            | 	     | 11.8, 12.1

---

Torch typically bundles pre-compiled CUDA binaries and does not require the system Cuda install.
```bash
# Current:
pip install torch torchvision torchaudio
```

---

## Installing

1. Update and download nvidia drivers ('nvidia' and 'nvidia-dkms' are interchangeable, no need to replace your 'nvidia' package if it is already installed)
```bash
sudo pacman -Syu nvidia-dkms opencl-nvidia nvidia-utils nvidia-settings curl
```

2. Download and install gcc12
```bash
curl -O https://archive.archlinux.org/packages/g/gcc12/gcc12-12.3.0-6-x86_64.pkg.tar.zst
curl -O https://archive.archlinux.org/packages/g/gcc12-libs/gcc12-libs-12.3.0-6-x86_64.pkg.tar.zst
sudo pacman -U gcc12-12.3.0-6-x86_64.pkg.tar.zst  gcc12-libs-12.3.0-6-x86_64.pkg.tar.zst
```

3. Download and install CUDA and cuDNN
```bash
curl -O https://archive.archlinux.org/packages/c/cuda/cuda-12.3.2-1-x86_64.pkg.tar.zst
curl -O https://archive.archlinux.org/packages/c/cudnn/cudnn-8.9.7.29-1-x86_64.pkg.tar.zst
sudo pacman -U cuda-12.3.2-1-x86_64.pkg.tar.zst cudnn-8.9.7.29-1-x86_64.pkg.tar.zst
```

4. Update /etc/pacman.conf to exclude cuda and cudnn
- Uncomment the line "#IgnorePkg =", then add cuda and cudnn
```conf
IgnorePkg = cuda cudnn 
```

### Common Tensorflow error
```bash
# ERROR: libdevice not found at ./libdevice.10.bc 
export XLA_FLAGS=--xla_gpu_cuda_data_dir=/opt/cuda
```


### Links for Cuda 11.8
gcc11:
- https://archive.archlinux.org/packages/g/gcc11/gcc11-11.3.0-5-x86_64.pkg.tar.zst
- https://archive.archlinux.org/packages/g/gcc11-libs/gcc11-libs-11.3.0-5-x86_64.pkg.tar.zst

cuda:
- https://archive.archlinux.org/packages/c/cuda/cuda-11.8.0-1-x86_64.pkg.tar.zst

cudnn:
- https://archive.archlinux.org/packages/c/cudnn/cudnn-8.6.0.163-1-x86_64.pkg.tar.zst
