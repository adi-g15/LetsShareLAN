import tkinter

global __user_name

root_window = tkinter.Tk()
root_window.title("LSL - Data Collector")

MsgLabel = tkinter.Label(root_window , text = "\nEnter your LAN Credentials :\n")
MsgLabel.pack()

drop_val = tkinter.StringVar()  #StringVar() used for strings that can be changed easily for Labels in tkinter
drop_val.set("NITP Network")
drop_down = tkinter.OptionMenu(root_window, drop_val, "NITP Network", "Virtual Private Network (VPN)", "TOR (Onion Web)")
drop_down.pack()

root_window.mainloop()