#!/usr/bin/python
# -*- coding:utf-8 -*-
import sys
import os
# sys.path.append(os.path.join(os.path.dirname(__file__), 'python'))
# sys.path.append(os.path.join(os.path.dirname(__file__), 'python/lib/waveshare_epd'))
picdir = os.path.join(os.path.dirname(os.path.dirname(os.path.realpath(__file__))), 'pic')
libdir = os.path.join(os.path.dirname(os.path.dirname(os.path.realpath(__file__))), 'lib')
if os.path.exists(libdir):
    sys.path.append(libdir)

print(sys.path)
import logging
from waveshare_epd import epd7in5_V2
import time
from PIL import Image,ImageDraw,ImageFont
import traceback
import get_shan_shui_bmp 


bmp_file = get_shan_shui_bmp()
print(f"got bmp {bmp_file}")
logging.basicConfig(level=logging.DEBUG)
exit()

try:
    logging.info("epd7in5_V2 Demo")
    epd = epd7in5_V2.EPD()
    
    logging.info("init and Clear")
    epd.init()
    epd.Clear()

    logging.info("3.read bmp file")
    Himage = Image.open(os.path.join(picdir, '7in5_V2.bmp'))
    epd.display(epd.getbuffer(Himage))
    time.sleep(2)

    logging.info("4.read bmp file on window")
    Himage2 = Image.new('1', (epd.width, epd.height), 255)  # 255: clear the frame
    bmp = Image.open(os.path.join(picdir, '100x100.bmp'))
    Himage2.paste(bmp, (50,10))
    epd.display(epd.getbuffer(Himage2))
    time.sleep(2)

    logging.info("Clear...")
    epd.init()
    epd.Clear()

    logging.info("Goto Sleep...")
    epd.sleep()
    
except IOError as e:
    logging.info(e)
    
except KeyboardInterrupt:    
    logging.info("ctrl + c:")
    epd7in5_V2.epdconfig.module_exit()
    exit()
