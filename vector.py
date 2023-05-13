import math
class vector:
    def __init__(self,coords):
        self.coords=coords
    def __add__(self, other):
        if len(self.coords)!=len(other.coords):
            raise Exception
        new=[]
        for i in range(len(self.coords)):
            new.append(self.coords[i]+other.coords[i])
        return vector(new)
    def __mul__(self,other):
        new=[]
        for i in self.coords:
            new.append(i*other)
        return vector(new)
    def dot(self,other):
        return sum(self.coords[i]*other.coords[i] for i in range(len(self)))
    def project(self,other):
        return self*(self.dot(other)/self.dot(self))
    def __abs__(self):
        return math.sqrt(self.dot(self))
    def toBytes(self):
        s=str(self.coords[0])
        for i in range(1,len(self.coords)):
            s+='\n'+str(self.coords[i])
        return s.encode()
    def __len__(self):
        return len(self.coords)
    def __sub__(self,other):
        return self+other*-1
    def __truediv__(self,other):
        return self*(1/other)
    def __getitem__(self, item):
        return self.coords[item]
def zero(n):
    return vector([0]*n)
