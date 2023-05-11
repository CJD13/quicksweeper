import base
import minesweeper
import freeze
def extendedClaim(self,n,x,y):
    self.board[x][y]=n
    for player in self.playerList:
        if player.name==n:
            player.send((str(x)+'\n'+str(y)+'\nc'+str(extendedMineCount(self,x,y))).encode())
        else:
            player.send((str(x)+'\n'+str(y)+'\no'+str(n)).encode())
def extendedMineCount(self,x,y):
    c=0
    for i in range(x-2,x+3):
        for j in range(y-2,y+3):
            if minesweeper.inBoard(self,i,j):
                if self.board[i][j]==1:
                    c+=1
    return c
def init(d):
    d=d.split(b'\n')
    boardsize=int(d[0])
    pMine=float(d[1])
    g=minesweeper.Minesweeper(boardsize, pMine, freeze.freezeLegalMove, freeze.freezeMineSelect, extendedClaim)
    g.freezeTime=15
    return g
base.gameTypes['extended']=init
