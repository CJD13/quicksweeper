import random
import math
#p is the list of points, s is the set of points to which they must be connected
#d is the distance function, defaulting to euclidean distance
def minConnectingForest(p,s,d=lambda a, b: (a[0]-b[0])**2+(a[1]-b[1])**2):
    l=[]
    for point in p:
        m=None
        neighbor=None
        for other in s:
            r=d(point,other)
            if m==None or r<m:
                neighbor=other
                m=r
        l.append([point,m,neighbor])
    new=None
    for point in l:
        #not already included
        if point[1]!=0:
            if new==None or point[1]<new[1]:
                new=point
    while new!=None:
        #add new to the forest
        new[1]=0
        #update distances and closest points
        for point in l:
            if point[2]!=0:
                r=d(point[0],new[0])
                if r<point[1]:
                    point[1]=r
                    point[2]=new[0]
        #find next point to add to the forest
        new=None
        for point in l:
            #not already included
            if point[1]!=0:
                if new==None or point[1]<new[1]:
                    new=point
    return [(triple[0],triple[2]) for triple in l]
def drawLine(p,q,grid,symbol):
    d=math.sqrt((p[0]-q[0])**2+(p[1]-q[1])**2)
    horizontal=[0]
    if q[0]<p[0]:
        hStep=-1
    else:
        hStep=1
    for i in range(int(p[0]+.5),int(q[0]+.5),hStep):
        x=i+.5
        horizontal.append(d*(x-p[0])/(q[0]-p[0]))
    vertical=[0]
    if q[1]<p[1]:
        vStep=-1
    else:
        vStep=1
    for i in range(int(p[1]+.5),int(q[1]+.5),vStep):
        y=i+.5
        vertical.append(d*(y-p[1])/(q[1]-p[1]))
    x=q[0]
    y=q[1]
    while len(horizontal)>0 and len(vertical)>0:
        grid[x][y]=symbol
        if horizontal[len(horizontal)-1]>vertical[len(vertical)-1]:
            horizontal.pop()
            x-=hStep
        else:
            vertical.pop()
            y-=vStep
def drawMaze(grid, symbol,density=.05, size=40):
    n=len(grid)
    num=int(n*n*density)
    l=[]
    for i in range(num):
        l.append((random.randint(0,n-1),random.randint(0,n-1)))
    l.append(None)
    for (p,q) in minConnectingForest(l,[l[i] for i in range(max(int(num/size),1))],squareEdgeDistance(n)):
        (p,q)=specifyEdge((p,q),n,n)
        drawLine(p,q,grid,symbol)
def squareEdgeDistance(w):
    def d(p,q):
        if p==None:
            if q==None:
                return 0
            return min(q[0],w-q[0],q[1],w-q[1])**2
        if q==None:
            return min(p[0],w-p[0],p[1],w-p[1])**2
        return (p[0]-q[0])**2+(p[1]-q[1])**2
    return d
def rectEdgeDistance(w,h):
    def d(p,q):
        if p==None:
            if q==None:
                return 0
            return min(q[0],w-q[0],q[1],h-q[1])**2
        if q==None:
            return min(p[0],w-p[0],p[1],h-p[1])**2
        return (p[0]-q[0])**2+(p[1]-q[1])**2
    return d
def specifyEdge(p,w,h):
    (a,b)=p
    if a==None:
        if b==None:
            return ((0,0),(0,0))
        mX=min(b[0],w-b[0])
        mY=min(b[1],h-b[1])
        if mX<mY:
            if b[0]<w-b[0]:
                return ((0,b[1]),b)
            else:
                return ((w,b[1]),b)
        else:
            if b[1]<h-b[1]:
                return ((b[0],0),b)
            else:
                return ((b[0],h),b)
    if b==None:
        return specifyEdge((b,a),w,h)
    return p
