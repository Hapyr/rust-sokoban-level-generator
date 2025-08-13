# read from txt file from game_state folder

import os
import pickle
# Get all files in the game_state directory
game_state_dir = "/home/jakob/Dokumente/rust-sokoban-level-generator/game_states"
files = os.listdir(game_state_dir)

percentage_valid_transitions = 0.5
# state machine enum
class StateMachine:
    DEFAULT = 0
    TRACE = 1

state = StateMachine.DEFAULT

traces = []

# iterate over all files
for file in files:
    # read the file
    with open(os.path.join(game_state_dir, file), "r") as f:
        # read the file
        lines = f.readlines()

        trace_state = []
        trace_action = []

        # iterate over all lines
        for line in lines:
            # if line starts with _
            if line.startswith("_") and state == StateMachine.DEFAULT:
                state = StateMachine.TRACE
                continue

            if state == StateMachine.TRACE and line.startswith("L"):
                trace_action.append("L")
            elif state == StateMachine.TRACE and line.startswith("R"):
                trace_action.append("R")
            elif state == StateMachine.TRACE and line.startswith("U"):
                trace_action.append("U")
            elif state == StateMachine.TRACE and line.startswith("D"):
                trace_action.append("D")
            elif state == StateMachine.TRACE and line.startswith("#"):
                if len(line.strip()) == 64:
                    trace_state.append(line.strip())
                else:
                    print(f"Invalid state: {line.strip()}")
                    
                    trace_state = []
                    trace_action = []
                    continue
                    # raise Exception(f"Invalid state: {line.strip()}, File: {file}")

            if state == StateMachine.TRACE and line.startswith("_") and len(trace_action) > 0:
                traces.append({
                    "state": trace_state,
                    "action": trace_action
                })
                trace_state = []
                trace_action = []
        
        state = StateMachine.DEFAULT

# pickle the traces
with open("traces.pkl", "wb") as f:
    pickle.dump(traces, f)

