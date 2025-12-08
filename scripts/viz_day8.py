import matplotlib.pyplot as plt
import numpy as np

points = np.genfromtxt("input/2025/day8.txt", delimiter=",")

fig = plt.figure(figsize=(10, 10))
ax = fig.add_subplot(111, projection="3d")

ax.scatter(points[:, 0], points[:, 1], points[:, 2], c="blue", marker="o", s=50, alpha=0.6)

ax.set_xlabel("X axis")
ax.set_ylabel("Y axis")
ax.set_zlabel("Z axis")
ax.set_title("3D Point Visualization")

ax.grid(True)

plt.show()
