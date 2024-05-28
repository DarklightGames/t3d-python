from t3dpy import read_t3d, T3dObject
from pprint import pprint

with open('./src/tests/data/terraininfo.t3d', 'r') as fp:
    objects = read_t3d(fp.read())
    # map_object = next(filter(lambda x: x.type_ == "Map", objects))
    # actor_objects = filter(lambda child: child.type_ == "Actor", map_object.children)
    for actor_object in objects:
        pprint(actor_object.properties)
        pprint(actor_object.vector_properties)
