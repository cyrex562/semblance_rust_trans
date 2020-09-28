'''
Classes for the network system.  I think. We'll see how it goes with twisted.
'''

from twisted.spread import pb
from direct.showbase import DirectObject

from Events import EndTurnRequest


class Game(pb.Cacheable):
	'''
	Master game server.
	Manages the entire universe state
	'''
	def __init__(self, evManager):
		'''
		self.players:	{userobject: [loggedin BOOL, endTurnRequest BOOL]}
		'''
		self.evManager = evManager
		self.evManager.registerListener(self)
		self.turn = 0
		self.players = {}	
		self.maxPlayers = 2
		
		self.structureTypes={}	# {'Foldername': {structure defs form yaml}}
		self.loadStructures()
		
		# TODO: Colonies will eventually be in Planets
		self.colonies = []

	def Notify(self, event):
		if isinstance(event, EndTurnRequest):
			# TODO: Submit request and set flag, not direct end turn
			# TODO: Check 
			self.endTurn()
		
	def getStateToCacheAndObserveFor(self, perspective, observer):
		'''
		Is called when a remote caching reference is first created, and retrieves the data with which the cache is first filled. It also provides an object called the "observer"4 that points at that receiver-side cache. Every time the state of the object is changed, you give a message to the observer, informing them of the change. 
		'''
		self.players[observer] = [True, False]
		return self.turn
	def stoppedObserving(self, perspective, observer):
		'''is called when the remote cache goes away, so that you can stop sending updates
		'''
		print 'Ending chache'
		#self.players[observer][0] = False	
	def login(self, player):
		if user in self.players:
			# TODO: Make sure the user isn't already logged in
			# TODO: Auto set to 0 if user drops
			self.players[player][0] = True
		else:
			self.addPlayer(player)	
	def start(self):
		pass	
	def addPlayer(self, player):
		self.players[player] = [1,0]	
	def endTurn(self):
		'''
		All End turn calculations and cleanup is here
		'''
		print 'End of turn ', self.turn
		print self.players
		self.turn = self.turn + 1
		for player in self.players:
			player.callRemote('endTurn')
		print 'Start of turn ', self.turn
	def send(self, remote):
		deferred = remote.callRemote('takeTurn', self.turn)
	
class PhonyGame(pb.RemoteCache, DirectObject.DirectObject):
	'''
	Class for the client copy of the server's authoratative model'
	Methods to call start with observe_ (like perspective_)
	'''
	
	def __init__(self):
		'''
		Sets up event handeling
		'''
		pass
		#self.accept('Logged In')
	
	def setCopyableState(self, turn):
		print 'I have aquired the game state.'
		print 'Turn:', turn
		self.turn = turn
	
	def observe_endTurn(self):
		'''
		All End turn calculations and cleanup is here
		'''
		print 'End of turn ', self.turn
		self.turn = self.turn + 1
		print 'Start of turn ', self.turn
	
	def observe_takeTurn(self, turn):
		self.turn = turn

pb.setUnjellyableForClass(Game, PhonyGame)

class NetworkServerView(pb.Root, DirectObject.DirectObject):
	'''
	We send events FROM client TO server with this bad boy.
	'''
	STATE_PREPARING = 0
	STATE_CONNECTING = 1
	STATE_CONNECTED = 2
	STATE_DISCONNECTING = 3
	STATE_DISCONNECTED = 4
	
	def __init__(self):
		self.pbClientFactory = pb.PBClientFactory()
		self.reactor = None
		self.server = None
		self.state = NetworkServerView.STATE_PREPARING
		self.accept('endTurnRequestClick', self.endTurnRequest)
	def setServer(self, server):
		print 'Server', server, 'is set.'
		self.server = server
		messenger.send('loggedIn', [server])
		#defrd = server.callRemote("clientConnect")
		#defrd.addErrback(self.ServerErrorHandler)
	def endTurnRequest(self):
		'''
		We create a server compatable event requesting the end of turn.
		'''
		print 'end turn request client'
		ev = EndTurnRequest(None)
		if self.server:
			print 'Client sending', ev
			remoteCall = self.server.callRemote("eventOverNetwork", ev)
		else:
			print 'Cannot send while disconnected'

#class NetworkServerController(pb.Root):
class NetworkServerController(pb.Referenceable, DirectObject.DirectObject):
	'''
	We recieve events formt he server into this object'''
	#pb.Referenceable?
	def __init__(self):
		self.accept('loggedIn', self.registerController)
	def remote_getGame(self, game):
		print 'I got a game!'
		self.game = game
	def registerController(self, server):
		'''
		Registers this controller with Client View Object
		'''
		print 'Attempting to register'
		defrd = server.callRemote("clientConnect", self)
		defrd.addErrback(self.ServerErrorHandler)
		
