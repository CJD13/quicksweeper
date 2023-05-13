from vector import vector
import itertools
#File supporting grids of squares.
class grid:
    def __init__(self, get=lambda x, y: None, s=lambda x, y, v: None, setRow=lambda x, r: None):
        self.get=get
        self.set=s
    def of(array2):
        #self.data=array2
        def get(x, y):
            if 0<=x and x<len(array2) and 0<=y and y<len(array2[x]):
                return array2[x][y]
            return None
        def s(x,y,v):
            array2[x][y]=v
        def setRow(x, r):
            array2[x]=r
        g=grid(get, s)
        return finiteGrid(g, 0, len(array2)-1,0,max(len(k)-1 for k in array2))
    def contains(self,x,y):
        return self[x][y]!=None
    #returns this grid represented with a new coordinate system, centered at (x,y)
    def recentered(self,x,y):
        return grid(get=lambda a, b: self[a+x][b+y])
    #returns this grid translated by (dx,dy)
    def translated(self, dx, dy):
        return self.recentered(-dx,-dy)
    #returns this grid rotated counterclockwise by an angle of n pi/2
    def rotated(self,n):
        return grid(get=lambda x, y: self[rotated((x,y),n)])
    #reflects over the line y=x
    def reflected(self):
        return grid(get=lambda x, y: self[y][x])
    #To conform with the semantics for arrays, slicing with a specified start point will move the coordinate system to place 0 at that point, and slicing with no start will keep the origin unchanged.
    def __getitem__(self, item):
        if type(item)==slice:
            offset = 0 if item.start==None else item.start
            step = 1 if item.step==None else item.step
            def get(x,y):
                x=x*step+offset
                if (item.start!=None and x<start) or (item.stop!=None and x>=item.stop):
                    return None
                return self[x,y]
            return grid(get=get).reflected()
        if type(item)==tuple:
            return self.get(item[0],item[1])
        return gridrow(get=lambda y: self.get(item,y),s=lambda y, v: self.set(x,y,v))
    def __setitem__(self, item, value):
        self.setRow(item,value)
class gridrow:
    def __init__(self, get=lambda x: None, s=lambda x, v: None):
        self.get=get
        self.set=s
    def __getitem__(self,item):
        return self.get(item)
    def __setitem__(self,item,value):
        self.set(item, value)
#A grid with a bounding rectangle
class finiteGrid:
    def __init__(self, grid, lx, rx, ly, uy):
        self.grid=grid
        self.bL=lx
        self.bR=rx
        self.bD=ly
        self.bU=uy
    def contains(self,x,y):
        return self.grid.contains(x,y)
    def recentered(self,x,y):
        return finiteGrid(self.grid.recentered(x,y),self.bL-x,self.bR-x,self.bD-y,self.bU-y)
    def translated(self,x,y):
        return self.recentered(-x,-y)
    def rotated(self,n):
        l=[self.bR,-self.bD,-self.bL,self.bU]
        return finiteGrid(self.grid.rotated(-n), -l[(n+2)%4],l[(n+0)%4],-l[(n+1)%4],l[(n+3)%4])
    def reflected(self):
        return finiteGrid(self.grid.reflected(),self.bD,self.bU,self.bL,self.bR)
    #After slicing, the origin will be at the bottom-left of the slice (conforms with array semantics)
    def __getitem__(self,item):
        if type(item)==slice:
            start = self.bL if item.start==None else item.start
            stop = self.bR+1 if item.stop==None else item.stop
            step = 1 if item.step==None else item.step
            def get(x,y):
                x=x*step+start
                if x<start or x>=stop:
                    return None
                return self[x,y]
            def s(x,y,v):
                x=x*step+start
                if x<start or x>=stop:
                    #TODO: what happens here?
                    return None
                self[x,y]=v
            return finiteGrid(grid(get,s),0,(stop-start-1)//step,self.bD,self.bU).reflected()
        if type(item)==tuple:
            return self.grid.get(item[0],item[1])
        return gridrow(get=lambda y: self.grid.get(item,y),s=lambda y, v: self.set(item,y,v))
    def set(self,x,y,v):
        self.grid.set(x,y,v)
        self.bL=min(self.bL,x)
        self.bR=max(self.bR,x)
        self.bD=min(self.bD,y)
        self.bU=max(self.bU,y)
    def __iter__(self):
        return (i for i in (self.grid[x][y] for (x,y) in itertools.product(range(self.bL,self.bR+1),range(self.bD,self.bU+1))) if i!=None)
        #(self.bL..self.bR).product(self.bD..self.bU).map(|(x,y)| self.grid[x][y]).flatten()
    def __str__(self):
        res=''
        for y in range(self.bU,self.bD-1,-1):
            for x in range(self.bL,self.bR+1):
                res+=str(self[x][y])+'\t'
            res+='\n\n\n'
        return res
#functions for transforming points
#rotates a point counterclockwise by an angle of npi/2
def rotated(p,n):
    rotX=vector([[1,0],[0,-1],[-1,0],[0,1]][n%4])
    rotY=vector([[0,1],[1,0],[0,-1],[-1,0]][n%4])
    return (vector(p).dot(rotX),vector(p).dot(rotY))
#translates p by the vector v
def translated(p,v):
    return (p[0]+v[0],p[1]+v[1])
