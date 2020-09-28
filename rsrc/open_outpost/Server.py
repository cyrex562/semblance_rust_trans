'''
Server for Open Outpost
Some code borrowed from sjbrown tutorial: http://ezide.com/games/writing-games.html
'''

# Network code yah!
from twisted.spread import pb
from twisted.spread.pb import DeadReferenceError
from twisted.internet import reactor, defer
from twisted.cred import checkers, portal
from zope.interface import implements
from twisted.cred.checkers import InMemoryUsernamePasswordDatabaseDontUse

# Game Modules
from Network import Game
from Events import EventManager, ClientConnectEvent, ClientDisconnectEvent, EndTurnRequest


class NetworkClientController(pb.Avatar):
	'''
	Controller which collects actions from the clients
	and proccesses them accordingly.  There is an instance of this class
	for each conencted client.
	
	Clients can call any method that starts with perspective_
	
	If they want to hack accounts I am going to make them work for it damn it!
	
	We RECEIVE events from the CLIENT through this object
	There is an instance of NetworkClientController for each connected
	client.
	'''
	#def __init__(self, username, server, clientRef, password = ''):
	def __init__(self, evManager, avatarID, realm):
		self.avatarID = avatarID
		self.realm = realm
		self.evManager = evManager
		self.evManager.registerListener(self)
	#	self.name = username
	#	self.server = server
	#	self.remote = clientRef
	#	self.password = password
	def Notify(self, event):
		pass
	def logout(self):
		print self, ' logging out'
	def perspective_getGame(self):
		deferred = factory.getRootObject()
		deferred.addCallback(self.game.send)
	def perspective_clientConnect(self, netClient):
		print 'Client connected callback a success'
		ev = ClientConnectEvent( netClient, self.avatarID )
		self.evManager.post( ev )
		self.evManager.post('sendGame')
	def perspective_eventOverNetwork(self, event):
		if isinstance(event, EndTurnRequest):
			ev = EndTurnRequest(self.avatarID)
			self.evManager.post(ev)


class Realm(object):
	'''
	Factory which accepts new client connections and create new NetworkServerViews/Controllers for each connection
	'''
	implements(portal.IRealm)
	def __init__(self, evManager):
		self.evManager = evManager
		
	def requestAvatar(self, avatarID, mind, *interfaces):
		print 'Requesting avatar id: ', avatarID
		
		if pb.IPerspective in interfaces:
			avatar = NetworkClientController(self.evManager, avatarID, self)
			return pb.IPerspective, avatar, avatar.logout
		else:
			print 'Failure in avatar request'
			raise NotImplementedError("no interface")
	
class NetworkClientView(object):
	'''
	We send information to the network clients with this.
	We call on the clients to do functions by the following process:
	TODO: Create event manager (use old pygame prototype's system)
		1) Network Client View recieves event
		2) NCV.Notify() logic will do a callback for one of the view's functions by something similar to 
				def phase1(self, remote):
					self.remote = remote
					d = remote.callRemote("takePond", self.pond)
					d.addCallback(self.phase2).addErrback(log.err)
	'''
	def __init__(self, evManager, game):
		self.evManager = evManager
		self.evManager.registerListener(self)
		self.game = game
		self.clients = {}
	#----------------------------------------------------------------------
	def RemoteCallError(self, failure, client):
		from twisted.internet.error import ConnectionLost
		#trap ensures that the rest will happen only 
		#if the failure was ConnectionLost
		failure.trap(ConnectionLost)
		self.HandleFailure(client)
		return failure
	def RemoteCall(self, client, functionName, *args):
		'''
		This will attempt to call client functions remotly
		'''
		try:
			remoteCall = client.callRemote(functionName, *args)
			# If we want something to happen when this occures (and break MVC)
			# remoteCall.addCallback(self.function)
			remoteCall.addErrback( self.RemoteCallError, client )
		except DeadReferenceError:
			self.HandleFailure(client)
	
	def Notify(self, event):
		'''
		Event Logic Parser
		'''
		if isinstance( event, ClientConnectEvent ):
			print "\nADDING CLIENT", event.client
			self.clients[event.avatarID] = event.client
		elif isinstance( event, ClientDisconnectEvent ):
			del self.clients[event.avatarID]

		ev = event
		
		if ev == 'endTurnEvent':
			for avatarID, client in self.clients.items():
				print "\n====server===sending: ", str(ev), 'to',
				print avatarID, '(', client, ')'
				self.RemoteCall( client, "endTurnEvent", ev )
		if ev == 'sendGame':
			#deferred = factory.getRootObject()
			#deferred.addCallback(sender.phase1)
			print 'In send Game logic'
			#deferred = defer.Deferred()
			#deferred.addCallback(self.sendGame)
			
			for avatarID, client in self.clients.items():
				print avatarID, client
				print "\n====server===sending: ", str(ev), 'to',
				print avatarID, '(', client, ')'
				self.RemoteCall( client, "getGame", self.game )
			'''
			Old method.  This will probably be more ideal instead of using twisted's cachable'
			#NOTE: this is very "chatty".  We could restrict 
			#      the number of clients notified in the future
			# or transmit to clients only what they can see
			if not self.clients:
			print '\n========= STILL NO CLIENTS'
			for avatarID, client in self.clients.items():
			print "\n====server===sending: ", str(ev), 'to',
			print avatarID, '(', client, ')'
			self.RemoteCall( client, "ServerEvent", ev )
			'''
	def sendGame(self, remote):
		print 'sendGame(',remote,')'
		remote.callRemote(getGame, self.game)
		

def main():
	eventManager = EventManager()
	
	game = Game(eventManager)
	realm = Realm(eventManager)
	portl = portal.Portal(realm)
	checker = InMemoryUsernamePasswordDatabaseDontUse(user1 = 'pass1')
	#checker.addUser("user1", "pass")
	
	clientView = NetworkClientView(eventManager, game)
	portl.registerChecker(checker)	
	reactor.listenTCP(1999, pb.PBServerFactory(portl))
	reactor.run()

if __name__ == '__main__':
	main()
