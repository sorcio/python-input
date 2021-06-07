import py_input_window

def handler(scancode, keycode, state):
    print("python handler", scancode, keycode, state)

print("Displaying main window.")
py_input_window.main_window(handler)
print("Finished.")
