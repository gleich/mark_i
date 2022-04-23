import inky
from PIL import ImageFont, Image, ImageDraw
import time

def main():
    display = inky.InkyPHAT_SSD1608('black')
    x = 0
    print("Started eink module")
    while True:
        out = Image.new("P", (display.WIDTH, display.HEIGHT))
        fnt = ImageFont.truetype("Pillow/Tests/fonts/FreeMono.ttf", 20)
        d = ImageDraw.Draw(out)
        d.multiline_text((10, 10), f"Hello\nWorld {x}", font=fnt, fill=display.BLACK)
        display.set_image(out)
        display.show()
        time.sleep(10)
        x += 1

if __name__ == "__main__":
    main()
