import tkinter
#from tkinter import messagebox

global __user_name
global __chosen_msg

root_window = tkinter.Tk()
root_window.title("LSL - Login Manager")
root_window.geometry("400x400")

def handle(event):
    global __chosen_msg
    chosen_option = str(drop_val.get())
    if chosen_option == "NITP Network":
        __connect_msg = connect_nitp()
    elif chosen_option == "Virtual Private Network (VPN)":
        __connect_msg = connect_vpn()
    elif chosen_option == "TOR (Onion Web)":
        __connect_msg = connect_tor()
    Connection_Label = tkinter.Label(root_window, text = __connect_msg)
    Connection_Label.pack()

def connect_selenium_nitp():
    #Connect using Selenium Firefox
    return "Connected to NIT Network"

def connect_nitp():
#   Login (file - login.xml)-    {"Form data":{"mode":"191","username":"190234","password":"Adi@15035","a":"1581583260261","producttype":"0"},"Request payload":{"EDITOR_CONFIG":{"text":"mode=191&username=190234&password=Adi%4015035&a=1581583260261&producttype=0","mode":"text/xml"}}}
    #191, 192, and 193 are the modes before login, after login, and after logout
#   Logout (file - logout.xml)-   {"Form data":{"mode":"193","username":"190234","a":"1581583554503","producttype":"0"},"Request payload":{"EDITOR_CONFIG":{"text":"mode=193&username=190234&a=1581583554503&producttype=0","mode":"text/xml"}}}
#   NewTab Login - {"Form data":{"mode":"191","username":"190234","password":"Adi@15035","a":"1581583664829","producttype":"0"},"Request payload":{"EDITOR_CONFIG":{"text":"mode=191&username=190234&password=Adi%4015035&a=1581583664829&producttype=0","mode":"text/xml"}}}
#   NewTab Logout - {"Form data":{"mode":"193","username":"190234","a":"1581583735564","producttype":"0"},"Request payload":{"EDITOR_CONFIG":{"text":"mode=193&username=190234&a=1581583735564&producttype=0","mode":"text/xml"}}}
    return "Connected to NIT Network"

def connect_vpn():
    return "Connected to VPN"

def connect_tor():
    return "Connected to TOR"

MsgLabel = tkinter.Label(root_window , text = "\nChose preferred connection :\n")
MsgLabel.pack()

drop_val = tkinter.StringVar()  #StringVar() used for strings that can be changed easily for Labels in tkinter
drop_val.set("NITP Network")
drop_down = tkinter.OptionMenu(root_window, drop_val, "NITP Network", "Virtual Private Network (VPN)", "TOR (Onion Web)")
#drop_down = tkinter.OptionMenu(root_window, drop_val, drop_val1, drop_val2, drop_val3)
drop_down.pack()
#drop_down.bind('<1>',handle) or any other 'widget' has the bind attribute, but StringVal don't
#drop_val1.bind('<1>',handle)  #<1> is shorthand for <ButtonPress-1>
#drop_val2.bind('<1>',handle)  #<1> is shorthand for <ButtonPress-1>
#drop_val3.bind('<1>',handle)  #<1> is shorthand for <ButtonPress-1>

#connect_btn = tkinter.Button(root_window, command = handle(drop_val))
#Did the function call in the above one, so that it be clicked only once, and doesn't 'listen' for later clicks
connect_btn = tkinter.Button(root_window, text="Click Me")#, command=handle)
connect_btn.bind('<1>', handle)
connect_btn.pack()

def on_closing():
    print("I am being closed!!!! :-| , So on next line i will exit gracefully!")
    exit(0)

root_window.protocol("WM_DELETE_WINDOW", on_closing)
root_window.mainloop()