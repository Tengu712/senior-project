import numpy as np
import matplotlib.pyplot as plt

# read data
count = 0
xs = []
ys = []
while True:
  try:
    line = input()
    xs.append(count)
    ys.append(int(line))
    count += 1
  except EOFError:
    break

# conver to np.array
xs = np.array(xs)
ys = np.array(ys)

# create a figure
fig = plt.figure()
ax = fig.add_subplot(1, 1, 1)
ax.set_xlabel("")
ax.set_ylabel("Timestamp Delta")
ax.plot(xs, ys)

# save as graph-all.png
plt.savefig("test.png")
