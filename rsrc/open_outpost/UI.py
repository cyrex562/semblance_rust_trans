'''
Classes and functions for the user interface
'''

from GUI.controls import Form, Vec2, Button
from pandac.PandaModules import CollisionTraverser,CollisionHandlerQueue,CollisionNode,CollisionRay,GeomNode
from direct.showbase import DirectObject
import math
import Meshes
import Menu

# Tree GUI
from GUI.core import GUI
from GUI.theme import Theme

#from direct.showbase import DirectObject


class AIMenu(Form):   
	""" 
		this is a free floating form on which to put
		widgits or controls  
	
	""" 
	def __init__(self):
		# call the super constructor 
		# all gui objects take pos and size as attributes 
		# and many also take default text as the first operand
		Form.__init__(self,"AI name here",pos=Vec2(45,45),size=Vec2(200,400))
		# adds window to the gui - 
		# you can add windows to most container objects
		self.endTurn = self.add(Button("End Turn",pos=Vec2(80,40),onClick=self.endTurnRequest))
		gui.add(self) 
	
	def endTurnRequest(self,button,key,mouse):
		print 'End Turn Requested'
		messenger.send('endTurnRequestClick')
		

class Picker(DirectObject.DirectObject):
	def __init__(self,ancestor=None):
		print "____________________________________________________"
		print "Class Picker"
		self.ancestor=ancestor
		#create traverser
		base.cTrav = CollisionTraverser()
		#create collision ray
		self.createRay(self,base.camera,name="mouseRay",show=True)
		#initialize mousePick
		#self.accept('mouse1-up', self.mousePick, [1,self.queue])
		#initialize mouseRightPick
		self.accept('mouse3', self.mousePick, [3,self.queue])
	
	#======================================================================
	def mouseRight(self,pickedObj,pickedPoint):
		print 'Right click'
		if pickedObj==None: 
			print 'Dont mean a thing if you aint picked a thing'
			return
		#get cell from pickedpoint
		cell=(int(math.floor(pickedPoint[0])),int(math.floor(pickedPoint[1])))
		#trace updated info about the cell
		print str(self.ancestor.grid.data[cell[0]][cell[1]].getDict())
		self.ancestor.txt.setText(str(self.ancestor.grid.data[cell[0]][cell[1]].getDict()))  
			
	#======================================================================
	def mouseLeft(self,pickedObj,pickedPoint):
		return
		if pickedObj==None: return
		#actions for AVATAR: select
		#------------------------------
		if pickedObj.getTag('pickable')=='avatar':
			av=pickedObj.getPythonTag('class')
			self.ancestor.avatars.select(av.num)
		#actions for TERRAIN: find path 
		#------------------------------
		if pickedObj.getTag('pickable')=='terrain': 
			av=self.ancestor.avatars.av
			if av.isMoving==False:
				av.choosePath(pickedPoint) 
			else:
				av.willMove=pickedPoint

	#======================================================================
	"""mouse pick"""    
	def mousePick(self, but, queue):
		#get mouse coords
		if base.mouseWatcherNode.hasMouse()==False: return
		mpos=base.mouseWatcherNode.getMouse()
		#locate ray from camera lens to mouse coords
		self.rayNP.reparentTo(base.camera)
		self.ray.setFromLens(base.camNode, mpos.getX(),mpos.getY())
		#get collision: picked obj and point
		pickedObj,pickedPoint=self.getCollision(queue)
		#call appropiate mouse function (left or right)
		if but==1:self.mouseLeft(pickedObj,pickedPoint)
		if but==3:self.mouseRight(pickedObj,pickedPoint)
		
	#======================================================================
	"""Returns the picked nodepath and the picked 3d point"""
	def getCollision(self, queue):
		#do the traverse
		base.cTrav.traverse(render)
		#process collision entries in queue
		if queue.getNumEntries() > 0:
			queue.sortEntries()
			for i in range(queue.getNumEntries()):
				collisionEntry=queue.getEntry(i)
				pickedObj=collisionEntry.getIntoNodePath()
				#iterate up in model hierarchy to found a pickable tag
				parent=pickedObj.getParent()
				for n in range(4):
					if parent.getTag('pickable')!="" or parent==render: break
					parent=parent.getParent()
				#return appropiate picked object
				if parent.getTag('pickable')!="":
					pickedObj=parent
					pickedPoint = collisionEntry.getSurfacePoint(pickedObj)
					#pickedNormal = collisionEntry.getSurfaceNormal(self.ancestor.worldNode)
					#pickedDistance=pickedPoint.lengthSquared()#distance between your object and the collision
					return pickedObj,pickedPoint         
		return None,None
	
	#======================================================================
	"""sets nodepath pickable state"""
	def makePickable(self,newObj,tag='true'):
		newObj.setTag('pickable',tag)
		#print "pickable: ",newObj,"as",tag
	
	#======================================================================
	"""creates a ray for detecting collisions"""
	def createRay(self,obj,ent,name,show=False,x=0,y=0,z=0,dx=0,dy=0,dz=-1):
		#create queue
		obj.queue=CollisionHandlerQueue()
		#create ray  
		obj.rayNP=ent.attachNewNode(CollisionNode(name))
		obj.ray=CollisionRay(x,y,z,dx,dy,dz)
		obj.rayNP.node().addSolid(obj.ray)
		obj.rayNP.node().setFromCollideMask(GeomNode.getDefaultCollideMask())
		base.cTrav.addCollider(obj.rayNP, obj.queue) 
		if show: obj.rayNP.show()

class ContextMenu(DirectObject.DirectObject):
	def __init__(self, parent, structureDatabase):
		self.parent = parent
		self.accept('mouse3', self.createContexMenu)
		self.structures = structureDatabase
		print 'SDB', structureDatabase, self.structures
	def createContexMenu(self):
		'''
		Generates the right click menu.  Will eventually be contex sensitive.
		'''
		
		myPopupMenu=Menu.PopupMenu(
			items = (('End Turn', False, self.endTurnRequest),
			         ('Build...', False, self.buildBuildableList())),
			parent = self.parent,
			baselineOffset=-.35,
			scale=.045, itemHeight=1.2, leftPad=.2,
			separatorHeight=.3,
			underscoreThickness=1,

			BGColor=(.9,.9,.8,.94),
			BGBorderColor=(.8,.3,0,1),
			separatorColor=(0,0,0,1),
			frameColorHover=(.3,.3,.3,1),
			frameColorPress=(0,1,0,.85),
			textColorReady=(0,0,0,1),
			textColorHover=(1,.7,.2,1),
			textColorPress=(0,0,0,1),
			textColorDisabled=(.65,.65,.65,1)
		)
	def buildBuildableList(self):
		'''
		Generates the list of buildabel buildings in the right click menu.
		Returns ()
		'''
		structureCache = []
		for structure in self.structures:
			if self.structures[structure]['build']:
				structureCache.append((self.structures[structure]['english']['name'], None, self.buildStructure, structure))
		return tuple(structureCache)
	def endTurnRequest(self):
		print 'End Turn Requested'
		messenger.send('endTurnRequestClick')
	def buildStructure(self, structureID):	
		print 'Build Structure:', structureID

class UI(object):
	'''
	Experiemtnal class for the entire planet GUI.
	Right now no worky
	'''
	def __init__(self):
		self.gui = GUI(theme=Theme("TreeGUI/data/affgui.png"))
		# create the form
		self.aiMenu = AIMenu()
		self.contexMenu = ContextMenu(self.gui.node)
		
