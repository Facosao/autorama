from __future__ import annotations
import csv
import math

# PATH = "C:\\Users\\leona\\Documents\\track.data"
PATH = "track_lap.csv"
FULL_LOCK = 0.366519 # radians

class Vec2:
    def __init__(self, a: float, b: float) -> None:
        self.a = a
        self.b = b

    def norma(self) -> float:
        return math.sqrt((self.a ** 2) + (self.b ** 2))
    
    def div_float(self, value: float) -> Vec2:
        return Vec2(self.a / value, self.b / value)

    def __sub__(self, obj: Vec2):
        return Vec2(self.a - obj.a, self.b - obj.b)
    
    def __str__(self):
        return "(" + str(self.a) + ", " + str(self.b) + ")"
    
class Sensor:
    def __init__(self, vecs: list[Vec2], idxl: int, idxr: int) -> None:
        self.vecs: list[Vec2] = vecs
        self.idxl: int = idxl
        self.idxr: int = idxr

def sgn(x: float) -> int:
    if x < 0:
        return -1
    if x == 0:
        return 0
    # x > 0
    return 1

def dot(x: Vec2, y: Vec2) -> float:
        return (x.a * y.a) + (x.b * y.b)

if __name__ == "__main__":
    file = open(PATH, "r", encoding="utf-8")
    raw_data = [[float(x) for x in row] for row in csv.reader(file)]
    file.close()
    track_data: list[list[float]] = []

    for i in range(0, len(raw_data), math.floor(len(raw_data) / 100)):
        track_data.append(raw_data[i])
    
    sensors: list[Sensor] = []

    for row in track_data:
        vec: list[Vec2] = []
        biggest_left: float = 0
        biggest_right: float = 0
        idxl: int = 0
        idxr: int = 0

        for i in range(19):
            vec.append(Vec2(-math.cos(i * math.pi / 18) * row[i], math.sin(i * math.pi / 18) * row[i]))
            if i < 9:
                if row[i] > biggest_left:
                    biggest_left = row[i]
                    idxl = i

            if i > 9:
                if row[i] > biggest_right:
                    biggest_right = row[i]
                    idxr = i

        sensors.append(Sensor(vec, idxl, idxr))

    # outline: list[list[Vec2]] = []  

    # for sensor_row in sensors:
    #     outline_row: list[Vec2] = []
    #     for i in range(18):
    #         outline_row.append(sensor_row[i + 1] - sensor_row[i])
    #     outline.append(outline_row)


    # print(outline[0])

    normalized_left: list[Sensor] = []
    normalized_right: list[Sensor] = []

    # 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18

    for sensor in sensors:
        # Calculate normalized right vector
        nr: list[Vec2] = []
        for i in range(sensor.idxl): # 0, ..., idxl - 1
            sub = sensor.vecs[i + 1] - sensor.vecs[i]
            nr.append(sub.div_float(sub.norma()))
        normalized_right.append(Sensor(nr, -1, sensor.idxr))

        # Calculate normalized left vector
        nl: list[Vec2] = []
        for i in range(17 - sensor.idxr + 1): # 0, ..., 17 - idxr
            sub = sensor.vecs[17 - i] - sensor.vecs[18 - i]
            nl.append(sub.div_float(sub.norma()))
        normalized_left.append(Sensor(nl, sensor.idxl, -1))

    # Calculate angles!
    angles: list[float] = []
    PI_180: float = 180 / math.pi

    # Left portion of sum
    left_sum: list[float] = []
    for n_left in normalized_left:
        local_sum: float = 0
        for i in range(n_left.idxl):
            l_i = n_left.vecs[i]
            l_i_p1 = n_left.vecs[i + 1]
            
            mul = dot(l_i_p1, l_i)
            normas = l_i_p1.norma() * l_i.norma()

            m = l_i.a * l_i_p1.b - l_i.b * l_i_p1.a

            local_sum += math.cos(mul / normas) * PI_180 * sgn(m)
        left_sum.append(local_sum)

    # Right portion of sum
    right_sum: list[float] = []
    for n_right in normalized_right:
        local_sum: float = 0
        for i in range(17 - n_right.idxr): # 16 - idxr
            r_i = n_right.vecs[i]
            r_i_p1 = n_right.vecs[i + 1]
            
            mul = dot(r_i_p1, r_i)
            normas = r_i_p1.norma() * r_i.norma()

            m = r_i.a * r_i_p1.b - r_i.b * r_i_p1.a

            local_sum += math.cos(mul / normas) * PI_180 * sgn(m)
        right_sum.append(local_sum)


    # Combine both partial sums
    for i in range(len(left_sum)):
        angles.append(left_sum[i] + right_sum[i])

    for angle in angles:
        print(abs(angle))

        

