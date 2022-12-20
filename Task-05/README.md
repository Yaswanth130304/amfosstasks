# Server side errors and missing packages ornamespace
1. - Added
   - using system.IO;
   - using System.Net;
   - using System.Net.Sockets;
2. - Changed ipAddress from 11000 to 8080
3. - Changed Socket listener = new socket(ipAddressFamily); 
   - to Socket listener = new Socket(ipAddress.AddressFamily, SocketType.Stream,ProtocolType.Tcp);
4. - declared bytesRec as of type of int
5. - String[] dataArr = data.Split(',');
6. - Changed if(/Add condition based on the code/) to if(File.Exists(fileName))
7. - in main function changed Start() to StartListening();

# Client side errors and missing packages or namespace
1. - added
   - using System.Net;
   - using System.Net.Sockets;
2. - I have replaced Socket sender = new Socket(ipAddressFamily);by Socket sender = new Socket(AddressFamily.InterNetwork,SocketType.Stream, ProtocolType.Tcp);
3. - Since name,interests and mail are of type of string so I declared them as string name, interests and mail.
4. - Since we store group of messages I used msg[] of byte type
5. - In main function Start() was changed to StartClient() 
   



