import bpy
from math import ceil
from mathutils import Vector

OBJ_NAME = "OBJECT"
MODEL_PATH = "cube_cube2.obj"
MODEL_SIZE = 10
SAMPLE = 1000
TIME = 5.0
FPS = 24

FRAME_COUNT = int(FPS * TIME)
FRAME_INC = max(int(FRAME_COUNT / SAMPLE), 1)

START_COLOR = (240, 60, 50)
END_COLOR = (255, 255, 221)
# START_COLOR = (255, 255, 221)
# END_COLOR = (226, 186-40, 217-40)

scene = bpy.context.scene
scene.frame_start = 0
scene.frame_end = FRAME_COUNT

for o in bpy.context.scene.objects:
    if o.type == 'MESH':
        o.select_set(True)
    else:
        o.select_set(False)
bpy.ops.object.delete()


def update_camera(name, location, focus_point=Vector((0.0, 0.0, 0.0)), distance=10.0):
    """
    Focus the camera to a focus point and place the camera at a specific distance from that
    focus point. The camera stays in a direct line with the focus point.

    :param camera: the camera object
    :type camera: bpy.types.object
    :param focus_point: the point to focus on (default=``mathutils.Vector((0.0, 0.0, 0.0))``)
    :type focus_point: mathutils.Vector
    :param distance: the distance to keep to the focus point (default=``10.0``)
    :type distance: float
    """
    looking_direction = location - focus_point
    rot_quat = looking_direction.to_track_quat('Z', 'Y')

    bpy.data.objects[name].rotation_euler = rot_quat.to_euler()
    bpy.data.objects[name].location = rot_quat @ Vector((0.0, 0.0, distance))
    bpy.data.cameras[name].lens = 70


update_camera('Camera', Vector((-0.1, 1.9, -4)),
              focus_point=Vector((0.0, 0, 0)), distance=10)


class MergedModel(object):
    def __init__(self, fname):
        self.faces = []
        self.verts1 = []
        self.verts2 = []

        with open(fname, "r") as f:
            for line in f.readlines():
                a = line.strip().split(" ")
                if len(a) < 4:
                    continue
                if a[0] == "f":
                    self.faces.append(list(map(lambda s: int(s) - 1, a[1:])))
                elif a[0] == "v":
                    self.verts1.append(list(map(float, a[1:])))
                elif a[0] == "u":
                    self.verts2.append(list(map(float, a[1:])))

    def interpolation(self, ratio):
        verts = []
        for (v1, v2) in zip(self.verts1, self.verts2):
            new_v = [0, 0, 0]
            for i in range(3):
                new_v[i] = v1[i] + (v2[i] - v1[i]) * ratio
            verts.append(new_v)
        return (verts, self.faces)


def create_object(name, verts, faces):
    if name in bpy.data.objects:
        scene.collection.objects.unlink(bpy.data.objects[name])

    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(verts, [], faces)
    obj = bpy.data.objects.new(name, mesh)

    scene.collection.objects.link(obj)
    return mesh, obj


if __name__ == "__main__":
    model = MergedModel(MODEL_PATH)

    verts, faces = model.interpolation(0)
    mesh, obj = create_object(OBJ_NAME, verts, faces)

    action = bpy.data.actions.new("MeshAnimation")
    mesh.animation_data_create()
    mesh.animation_data.action = action

    mat = bpy.data.materials.new("PKHG")
    mat.diffuse_color = (
        START_COLOR[0]/255.0, START_COLOR[1]/255.0, START_COLOR[2]/255.0, 1.0)
    obj.data.materials.append(mat)

    fcurves = []
    for v in mesh.vertices:
        fcurves.append([action.fcurves.new("vertices[%d].co" %
                                           v.index, index=i) for i in range(3)])
    n = len(fcurves)

    for i in range(0, FRAME_COUNT, FRAME_INC):
        ratio = i / FRAME_COUNT
        verts, _ = model.interpolation(ratio)

        color = [END_COLOR[i] * ratio + START_COLOR[i]
                 * (1 - ratio) for i in range(3)]
        mat.diffuse_color = (
            color[0]/255.0, color[1]/255.0, color[2]/255.0, 1.0)
        mat.keyframe_insert(data_path='diffuse_color', frame=i)

        for fcur, v in zip(fcurves, verts):
            for k in range(3):
                fcur[k].keyframe_points.insert(i, v[k], options={'FAST'})
