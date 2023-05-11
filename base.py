import random
import base64
import hashlib
import time
gameTypes={}
class game:
    def __init__(self, addPlayer, process, update):
        self.process=lambda p,d: process(self,p,d)
        self.addPlayer=lambda p: addPlayer(self, p)
        self.update=lambda:update(self)
        self.playerList=[]
        self.playersJoined=0
class player:
    def __init__(self, connection, send, receive):
        self.connection=connection
        self.receive=lambda:receive(self)
        self.send=lambda msg:send(self,msg)
    def disconnect(self):
        self.messageQueue=[]
        self.send=lambda data: self.messageQueue.append(data)
        self.receive=lambda: False
    def reconnect(self, other):
        self.connection=other.connection
        self.send=other.send
        self.receive=other.receive
        for m in self.messageQueue:
            self.send(m)
def lengthPlayer(connection):
    def send(self, msg):
        l=len(msg)
        ll=[]
        while l>0:
            ll.append(l%255)
            l//=255
        b=bytes(ll[-1-i] for i in range(len(ll)))
        b+=b'\xff'+msg
        self.connection.send(b)
    def receive(self):
        try:
            l=0
            b=self.connection.recv(1)[0]
            while b!=255:
                l=255*l+b
                b=self.connection.recv(1)[0]
            msg=self.connection.recv(l)
            return msg
        except BlockingIOError:
            return False
    return player(connection, send, receive)
def websocketPlayer(connection, handshake):
    d = [l.split(': ') for l in handshake.decode().split('\r\n')]
    d = {l[0]:l[-1] for l in d}
    response='''HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: '''
    m=hashlib.sha1()
    m.update((d['Sec-WebSocket-Key']+"258EAFA5-E914-47DA-95CA-C5AB0DC85B11").encode())
    response+=base64.b64encode(m.digest()).decode()+'\r\n\r\n'
    connection.send(response.encode())
    def send(self, msg):
        self.connection.send(b'\x81'+bytes([len(msg)])+msg)
    def receive(self):
        try:
            code=self.connection.recv(2)
            print(code)
            l=code[1]%128
            if code[0]%16==8:
                print('Connection closed')
                raise Exception
            if l==126:
                l=self.connection.recv(2)
                l=256*el[0]+el[1]
            elif l==127:
                l=self.connection.recv(4)
                l=l[3]+256*l[2]+65536*l[1]+16774656*l[0]
            if code[1]//128:
                mask=self.connection.recv(4)
            else:
                mask=b'\x00\x00\x00\x00'
            data=self.connection.recv(l)
            data=bytes(data[i]^mask[i%4] for i in range(len(data)))
            print(data)
            if code[0]%16==10:
                self.connection.send(b'\x8A'+bytes[l]+data)
                return False
            else:
                return data
        except BlockingIOError:
            return False
    return player(connection,send,receive)
