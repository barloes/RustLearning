# Websocket project 

# Starting out for connecting to websocket
I first install the golang package and run the command to connect to the websocket server.  
  
go get -u github.com/hashrocket/ws  
~/go/bin/ws ws://127.0.0.1:8081/ws/  
  
Summary of this project:  
Websocket backend in rust  
  
    
# Milestone  
- [x] RustOne - working Websocket that can send messages (No Identification) [x]      
- [ ] RustTwo - working Websocket to update status of products  
    - [ ] product can be add(initialised with 0 when it is created),deleted and updated.  
    - [ ] each product have a corresponding value attached.  
    - [ ] after someone update it, list of products will be broadcasted to everyone
  
# Credit
The backend implementation is inspired and learned from the examples in Actix web websocket  
[websocket chat example](https://github.com/actix/examples/tree/master/websocket-chat)  
