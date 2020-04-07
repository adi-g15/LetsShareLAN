import tkinter
import requests

class LoginCredentials(object):
    def __init__(self,uid,passwd):
        self.__user_name = uid
        self.__password = passwd

class Data_Collector(object):
    __user_name = "Default"
    def __init__(self):
        self.__user_name = "Aditya Gupta"
        self.root_window = tkinter.Tk()
        self.root_window.title("LSL - Data Collector")
        self.root_window.geometry("400x400")
#        self.root_frame_window = tkinter.Frame(self.root_window)

        #Setting the looks
        MsgLabel = tkinter.Label(self.root_window , text = "\nEnter your LAN Credentials :\n")
        MsgLabel.pack()
        tkinter.Label(self.root_window, text = "UserName").grid(row = 0)
        tkinter.Label(self.root_window, text = "Password").grid(row = 1)
        #For elements of a grid... No need to pack()
        uid_box = tkinter.Entry(self.root_window)
        pass_box = tkinter.Entry(self.root_window)
        uid_box.grid(row = 0, column = 1)
        pass_box.grid(row = 1, column = 1)

main_window = Data_Collector()
main_window.root_window.mainloop()