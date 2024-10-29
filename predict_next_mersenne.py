import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from sklearn.linear_model import LinearRegression

# Load Mersenne prime exponents
with open('mersenne_primes.txt', 'r') as file:
    exponents = [int(line.strip()) for line in file.readlines()]

# Calculate differences between consecutive exponents
differences = np.diff(exponents)

# Prepare DataFrame
df = pd.DataFrame({
    'Index': range(1, len(exponents) + 1),
    'Exponent': exponents,
    'Difference': [None] + list(differences)
})

# Plot Exponents
plt.figure(figsize=(10, 5))
plt.plot(df['Index'], df['Exponent'], marker='o')
plt.title('Mersenne Prime Exponents Over Time')
plt.xlabel('Index')
plt.ylabel('Exponent')
plt.grid(True)
plt.show()

# Plot Differences
plt.figure(figsize=(10, 5))
plt.plot(df['Index'], df['Difference'], marker='o', color='orange')
plt.title('Differences Between Consecutive Mersenne Prime Exponents')
plt.xlabel('Index')
plt.ylabel('Difference')
plt.grid(True)
plt.show()

# Predict next exponent using linear regression
X = np.array(df['Index'].dropna()).reshape(-1, 1)
y = np.array(df['Exponent'].dropna())

model = LinearRegression()
model.fit(X, y)

next_index = np.array([[df['Index'].iloc[-1] + 1]])
predicted_exponent = model.predict(next_index)[0]

print(f'Predicted next Mersenne prime exponent: {int(predicted_exponent)}')
