### Inferences

> Taken from NoCaptive:/"GET and PUSH requests of the Captive Portal.txt"
>
> Usme kai baate likhi hui thi periodic live requests, aur `oncloselogout.js` ke baare me. Ab wo file hai hi nhi 😕.
> 
> Plus most probably ye auto logout after sometime of inactivity wala cheez hta diya hai likely, the `liveReqTimeInJS` is not used anymore

1. The 'a' argument passed is actually equal to '(new Date).getTime()' [found in httpclient.js]
2. producttype is 1 for iphone, and ipad; And producttype is 2 for android (found in lin 176 in httpclient.js Beatified code)

