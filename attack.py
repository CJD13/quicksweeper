from minesweeper import *
def adjacentClaimed(self,p,x,y):
    for i in range(x-1,x+2):
        for j in range(y-1,y+2):
            if inBoard(self,i,j):
                if self.board[i][j]==p.name:
                    return True
    return False


def attackLegalMove(self,p,x,y):
    if p.spectating:
        return False
    if not self.started:
        message(p,'Wait for all players to choose a starting square.')
        return False
    if not inBoard(self,x,y):
        return False
    if not adjacentClaimed(self,p,x,y):
        message(p,'You can only claim squares adjacent to your area.')
        return False
    if self.board[x][y]==None:
        return False
    return True


def removeSquare(self,x,y):
    if self.board[x][y]==1:
        self.numMines-=1
    self.board[x][y]=None
    for p in self.playerList:
        p.send((str(x)+'\n'+str(y)+'\nsN').encode())
    self.squaresRemoved+=1
def resetSquare(self,x,y):
    if self.board[x][y]==1:
        self.numMines-=1
    for p in self.playerList:
        if p.name==self.board[x][y]:
            p.squaresFound-=1
            self.squaresFound-=1
            p.send((str(x)+'\n'+str(y)+'\nsy').encode())
        else:
            p.send((str(x)+'\n'+str(y)+'\nso').encode())
    self.board[x][y]=0
def attackMineSelect(self,p,x,y):
    removeSquare(self,x,y)
    for i in range(x-3,x+4):
        for j in range(y-3,y+4):
            if inBoard(self,i,j) and self.board[i][j]!=None:
                resetSquare(self,i,j)
                if random.random()<self.pMine:
                    self.board[i][j]=1
                    self.numMines+=1
    for i in range(x-4,x+5):
        for j in range(y-4,y+5):
            if inBoard(self,i,j) and self.board[i][j]!=None:
                if abs(x-i)==4 or abs(y-j)==4:
                    for player in self.playerList:
                        if player.name==self.board[i][j]:
                            player.send((str(i)+'\n'+str(j)+'\nc'+str(mineCount(self,i,j))).encode())
def init(d):
    d=d.split(b'\n')
    boardsize=int(d[1])
    pMine=float(d[2])
    g=Minesweeper(boardsize,pMine,attackLegalMove,attackMineSelect)
    g.pMine=pMine
    return g
gameTypes['attack']=init
