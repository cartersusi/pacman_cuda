## LAST UPDATED: (09/21/2023)

### ❗❗❗DO NOT USE YAY OR GIT❗❗❗

##### My experiences from the first 10+ times using yay and git
- 30 minutes gcc compile times ✅
- Gcc overwrites ✅
- Nvidia doesn't like you ✅
- They actually hate you ✅
- Auto-updates & Version Mismatches ✅
- Have to recompile when it inevitably breaks ✅

---

### Compatability
https://www.tensorflow.org/install/source#gpu

Version	            | Python version	| Compiler	    | Build tools	| cuDNN	 | CUDA
| :----:            |    :----:         |        :----: |  :----:       |:----:  |:----:
tensorflow-2.13.0	| 3.8-3.11	        |  Clang 16.0.0	| Bazel 5.3.0	| 8.6	 | 11.8
Pytorch(Stable)	    |  3.8+             |  	            | 	            | 	     | 11.7 or 11.8
Pytorch(Nightly)	|  3.8+             |  	            | 	            | 	     | 11.8 or 12.1

---

#### 1. If you have drivers run this just to make sure all nvidia pkgs are installed

```zsh
sudo pacman -Syu
sudo pacman -S nvidia-dkms opencl-nvidia nvidia-utils nvidia-settings curl
nvidia-smi
```

---

#### 2. (Optional) Enable Multiple Threads for makepkg

My CPU is 8core/16 threads so I used -j17 \
If your CPU had 16 cores/32 threads, use -j33
```zsh
sudo nano /etc/makepkg.conf
```
My MAKEFLAGS value was on line 49 \
If not use ctrl+W type:'MAKEFLAGS' \
Change MAKEFLAGS="-j2" to -> MAKEFLAGS="-j17"

Keep your source files together
```zsh
mkdir ~/.cuda_sources
cd ~/.cuda_sources
```

---

#### 3. Install gcc11 alongisde gcc
AUR: https://archive.archlinux.org/packages/g/gcc11/ 

```zsh
curl -O https://archive.archlinux.org/packages/g/gcc11/gcc11-11.3.0-5-x86_64.pkg.tar.zst
curl -O https://archive.archlinux.org/packages/g/gcc11/gcc11-11.3.0-5-x86_64.pkg.tar.zst.sig
gpg --verify gcc11-11.3.0-5-x86_64.pkg.tar.zst.sig 2>&1 | grep "using RSA key" | awk '{print $NF}'
curl -O https://archive.archlinux.org/packages/g/gcc11-libs/gcc11-libs-11.3.0-5-x86_64.pkg.tar.zst
curl -O https://archive.archlinux.org/packages/g/gcc11-libs/gcc11-libs-11.3.0-5-x86_64.pkg.tar.zst.sig
gpg --verify gcc11-libs-11.3.0-5-x86_64.pkg.tar.zst.sig 2>&1 | grep "using RSA key" | awk '{print $NF}'

sudo pacman -U gcc11-11.3.0-5-x86_64.pkg.tar.zst gcc11-libs-11.3.0-5-x86_64.pkg.tar.zst
```

---

#### 4. Install cuda 11.8.0
AUR: https://archive.archlinux.org/packages/c/cuda/

```zsh
curl -O https://archive.archlinux.org/packages/c/cuda/cuda-11.8.0-1-x86_64.pkg.tar.zst
curl -O https://archive.archlinux.org/packages/c/cuda/cuda-11.8.0-1-x86_64.pkg.tar.zst.sig
gpg --verify cuda-11.8.0-1-x86_64.pkg.tar.zst.sig 2>&1 | grep "using RSA key" | awk '{print $NF}'
sudo pacman -U cuda-11.8.0-1-x86_64.pkg.tar.zst
sudo reboot
```

---

#### 5. Install cuDNN
AUR: https://archive.archlinux.org/packages/c/cudnn/

```zsh
curl -O https://archive.archlinux.org/packages/c/cudnn/cudnn-8.6.0.163-1-x86_64.pkg.tar.zst
curl -O https://archive.archlinux.org/packages/c/cudnn/cudnn-8.6.0.163-1-x86_64.pkg.tar.zst.sig
gpg --verify cudnn-8.6.0.163-1-x86_64.pkg.tar.zst.sig 2>&1 | grep "using RSA key" | awk '{print $NF}'
sudo pacman -U cudnn-8.6.0.163-1-x86_64.pkg.tar.zst
```

---

#### 6. Other tools

nsight compute - https://developer.nvidia.com/tools-overview/nsight-compute/get-started \
nsight systems - https://developer.nvidia.com/gameworksdownload#?dn=nsight-systems-2023-3

```zsh
chmod +x nsight-compute-linux-2023.2.2.3-33188574.run       
chmod +x NsightSystems-linux-public-2023.3.1.92-3314722.run
sudo ./nsight-compute-linux-2023.2.2.3-33188574.run
sudo ./NsightSystems-linux-public-2023.3.1.92-3314722.run
echo "export PATH=$PATH:/opt/nvidia/nsight-systems/2023.3.1/bin" >> ~/.zshrc
echo "export PATH=$PATH:/usr/local/NVIDIA-Nsight-Compute-2023.2" >> ~/.zshrc
```

compute:
```zsh
ncu
ncu-ui
```

systems:
```zsh
nsys
nsys-ui
```

---

#### 7. Python Local Environemnt

Test for nvidia, cuda, cudnn, gcc, tensorflow, torch:
```zsh
git clone https://github.com/carter4299/cuda_tf_torch.git
cd cuda_tf_torch
chmod +test.sh
./test
```

---
or \
Configure:
```zsh
python3 -m venv torch_n_venv torch_s_venv tf_venv

# Torch (nightly)
source torch_n_venv/bin/activate
pip install --pre torch torchvision torchaudio --index-url https://download.pytorch.org/whl/nightly/cu118
deactivate

# Torch (stable)
source torch_s_venv/bin/activate
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
deactivate

# Tensorflow
source tf_venv/bin/activate
pip install nvidia-cudnn-cu11==8.6.0.163 tensorflow==2.13.0
deactivate
```

#### 8. Cleanup
```zsh
sudo nano /etc/pacman.conf
# uncomment 'IgnorePkg  =' , then add 'cuda cudnn' 
# 
#IgnorePkg   = cuda cudnn
```

---

### Errors:

Only one I encountered using the method above

test_tf.py:
```zsh
python3 test_tf.py                                 
```
The GPU was found:
1 Physical GPUs, 1 Logical GPUs \
    ... \
Node: 'Adam/StatefulPartitionedCall_19' \
libdevice not found at ./libdevice.10.bc \
	[[{{node Adam/StatefulPartitionedCall_19}}]] [Op:__inference_train_function_3253]

to fix:
```zsh
find / -type d -name nvvm 2>/dev/null
```
/opt/cuda/nvvm
```zsh
cd /opt/cuda/nvvm
ls
```
bin
include
lib64
libdevice
libnvvm-samples
```zsh
cd libdevice
ls
```
libdevice.10.bc

Add path to ~/.zshrc:
```zsh
echo "export XLA_FLAGS=--xla_gpu_cuda_data_dir=/opt/cuda" >> ~/.zshrc
```

---

### Links
NVIDIA Compatablity - https://docs.nvidia.com/deeplearning/cudnn/pdf/cuDNN-Support-Matrix.pdf

Tensorflow Compatablity - https://www.tensorflow.org/install/source#gpu

Torch Compatablity - https://pytorch.org/get-started/locally/

gcc11(AUR) - https://archive.archlinux.org/packages/g/gcc11/

gcc11-libs(AUR) - https://archive.archlinux.org/packages/g/gcc11-libs/

cuda(AUR) - https://archive.archlinux.org/packages/c/cuda/

cudnn(AUR) - https://archive.archlinux.org/packages/c/cudnn/

nsight compute - https://developer.nvidia.com/tools-overview/nsight-compute/get-started

nsight systems - https://developer.nvidia.com/gameworksdownload#?dn=nsight-systems-2023-3

---

### Libraries
Pytorch (nighly):
```zsh
pip install --pre torch torchvision torchaudio --index-url https://download.pytorch.org/whl/nightly/cu118
```

Pytorch (stable):
```zsh
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
```

Tensorflow:
```zsh
pip install nvidia-cudnn-cu11==8.6.0.163 tensorflow==2.13.0
```

