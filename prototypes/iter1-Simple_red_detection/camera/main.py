# simple red detection test for lego door project
# prints RED or NO_RED over usb serial

import sensor
import time

# camera setup
sensor.reset()
sensor.set_pixformat(sensor.RGB565)
sensor.set_framesize(sensor.QVGA)
sensor.skip_frames(time=2000)
sensor.set_auto_gain(False)
sensor.set_auto_whitebal(False)

# red colour threshold
# this may need tuning depending on lighting
red_threshold = (30, 100, 15, 127, 15, 127)

# minimum blob size to count as red
min_pixels = 150
min_area = 150

while True:
    img = sensor.snapshot()

    # find red areas in the image
    blobs = img.find_blobs(
        [red_threshold],
        pixels_threshold=min_pixels,
        area_threshold=min_area,
        merge=True
    )

    if blobs:
        biggest_blob = max(blobs, key=lambda b: b.pixels())

        # draw box in openmv ide preview
        img.draw_rectangle(biggest_blob.rect())
        img.draw_cross(biggest_blob.cx(), biggest_blob.cy())

        print("RED")
    else:
        print("NO_RED")

    time.sleep_ms(500)
