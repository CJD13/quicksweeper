import base
import maze
import minesweeper
import math
from grid import *
#returns all unblocked squares which can be seen from (0,0) betwen the angles of -pi/4 and pi/4
def lsRight(grid,unblocked=lambda a: a==0):
    #print(grid[0][0])
    if not unblocked(grid[0][0]): return []
    slopes=[(-1,1)]
    d=1
    newSlopes=[]
    res=[(0,0)]
    while len(slopes)>0:
        print(slopes)
        for (l,u) in slopes:
            i=math.floor((d+.5 if l<0 else d-.5)*l+.5)
            ui=math.ceil((d+.5 if u>0 else d-.5)*u-.5)
            print((i,ui))
            while i<=ui:
                if unblocked(grid[d][i]):
                    res.append((d,i))
                else:
                    nu = (i-.5)/(d-.5) if i<=0 else (i-.5)/(d+.5)
                    if nu>l:
                        newSlopes.append((l,nu))
                    l = (i+.5)/(d-.5) if i>=0 else (i+.5)/(d+.5)
                i+=1
            if u>l:
                newSlopes.append((l,u))
        d+=1
        slopes=newSlopes
        newSlopes=[]
    return res
def lineOfSight(grid, x, y, unblocked=lambda a: a==0):
    g=grid.recentered(x,y)
    res=[]
    for i in range(4):
        res+=[translated(rotated(p,-i),(x,y)) for p in lsRight(g.rotated(i),unblocked)]
    return res
'''grid = [
    [0,0,0,0,0,0],
    [0,0,0,0,0,0],
    [0,0,0,1,0,0],
    [0,0,0,0,0,0],
    [0,0,0,0,0,0],
    [0,0,0,0,0,0]]
for (i,j) in lineOfSight(grid,0,3,lambda a: a==0):
    grid[i][j]=2
for l in grid:
    print(l)'''
import random
g=grid.of([[0]*21 for i in range(21)])
for i in range(50):
    g[random.randint(0,20)][random.randint(0,20)]=1
g[10][10]=0
for (i,j) in lineOfSight(g,10,10,lambda a: a==0):
    g[i][j]=2
print(g)

        
