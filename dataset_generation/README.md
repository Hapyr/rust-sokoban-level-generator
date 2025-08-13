# Dataset Generation

This folder contains scripts to generate training datasets from Sokoban game traces.

## Files

- **`text_to_pkl.py`** - Parses text files from the `game_states` directory containing Sokoban game traces and converts them into structured data. Extracts game states strings and player actions (L/R/U/D) to create trace sequences, saved as `traces.pkl`.

- **`random_transitions.py`** - Processes the traces to generate state transitions with varying distances between states. Creates transitions with immediate next states (distance=1) or random jumps (distance=2-5), saved as `transitions.pkl`.

- **`transitions2imagetensor.py`** - Converts transitions data into image tensor format. Uses BMP files from the `elements/` directory to represent different game pieces (wall, box, player, etc.) and transforms Sokoban game states into grayscale image tensors, saved as `tensors.pkl` and `transition_sample.pkl`.

- **`test_image.py`** - Testing and visualization script that loads tensor data from `tensors.pkl` and generates sample visualization images (state.png and successor.png) to display game states as grayscale images.

- **`traces.pkl`** - Generated dataset containing sequences of game states and corresponding actions.

- **`transitions.pkl`** - Generated dataset containing state-to-state transitions with actions and distance values.

- **`tensors.pkl`** - Generated image tensor dataset containing grayscale representations of game states and successors with corresponding actions and values.

- **`transition_sample.pkl`** - Generated sample dataset containing individual transition examples in tensor format.

## Usage

1. Run `text_to_pkl.py` to convert raw game trace files into structured data
2. Run `random_transitions.py` to generate transition datasets from the traces

The generated pickle files can be used for training machine learning models on Sokoban puzzle solving.
