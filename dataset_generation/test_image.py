# load the tensors from the pkl file
import pickle
import matplotlib.pyplot as plt
with open("tensors.pkl", "rb") as f:
    tensors = pickle.load(f)

print(len(tensors["state_gray_tensor"]))

index = 190

# load the state_gray_tensor
state_gray_tensor = tensors["state_gray_tensor"][index]
successor_gray_tensor = tensors["successor_gray_tensor"][index]
action_list = tensors["action_list"][index]
value_list = tensors["value_list"][index]

print(state_gray_tensor.shape)
print(successor_gray_tensor.shape)
print(action_list)
print(value_list)


# make a matplotlib of state and successor
plt.imshow(state_gray_tensor, cmap='gray')
plt.savefig('state.png')
plt.close()
plt.imshow(successor_gray_tensor, cmap='gray')
plt.savefig('successor.png') 