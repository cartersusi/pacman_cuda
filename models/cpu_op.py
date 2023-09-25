import subprocess
print("\n**------CPU OPERATIONS------**\n")
p1 = subprocess.Popen(["snakeviz", "models/torch/torch_stable.prof"])
p2 = subprocess.Popen(["snakeviz", "models/torch/torch_nightly.prof"])
p3 = subprocess.Popen(["snakeviz", "models/tf/tensorflow.prof"])

p1.wait()
p2.wait()
p3.wait()