# t3d-python

A Python 3.10+ module for reading [T3D files](https://wiki.beyondunreal.com/Legacy:T3D_File).

This library is still a work-in-progress.

# Installation

```
virtualenv venv
source ./venv/Scripts/activate
pip install maturin
maturin develop
```

# Usage

```python
from t3dpy import read_t3d

with open('./data.t3d', 'r') as fp:
    # Read the T3D file.
    objects = read_t3d(fp.read())
    
    # Iterate over the top-level objects.
    for t3d_object in objects:
        # Print the type of object.
        print(t3d_object.type_)
        
        # Print the child objects.
        print(t3d_object.children)

        # Print the list of properties (order not guaranteed to match input)
        print(t3d_object.properties)
        
        # Get a specific property by name.
        print(t3d_object["SomeProperty"])
```