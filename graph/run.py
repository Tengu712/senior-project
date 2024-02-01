import numpy as np
import matplotlib.pyplot as plt

# read data
x = 1
xs = []
ys = []
while True:
  try:
    for _ in range(0, 10):
      line = input()
      xs.append(x)
      ys.append(int(line))
    x += 1
  except EOFError:
    break

# conver to np.array
xs = np.array(xs)
ys = np.array(ys)

# create a figure
fig = plt.figure()
ax = fig.add_subplot(1, 1, 1)
ax.set_xlabel("Generation")
ax.set_ylabel("Timestamp Delta")
ax.plot(xs, ys, ".")

# save as graph-all.png
plt.savefig("run.png")
