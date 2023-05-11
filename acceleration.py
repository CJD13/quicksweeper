import base
import math
import random
import time
import vector
def update(self):
    dt=time.time()-self.lastTime
    self.lastTime+=dt
    for p in self.playerList:
        p.pos+=p.vel*dt
        p.vel+=p.acc*dt
    for p in self.playerList:
        if p.pos.coords[0]<0 or p.pos.coords[0]>self.size or p.pos.coords[1]<0 or p.pos.coords[1]>self.size:
            fail=True
            while fail:
                p.pos=vector.vector([random.random(),random.random()])*self.size
                fail = False
                for q in self.playerList:
                    if q!=p:
                        if abs(q.pos-p.pos)<q.r+p.r:
                            fail=True
            p.vel=vector.zero(2)
        for q in self.playerList:
            if q!=p:
                disp=q.pos-p.pos
                
                if abs(disp)<p.r+q.r:
                    rVel=p.vel-q.vel
                    parD=rVel.project(disp)
                    perD=disp-parD
                    bT=(math.sqrt((p.r+q.r)**2-abs(perD)**2)-abs(parD))/abs(rVel)
                    p.pos-=p.vel*bT
                    q.pos-=q.vel*bT
                    disp=q.pos-p.pos
                    radVel=disp.project(rVel)
                    p.vel-=radVel*(2*q.m/(p.m+q.m))
                    q.vel+=radVel*(2*p.m/(q.m+p.m))
                    p.pos+=p.vel*bT
                    q.pos+=q.vel*bT
    if self.lastTime-self.lastSend>self.sendFreq:
        self.lastSend=self.lastTime
        for p in self.playerList:
            m=p.name.encode()+b'\n'+p.pos.toBytes()
            for q in self.playerList:
                q.send(m)
def process(self,p,d):
    d=d.decode().split('\n')
    a=vector.vector([float(d[0]),float(d[1])])
    if abs(a)>p.maxAccel:
        a=a*p.maxAccel/abs(a)
    p.acc=a
def addPlayer(self,p):
    if len(self.playerList)*4*math.pi>self.size*self.size:
        return False
    p.send(('acceleration\n'+str(self.size)).encode())
    p.m=self.pMass
    p.r=self.pr
    p.maxAccel=self.maxAccel
    p.vel=vector.zero(2)
    fail=True
    while fail:
        p.pos=vector.vector([random.random(),random.random()])*self.size
        fail = False
        for q in self.playerList:
            if q!=p:
                if abs(q.pos-p.pos)<q.r+p.r:
                    fail=True
    p.acc=vector.zero(2)
    self.playerList.append(p)
    return True
def init(d):
    d=[int(i) for i in d.decode().split('\n')]
    g=base.game(addPlayer,process,update)
    g.pMass=1
    g.maxAccel=d[1]
    g.pr=1
    g.size=d[0]
    g.lastTime=time.time()
    g.lastSend=time.time()
    g.sendFreq=.01
    return g
base.gameTypes['acceleration']=init
