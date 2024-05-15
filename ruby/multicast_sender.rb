#!/usr/bin/env ruby
require "socket"
require "ipaddr"



# 224.0.0.50:55583
MULTICAST_ADDR = "224.0.0.50"
BIND_ADDR = "0.0.0.0"
PORT = 55583


socket = UDPSocket.open
socket.setsockopt(:IPPROTO_IP, :IP_MULTICAST_TTL, 1)

count=0

loop do

  msg="Hello #{count}"

  print("Sending msg=<#{msg}>\n")
  STDOUT.flush()
  socket.send(msg, 0, MULTICAST_ADDR, PORT)
  sleep(0.300)

  count +=1
end
  

socket.close

