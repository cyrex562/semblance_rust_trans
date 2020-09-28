import Meshes
from pandac.PandaModules import NodePath,LineSegs
from direct.directtools.DirectGeometry import LineNodePath

# temp import
from Structure import Structure

class Grid():
	def __init__(self,ancestor=None,data=None):
		print "____________________________________________________"
		print "Class Grid"
		self.ancestor=ancestor
		self.root=NodePath("rootGrid")
		self.root.reparentTo(self.ancestor.root)
		#generate grid data from terrain data
		self.generateGrid(data)
		#self.traceGrid()
		#initialize astar
		self.mh = SQ_MapHandler(self.array, self.xs,self.ys)
		self.astar = AStar(self.mh)
		#test
		#self.pathFinder((0,0),(4,3))
	
	#======================================================================
	def pathFinder(self,av,start,end):
	
		#we have to make the start cell walkable first and restore it later
		startWalkable=self.data[start[0]][start[1]]['walkable']
		self.setCellWalkable(start[0],start[1],1)
		#-----------------------
		#find path
		self.start=start
		self.end=end
		self.path = self.astar.findPath(SQ_Location(start[0],start[1]),SQ_Location(end[0],end[1]))
		locs=self.tracePath(av,self.path)
		#------------------------
		#we have to restore the start cell to original walkable
		self.setCellWalkable(start[0],start[1],startWalkable)
	
		return locs
	
	#======================================================================
	def tracePath(self,av,path):
		#delete previous traced path
		self.deletePath(av)
		#if path not found return
		if path==None:
			self.ancestor.txt.setText("Path not found!")
			return []
		#print "Path from",self.start,"to",self.end,path,"totalCost: "+str(path.totalCost)
		#self.ancestor.txt.setText(str("path found in 0 seconds."))
		#trace the path with cubes
		i=0
		locs=[]
		for c in path.cells:
			#add c props to locs list
			x,y,z=self.ancestor.terrain.getCellPos(c.location.x,c.location.y)
			locs.append((x,y,z,c.score,c.mCost))
			#create cubePaths	      
			av.pathCubes.append(Meshes.loadModel(parent=self.ancestor.root,
								path="_models/shapes/cube",name=av.name+"_pathCube",
								size=.05,pos=(x,y,z+av.dz/2),color=(0,0,0,1)))
			i+=1
		#print "--->",locs
		if locs!=[]: av.drawPathLine(locs)
		return locs

	#======================================================================
	def deletePath(self,av):
		#remove path cubes
		if av.lineNode!=None: av.lineNode.removeNode()
		for n in range(len(av.pathCubes)):
			av.pathCubes[n].removeNode()
		av.pathCubes=[]
		av.path=[]
	
	#======================================================================
	def generateGrid(self,data):
		self.xs=len(data)-1
		self.ys=len(data[0])-1
		#init array
		self.array=[]
		for i in range(self.xs*self.ys):
			self.array.append(0)
		#generate data and array    
		self.data=[]
		for x in range(self.xs):
			self.data.append([])
			for y in range(self.ys):
				h1=data[x][y]['h']
				h2=data[x+1][y]['h']
				h3=data[x+1][y+1]['h']
				h4=data[x][y+1]['h']
				#set cell h
				h=(h1+h2+h3+h4)/4.0
				#set cell pendent
				if h1==h2 and h1==h3 and h1==h4 and h2==h3 and h2==h4 and h3==h4:
					pendent=0
				else: 
					pendent=1 
				#set cell score
				score=data[x][y]['score']#int(h*2)
				if self.ancestor.terrain.retro==True and pendent==1: score+=20
				#set cell walkable
				walkable=score#int(h)#score
				#set cell color
				tex=data[x][y]['tex']
				#------------------------
				#set grid data dictionary
				self.data[x].append(Tile(self, {'name':'c_'+str(x)+'_'+str(y),'h':h,'pendent':pendent,'score':score,'walkable':walkable,'building':None,'tex':tex}))
				#add walkable to array
				locationid = (y*self.ys+x)             
				self.array[locationid] = walkable
	
	#======================================================================
	def setCellWalkable(self,x,y,walkable):
		#remind walkable score
		if walkable!=0: walkable=self.data[x][y]['score']
		#set cell walkable/unwalkable
		self.data[x][y]['walkable']=walkable
		locationid = (y*self.ys+x)             
		self.array[locationid] = walkable
		#return the cell id in the searchpath list(not required for anything dough)
		return locationid
	
	#======================================================================
	def traceGrid(self):
		self.infoCubes=[]
		print (self.xs,self.ys)
		for y in range(self.ys):
			for x in range(self.xs):
				locationid = (y*self.ys+x)      
				#print grid
				if x==self.xs-1: 
					print self.array[locationid]
				else:
					print self.array[locationid]
	
	def get3DCoor(self, tilex, tiley):
		'''
		Returns the 3D space coordinates of the specific tile coordinates.
		'''
		return x, y

class SQ_MapHandler:
	"""A simple Square Map implementation"""
	def __init__(self,mapdata,width,height):
		self.m = mapdata
		self.w = width   
		self.h = height  

	#======================================================================
	def getCell(self, location):
		"""MUST BE IMPLEMENTED"""
		x = location.x 
		y = location.y
		if x<0 or x>=self.w or y<0 or y>=self.h:
			return None
		d = self.m[(y*self.w)+x]
		if d <= 0:
			return None
		#this has the weird ID value- I suggest str(x)+"-"+str(y)
		return Cell(location,d,((y*self.w)+x));       

	#======================================================================
	def getAdjacentCells(self, curcell, dest):
		"""MUST BE IMPLEMENTED""" 
		#all these use the SQ_location object. Need to change to Vec2      
		result = []
		cl = curcell.location
		dl = dest
		#diagonals
		addCost=3
		n = self._handleCell(cl.x+1,cl.y+1,curcell,dl.x,dl.y,addCost) 
		if n: result.append(n)
		n = self._handleCell(cl.x-1,cl.y-1,curcell,dl.x,dl.y,addCost)
		if n: result.append(n) 
		n = self._handleCell(cl.x+1,cl.y-1,curcell,dl.x,dl.y,addCost)
		if n: result.append(n)
		n = self._handleCell(cl.x-1,cl.y+1,curcell,dl.x,dl.y,addCost)
		if n: result.append(n)
		#verticals
		addCost=2
		n = self._handleCell(cl.x+1,cl.y,curcell,dl.x,dl.y,addCost) 
		if n: result.append(n)
		n = self._handleCell(cl.x-1,cl.y,curcell,dl.x,dl.y,addCost)
		if n: result.append(n)
		n = self._handleCell(cl.x,cl.y+1,curcell,dl.x,dl.y,addCost)
		if n: result.append(n)
		n = self._handleCell(cl.x,cl.y-1,curcell,dl.x,dl.y,addCost)
		if n: result.append(n)
		#return
		return result
	
	#======================================================================
	def _handleCell(self,x,y,fromcell,destx,desty,addCost=0):
		n = self.getCell(SQ_Location(x,y))
		if n is not None:
			dx = max(x,destx) - min(x,destx)
			dy = max(y,desty) - min(y,desty)
			emCost = dx+dy
			n.mCost += fromcell.mCost +addCost                                  
			n.score = n.mCost+emCost
			n.rpgCost=addCost
			n.parent=fromcell
			return n
		return None 

class AStar:
	def __init__(self,maphandler):
		self.mh = maphandler
			   
	#======================================================================
	def _getBestOpenCell(self):
		bestCell = None       
		for n in self.on:
			if not bestCell:
				bestCell = n
			else:
				if n.score<=bestCell.score:
					bestCell = n
		return bestCell
	
	#======================================================================
	def _tracePath(self,n):
		cells = [];
		totalCost = n.mCost;   #'n' is the end pointcell.
		p = n.parent;
		cells.insert(0,n);       
		while 1:
			if p.parent is None:
				break

			cells.insert(0,p)
			p=p.parent      #loop through everything- make a list of cells, insert the string of parents, with each parent at the beginning of the list working backward from the endpoint node.
		return Path(cells,totalCost) #This is the only time in the script the Path object is called.

	#======================================================================
	def _handleCell(self,cell,end):       
		i = self.o.index(cell.lid)    #returns the place in the list where this cell is mentioned.
		self.on.pop(i)            #removes that entry in the open cell list
		self.o.pop(i)            #removes that entry in the list of location open id's
		self.c.append(cell.lid)      #adds that id to the list of closed cells

		cells = self.mh.getAdjacentCells(cell,end)
				   
		for n in cells:
			if n.location == end:
				# reached the destination
				return n
			elif n.lid in self.c:
				# already in close, skip this
				continue
			elif n.lid in self.o:
				# already in open, check if better score
				i = self.o.index(n.lid)
				on = self.on[i];
				if n.mCost<on.mCost:
					self.on.pop(i);
					self.o.pop(i);
					self.on.append(n);
					self.o.append(n.lid);
			else:
				# new cell, append to open list
				self.on.append(n);               
				self.o.append(n.lid);

		return None
	
	#======================================================================
	def findPath(self,fromlocation, tolocation):
		self.o = []    #list of open locations
		self.on = []   #list of open nodes
		self.c = []    #list of closed locations

		end = tolocation
		fcell = self.mh.getCell(fromlocation)   #returns a Cell object if the location 1) is inside the grid boundaries  AND 2) is not an obstacle valued cell. In which case it returns 'None'
		self.on.append(fcell)
		self.o.append(fcell.lid)
		nextCell = fcell
		
		#Key pathfinding loop       
		while nextCell is not None:                  
			finish = self._handleCell(nextCell,end)  #Returns a cell if the nextCell is the end location, otherwise, returns None
			if finish:                         
				return self._tracePath(finish)       #if a cell object was returned that means it was the endpoint so go ahead and trace the path to this cell
			nextCell=self._getBestOpenCell()         #find a new cell to run through the loop
			   
		return None
	
class Tile(object):
	'''
	Tile in grid.
	'''
	def __init__(self, ancestor, dictionary):
		
		self.ancestor = ancestor
		self.root=NodePath("rootTile")
		self.root.reparentTo(self.ancestor.root)
		self.name = dictionary['name']
		self.pendent = dictionary['pendent']
		self.height = dictionary['h']
		self.score = dictionary['score']
		self.passable = dictionary['walkable']
		self.structure = dictionary['building']
		self.texture = dictionary['tex']
	
	def getDict(self):
		'''
		Returns a dictionary compatable with previous example code
		'''
		dictionary = {'name': self.name,'h': self.height,'pendent': self.pendent,'score': self.score,'walkable':self.passable,'structure': self.structure,'tex': self.texture}
		#print self.root.getPos()
		#print self.ancestor.root.getPos()
		return dictionary
	
	def placeStructure(self, structure = None):
		'''
		Places a generic structure into the tile
		TODO: structure will be an identifyer string 
		'''
		self.structure = Structure(self, isLegacy = True)
