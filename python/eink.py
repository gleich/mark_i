import inky
from PIL import ImageFont, Image, ImageDraw
import time
from datetime import datetime, timedelta
from num2words import num2words
import schedule
from loguru import logger

display = inky.InkyPHAT_SSD1608("black")
main_font = ImageFont.truetype("./inter-bold.otf", 20)
big_font = ImageFont.truetype("./inter-bold.otf", 80)
medium_font = ImageFont.truetype("./inter-bold.otf", 40)
little_font = ImageFont.truetype("./inter-regular.otf", 12)

def main(now):
    logger.info("Starting update")
    out = Image.new("P", (display.WIDTH, display.HEIGHT + 110))
    d = ImageDraw.Draw(out)
    d.multiline_text(
        (15, 0), now.strftime("%B"), font=main_font, fill=display.BLACK
    )
    d.multiline_text(
        (10, 10), now.strftime("%d"), font=big_font, fill=display.BLACK
    )
    d.multiline_text(
        (105, 15),
        num2words(now.day, to="ordinal_num").strip(str(now.day)),
        font=main_font,
        fill=display.BLACK,
    )
    d.rectangle((15, 95, 125, 100), outline=display.BLACK, fill=display.BLACK)
    d.multiline_text(
        (15, 100),
        now.strftime("%A"),
        font=main_font,
        fill=display.BLACK,
    )
    d.multiline_text(
        (15, 130),
        "{} day of week".format(
            num2words(int(now.strftime("%w")) + 1, to="ordinal_num")
        ),
        font=little_font,
        fill=display.BLACK,
    )
    d.multiline_text(
        (15, 145),
        "{} day of year".format(
            num2words(int(now.strftime("%-j")), to="ordinal_num")
        ),
        font=little_font,
        fill=display.BLACK,
    )
    d.multiline_text(
        (15, 160),
        "{} week of year".format(
            num2words(int(now.strftime("%W")) + 1, to="ordinal_num")
        ),
        font=little_font,
        fill=display.BLACK,
    )
    d.rectangle((15, 185, 125, 190), outline=display.BLACK, fill=display.BLACK)
    d.multiline_text(
        (15, 188), now.strftime("%I:%M"), font=medium_font, fill=display.BLACK
    )
    display.set_image(out.rotate(270))
    display.show()
    logger.info("Updated")


if __name__ == "__main__":
    logger.info("Starting eink task")
    while True:
        now = datetime.now()
        if now.second == 55:
            main(now + timedelta(seconds=5))
