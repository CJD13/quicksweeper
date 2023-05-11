from base import *
def guessTheNumber(lower, upper):
    def addPlayer(self,p):
        self.playerList.append(p)
        self.playersJoined+=1
        p.send(b'Guess the number!')
        return True
    def process(self,p,d):
        d=int(d)
        if d==self.n:
            p.send(('You win! The number was '+str(self.n)+'. Choosing a new number...').encode())
            self.n=random.randint(lower,upper)
        else:
            p.send(b'Incorrect')
    g=game(addPlayer, process)
    g.n=random.randint(lower,upper)
    return g
def init(d):
    d=d.split(b'\n')
    return guessTheNumber(int(d[0]),int(d[1]))
gameTypes['number guess']=init
