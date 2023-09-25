#!/usr/bin/sh

TORCH_MODEL="./test_models/test_torch.py"
TF_MODEL="./test_models/test_tf.py"
TORCH_NIGHTLY="torch_n_venv"
TORCH_STABLE="torch_s_venv"
TF="tf_venv"
CPU_TESTS="./models/cpu_op.py"

mkdir -p data models/tf models/torch test_models


check_init() {
    python3 check.py 0
    exit_code=$?
    if [ $exit_code -eq 0 ]; then
        echo "All requirements satisfied"
    else
        echo "Missing requirements"
    fi
}

chcek_torch() {
    python3 check.py 1
    exit_code=$?
    if [ $exit_code -eq 0 ]; then
        echo "Torch is installed"
    else
        echo "Torch is not installed"
        exit 1
    fi
}

check_tf() {
    python3 check.py 2
    exit_code=$?
    if [ $exit_code -eq 0 ]; then
        echo "Tensorflow is installed"
    else
        echo "Tensorflow is not installed"
        exit 1
    fi
}

torch_nightly(){
    source $TORCH_NIGHTLY/bin/activate
    pip install --pre torch torchvision torchaudio --index-url https://download.pytorch.org/whl/nightly/cu118
    chcek_torch
    python3 $TORCH_MODEL 0
    deactivate
}

torch_stable(){
    source $TORCH_STABLE/bin/activate
    pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
    chcek_torch
    python3 $TORCH_MODEL 1
    deactivate
}

tf(){
    source $TF/bin/activate
    pip install nvidia-cudnn-cu11==8.6.0.163 tensorflow==2.13.0
    check_tf
    python3 $TF_MODEL
}


create_venvs(){
    python3 -m venv $TORCH_NIGHTLY $TORCH_STABLE $TF
    if ls $TORCH_NIGHTLY $TORCH_STABLE $TF; then
        echo "Virtual environments created"
    else
        echo "Virtual environments not created"
        exit 1
    fi

    echo "Virtual environments created"
}

load_tests(){
    pip install snakeviz
    python3 $CPU_TESTS
    deactivate
}

main(){
    check_init
    export XLA_FLAGS=--xla_gpu_cuda_data_dir=/opt/cuda
    create_venvs
    torch_nightly
    torch_stable
    tf
    load_tests

    echo "All tests passed"
    echo "export XLA_FLAGS=--xla_gpu_cuda_data_dir=/opt/cuda" >> ~/.zshrc

}

main
