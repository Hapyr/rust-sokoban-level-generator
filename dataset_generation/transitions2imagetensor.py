# read the transitions
import pickle
import torch
import matplotlib.pyplot as plt
from PIL import Image
import numpy as np


with open("transitions.pkl", "rb") as f:
    transitions = pickle.load(f)


tile_resolution_bmp = 56
tile_resolution = 10
number_tiles_y = 8
number_tiles_x = 8

# convert the transitions to image grayscale tensors
# Load BMP files for each encoding
encoding = {}
element_mappings = {
    " ": "floor.bmp",      # Empty/Floor
    "#": "wall.bmp",       # Wall
    ".": "goal.bmp",      # Goal (using floor as base)
    "$": "box.bmp",        # Box
    "*": "box_goal.bmp",   # Box on Goal
    "@": "player.bmp",     # Player
    "+": "player_goal.bmp",     # Player on Goal (using player image)
}

# Load each BMP file and convert to tensor
for char, filename in element_mappings.items():
    img_path = f"elements/{filename}"
    img = Image.open(img_path).convert('L')  # Convert to grayscale
    img = img.resize((tile_resolution, tile_resolution))  # Resize to target resolution
    img_array = np.array(img)
    encoding[char] = torch.from_numpy(img_array).float()


def convert_to_image_tensor(transition):
    state = transition["state"]
    successor = transition["successor"]

    # convert the state and successor to grayscale
    state_gray = torch.zeros(number_tiles_y * tile_resolution, number_tiles_x * tile_resolution)
    successor_gray = torch.zeros(number_tiles_y * tile_resolution, number_tiles_x * tile_resolution)

    for i in range(number_tiles_y * number_tiles_x):
        x = i % number_tiles_x
        y = i // number_tiles_y
        state_gray[x*tile_resolution:x*tile_resolution+tile_resolution, y*tile_resolution:y*tile_resolution+tile_resolution] = encoding[state[i]]
        successor_gray[x*tile_resolution:x*tile_resolution+tile_resolution, y*tile_resolution:y*tile_resolution+tile_resolution] = encoding[successor[i]]

    return state_gray, successor_gray


state_gray_list = []
successor_gray_list = []

transition_sample = []

encoding_action = {
    "L": 0,
    "R": 1,
    "U": 2,
    "D": 3,
    "I": 4,
}

interaction = 0
for transition in transitions:
    if interaction % 100 == 0:
        progress = (interaction / len(transitions)) * 100
        bar_length = 40
        filled_length = int(bar_length * interaction // len(transitions))
        bar = '=' * filled_length + '-' * (bar_length - filled_length)
        print(f'\rProgress: [{bar}] {progress:.1f}%', end='')
    
    state_gray, successor_gray = convert_to_image_tensor(transition)
    state_gray_list.append(state_gray.transpose(0, 1))
    successor_gray_list.append(successor_gray.transpose(0, 1))

    transition_sample.append((state_gray, successor_gray, encoding_action[transition["action"]], transition["value"]))
    
    interaction += 1

state_gray_tensor = torch.stack(state_gray_list, dim=0)
successor_gray_tensor = torch.stack(successor_gray_list, dim=0)
action_list = torch.tensor([encoding_action[transition["action"]] for transition in transitions])
value_list = torch.tensor([transition["value"] for transition in transitions])

# pickle the tensors as single dictionary
tensors = {
    "state_gray_tensor": state_gray_tensor,
    "successor_gray_tensor": successor_gray_tensor,
    "action_list": action_list,
    "value_list": value_list
}

with open("tensors.pkl", "wb") as f:
    pickle.dump(tensors, f)

with open("transition_sample.pkl", "wb") as f:
    pickle.dump(transition_sample, f)
