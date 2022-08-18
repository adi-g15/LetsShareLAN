# NOTE: CODE NOT IN USE. Leave this file for personal reasons. Do NOT Delete.

import tkinter
import requests

#Test Credentials
user_name = 'enrollment_number'
pass_wd = 'password'

class LoginCredentials(object):
    def __init__(self, uid, passwd):
        self.__user_name = uid
        self.__password = passwd

#We may have another class for firebase works, then pack the data in a LoginCredentials object, and then pass it on to JG15Window, in constructor

class JG15Window(object):
    __user_name = "Default"
    __chosen_msg = "Null"
    login_cred = LoginCredentials('default_user','default')
    root_window = tkinter.Tk()
    login_Session = requests.Session()
    drop_val = tkinter.StringVar()  #StringVar() used for strings that can be changed easily for Labels in tkinter
    nitp_mode = 193 #means LoggedOut
    def __init__(self): #the constructor for the class
        #print("Initiated...")
#        self.root_window = tkinter.Tk()
        self.__user_name = "Aditya Gupta"
        self.root_window.title("LSL - Login Manager")
        self.root_window.geometry("400x400")
        self.root_frame_window = tkinter.Frame(self.root_window)
        self.root_frame_window.pack()
        self.usr_name = ''
        self.pwd = ''

    def update_cred(self, New_Credentials):
        self.login_cred = New_Credentials

    def set_lsl_menu(self):
        for widget in self.root_frame_window.winfo_children():    #Making sure it is empty
            widget.destroy()

        MsgLabel = tkinter.Label(self.root_frame_window , text = "\nChose preferred connection :\n")
        MsgLabel.pack()

        self.drop_val.set("NITP Network")
        drop_down = tkinter.OptionMenu(self.root_frame_window, self.drop_val, "NITP Network", "Virtual Private Network (VPN)", "TOR (Onion Web)")
        #drop_down = tkinter.OptionMenu(root_window, drop_val, drop_val1, drop_val2, drop_val3)
        drop_down.pack()
        #drop_down.bind('<1>',handle) or any other 'widget' has the bind attribute, but StringVal don't
        #drop_val1.bind('<1>',handle)  #<1> is shorthand for <ButtonPress-1>
        #drop_val2.bind('<1>',handle)  #<1> is shorthand for <ButtonPress-1>
        #drop_val3.bind('<1>',handle)  #<1> is shorthand for <ButtonPress-1>

        #connect_btn = tkinter.Button(root_window, command = handle(drop_val))
        #Did the function call in the above one, so that it be clicked only once, and doesn't 'listen' for later clicks
        connect_btn = tkinter.Button(self.root_frame_window, text="Click Me")#, command=handle)
        connect_btn.bind('<1>', self.handle)
        connect_btn.pack()

    def handle(self,event):
        #AT END OF THIS FUNCTION< replace with a greeting kind of thing
        for widget in self.root_frame_window.winfo_children():
            widget.destroy()

        ConnectingLabel = tkinter.Label(self.root_frame_window, text = "Connecting...") #Can convert this into one-liner
        ConnectingLabel.pack()

        chosen_option = str(self.drop_val.get())
        if chosen_option == "NITP Network":
            self.__connect_msg = self.connect_nitp()
            Logout_Btn = tkinter.Button(self.root_frame_window, text = "Log Out")
            Logout_Btn.bind('<1>', self.disconnect_nitp)
            Logout_Btn.pack()

        elif chosen_option == "Virtual Private Network (VPN)":
            self.__connect_msg = self.connect_vpn()
        elif chosen_option == "TOR (Onion Web)":
            self.__connect_msg = self.connect_tor()
        ConnectingLabel.destroy()

        Connection_Label = tkinter.Label(self.root_frame_window, text = self.__connect_msg)
        Connection_Label.pack()

    def __enter__(self):    #Not being called anywhere
        print("Entered...")
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):  #Not being called anywhere
        print("Do anything here, the Main Window is closing")

    def on_closing(self):
        print("I am being closed!!!! :-| , So on next line i will exit gracefully!")
        print("Closing Connections if any")
        self.disconnect_nitp()
        #similarlu employ other disconnect functions too...
        exit(0)

    def connect_selenium_nitp(self):
        #Connect using Selenium Firefox
        return "Connected to NIT Network"

    def connect_nitp(self):
    #   Login (file - login.xml)-    {"Form data":{"mode":"191","username":"190234","password":"Adi@15035","a":"1581583260261","producttype":"0"},"Request payload":{"EDITOR_CONFIG":{"text":"mode=191&username=190234&password=Adi%4015035&a=1581583260261&producttype=0","mode":"text/xml"}}}
        #191, 192, and 193 are the modes before login, for checking 'live' status or not, and after logout
    #   Logout (file - logout.xml)-   {"Form data":{"mode":"193","username":"190234","a":"1581583554503","producttype":"0"},"Request payload":{"EDITOR_CONFIG":{"text":"mode=193&username=190234&a=1581583554503&producttype=0","mode":"text/xml"}}}
    #   NewTab Login - {"Form data":{"mode":"191","username":"190234","password":"Adi@15035","a":"1581583664829","producttype":"0"},"Request payload":{"EDITOR_CONFIG":{"text":"mode=191&username=190234&password=Adi%4015035&a=1581583664829&producttype=0","mode":"text/xml"}}}
    #   NewTab Logout - {"Form data":{"mode":"193","username":"190234","a":"1581583735564","producttype":"0"},"Request payload":{"EDITOR_CONFIG":{"text":"mode=193&username=190234&a=1581583735564&producttype=0","mode":"text/xml"}}}

        if self.nitp_mode == 191:
            return "Already Connected to the NITP Network"

        login_url = "http://172.172.172.100:8090/httpclient.html"

        header = {
            'user-agent' : 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:72.0) Gecko/20100101 Firefox/72.0'
    #        'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.100 Safari/537.36'
        }

        form_data = {
            "mode":"191",
            "username":self.usr_name,
            "password":self.pwd,
            "a":"1581616003458",
            "producttype":"0"
            }

        login_resp = self.login_Session.get(login_url, headers = header)
        login_resp = self.login_Session.post(login_url, data = form_data, headers = header)
        print(login_resp)
        #May verify the login respnse too... whether it's code is 200 or not
        self.nitp_mode = 191

        return "Connected to NIT Network"

    def disconnect_nitp(self,event = None):

        if self.nitp_mode == 193:
            return "Not Connected"

        login_url = "http://172.172.172.100:8090/httpclient.html"

        header = {
            'user-agent' : 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:72.0) Gecko/20100101 Firefox/72.0'
    #        'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.100 Safari/537.36'
        }

        logout_data = {
            "mode":"193",
            "username":self.usr_name,
            "a":"1581619053328",
            "producttype":"0"
        }

        logout_resp = self.login_Session.post(login_url, data = logout_data, headers = header)
        print(logout_resp)

        self.set_lsl_menu()
        LogOut_Label = tkinter.Label(self.root_frame_window, text = "You have been successfully logged out!")

        LogOut_Label.pack()
        self.nitp_mode = 193

        return "Logged Out of NITP"

    def connect_vpn(self):
        return "Connected to VPN"

    def connect_tor(self):
        return "Connected to TOR"

#Contact firebase here...   #Actually this should have been done after user choses that 'NITP Network'
#And update the credentials

main_window = JG15Window()
main_window.update_cred(LoginCredentials(user_name,pass_wd))

main_window.root_window.protocol("WM_DELETE_WINDOW", main_window.on_closing)
main_window.set_lsl_menu()
main_window.root_window.mainloop()

print("End of Script")
