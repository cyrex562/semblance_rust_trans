
from pandac.PandaModules import PNMImage,Filename
import os,random

##############################################################################

"""
DiamondMap: 
   Based on a (i think) Kris Schnee schrieb script that i found in the net, originally implemented in pygame, thanks!
   Generates a random procedural heighmap of given size(**2+1) usign Diamond Square algorythm and saves it to disk
   If wanted, splits the heightmap in x*y subimages and saves them to disk
"""

class DiamondMap:
    def __init__(self,ancestor=None,hmapfile=None,size=33,alt=2):
        #print "____________________________________________________"
        #print "Class DiamondMap"
        self.ancestor=ancestor
        #init heightmap
        heightmap=self.initMap(size,False)
        #make random heightmap
        heightmap=self.randomMap(heightmap,False,(0.0,0.0,0.0,0.0))
        #draw hmap on PNMIamge
        hmap=self.drawMap(heightmap,alt)
        #save map
        hmap.write(Filename(hmapfile))
        print "    /height map saved in",hmapfile
    
    #=========================================================================
    
    def initMap(self,size,random_data=False):
        """Generate random static or a blank map."""
        heightmap = []
        if random_data:
            #random noise background
            for x in range(size):
                heightmap.append([])
                for y in range(size):
                    heightmap[-1].append( random.random() )
        else:
            #black background
            for x in range(size):
                heightmap.append([])
                for y in range(size):
                    heightmap[-1].append(0.0)
        return heightmap

    #=========================================================================
    def drawMap(self,heightmap,alt):
        """Draws the heighmap in the image
        The range of values is assumed to be 0.0 to 1.0, in a 2D, square array.
        The range is converted to greyscale values of 0 to 255."""
        #get size
        size=(len(heightmap),len(heightmap[0]))
        #create image
        hmap = PNMImage(size[0], size[1])
        #draw map
        for y in range(size[0]):
            for x in range(size[1]): 
                h = (heightmap[x][y])*alt# * 255)*1
                try:
                    #print heightmap[x][y],"->",h
                    hmap.setXel(x,y, h )
                    #draw.point((x,y), fill=(h,h,h))
                except:
                    print "Error on x,y: "+str((x,y))+"; map --> 0-255 value: "+str((heightmap[x][y],h))    
        return hmap
    
    #=========================================================================
    def randomMap(self,heightmap,show_in_progress=False,seed_values=(.5,.5,.5,.5)):
        """Make procedural terrain using the D/S Algorithm."""
        #get size
        size = len(heightmap)
        # Seed the corners.
        corners = ((0,0),(size-1,0),(0,size-1),(size-1,size-1))
        for n in range(4):
            point = corners[n]
            heightmap[point[0]][point[1]] = seed_values[n]
    
        # Starting values.
        size_minus_1 = size - 1 ## For edge-wrapping purposes.
        cell_size = size-1 ## Examine squares of this size within the heightmap.
        cell_size_half = cell_size / 2
        iteration = 0 ## How many times have we done the algorithm? (just FYI)
        chaos = 1.0 ## Total possible variation of a height from expected average.
        chaos_half = chaos * .5 ## Possible variation up or down.
        diamond_chaos_half = (chaos/1.414)*.5 ## Reduced by sqrt(2) for D step.
        ## (Wouldn't "Diamond Chaos Half" be a cool anime series title?)
    
        while cell_size > 1: # For actual use.
            # Begin the algorithm.
            ##print "Iteration: "+str(iteration)
            ##print "Chaos: "+str(chaos)+" C.Half: "+str(chaos_half)+" D.C.Half: "+str(diamond_chaos_half)
            iteration += 1
    
            #Find the "anchor points" that mark the upper-left corner of each cell.
            for anchor_y in range(0,size-1,cell_size):
                for anchor_x in range(0,size-1,cell_size):
    
                    # Calculate the center of the cell.
                    cx = anchor_x + cell_size_half
                    cy = anchor_y + cell_size_half
    
                    # The "Diamond" phase.
    
                    # Find the center's diagonal "neighbors."
                    neighbors = ([cx-cell_size_half,cy-cell_size_half],
                                 [cx+cell_size_half,cy-cell_size_half],
                                 [cx-cell_size_half,cy+cell_size_half],
                                 [cx+cell_size_half,cy+cell_size_half])
    
                    # Correct for points outside the map.
                    for n in range(4):
                        neighbor = neighbors[n]
                        if neighbor[0] < 0:
                            neighbors[n][0] += size
                        elif neighbor[0] > size:
                            neighbors[n][0] -= size
                        if neighbor[1] < 0:
                            neighbors[n][1] += size
                        elif neighbor[1] > size:
                            neighbors[n][1] -= size
                    average = sum([heightmap[n[0]][n[1]] for n in neighbors]) * .25
                    h = average - chaos_half + (random.random() * chaos)
                    ##h = average ## Test: No randomness.
                    h = max(0.0,min(1.0,h))
                    #if h>.5: h+=1
                    heightmap[cx][cy] = h
    
                    # The "Square" phase.
    
                    # Calculate four "edge points" surrounding the center.
                    edge_points = ((cx,cy-cell_size_half),
                                   (cx-cell_size_half,cy),
                                   (cx+cell_size_half,cy),
                                   (cx,cy+cell_size_half))
                    for point in edge_points:
                        neighbors = [[point[0],point[1]-cell_size_half],
                                     [point[0]-cell_size_half,point[1]],
                                     [point[0]+cell_size_half,point[1]],
                                     [point[0],point[1]+cell_size_half]]
    
                        # Correct for points outside the map.
                        for n in range(4):
                            neighbor = neighbors[n]
                            if neighbor[0] < 0:
                                neighbors[n][0] += size
                            elif neighbor[0] > size_minus_1:
                                neighbors[n][0] -= size
                            if neighbor[1] < 0:
                                neighbors[n][1] += size
                            elif neighbor[1] > size_minus_1:
                                neighbors[n][1] -= size
    
                        average = sum([heightmap[n[0]][n[1]] for n in neighbors]) * .25
                        h = average - chaos_half + (random.random() * chaos)
                        h = max(0.0,min(1.0,h))
                        #if h>.5: h+=1
                        ##h = average ## Test: No randomness.
                        heightmap[point[0]][point[1]] = h
                        
            # End of iteration. Reduce cell size and chaos.
            cell_size /= 2
            cell_size_half /= 2
            chaos *= .45
            chaos_half = chaos * .55#.5
            diamond_chaos_half = (chaos / 1.414) * .25
  
        return heightmap


################################################################################

class ColorMap:
    #===========================================================================
    """initialize"""
    def __init__(self,ancestor=None,hmapfile=None,cmapfile=None,theme="basic",randomTiles=False):
        #print "____________________________________________________"
        #print "Class ColorMap"
        self.ancestor=ancestor

        #setup tiles
        tilepath=self.setupTiles(theme,randomTiles)
        #open heightmap for reading pixel data
        hmap=PNMImage()
        hmap.read(Filename(hmapfile)) 
        #paint colormap
        cmap = self.PaintTerrain(hmap,20)
        #save colormap
        cmap.write(Filename(cmapfile))
        print "    /color map saved in",cmapfile
        print "    /tile theme:",tilepath
        
    #=========================================================================
    def setupTiles(self,theme,randomTiles):
        #setup default tile theme
        texpath="_media/tiles/"+theme+"/"
        self.tiles=[{'rgb':(16,100,255),    'tex':'_media/tiles/sea/waterdeep.jpg',   'score':0,   'h':0.5},
                     {'rgb':(16,190,255),    'tex':'_media/tiles/sea/water.jpg',       'score':0,   'h':1},
                     {'rgb':(130,150,60),    'tex':texpath+'desert.jpg',                  'score':2,   'h':2},
                     {'rgb':(100,70,0),      'tex':texpath+'plains.jpg',                  'score':2,   'h':3},
                     {'rgb':(0,153,0),       'tex':texpath+'grass.jpg',                   'score':2,   'h':4},
                     {'rgb':(5,56,5),        'tex':texpath+'jungle.jpg',                  'score':2,   'h':5},
                     {'rgb':(80,70,50),      'tex':texpath+'mountain.jpg',                'score':2,   'h':6},
                     {'rgb':(110,110,110),   'tex':texpath+'rock.jpg',                    'score':2,   'h':7},
                     {'rgb':(220,220,220),   'tex':texpath+'rock.jpg',                    'score':2,   'h':8},
                     {'rgb':(255,255,255),   'tex':texpath+'snow.jpg',                    'score':2,   'h':9}]    
        #choose random non-sea tiles
        if randomTiles==True: path=self.chooseRandomTiles(theme,2,len(self.tiles))
        #assign tiles to terrain
        self.ancestor.tiles=self.tiles
        
        return path
    
    #=========================================================================    
    def chooseRandomTiles(self,theme,start,end):
        #choose theme
        path="_media/tiles/"+theme+"/"
        if theme=='random': path="_media/tiles/"+random.choice(self.ancestor.themes)+"/"
        #print "Theme:",path
        #get images in given path
        dir=[]
        for root, dirs, files in os.walk(path):
            for name in files:       
                filepath = os.path.join(root, name)
                filename=os.path.basename(filepath)
                ext=os.path.splitext( filepath )[1]
                if ext==".db": continue
                dir.append(filename)
        #insert random image in tiles dictionary
        for i in range(start,end):
            self.tiles[i]['tex']=path+random.choice(dir)
            #self.tiles[i]['tex']='_media/tiles/terrain/B000M800.BMP'
            #print self.tiles[i]['tex']
        
        return path
    
    #=========================================================================
    def PaintTerrain(self,hmap,offset=0):
        """
        Paint a greyscale heightmap image to look like terrain.
        Besides looking cool, this can be used to determine textures
        for gameplay purposes.
        """ 
        #get size
        size=(hmap.getXSize(),hmap.getYSize())
        #create image
        cmap = PNMImage(size[0], size[1])
        #draw map
        for y in range(size[0]):
            for x in range(size[1]): 
                h=(hmap.getXel(x,y)[0]*10)
                #choose pixel color
                for item in self.tiles:
                    if h <= item['h']:
                        rgb=item['rgb']
                        break
                #draw pixel color
                cmap.setXel(x, y, rgb[0]/255.0, rgb[1]/255.0, rgb[2]/255.0)
                #print rgb,"->",rgb[0]/255.0,rgb[1]/255.0,rgb[2]/255.0
        return cmap

##############################################################################

##if __name__ == "__main__":
    ##map=DiamondMap(None,"maps/hmap.bmp",33)
    ##cmap=ColorMap(None,"maps/hmap.bmp","maps/cmap.bmp",2)
