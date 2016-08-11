wsta(1)                     General Commands Manual                    wsta(1)



## NAME
       wsta - The WebSocket Transfer Agent

## SYNOPSIS
       wsta [OPTIONS] [URL] [MESSAGES...]


## DESCRIPTION
       wsta is a program made with the philosophy that developing for WebSock-
       ets need not be hard. It therefore gets out of your way and lets you do
       your  unix magic in peace.  wsta provides the tools to work efficiently
       with websockets, from piping messages directly to the server, and  then
       piping  the output into neat UNIX utils. Thus you are able to use it in
       a variety of tasks, from development to monitoring of system uptime.


## USAGE
       Specify a URL to connect to  a  server,  any  output  will  then  start
       streaming  from  the server to stdout. If you need to send a message to
       the server, this can be done using stdin, or  MESSAGES  arguments  (see
       ARGUMENTS  for  more  info).   wsta  does  not print outgoing frames by
       default. This is to be as pipe-friendly as possible. If you wish to see
       outgoing frames, the -e option may be for you.




## EXIT CODES
       wsta  will return exit code 130 if you exit the connection manually. If
       fatal errors during normal operations were encountered, it will  return
       1.  If the connection with the server was unexpectedly disconnected, it
       will return 2.


## ARGUMENTS
       URL    The URL to connect to in the format ws[s]://example.com. This  a
              required argument.  The argument is not required if provided via
              a configuration file.

              config key: url (String)


       MESSAGES
              The messages to send to the server  after  connection  has  been
              established.

              config key: messages (Array<String>)


## OPTIONS
       -H, --header HEADER
              This  option  will add a custom header to the WebSocket request.
              This can be any HTTP header and value, as well as  custom  ones.
              The input is expected to be in the format of key:value.  If this
              format is not encountered, the header will not be added.

              config key: headers (Array<String>)


       -I, --head
              Print the headers of requests and responses  that  are  sent  to
              stdout,  including any and all headers of said requests. This is
              very useful for debugging why wsta is not able to connect  to  a
              server, as you will see the response code it sent.

              config key: print_headers (Boolean)


       -p, --ping SECONDS
              Send  "ping" frames to the server every SECONDS seconds. This is
              helpful if you want to have a an automated script  with  a  con-
              stant connection to the server without getting disconnected, for
              example to monitor uptime.


       -e, --echo
              By default, wsta does not echo outgoing frames. This is to be as
              pipe-friendly  as possible. By providing the -e options, you can
              tell wsta to echo outgoing frames to stdout as well.

              config key: echo (Boolean)


       -l, --login URL
              Passing this parameter will make wsta send an HTTP  GET  request
              before  connecting to the WebSocket. This request is expected to
              be a login URL, which returns  a  Set-Cookie  header  containing
              some  sort  of  session cookie. This cookie is the extracted and
              placed into the WebSocket request. Using this method,  wsta  can
              connect to WebSockets behind a login.

              config key: login_url (String)


       -b, --binary
              Setting  this  flag  will  set  wsta into a binary mode. In this
              mode, wsta will read binary data from stdin and send it in  256B
              frames  to  the sever. If larger or smaller frames are required,
              the WSTA_BINARY_FRAME_SIZE environment variable can be  provided
              to  override  this.   WSTA_BINARY_FRAME_SIZE is specified as the
              max number of Bytes in each frame.  Binary data  sent  from  the
              server is automatically recognized and printed, there is no need
              to specify this flag when binary output is expected.

              config key: binary_mode (Boolean)


       --follow-redirect
              Related to the --login option above, this  request  will  change
              the  default  behavior.  By default --login will not follow HTTP
              redirects. But if provided  with  the  --follow-redirect  option
              wsta will honour any redirects the server requests.

              config key: follow_redirect (Boolean)


       -v, --verbose
              Make wsta more verbose. This option will print varying levels of
              output to stdout. It can be provided up to three times in  order
              to  log  more  verbose  output. The first level will mostly just
              tell you which step wsta is currently executing and provide more
              detailed  error reports. The two other options are for debugging
              purposes.


       -V, --version
              Show the installed version of wsta, then exits.


       -h, --help
              Shows a helpful message containing all supported  input  parame-
              ters, then exits.


## CONFIGURATION OPTIONS
       Some  options  are not configurable as a command line argument, only as
       part of a configuration file. These options are listed here


       binary_frame_size
              If used with the --binary flag, binary_frame_size  will  specify
              the  maximum  size  of  each  binary  frame. This is a number in
              Bytes.  If --binary is used, but this variable is not set,  then
              a default of 256 Bytes will be used.  This may be small for per-
              sistent streaming data, and a "overrun!!!" message may show,  in
              which case simply increase the fame size using this variable.

              config key: binary_frame_size (String)


## EXAMPLES
       More examples can be found in the README.


       wsta ws://echo.websocket.org ping
              Send  a  ping  frame to a server and see the response printed to
              stdout.


       wsta -I -v ws://test.example.com
              Show more information about an error, as  well  as  any  headers
              send  and  received.   In this case we can see "failed to lookup
              address", which means it is an invalid URL.


## FILES
       There is the possibility to configure wsta's behavior using  configura-
       tion  files. The options above specify what the option keys in the con-
       figuration files are. If both a command line argument and  the  equiva-
       lent  option  is  configured  in a configuration file, the command line
       argument is used.

       Windows users can replace $XDG_CONFIG_HOME below with %APPDATA%.

       The syntax of the configuration files are as follows:

       url = "ws://echo.websocket.org";
       headers = ["Origin:google.com","Foo:Bar"];
       show_headers = true;


       $XDG_CONFIG_HOME/wsta/wsta.conf
              The main configuration file used if no profile is specified.


       $XDG_CONFIG_HOME/wsta/<profile_name>/wsta.conf
              Any profiles are simply fonders inside the  wsta  config  direc-
              tory.  Any  files names wsta.conf placed in the config directory
              can later be loaded using -p <profile_name>.



## AUTHOR
       Written with love by Espen Henriksen and contributors.



## BUGS
       When submitting bugs, please provide as  verbose  output  as  possible.
       This  can be done using a combination of -vvv and -I.  Please also pro-
       vide the output of wsta --version.  You should also  provide  a  public
       server which you can consistently reproduce your issue against, as well
       as the exact word-for-word command which reproduces the issue.  If  the
       only  server  you can reproduce against is private, feel free to send a
       pull request with a fix, as I will likely not be able to help you.

       Bugs can be submitted at https://github.com/esphen/wsta/issues.




0.3.0                             11 Aug 2016                          wsta(1)
