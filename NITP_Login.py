import requests

login_url = "http://172.172.172.100:8090/httpclient.html"

global login_session
login_session = requests.Session()

def requests_login(action_val): #action_val=0 means login
    header = {
        'user-agent' : 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:72.0) Gecko/20100101 Firefox/72.0'
#        'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.100 Safari/537.36'
    }

    form_data = {
        "mode":"191",
        "username":"190234",
        "password":"Adi@15035",
        "a":"1581616003458",
        "producttype":"0"
        }

    logout_data = {
        "mode":"193",
        "username":"190234",
        "a":"1581619053328",
        "producttype":"0"
        }

    global login_session
    if action_val == 0:
        login_resp = login_session.get(login_url, headers = header)
        #page_soup = BeautifulSoup(login_resp.content, 'html5lib')
        #form_a_element = page_soup.find('input', attrs = { '' : '' })
        login_resp = login_session.post(login_url, data = form_data, headers = header)
        print(login_resp)
    elif action_val == 1:
        logout_resp = login_session.post(login_url, data = logout_data, headers = header)
        print(logout_resp)

def selenium_login():
    import selenium
    from selenium import webdriver
    import time

    path_to_geckodriver = r"C:\Users\adity\Desktop" #the r'' allows us to pass backslash instead of forwardslash
    ######SHOWING DIRECTORY NAME INVALID, EVEN THOUGH LOCATION VERIFIED
    global __userid
    global __passwd

    def try_cred():
        global __userid
        global __passwd
        __userid = "190234"
        __passwd = "Adi@15035"

    def get_cred():
        global __userid
        global __passwd
        __userid = input("Enter your UserID")
        __passwd = input("Enter the Password : ")

    def login():
    #    try:
        userid = web_driver.find_element_by_name('username')   #given in captive portal one was that use find_by_name
        passid = web_driver.find_element_by_name('password')
        signinButton = web_driver.find_element_by_name('btnSubmit')

        userid.send_keys(__userid)
        passid.send_keys(__passwd)
        signinButton.click()
        time.sleep(3)     #rjust to make sure that the favourites page has opened up
        print(web_driver.current_url)

    #    except Exception:
    #       print ("Some error!")
    #      return 1
    #   web_driver.quit()
        return 0    #Success

    web_driver = webdriver.Firefox()#path_to_geckodriver)
#    web_driver.manage().window().setPosition(Point(-2000,0))
    web_driver.get(login_url)
    try_cred()
    login()

requests_login(0)
print("You are already here, now anythin between these...")
input("When Done, press any key")
requests_login(1)