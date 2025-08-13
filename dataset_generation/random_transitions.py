# read the pck file
import pickle
import random

with open("traces.pkl", "rb") as f:
    traces = pickle.load(f)

transitions = []

# schema
# {
#     "state": str
#     "successor": str
#     "action": str
#     "value": int
# }

for trace in traces:
    for i in range(len(trace["state"]) - 1):

        if i < len(trace["state"]) - 2:
            trans_dist = 1 if random.random() < 0.5 else random.randint(2, min(len(trace["state"]) - i - 1, 5))
        else:
            trans_dist = 1

        print(trace["state"][i], trace["state"][i + trans_dist])
        transitions.append({
            "state": trace["state"][i],
            "successor": trace["state"][i + trans_dist],
            "action": "I" if trans_dist != 1 else trace["action"][i],
            "value": trans_dist
        })

# pickle the transitions
with open("transitions.pkl", "wb") as f:
    pickle.dump(transitions, f)