

import requests
import cairosvg
import random
from PIL import Image

def get_shan_shui_bmp():
    img_num = random.randint(0, 200)
    url = f"http://localhost:6767/boat/{img_num}"
    svg_path = f"tmp/downloadedimage{img_num}.svg"
    png_path = f"tmp/converted_image{img_num}.png"
    bmp_path = f"tmp/converted_image{img_num}.bmp"

    response = requests.get(url)
    if response.status_code == 200:
        with open(svg_path, 'wb') as f:
            f.write(response.content)
        print(f"SVG saved to {svg_path}")

        cairosvg.svg2png(url=svg_path, write_to=png_path)
        print(f"Png image saved to {png_path}")

        with Image.open(png_path) as img:
            img.save(bmp_path, format="BMP")

        print(f"Conversion complete: {png_path} → {bmp_path}")
        return bmp_path
    else:
        print(f"Failed to download SVG. Status code: {response.status_code}")


get_shan_shui_bmp()

