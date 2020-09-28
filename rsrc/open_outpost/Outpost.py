title="Open Outpost"
"""
Here it is!
"""
#setup config file
from pandac.PandaModules import loadPrcFileData
loadPrcFileData( '', 'fullscreen 0')
loadPrcFileData( '', 'win-size 800 600' )
loadPrcFileData( '', 'win-origin 450 90' )
loadPrcFileData( '', 'frame-rate-meter-scale 0.035' )
loadPrcFileData( '', 'frame-rate-meter-side-margin 0.1' )
loadPrcFileData( '', 'show-frame-rate-meter 1' )

loadPrcFileData( '', 'load-display pandagl' )
#loadPrcFileData( '', 'load-display pandadx9' )
loadPrcFileData( '', 'notify-level-util error' )
loadPrcFileData( '', 'window-title '+title )

#loadPrcFileData("", "want-directtools #t")
#loadPrcFileData("", "want-tk #t")

#loadPrcFileData( '', 'auto-generate-mipmap 1' )
#loadPrcFileData( '', 'egg-flat-shading 1' )
#loadPrcFileData("", "sync-video 0")
#loadPrcFileData("", "even-animation 1")

loadPrcFileData("", "interpolate-frames 1")
loadPrcFileData("", "clock-mode limited")
loadPrcFileData("", "clock-frame-rate 25")


#import panda modules
import direct.directbase.DirectStart
from direct.showbase import DirectObject
#from pandac.PandaModules import Camera
from pandac.PandaModules import OrthographicLens
from direct.gui.OnscreenText import OnscreenText,TextNode
from direct.gui.DirectGui import *


#import python modules
import sys

#import custom modules
from Grid import Grid
from Structure import Structure, loadStructures
from UI import AIMenu, Picker, ContextMenu, UI
from Network import PhonyGame, NetworkServerView, NetworkServerController

# Networking
from twisted.spread import pb
from twisted.internet.task import LoopingCall
from twisted.internet.selectreactor import SelectReactor
from twisted.internet.main import installReactor
from twisted.cred import credentials



#define constants
TILEPATH="Pada3DTerrain1/media/tiles/"
HPATH="Pada3DTerrain1/media/hmaps/"
HFILENAME="hmap"
CFILENAME="tile128"
SIZE=65 # has to be a 2 exponent number plus one
ALT=1.0
RETRO=True
SUBDIVIDE=3

ASSETSPATH='Assets/'
import Meshes

""" 
Based on Mavasher's implemenation for panda of the astar pygame example. 
Added 8 directions + calculating diferent terrain movement costs.
"""

class Path: 
	#it seems as if this path object is totally unnecessary- 
	#it could rather just be a List of Cells within another object.
	def __init__(self,cells, totalCost):
		self.cells = cells;
		self.totalCost = totalCost;
	def getCells(self): #function not used in the demo at any time.
		return self.cells   
	def getTotalMoveCost(self): #function not used in the demo at any time.
		return self.totalCost
########################################################################

	
########################################################################
class SQ_Location:
	"""A simple Square Map Location implementation"""
	def __init__(self,x,y):
		self.x = x
		self.y = y
	def __eq__(self, l):
		"""MUST BE IMPLEMENTED"""
		if l.x == self.x and l.y == self.y:
			return 1
		else:
			return 0    
		
########################################################################

	
########################################################################
class Cell: #Pathfinding Cell in the grid.
	def __init__(self,location,mCost,lid,parent=None):
		self.location = location # where is this cell located
		self.mCost = mCost # total move cost to reach this cell
		self.parent = parent # parent cell
		self.score = mCost # calculated score for this cell
		self.rpgCost=0
		self.lid = lid # set the location id - unique for each loc in the map
		#print "****",(self.location.x,self.location.y),self.mCost,self.score
		self.building = None	# points to the building object at this location
		
	#=======================================================================
	def __eq__(self, n):
		if n.lid == self.lid:
			return 1
		else:
			return 0
		
########################################################################

# From Lights.py
from pandac.PandaModules import Vec2,Vec3,Vec3D,VBase4
from pandac.PandaModules import Spotlight,PerspectiveLens,Fog,OrthographicLens
from pandac.PandaModules import PointLight, AmbientLight, DirectionalLight, VBase4, Vec3,NodePath
import random

class Lights:
	#======================================================================
	"""initialize"""
	def __init__(self,ancestor,lightsOn=True,showLights=False):
		print "____________________________________________________"
		print "Class Lights"   
		self.ancestor=ancestor
		
		#Initialize bg colour
		colour = (0,0,0)
		base.setBackgroundColor(*colour)
		
		if lightsOn==False: return

		# Initialise lighting
		self.alight = AmbientLight('alight')
		self.alight.setColor(VBase4(0.25, 0.25, 0.25, 1))
		self.alnp = render.attachNewNode(self.alight)
		render.setLight(self.alnp) 
		
		self.dlight = DirectionalLight('dlight')
		self.dlight.setColor(VBase4(1.0, 1.0, 0.8, 1))
		self.dlnp = render.attachNewNode(self.dlight)
		self.dlnp.setHpr(45, -45, 32)
		render.setLight(self.dlnp)
			
		## self.plight = PointLight('plight')
		## self.plight.setColor(VBase4(0.8, 0.8, 0.5, 1))
		## self.plnp = render.attachNewNode(self.plight)
		## self.plnp.setPos(32, 32, 16)
		
		## self.slight = Spotlight('slight')
		## self.slight.setColor(VBase4(1, 1, 1, 1))
		## lens = PerspectiveLens()
		## self.slight.setLens(lens)
		## self.slnp = render.attachNewNode(self.slight)
		## self.slnp.setPos(-16, -16, 4)
		## self.slnp.lookAt(64,64,0)
		
		#Initialize linear fog
		#self.fog = Fog("Fog object") 
		#self.fog.setMode(Fog.MLinear)
		#self.fog.setLinearRange(14.0,40.0)
		#self.fog.setColor(*colour)
		#base.camLens.setFar(40.0)
		#render.setFog(self.fog)

# From Camera.py
from pandac.PandaModules import NodePath,Vec3,Point3
from direct.task.Task import Task

import Meshes

########################################################################
class Camera(DirectObject.DirectObject):
	'''
	Camera Object which will be used.
	Desired Function (for "2.5D" engine):
		Camera always oriented to the isometric North
		No camera rotation
		No camera zoom (wont be important when in orthographic mode)
		Edge screen panning
		Middle mouse panning
	'''
	def __init__(self, ancestor = None, isometric = True):
		'''
		ancestor: Parent Node
		isometric: orthographic view
		'''
		self.ancestor=ancestor
		self.dz=.5
		center=(len(self.ancestor.terrain.data)/2,len(self.ancestor.terrain.data[0])/2,self.dz)
		base.disableMouse()
		base.camLens.setFov(50)
		base.camera.reparentTo(self.ancestor.root)
		#base.camera.setPos(-center[0]*2,-center[1]*2,5)
		base.camera.setPos(0,-20,5)
		base.camera.setHpr(0,-30,0)
		
		if isometric:
			lens = OrthographicLens()
			lens.setFilmSize(6)
			base.cam.node().setLens(lens)
		
		self.isPanning = False
		
		self.accept('mouse2', self.middleMouseDown, [])
		self.accept('mouse2-up', self.middleMouseUp, [])
		
		taskMgr.add(self.update,"updateCameraTask")
	
	def middleMouseDown(self):
		#print "middleMouseDown"
		self.mousePosition0=[base.win.getPointer(0).getX(),base.win.getPointer(0).getY()]
		self.isPanning = True
		
	def middleMouseUp(self):
		#print "middleMouseUp"
		self.isPanning = False
	
	def update(self, task):
		if self.isPanning is True:
			mousePosition1 = [base.win.getPointer(0).getX(),base.win.getPointer(0).getY()]
			delta = [(mousePosition1[0]- self.mousePosition0[0])*0.005, (mousePosition1[1]- self.mousePosition0[1]) * 0.005]
			#print delta
			base.camera.setPos(base.camera.getPos()[0] + delta[0], base.camera.getPos()[1] + delta[1], base.camera.getPos()[2])
			
		return Task.cont


# From Terrain.py
from pandac.PandaModules import GeoMipTerrain
from pandac.PandaModules import NodePath,GeomVertexData,GeomVertexFormat,Geom,GeomVertexWriter,GeomTriangles,GeomNode,Vec3
from pandac.PandaModules import Texture,DepthTestAttrib
from pandac.PandaModules import PNMImage,Filename
import random,math
import Meshes
import Map
class Terrain:
#=======================================================================
	"""initialize"""
	def __init__(self,ancestor=None,size=65,alt=2.0,div=5,retro=True):
		print "____________________________________________________"
		print "Class Terrain"
		self.ancestor=ancestor
		
		#------------------------------------------------------------
		#initialize basic props
		self.alt=alt
		self.div=div
		self.divsep=1.0
		self.retro=retro
		self.mappath='_media/maps/'
		self.texpath='_media/tiles/basic/'
		self.themes=['basic','terrain']#'wood','room','wall','pavement','stone','grass','sea']
		self.tiles=[{'rgb':(0,0,0),    'tex':'',   'score':0,    'h':0}]
		#------------------------------------------------------------
		#create terrain nodepath
		self.root=NodePath("rootTerrain")
		self.root.setSz(1.0)
		self.root.reparentTo(self.ancestor.root)
		#make terrain pickable as 'terrain'
		#print dir(self.ancestor)
		self.ancestor.picker.makePickable(self.root,'terrain')
		#------------------------------------------------------------
		#create a random heightmap  of given size
		self.heightmap=Map.DiamondMap(self,self.mappath+"hmap.png",size,alt)
		#------------------------------------------------------------
		#create a colormap based on the heightmap
		self.colormap = Map.ColorMap(ancestor=self,hmapfile=self.mappath+"hmap.png",cmapfile=self.mappath+"cmap.png", theme='random', randomTiles=True)
		#------------------------------------------------------------
		#generate data from hmap and cmap
		self.data=self.makeData(self.mappath+"hmap.png",self.mappath+"cmap.png")
		#print self.makeData(self.mappath+"hmap.png",self.mappath+"cmap.png")
		#------------------------------------------------------------
		#generate subdatas from data
		#print self.data
		self.subdata=self.makeSubData(baseData=self.data,div=self.div)
		#------------------------------------------------------------
		#generate mesh tiles
		for a in range(0,len(self.subdata)):
			for b in range(0,len(self.subdata[a])):
				#generate base mesh
				mesh = self.makeBase(a,b)
				#generate layer meshes
				for i in range(0,len(self.tiles)): 
						layer=self.makeLayer(i,a,b)
		print "    /terrain mesh created in ",self.div,"x",self.div,"tiles"
		#------------------------------------------------------------
		#generate num cubes
		self.numCubes()
	
	#=======================================================================
	"""make data matrix from heightmap"""
	def makeData(self,hmapfile,cmapfile):
		#print "/makeData"
		#open heightmap for reading pixel data
		hmap=PNMImage()
		hmap.read(Filename(hmapfile)) 
		xs=hmap.getXSize()
		ys=hmap.getYSize()
		#open colormap for reading color data
		cmap=PNMImage()
		cmap.read(Filename(cmapfile))
		#generate data bidimensional array
		data=[]
		for x in range(xs):
			data.append([])
			for y in range(ys):
				#set data dictionary props
				#-------------------------
				#name
				name="cell_"+str(x)+"_"+str(y)
				#h
				h=(hmap.getXel(x,ys-y-1)[0]*10)
				if self.retro==True: 
					if h<1: h=h/5
					h=int(h)
				#c and rgb
				c=cmap.getXel(x,ys-y-1)
				rgb=(int(c[0]*255),int(c[1]*255),int(c[2]*255))
				#default texture
				tex=self.tiles[0]['tex']
				texnum=0
				score=self.tiles[0]['score']
				#from rgb we assign tex and score
				for n in range(len(self.tiles)):
					if rgb==self.tiles[n]['rgb']:
						#tex=self.tiles[n]['tex']
						tex = 'Assets/Planets/Mars/Art/Tile/Surface/Dozed.png'
						texnum=n
						score=self.tiles[n]['score']
						break
				#set terrain data dictionary
				#---------------------------
				data[x].append({'name':name,'h': 0,'c':c,'rgb':rgb,'tex':tex,'texnum':texnum,'score':score}) 
		return data
	#======================================================================
	
	def makeSubData(self,baseData=[[]],div=3):
		"""make subdata matrix from data"""
		#print "/makeSubData"
		#generate subdatas
		subdata=[]
		for a in range(0,div):
			subdata.append([])
			for b in range(0,div):
				subdata[a].append([])
				#get max2,ymax2
				xmax=int(len(baseData)/div)
				xmax2=xmax+1
				if len(baseData)/(div*1.0)==xmax:
					 if a==div-1: xmax2-=1
				else:
					 if a==div-1: xmax2+=1
				#print baseData
				#print a
				ymax=int(len(baseData[a])/div)
				ymax2=ymax+1
				if len(baseData[0])/(div*1.0)==ymax:
					 if b==div-1: ymax2-=1
				else:
					 if b==div-1: ymax2+=1
				#get x1,y1
				x1=a*xmax
				y1=b*ymax
				#generate subdata
				for x in range(0,xmax2):
					subdata[a][b].append([])
					for y in range(0,ymax2): 
						subdata[a][b][x].append([])
						subdata[a][b][x][y]=baseData[x+x1][y+y1]
		#return subdata
		return subdata

#=======================================================================
	"""make terrain base mesh from data"""
	def makeBase(self,a,b):
		#print "---------------"
		#print "/makeBase"
		#get data
		data=self.subdata[a][b]
		#set vertex data
		vdata = GeomVertexData('plane',GeomVertexFormat.getV3n3c4t2() , Geom.UHStatic) 
		vertex = GeomVertexWriter(vdata, 'vertex')
		normal = GeomVertexWriter(vdata, 'normal')
		color = GeomVertexWriter(vdata, 'color')
		uv = GeomVertexWriter(vdata, 'texcoord')
		#set vertices
		number = 0
		for x in range(0,len(data)-1):
			for y in range(0,len(data[x])-1):
				#get vertex data
				v1 = Vec3(x,y,data[x][y]['h'])
				v2 = Vec3(x+1,y,data[x+1][y]['h'])
				v3 = Vec3(x+1,y+1,data[x+1][y+1]['h'])
				v4 = Vec3(x,y+1,data[x][y+1]['h'])
				#n=(0,0,1)#normal
				
				#assign vertex colors+alpha
				#--------------------------
				option=1
				#option1: black
				if option==1:
					c=0
					c1=[c,c,c,1]
					c2=[c,c,c,1]
					c3=[c,c,c,1]
					c4=[c,c,c,1]
				#option2: color vertices
				if option==2:
					alpha=.1
					c1 = [data[x][y]['c'][0],data[x][y]['c'][1],data[x][y]['c'][2],alpha]
					c2 = [data[x+1][y]['c'][0],data[x+1][y]['c'][1],data[x+1][y]['c'][2],alpha]
					c3 = [data[x+1][y+1]['c'][0],data[x+1][y+1]['c'][1],data[x+1][y+1]['c'][2],alpha]
					c4 = [data[x][y+1]['c'][0],data[x][y+1]['c'][1],data[x][y+1]['c'][2],alpha]
				#option3: color tiles
				if option==3:
					c1=[data[x][y]['c'][0],data[x][y]['c'][1],data[x][y]['c'][2],1]
					c2=[data[x][y]['c'][0],data[x][y]['c'][1],data[x][y]['c'][2],1]
					c3=[data[x][y]['c'][0],data[x][y]['c'][1],data[x][y]['c'][2],1]
					c4=[data[x][y]['c'][0],data[x][y]['c'][1],data[x][y]['c'][2],1]
				#--------------------------
				#add vertex h
				vertex.addData3f(v1)
				#normal.addData3f(*n)
				vertex.addData3f(v2)
				#normal.addData3f(*n)
				vertex.addData3f(v3)
				#normal.addData3f(*n)
				vertex.addData3f(v1)
				#normal.addData3f(*n)
				vertex.addData3f(v3)
				#normal.addData3f(*n)
				vertex.addData3f(v4)
				#normal.addData3f(*n)
				#add vertex color
				color.addData4f(*c1)    
				color.addData4f(*c2)
				color.addData4f(*c3)   
				color.addData4f(*c1)
				color.addData4f(*c3)
				color.addData4f(*c4)
				#iterate
				number = number + 2
		#add triangles  
		prim = GeomTriangles( Geom.UHStatic )
		for n in range(number):
			prim.addVertices(n*3+2,n*3+0,n*3+1)
		prim.closePrimitive()
		#make geom
		geom = Geom(vdata)
		geom.addPrimitive(prim)
		#make geom node
		node = GeomNode("base"+"_"+str(a)+"_"+str(b))
		node.addGeom(geom)
		#make mesh nodePath
		mesh= NodePath(node)
		#set render order
		mesh.setBin("",1)
		#locate mesh
		mesh.setPos(self.divsep*(a*int(len(self.data[a])/self.div)),self.divsep*(b*int(len(self.data[b])/self.div)),0)
		#reparent mesh
		mesh.reparentTo(self.root) 
		#return mesh
		return mesh
	
	def numCubes(self):
		#create axis cubes
		self.numCubes=[]
		for n in range(len(self.data)):
			self.numCubes.append(Meshes.loadModel(parent=self.ancestor.root,
								path="_models/shapes/cube",name="numCube"+str(n),size=.05,pos=(n,-.5,0),color=None))
	
	
	#=====================================================================
	"""make terrain node mesh from data"""
	def makeLayer(self,i,a,b):
		#print "/makeLayer",i
		#get data
		data=self.subdata[a][b]
		#set color+alpha of vertex texture
		def ap(n):
			alpha = 0
			if i == n:
				alpha = 1.0
			return alpha
		def tp(n):
			list = [0,0,0,0]
			if i == n:
				list = [1,1,1,.75]  
			return list
		#set vertex data
		vdata = GeomVertexData('plane',GeomVertexFormat.getV3n3c4t2() , Geom.UHStatic) 
		vertex = GeomVertexWriter(vdata, 'vertex')
		normal = GeomVertexWriter(vdata, 'normal')
		color = GeomVertexWriter(vdata, 'color')
		uv = GeomVertexWriter(vdata, 'texcoord')
		
		#set vertices
		number = 0
		for x in range(0,len(data)-1):
			for y in range(0,len(data[x])-1):
				#get vertex data
				v1 = Vec3(x,y,data[x][y]['h'])
				c1 = data[x][y]['c']
				t1 = data[x][y]['texnum']
				v2 = Vec3(x+1,y,data[x+1][y]['h'])
				c2 = data[x+1][y]['c']
				t2 = data[x+1][y]['texnum']
				v3 = Vec3(x+1,y+1,data[x+1][y+1]['h'])
				c3 = data[x+1][y+1]['c']
				t3 = data[x+1][y+1]['texnum']
				v4 = Vec3(x,y+1,data[x][y+1]['h'])
				c4 = data[x][y+1]['c']
				t4 = data[x][y+1]['texnum']
				n=(0,0,1)#normal
			   
				#assign vertex colors+alpha
				#--------------------------
				a1,a2,a3,a4 = ap(t1),ap(t2),ap(t3),ap(t4)
				t1,t2,t3,t4 = tp(t1),tp(t2),tp(t3),tp(t4)
				
			   # print v1,v2,v3,v4
				if v1[2]==0: t1 = [data[x][y]['c'][0],data[x][y]['c'][1],data[x][y]['c'][2],a1]
				if v2[2]==0: t2 = [data[x+1][y]['c'][0],data[x+1][y]['c'][1],data[x+1][y]['c'][2],a2]
				if v3[2]==0: t3 = [data[x+1][y+1]['c'][0],data[x+1][y+1]['c'][1],data[x+1][y+1]['c'][2],a3]
				if v4[2]==0: t4 = [data[x][y+1]['c'][0],data[x][y+1]['c'][1],data[x][y+1]['c'][2],a4]
				
				## if v1==0 and v2==0 and v3==0 and v4==0:
					## t1 = [data[x][y]['c'][0],data[x][y]['c'][1],data[x][y]['c'][2],a1]
					## t2 = [data[x+1][y]['c'][0],data[x+1][y]['c'][1],data[x+1][y]['c'][2],a2]
					## t3 = [data[x+1][y+1]['c'][0],data[x+1][y+1]['c'][1],data[x+1][y+1]['c'][2],a3]
					## t4 = [data[x][y+1]['c'][0],data[x][y+1]['c'][1],data[x][y+1]['c'][2],a4]
					
				#discard invisible vertices
				if a1 == 0 and a2 == 0 and a3 == 0 and a4 == 0 :continue
				#--------------------------
				#add vertices
				vertex.addData3f(v1)
				normal.addData3f(*n)
				color.addData4f(*t1)  
				uv.addData2f(0,0)
				vertex.addData3f(v2)
				normal.addData3f(*n)
				color.addData4f(*t2)
				uv.addData2f(1,0)               
				vertex.addData3f(v3)
				normal.addData3f(*n)
				color.addData4f(*t3)
				uv.addData2f(1,1)
				vertex.addData3f(v1)
				normal.addData3f(*n)
				color.addData4f(*t1)
				uv.addData2f(0,0)
				vertex.addData3f(v3)
				normal.addData3f(*n)
				color.addData4f(*t3)
				uv.addData2f(1,1)
				vertex.addData3f(v4)
				normal.addData3f(*n)
				color.addData4f(*t4)
				uv.addData2f(0,1)
				#
				number = number + 2      
		#add triangles  
		prim = GeomTriangles( Geom.UHStatic )
		for n in range(number):
			prim.addVertices(n*3+2,n*3+0,n*3+1)
		prim.closePrimitive()
		#make geom
		geom = Geom(vdata)
		geom.addPrimitive(prim)
		#make geom node
		node = GeomNode("layer"+str(i)+"_"+str(a)+"_"+str(b))
		node.addGeom(geom)
		#make mesh nodePath
		mesh= NodePath(node)
		#load and assign texture
		#txfile=self.tiles[i]['tex']
		txfile = 'Assets/Planets/Mars/Art/Tile/Surface/Dozed.png'
		#print txfile
		tx = loader.loadTexture(txfile)
		tx.setMinfilter(Texture.FTLinearMipmapLinear)
		mesh.setDepthTest(DepthTestAttrib.MLessEqual)
		mesh.setDepthWrite(False)
		mesh.setTransparency(True)  
		mesh.setTexture(tx)
		#set render order
		mesh.setBin("",1)
		#locate mesh
		mesh.setPos(self.divsep*(a*int(len(self.data[a])/self.div)),self.divsep*(b*int(len(self.data[b])/self.div)),.005)
		#reparent mesh
		mesh.reparentTo(self.root)
		#return mesh
		return mesh
	
	#=======================================================================
	
	def getCellPos(self,x,y):
		""" returns the center location of the current cell at the given location """
		#get cell vertices
		v1,v2,v3,v4=self.getCellVertices(x,y)
		#get cell center point
		x=(v1[0]+v2[0]+v3[0]+v4[0])/4.0
		y=(v1[1]+v2[1]+v3[1]+v4[1])/4.0
		z=(v1[2]+v2[2]+v3[2]+v4[2])/4.0
		return x,y,z
	
	def getElevation(self,x,y):
		""" returns the terrain elevation at the given xy location 
			Uses bilinear interpolation (thanks pro-soft for let me know about that)
			(1-a)(1-b)      a(1-b)    where a and b are the horizontal and vertical
			b(1-a)            ab      distances to the nearest top-left pixel.
		"""
		
		v1,v2,v3,v4=self.getCellVertices(x,y)
		#get elevation: bilinear interpolation
		a=abs(x-v4[0])#x distance to top-left
		b=abs(y-v4[1])#y distance to top-right
		i1=v4[2]#top-left
		i2=v3[2]#top-right
		i3=v1[2]#bottom-left
		i4=v2[2]#bottom-right
		#bilinear interpolation equation
		z=(1-a)*(1-b)*i1 + (a)*(1-b)*i2 + (1-a)*(b)*i3 + (a)*(b)*i4
		
		#solve exceptions (not working)
		## h1,h2,h3,h4=self.getElevationExceptions(v1[2],v2[2],v3[2],v4[2])
		## i1=h4#top-left
		## i2=h3#top-right
		## i3=h1#bottom-left
		## i4=h2#bottom-right
		## #bilinear interpolation equation
		## z=(1-a)*(1-b)*i1 + (a)*(1-b)*i2 + (1-a)*(b)*i3 + (a)*(b)*i4
		
		return z

	def getCellVertices(self,x,y,debug=False):
		""" calculates the location of the 4 vertices of the current cell at the given location """
		#vertex 1
		x1=int(x)
		y1=int(y)
		z1=self.data[x1][y1]['h']
		if debug==True: self.cube1.setPos(x1,y1,z1)
		#vertex2
		x2=x1+1
		y2=y1
		z2=self.data[x2][y2]['h']
		if debug==True: self.cube2.setPos(x2,y2,z2)
		#vertex3
		x3=x1+1
		y3=y1+1
		z3=self.data[x3][y3]['h']
		if debug==True: self.cube3.setPos(x3,y3,z3)
		#vertex4
		x4=x1
		y4=y1+1
		z4=self.data[x4][y4]['h']
		if debug==True: self.cube4.setPos(x4,y4,z4)
		
		return (x1,y1,z1),(x2,y2,z2),(x3,y3,z3),(x4,y4,z4)
		
		
	#=======================================================================
	
	def getElevationExceptions(self,h1,h2,h3,h4):
		
		d=1.05
		d2=0.95
		if h1==h2 and h1==h3 and h1!=h4: 
			h1=h1*d
			h2=h2*d
			h3=h3*d 
			h4=h4*d2    
		if h1==h2 and h1==h4 and h1!=h3: 
			h1=h1*d
			h2=h2*d
			h4=h4*d
			h3=h3*d2
		if h1==h3 and h1==h4 and h1!=h2:
			h1=h1*d
			h3=h3*d
			h4=h4*d
			h2=h2*d2
		if h2==h3 and h2==h4 and h2!=h1: 
			h2=h2*d
			h3=h3*d
			h4=h4*d
			h1=h1*d2

		return h1,h2,h3,h4


#This below belongs here
class World(DirectObject.DirectObject):
	def __init__(self):
		"""initialize"""
		print "____________________________________________________"
		print "Class Main"
		#setup application
		#-----------------------
		base.setFrameRateMeter(True)
		#self.toggleWireFrame()
		self.keys()
		self.txt=self.info((-1.32,0.96),title)
		
		# Initialize classes
		self.grid = None
		self.camera = None
		
		self.root = NodePath('rootMain')
		self.root.reparentTo(render)
		
		self.picker = Picker(self)
		
		self.terrain = Terrain(self)
		self.grid = Grid(self, self.terrain.data)
		
		self.terrain.root.setHpr(-135,0,0)
		
		#self.camera = Camera(self, isometric = False)
		self.camera = Camera(self, isometric = True)
		
		self.lights = Lights(self, lightsOn = True, showLights = False)
		
		# Load the structure database
		print 'Loading structures:', loadStructures()
		self.structuresDatabase = loadStructures()
		
		# GUI attempts
		self.contextMeny = ContextMenu(None, self.structuresDatabase)
		#self.gui = UI()
		#self.controlPan = DirectFrame()
		#self.endTurnButton = DirectButton(text = 'End Turn', command = self.endTurnRequest, scale = 0.5)
		
		#self.rightMenu = Menu.DropDownMenu(('End Turn', self.endTurnRequest))
		#DO=DirectObject()
		#self.DO.accept('mouse3',createContextMenu)
		#self.DO.accept('escape',exit)
		#self.DO.accept('window-event',resetModButtons)
		#base.disableMouse()
		
		#self.gui = GUI(theme=Theme("TreeGUI/data/affgui.png"))
		# create the form
		#self.aiMenu = AIMenu()
		
		#print self.grid.data
		#self.grid.data[0][1]['building'] = Structure(self.grid.mh.getCell(SQ_Location(0,1)))
		self.grid.data[5][5].structure = Structure(self.terrain, isLegacy=True)
		position = self.terrain.getCellVertices(5,5)
		self.grid.data[5][5].structure.mesh.setPos(self.terrain.root, position[2][0],position[2][1],position[2][2]+0.5)
		self.grid.data[5][5].structure.mesh.setHpr(135,-30,0)
		self.grid.data[5][5].structure.mesh.setScale(0.5,0.5,0.5)
		
		#self.grid.data[4][5].structure = Structure(self.terrain, isLegacy=True)
		#position = self.terrain.getCellVertices(4,5)
		#self.grid.data[4][5].structure.mesh.setPos(self.terrain.root, position[2][0],position[2][1],position[2][2]+0.5)
		#self.grid.data[4][5].structure.mesh.setHpr(135,-30,0)
		#self.grid.data[4][5].structure.mesh.setScale(0.5,0.5,0.5)
		#print position[0]
		#print self.grid.data[0][1].structure.root.getPos()
		#print self.grid.data[0][1].structure.root.getParent()
		#print self.terrain.root.ls()
		#print self.terrain.root.find('structure')
		#self.grid.data[0][1].structure.root.setPos()
		#print len(self.grid.data)
		#self.grid.data[0][4].placeStructure()
	
	def info(self,pos, msg):
		self.font = loader.loadFont('_media/fonts/OCR.otf')
		return OnscreenText(font=self.font,text=msg, style=1, fg=(1,1,1,1),
							pos=pos, align=TextNode.ALeft, scale = .035, mayChange=True)
	def keys(self):
		"""keys"""
		self.accept('w',self.toggleWireFrame)
		self.accept('t',self.toggleTexture)
		self.accept('s',self.snapShot)
		self.accept('escape',sys.exit)
	def snapShot(self):
		base.screenshot("Snapshot")
	def toggleWireFrame(self):
		base.toggleWireframe()
	def toggleTexture(self):
		base.toggleTexture()

class Spinner(object):
	'''
	Currently unused
	'''
	def __init__(self):
		self.reactor = SelectReactor()
		installReactor(self.reactor)
		self.Desired_FPS = 60
		LoopingCall(taskMgr.step).start(1 / self.Desired_FPS)
	def Run(self):
		#reactor.run()
		#run()
		self.reactor.run()
	def Stop(self):
		self.reactor.stop()


def main():	
	#messenger.toggleVerbose()
	serverView = NetworkServerView()
	serverController = NetworkServerController()
	Desired_FPS = 60
	world=World()
	
	LoopingCall(taskMgr.step).start(1 / Desired_FPS)
	serverHost = 'localhost'
	# In memory of the fall of Sierra
	serverPort = 1999
	factory = serverView.pbClientFactory
	from twisted.internet import reactor
	reactor.connectTCP(serverHost, serverPort, factory)
	deferred = factory.login(credentials.UsernamePassword("user1", "pass1"))
	deferred.addCallback(serverView.setServer)
	
	reactor.run()
	#spinner = Spinner()
	#spinner.Run()
	

if __name__ == '__main__':
	main()
