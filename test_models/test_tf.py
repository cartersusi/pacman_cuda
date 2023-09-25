import time
import cProfile
import os
os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'  
import tensorflow as tf
from tensorflow.config import experimental as config_experimental
gpus = config_experimental.list_physical_devices('GPU')
if gpus:
    try:
        config_experimental.set_virtual_device_configuration(
            gpus[0],
            [config_experimental.VirtualDeviceConfiguration(memory_limit=4096)]) 
        logical_gpus = config_experimental.list_logical_devices('GPU')
        print(len(gpus), "Physical GPUs,", len(logical_gpus), "Logical GPUs")
    except RuntimeError as e:
        print(e)
from tensorflow.keras import layers, models
from tensorflow.keras.datasets import cifar10

(train_images, train_labels), (test_images, test_labels) = cifar10.load_data()
train_images, test_images = train_images / 255.0, test_images / 255.0

model = models.Sequential([
    layers.Conv2D(64, (3, 3), activation='relu', input_shape=(32, 32, 3)),
    layers.MaxPooling2D((2, 2)),
    layers.Conv2D(128, (3, 3), activation='relu'),
    layers.MaxPooling2D((2, 2)),
    layers.Conv2D(256, (3, 3), activation='relu'),
    layers.MaxPooling2D((2, 2)),
    layers.Flatten(),
    layers.Dense(1024, activation='relu'),
    layers.Dense(512, activation='relu'),
    layers.Dense(10)
])

profile = cProfile.Profile()
profile.enable()
start_time = time.time()
model.compile(optimizer='adam',
              loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True),
              metrics=['accuracy'])


history = model.fit(train_images, train_labels, epochs=5, batch_size=128, validation_split=0.2)

end_time = time.time()
profile.disable()
print(f"Elapsed time: {end_time - start_time}")
profile.dump_stats('./models/tf/tensorflow.prof')
