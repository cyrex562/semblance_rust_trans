'''
Legacy Manager
Manages/converts, does whatever is needed with the legacy Outpost content
'''
from pandac.PandaModules import PNMImage, Filename, Texture, VBase4D

def Tile(sourceImgPath = '', alphaColor = (None, None, None), tileSize = (0, 0), location = (0,0)):
	'''
	Generates a "tile" (image) from a sprite-table-like image file. Returns PNMImage
	sourceImgPath: string path to image
	alphaColor: (1, 1, 1) A tuple of RGB values that should be transparent.  None for values that should be ignored
	tileSize: A (x, y) tuple for the tile size used in the source image
	location:	A (x, y) tuple for the top left corner of the tile in the source image
	'''
	tiles = PNMImage()
	textureFile = Filename(sourceImgPath)
	tiles.read(textureFile)
	tile = PNMImage(tileSize[0], tileSize[1])
	tile.copySubImage(tiles, 0, 0, location[0], location[1])
	tile.addAlpha()
	tile.alphaFill(1)	
	# This is terrable, ugly, horrible mess
	for xi in range(tile.getXSize()):
		for yi in range(tile.getYSize()):
			matchPattern = []
			for i in range(len(alphaColor)):
				if alphaColor[i]:
					matchPattern.append( alphaColor[i])
				else:
					matchPattern[i] = self.tile.getXelA(xi,yi)[i]
			matcher =  VBase4D(matchPattern[0], matchPattern[1], matchPattern[2], 1)
			if tile.getXelA(xi,yi) == matcher:
				tile.setAlpha(xi,yi,0)
				
	return tile
