# OpenMV Lego Camera Capture Test

# this code works as a logic for the camera to work before i work on the detection system.

# MODE = 1:
#   Permanent training mode.
#   First checks if FIGURE_NAME already exists in /flash/trained.
#   If it exists, prints "Lego recognised".
#   Then captures 20 photos to /flash/trained/<FIGURE_NAME>/
#
# MODE = 2:
#   Temporary training mode.
#   Clears /flash/temp on boot/start.
#   First checks if TEMP_DETECT_NAME already exists in /flash/trained.
#   If it exists, prints "Lego recognised -> cancelled training"
#   and does not capture.
#   If it does not exist, captures 20 photos to /flash/temp/figure1/

import sensor
import image
import time
import os


# -------------------------
# Choose mode here
# -------------------------

MODE = 2              # 1 = permanent training, 2 = temporary training.

# Later we can have a button decide it's training moded or only preinstall permanent training.
# The two buttons already have the infrastructure to select between access and train mode

# Used in MODE 1
FIGURE_NAME = "batman"

# Used in MODE 2 for the fake/simple detect check
# if "batman" already exists in the folder, this will be recognised (same flag as future detected).
TEMP_DETECT_NAME = "bat"

PHOTO_COUNT = 20

TRAINED_FOLDER = "/flash/trained"
TEMP_FOLDER = "/flash/temp"


# -------------------------
# Folder helper functions
# -------------------------

def folder_exists(folder_path):
    try:
        os.listdir(folder_path)
        return True
    except OSError:
        return False


def make_folder(folder_path):
    if folder_exists(folder_path):
        print("Folder exists:", folder_path)
    else:
        os.mkdir(folder_path)
        print("Created folder:", folder_path)


def delete_folder_contents(folder_path):
    try:
        item_list = os.listdir(folder_path)

        for item_name in item_list:
            item_path = folder_path + "/" + item_name

            try:
                # Try deleting it as a file first
                os.remove(item_path)
                print("Deleted file:", item_path)

            except OSError:
                # If that fails, assume it is a folder
                try:
                    sub_files = os.listdir(item_path)

                    for sub_file_name in sub_files:
                        sub_file_path = item_path + "/" + sub_file_name
                        os.remove(sub_file_path)
                        print("Deleted file:", sub_file_path)

                    os.rmdir(item_path)
                    print("Deleted folder:", item_path)

                except OSError as e:
                    print("Could not delete:", item_path, e)

    except OSError as e:
        print("Could not read folder:", folder_path, e)


def clear_temp_folder():
    make_folder(TEMP_FOLDER)

    print("")
    print("Clearing temp folder...")
    delete_folder_contents(TEMP_FOLDER)
    print("Temp folder cleared.")
    print("")


# -------------------------
# Simple temporary detect function
# -------------------------

def temp_detect(figure_name):
    trained_figure_folder = TRAINED_FOLDER + "/" + figure_name

    print("")
    print("Checking trained folder for:", figure_name)
    print("Looking for:", trained_figure_folder)

    if folder_exists(trained_figure_folder):
        print("Lego recognised:", figure_name)
        return True
    else:
        print("Lego not recognised:", figure_name)
        return False


# -------------------------
# Figure naming
# -------------------------

temp_train_count = 1

def get_next_temp_figure_name():
    global temp_train_count

    figure_name = "figure" + str(temp_train_count)
    temp_train_count += 1

    return figure_name


# -------------------------
# Camera setup
# -------------------------

sensor.reset()
sensor.set_pixformat(sensor.RGB565)
sensor.set_framesize(sensor.QVGA)
sensor.skip_frames(time=2000)

# Keep settings consistent after warm-up.
sensor.set_auto_gain(False)
sensor.set_auto_whitebal(False)

clock = time.clock()


# -------------------------
# Capture function
# -------------------------

def capture_photos(save_folder, label_name):
    make_folder(save_folder)

    print("")
    print("Starting capture")
    print("Figure label:", label_name)
    print("Saving to:", save_folder)
    print("Photos:", PHOTO_COUNT)
    print("Keep the Lego figure still.")
    print("")

    for i in range(PHOTO_COUNT):
        clock.tick()

        img = sensor.snapshot()

        file_name = label_name + "_" + str(i + 1) + ".jpg"
        file_path = save_folder + "/" + file_name

        img.save(file_path)

        print("Saved:", file_path)

        time.sleep_ms(500)

    print("")
    print("Capture complete.")
    print("")


# -------------------------
# Boot/start actions
# -------------------------

make_folder(TRAINED_FOLDER)
make_folder(TEMP_FOLDER)

# Clear temp every time the script starts
clear_temp_folder()


# -------------------------
# Main program
# -------------------------

if MODE == 1:
    # First check if this figure already exists
    detected = temp_detect(FIGURE_NAME)

    if detected:
        print("Lego recognised.")
        print("Existing trained figure:", FIGURE_NAME)

    # Still allow permanent capture in MODE 1
    figure_folder = TRAINED_FOLDER + "/" + FIGURE_NAME
    capture_photos(figure_folder, FIGURE_NAME)

elif MODE == 2:
    # Check whether the current Lego is already trained
    detected = temp_detect(TEMP_DETECT_NAME)

    if detected:
        print("Lego recognised -> cancelled training.")
        print("Training was not saved to temp.")

    else:
        print("Unknown Lego -> temporary training allowed.")

        temp_figure_name = get_next_temp_figure_name()
        temp_figure_folder = TEMP_FOLDER + "/" + temp_figure_name

        capture_photos(temp_figure_folder, temp_figure_name)

else:
    print("Invalid MODE.")
    print("Use MODE = 1 or MODE = 2.")
