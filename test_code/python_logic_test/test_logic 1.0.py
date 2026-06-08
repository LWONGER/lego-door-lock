# Text-based Lego door recognition system test using colours

allowed_figures = []
door_open = False

while True:
    print("\nLEGO DOOR SYSTEM")
    print("----------------")
    print("1. Train new figure")
    print("2. Test figure")
    print("3. Show allowed figures")
    print("4. Exit")

    choice = input("Choose option -> ")

    if choice == "1":
        print("\nTraining mode selected.")
        print("Camera captures figure...")

        figure_name = input("Enter figure name/id -> ")
        figure_colour = input("Enter main colour of figure -> ").lower()

        allowed_figures.append({
            "name": figure_name,
            "colour": figure_colour
        })

        print("Saved", figure_name, "as an allowed figure.")
        print("Training colour:", figure_colour)
        print("Training complete.")

    elif choice == "2":
        print("\nRecognition mode selected.")
        print("Camera captures figure...")

        detected_colour = input("Detected main colour -> ").lower()

        match_found = False
        matched_figure = None
        confidence = 0

        for figure in allowed_figures:
            if detected_colour == figure["colour"]:
                match_found = True
                matched_figure = figure
                confidence = 95
                break

        if match_found:
            door_open = True

            print("Figure recognised.")
            print("I think it is:", matched_figure["name"])
            print("Matched colour:", matched_figure["colour"])
            print("Confidence:", str(confidence) + "%")
            print("Door opened.")

        else:
            door_open = False

            print("Figure not recognised.")
            print("I think it is: UNKNOWN")
            print("Detected colour:", detected_colour)
            print("Confidence: 0%")
            print("Access rejected.")

    elif choice == "3":
        print("\nAllowed figures:")

        if len(allowed_figures) == 0:
            print("No figures trained yet.")
        else:
            for figure in allowed_figures:
                print("-", figure["name"], "| colour:", figure["colour"])

    elif choice == "4":
        print("System stopped.")
        break

    else:
        print("Invalid option.")

    print("Door open:", door_open)