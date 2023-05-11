#Tools for standard minesweeper variants in release 1
from base import *
def claimDefault(self,n,x,y):
    self.board[x][y]=n
    for player in self.playerList:
        if player.name==n:
            player.send((str(x)+'\n'+str(y)+'\nc'+str(mineCount(self,x,y))).encode())
        else:
            player.send((str(x)+'\n'+str(y)+'\no'+str(n)).encode())
def Minesweeper(boardsize, pMine, isLegal, mineSelect,claimSquare=claimDefault):
    def addPlayer(self, p):
        p.send(('multiplayer minesweeper\n'+str(boardsize)+'\n'+str(pMine)).encode())
        if self.started:
            message(p,'This game has already started. You are a spectator.')
            p.spectating=True
            p.started=True
        else:
            p.spectating=False
            p.started=False
            for q in self.playerList:
                if q!=p:
                    q.send(('0\n0\nj'+p.name).encode())
        self.playersJoined+=1
        p.frozen=False
        p.squaresFound=0
        self.playerList.append(p)
        for q in self.playerList:
            if q!=p:
                p.send(('0\n0\nj'+q.name).encode())
        for x in range(self.boardsize):
            for y in range(self.boardsize):
                if self.board[x][y]==None:
                    p.send((str(x)+'\n'+str(y)+'\nsN').encode())
                elif self.board[x][y]==-1:
                    p.send((str(x)+'\n'+str(y)+'\nc'+str(mineCount(self,x,y))).encode())
                elif self.board[x][y]!=0 and self.board[x][y]!=1:
                    p.send((str(x)+'\n'+str(y)+'\no'+self.board[x][y]).encode())
        return True
    def process(self,p,d):
        l=d.split(b'\n')
        x=int(l[0])
        y=int(l[1])
        if not p.started:
            initSelection(self,p,x,y)
            return
        if not self.isLegal(self,p,x,y):
            return False
        if self.board[x][y]==1:
            self.mineSelect(self,p,x,y)
        elif self.board[x][y]==0 or self.board[x][y]==p.name:
            if self.board[x][y]==0:
                p.squaresFound+=1
                self.squaresFound+=1
            claimSquare(self,p.name,x,y)
            if self.squaresFound+self.numMines+self.squaresRemoved==self.boardsize*self.boardsize:
                for player in self.playerList:
                    message(player,'Game over. Your score was '+str(player.squaresFound))
        elif self.board[x][y]==-1:
            pass
        elif self.board[x][y]!=None:
            p.send((str(x)+'\n'+str(y)+'\no'+self.board[x][y]).encode())
    def initSelection(self,p,x,y):
        if self.board[x][y]==1 or self.board[x][y]==0:
                for i in range(x-2,x+3):
                    if i>=0 and i<=self.boardsize:
                        for j in range(y-2,y+3):
                            if j>=0 and j<self.boardsize:
                                if self.board[i][j]==1:
                                    self.board[i][j]=0
                                    self.numMines-=1
                self.board[x][y]=p.name
                p.squaresFound+=1
                self.squaresFound+=1
                p.started=True
                p.startM=str(x)+'\n'+str(y)+'\n'
                for player in self.playerList:
                    player.send((p.startM+'o'+p.name).encode())
                self.started=True
                for player in self.playerList:
                    if not player.started:
                        self.started=False
                if self.started:
                    for player in self.playerList:
                        player.send((player.startM+'c'+'0').encode())
    g = game(addPlayer, process,lambda self:None)
    g.started=False
    g.numMines=0
    g.squaresFound=0
    g.squaresRemoved=0
    g.board=[[0]*boardsize for i in range(boardsize)]
    g.boardsize=boardsize
    g.isLegal=isLegal
    g.mineSelect=mineSelect
    for i in range(boardsize):
        for j in range(boardsize):
            if random.random()<pMine:
                g.board[i][j]=1
                g.numMines+=1
    return g
def message(p,m):
    p.send(('0\n0\nm'+m).encode())
def inBoard(self,x,y):
    return 0<=x and x<self.boardsize and 0<=y and y<self.boardsize
def mineCount(self,x,y):
    c=0
    for i in range(x-1,x+2):
        for j in range(y-1,y+2):
            if inBoard(self,i,j):
                if self.board[i][j]==1:
                    c+=1
    return c
