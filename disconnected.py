import base
import freeze
import minesweeper
import random
def disconnectedClaim(self,n,x,y):
    self.board[x][y]=n
    for player in self.playerList:
        if player.name==n:
            player.send((str(x)+'\n'+str(y)+'\nc').encode())
        else:
            player.send((str(x)+'\n'+str(y)+'\no'+str(n)).encode())
    remaining=self.boardsize*self.boardsize-self.squaresFound-self.numMines-self.squaresRemoved
    num=0
    for i in range(self.boardsize):
        for j in range(self.boardsize):
            if self.board[i][j]==0:
                if random.randint(1,remaining-num)==1:
                    self.board[i][j]=n
                    for player in self.playerList:
                        if player.name==n:
                            player.squaresFound+=1
                            self.squaresFound+=1
                            player.send((str(i)+'\n'+str(j)+'\nc'+str(minesweeper.mineCount(self,i,j))).encode())
                        else:
                            player.send((str(i)+'\n'+str(j)+'\no'+str(n)).encode())
                    return
                else:
                    num+=1
def init(d):
    d=d.split(b'\n')
    boardsize=int(d[0])
    pMine=float(d[1])
    g=minesweeper.Minesweeper(boardsize,pMine,freeze.freezeLegalMove,freeze.freezeMineSelect,disconnectedClaim)
    g.freezeTime=15
    remaining=g.boardsize*g.boardsize-g.squaresFound-g.numMines-g.squaresRemoved
    n=0
    r=int(.1*boardsize**2)
    for i in range(boardsize):
        for j in range(boardsize):
            if g.board[i][j]==0:
                if random.randint(1,remaining-n)<=r:
                    g.board[i][j]=-1
                    g.squaresRemoved+=1
                    r-=1
                n+=1
    return g
base.gameTypes['disconnected']=init
