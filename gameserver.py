import base
import number_guess
import area_attack
import socket
import disconnected
import extended
import acceleration
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
t = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
#addr='192.168.236.118'
#addr='192.168.1.240'
#s.bind(('10.144.119.197',80))
#s.bind(('192.168.1.240',8000))
addr='localhost'
#addr='192.168.0.37'
#addr = '192.168.1.240'
#addr = '192.168.137.1'
#addr='192.168.100.236'
s.bind((addr,8000))
t.bind((addr,80))
#s.bind(('172.20.10.2',8000))
s.listen()
s.setblocking(False)
t.listen()
t.setblocking(False)
with open(r'test.html') as f:
    clientCode=f.read()
games = {}
cList=[]
waitroom=[]
pList=[]
pDict={}
while True:
    try:
        (connection,address)=t.accept()
        connection.send(("""HTTP/1.1 200 OK

"""+clientCode).encode())
    except BlockingIOError:
        pass
    try:
        (connection,address)=s.accept()
        cList.append(connection)
    except BlockingIOError:
        pass
    for c in cList:
        try:
            code=c.recv(1)
            print(code)
            if code==b'l':
                cList.remove(c)
                c=base.lengthPlayer(c)
                waitroom.append(c)
            if code==b'G':
                handshake=c.recv(1000)
                cList.remove(c)
                c=base.websocketPlayer(c,handshake)
                print('ws')
                waitroom.append(c)
            c.send(b'Send your name.')
        except BlockingIOError:
            pass
        except:
            pass
    for p in waitroom:
        try:
            name=p.receive()
            if name:
                print(name)
                name=name.decode()
                if '\n' in name:
                    p.send(b'Names cannot contain newlines.')
                elif name in pDict:
                    #pDict[name].reconnect(p)
                    p.send(b'This name is already in use.')
                    waitroom.remove(p)
                else:
                    p.name=name
                    waitroom.remove(p)
                    pList.append(p)
                    pDict[name]=p
        except BlockingIOError:
            pass
        except Exception as e:
            print(e)
            print('Connection closed?')
            waitroom.remove(p)
            pass
    for p in pList:
        try:
            msg=p.receive()
            if msg:
                code=msg[0]
                msg=msg[1:]
                print(code)
                if code==b'n'[0]:
                    l=msg.split(b'\n',2)
                    print(l)
                    name=l[0].decode()
                    typ=l[1].decode()
                    data=l[2]
                    if name in games:
                        p.send(b'This name is already in use.')
                    if typ not in base.gameTypes:
                        p.send(b'This game type is not supported.')
                    else:
                        games[name]=base.gameTypes[typ](data)
                if code==b'j'[0]:
                    name=msg.decode()
                    if games[name].addPlayer(p):
                        pList.remove(p)
                    else:
                        p.send(b'Rejected from game')
                if code==b'g'[0]:
                    bl=b''
                    for name in games:
                        bl+=name.encode()+b'\n'
                    bl=bl[:-1]
                    p.send(bl)
        except BlockingIOError:
            pass
        except Exception as e:
            #raise e
            print(e)
            print('Connection closed?')
            pList.remove(p)
            del pDict[p.name]
            pass
    deleted=[]
    for g in games:
        games[g].update()
        for p in games[g].playerList:
            try:
                d=p.receive()
                if d:
                    if d[0]==0:
                        games[g].playerList.remove(p)
                        pList.append(p)
                        if len(games[g].playerList)==0:
                            deleted.append(g)
                    elif d[0]==1:
                        games[g].process(p,d[1:])
                    else:
                        games[g].process(p,d)
            except BlockingIOError:
                pass
            except Exception as e:
                #raise e
                print(e)
                print('Connection closed?')
                #p.disconnect()
                games[g].playerList.remove(p)
                del pDict[p.name]
    for g in deleted:
        del games[g]
