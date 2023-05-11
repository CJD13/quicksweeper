import base
import time
import freeze
import attack
import maze
from minesweeper import *
def areaAttackLegalMove(self,p,x,y):
    if self.stage==0:
        if self.squaresFound+self.numMines+self.squaresRemoved>=.8*self.boardsize*self.boardsize:
            self.stage=1
            self.attackTime=time.time()
            for player in self.playerList:
                message(player,'Attack mode has started.')
            return areaAttackLegalMove(self,p,x,y)
        else:
            return freeze.freezeLegalMove(self,p,x,y)
    elif self.stage==1:
        if freeze.isFrozen(self,p):
            return False
        if time.time()-self.attackTime>60:
                for player in self.playerList:
                    message(player,'Stage 3 has started.')
                self.stage=2
                self.freezeTime=60
                self.attackEnd=self.attackTime+18
                return False
        return attack.attackLegalMove(self,p,x,y)
    elif self.stage==2:
        if freeze.isFrozen(self,p):
            return False
        if time.time()-self.attackEnd>60:
            for player in self.playerList:
                message(player,'Game over. Your score was '+str(player.squaresFound))
            return False
        return attack.attackLegalMove(self,p,x,y)
def areaAttackMineSelect(self,p,x,y):
    if self.stage==0 or self.stage==2:
        return freeze.freezeMineSelect(self,p,x,y)
    else:
        return attack.attackMineSelect(self,p,x,y)
def init(d):
    d=d.split(b'\n')
    boardsize=int(d[0])
    pMine=float(d[1])
    g=Minesweeper(boardsize,pMine,areaAttackLegalMove,areaAttackMineSelect)
    g.pMine=pMine
    g.freezeTime=15
    g.stage=0
    '''for i in range(boardsize):
        for j in range(boardsize):
            if (i-boardsize/2)**2+(j-boardsize/2)**2>boardsize**2/4:
                attack.removeSquare(g,i,j)'''
    #maze.drawMaze(g.board,None)
    return g
base.gameTypes['area attack']=init
