import numpy as np
import matplotlib.pyplot as plt

from sklearn.model_selection import train_test_split
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense

a, b, c, d = 0.1, -0.5, 2.0, 1.0

x = np.linspace(-10, 10, 20000).astype(np.float32)
y = (a*x**3 + b*x**2 + c*x + d).astype(np.float32)

# keras verwacht 2D input (N,1)
X = x.reshape(-1, 1)
Y = y.reshape(-1, 1)

X_train, X_test, Y_train, Y_test = train_test_split(X, Y, test_size=0.2, random_state=42)

model = Sequential([
    Dense(64, activation='relu', input_shape=(1,)),
    Dense(64, activation='relu'),
    Dense(1)
])

model.compile(optimizer='adam', loss='mse', metrics=['mae'])
model.summary()

history = model.fit(
    X_train, Y_train,
    epochs=100,
    batch_size=256,
    validation_split=0.2,
    verbose=1
)

plt.plot(history.history['loss'], label='train loss')
plt.plot(history.history['val_loss'], label='val loss')
plt.legend()
plt.xlabel("epoch")
plt.ylabel("MSE loss")
plt.show()

pred = model.predict(X_test).astype(np.float32)

abs_err = np.abs(pred - Y_test)
mean_err = float(np.mean(abs_err))
max_err = float(np.max(abs_err))

mse = float(np.mean((pred - Y_test)**2))

print("MSE:", mse)
print("Mean abs error:", mean_err)
print("Max abs error:", max_err)

# 4 decimalen ~ fout < 0.0001 (gemiddeld) is streng; <0.001 is vaak al '4 decimalen-ish' afhankelijk van schaal.

idx = np.random.choice(len(X_test), size=2000, replace=False)

plt.scatter(X_test[idx], Y_test[idx], s=2, label="true")
plt.scatter(X_test[idx], pred[idx], s=2, label="pred")
plt.legend()
plt.show()
