import time
import psutil
import colorsys
import blinkt

def main():
    print("Started main method")
    blinkt.set_clear_on_exit()
    blinkt.set_brightness(0.1)
    while True:
        # render_percent(psutil.cpu_percent() / 100)
        render_percent(1)
        time.sleep(0.001)

def render_percent(percent: float) -> None:
    x = round(percent * 8)
    for i in range(x):
        r, g, b = tuple(round(j * 255) for j in colorsys.hls_to_rgb(((abs(i - 7) / 8) * 120) / 360, 0.5, 1))
        blinkt.set_pixel(i, r, g, b)
    blinkt.show()

if __name__ == "__main__":
    main()
