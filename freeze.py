from minesweeper import *
print(inBoard)
def isFrozen(self,p):
    if p.frozen:
        if time.time()-p.frozen>=self.freezeTime:
            p.frozen=False
            return False
        return True
def freezeLegalMove(self,p,x,y):
    if p.spectating:
        return False
    if not self.started:
        message(p,'Wait for all players to choose a starting square.')
        return False
    if not inBoard(self,x,y):
        return False
    if isFrozen(self,p):
        message(p,'You are frozen.')
        return False
    return True
def freezeMineSelect(self,p,x,y):
    p.frozen=time.time()
    for player in self.playerList:
        player.send((str(x)+'\n'+str(y)+'\nf'+p.name).encode())
def init(d):
    d=d.split(b'\n')
    boardsize=int(d[0])
    pMine=float(d[1])
    g=Minesweeper(boardsize,pMine,freezeLegalMove,freezeMineSelect)
    g.freezeTime=15
    return g
gameTypes['freeze']=init
