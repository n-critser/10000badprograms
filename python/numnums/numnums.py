import numpy as np
import scipy as sc
import matplotlib as mpl


def lde(a,b,c,x,y):
    res= (a*x) + (b*y)
    if res == c:
        print ("a:{} , b:{}, c:{} - x:{} , y:{}".format(a,b,c,x,y))

    else :
        pass
        #print ("no match found")

    return res == c


lde(2,4,6,-1,1)

for i in range(-12,100):
    for j in range(-12,100):
        lde(2,4,6,i,j)
