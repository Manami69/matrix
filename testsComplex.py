import numpy as np;
from numpy import linalg as LA
import matplotlib.pyplot as plt

A=np.array([1 - 1j, 0])
B=np.array([1,1])

# TEST DOT
dot = np.dot(A, B)
print(f"Dot : {dot}")
# TEST NORM
norm = LA.norm(A)
print(f"Norm : {norm}")


# TEST COSINE
# test from https://towardsdatascience.com/cosine-similarity-how-does-it-measure-the-similarity-maths-behind-and-usage-in-python-50ad30aad7db
# consider two vectors A and B in 2-D
cos_sim=np.dot(A,B)/(LA.norm(A)*LA.norm(B))
print (f"Cosine Similarity between A and B:{cos_sim}")
print (f"Cosine Distance between A and B:{1-cos_sim}")