import Meshes, Legacy
from pandac.PandaModules import NodePath,LineSegs,TextureStage,TexGenAttrib, Texture


class Structure(object):
	'''
	Generic structure class.  Should be usuable as is for all building types.
	TODO: Load stats from xml or yaml file.
	'''
	def __init__(self, ancestor, is3D = False, isLegacy = False):
		'''
		Name: Name of structure
		ancestor:	Parent node
		is3D: Is mesh 3D building or "2.5D"
		isLegacy: Is texture from legacy content
		meshPath: Path to mesh
		texturePath: Path to texture
		mesh:	The actuall mesh object
		texture:	The actual texture object
		attributes: Dict for the various attributes of the building such as power generated or residence held
		self.employs:	What kind and number of employees needed for building to function at capaity
		self.resources:	Resource type and quantity consumed for building to operate fully.
		
		Do I want to do it this way?
		A* might be better for connection check
		
		hasPowerConnection, hasLifeSupportConnection  Boolians which signafy if the building is connected to a network?
		hasPower = False
		hasLifeSupport = False
		'''
		self.name = 'Structure'
		self.ancestor = ancestor
		
		self.root=NodePath(self.name)
		self.root.reparentTo(self.ancestor.root)
		
		self.is3D = is3D
		self.isLegacy = isLegacy
		self.meshPath = None
		self.texturePath = None
		
		self.mesh = None
		self.texture = None
		
		self.attributes = {'Power': -50, 'Food': 50, 'Residence': 0}
		self.employs = {'Worker': 3}
		self.resources = {}
		
		self.hasPower = False
		self.hasLifeSupport = False
		
		if self.is3D is False:
			self.meshPath = 'Assets/UI/2DBuildingMesh.egg'
			if self.isLegacy:
				self.texturePath = 'Assets/Legacy/BITMAPS/TILE2.BMP'
				self.tile = Legacy.Tile(self.texturePath, (1,1,1), (94, 73), (417, 15))
		
		'''
		self.grid.data[5][5].structure = Structure(self.terrain, isLegacy=True)
		position = self.terrain.getCellVertices(5,5)
		self.grid.data[5][5].structure.mesh.setPos(self.terrain.root, position[2][0],position[2][1],position[2][2]+0.5)
		self.grid.data[5][5].structure.mesh.setHpr(135,-30,0)
		self.grid.data[5][5].structure.mesh.setScale(0.5,0.5,0.5)
		'''
		
		#self.mesh = loader.loadModel(self.meshPath)
		#self.mesh.setPos(0,0,0)
		self.mesh = Meshes.loadModel(parent=self.root,
								path=self.meshPath,name=self.name, pos=(0,0,0),color=None)
		#self.texture =  loader.loadTexture('Assets/Structures/AgroDome0/Texture/AgroDome0.png')
		#self.texture = loader.loadTexture(self.texturePath)
		self.texture = Texture()
		self.texture.load(self.tile)
		#self.mesh.setTexGen(TextureStage.getDefault(), TexGenAttrib.MWorldPosition)
		#self.mesh.setTexProjector(TextureStage.getDefault(), render, self.mesh)
		self.mesh.setTexture(self.texture)
		self.mesh.setTransparency(True)

def loadStructures():
		'''
		Scans the structure directory and converts yaml into structure class.
		Returns database (dictioanry)
		'''
		import glob, yaml
		# TODO: Make the 'Assets' import from config
		pathList = glob.glob('Assets/Structures/*/')
		structureTypes = {}
		print pathList
		for path in pathList:
			path = path.replace('\\','/')
			configFile = open(path + 'structure.yaml', 'r')
			stream = configFile.read()
			configFile.close()
			structureDictionary = yaml.load(stream)
			print path.split('/')[2]
			structureTypes[path.split('/')[2]] = structureDictionary
		return structureTypes
