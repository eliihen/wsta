


1mNAME0m
       wsta - The WebSocket Transfer Agent

1mSYNOPSIS0m
       1mwsta 22m[4mOPTIONS24m] URL [4mMESSAGES24m...]


1mDESCRIPTION0m
       1mwsta 22mis a program made with the philosophy that developing for WebSock-
       ets need not be hard. It therefore gets out of your way and lets you do
       your  unix magic in peace.  1mwsta 22mprovides the tools to work efficiently
       with websockets, from piping messages directly to the server, and  then
       piping  the output into neat UNIX utils. Thus you are able to use it in
       a variety of tasks, from development to monitoring of system uptime.


1mUSAGE0m
       Specify a URL to connect to  a  server,  any  output  will  then  start
       streaming  from  the server to stdout. If you need to send a message to
       the server, this can be done using stdin, or  1mMESSAGES  22marguments  (see
       1mARGUMENTS  22mfor  more  info).   1mwsta  22mdoes  not print outgoing frames by
       default. This is to be as pipe-friendly as possible. If you wish to see
       outgoing frames, the 1m-e 22moption may be for you.


1mEXIT CODES0m
       1mwsta  22mwill return exit code 130 if you exit the connection manually. If
       fatal errors during normal operations were encountered, it will  return
       1.  If the connection with the server was unexpectedly disconnected, it
       will return 2.


1mARGUMENTS0m
       1mURL    22mThe URL to connect to in the format 1mws[s]://example.com. This  a0m
              1mrequired argument.0m


       1mMESSAGES0m
              The  messages  to  send  to the server after connection has been
              established.


1mOPTIONS0m
       1m-H, --header HEADER0m
              This option will add a custom header to the  WebSocket  request.
              This  can  be any HTTP header and value, as well as custom ones.
              The input is expected to be in the format of 1mkey:value.  22mIf this
              format is not encountered, the header will not be added.


       1m-I, --head0m
              Print  the  headers  of  requests and responses that are sent to
              stdout, including any and all headers of said requests. This  is
              very  useful  for debugging why 1mwsta 22mis not able to connect to a
              server, as you will see the response code it sent.


       1m-p, --ping SECONDS0m
              Send "ping" frames to the server every 1mSECONDS 22mseconds. This  is
              helpful  if  you  want to have a an automated script with a con-
              stant connection to the server without getting disconnected, for
              example to monitor uptime.


       1m-e, --echo0m
              By default, 1mwsta 22mdoes not echo outgoing frames. This is to be as
              pipe-friendly as possible. By providing the 1m-e 22moptions, you  can
              tell 1mwsta 22mto echo outgoing frames to stdout as well.


       1m-l, --login URL0m
              Passing  this  parameter will make 1mwsta 22msend an HTTP GET request
              before connecting to the WebSocket. This request is expected  to
              be  a  login  URL,  which returns a 1mSet-Cookie 22mheader containing
              some sort of session cookie. This cookie is  the  extracted  and
              placed  into  the WebSocket request. Using this method, 1mwsta 22mcan
              connect to WebSockets behind a login.


       1m--follow-redirect0m
              Related to the 1m--login 22moption above, this  request  will  change
              the  default  behavior.  By default 1m--login 22mwill not follow HTTP
              redirects. But if provided  with  the  1m--follow-redirect  22moption
              1mwsta 22mwill honour any redirects the server requests.


       1m-v, --verbose0m
              Make 1mwsta 22mmore verbose. This option will print varying levels of
              output to stdout. It can be provided up to three times in  order
              to  log  more  verbose  output. The first level will mostly just
              tell you which step 1mwsta 22mis currently executing and provide more
              detailed  error reports. The two other options are for debugging
              purposes.


       1m-V, --version0m
              Show the installed version of 1mwsta, 22mthen exits.


       1m-h, --help0m
              Shows a helpful message containing all supported  input  parame-
              ters, then exits.


1mEXAMPLES0m
       1mwsta ws://echo.websocket.org ping0m
              Send  a  ping  frame to a server and see the response printed to
              stdout.


       1mwsta -I -v ws://test.example.com0m
              Show more information about an error, as  well  as  any  headers
              send  and  received.   In this case we can see "failed to lookup
              address", which means it is an invalid URL.



1mBUGS0m
       When submitting bugs, please provide as  verbose  output  as  possible.
       This  can be done using a combination og 1m-vvv 22mand 1m-I.  22mPlease also pro-
       vide the output of 1mwsta --version.  22mYou should also  provide  a  public
       server which you can consistently reproduce your issue against, as well
       as the exact word-for-word command which reproduces the issue.  If  the
       only  server  you can reproduce against is private, feel free to send a
       pull request with a fix, as I will likely not be able to help you.

       Bugs can be submitted at 1mhttps://github.com/esphen/wsta/issues.0m




