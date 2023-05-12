import base
import maze
import minesweeper
import math
#returns all squares which can be seen from the point at the center of square (x,y)
def lineOfSight(grid,x,y,unblocked=lambda a: a==0 or a==1):
    if not unblocked(grid[x][y]): return []
    slopes=[(-1,1)]
    d=1
    newSlopes=[]
    res=[(x,y)]
    while len(slopes)>0 and x+d<len(grid):
        print(slopes)
        for (l,u) in slopes:
            i=max(-y, math.floor((d+.5 if l<0 else d-.5)*l+.5))
            ui=min(len(grid)-y-1,math.ceil((d+.5 if u>0 else d-.5)*u-.5))
            print((i,ui))
            while i<=ui:
                if unblocked(grid[x+d][y+i]):
                    res.append((x+d,y+i))
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
grid=[[0]*201 for i in range(201)]
for i in range(5000):
    grid[random.randint(0,200)][random.randint(0,200)]=1
grid[100][100]=0
for (i,j) in lineOfSight(grid,100,100,lambda a: a==0):
    grid[i][j]=2
for l in grid:
    print(str(l)+',')

        
