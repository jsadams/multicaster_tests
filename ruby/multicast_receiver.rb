#!/usr/bin/env ruby
require "socket"
require "ipaddr"

# 224.0.0.50:55583
MULTICAST_ADDR = "224.0.0.50"
BIND_ADDR = "0.0.0.0"
PORT = 55583

socket = UDPSocket.new
membership = IPAddr.new(MULTICAST_ADDR).hton + IPAddr.new(BIND_ADDR).hton

socket.setsockopt(:IPPROTO_IP, :IP_ADD_MEMBERSHIP, membership)
socket.setsockopt(:SOL_SOCKET, :SO_REUSEPORT, 1)

socket.bind(BIND_ADDR, PORT)

print("Waiting for data from #{MULTICAST_ADDR}:#{PORT}")
print("bind_addr=#{BIND_ADDR}}\n\n")
loop do
  message, _ = socket.recvfrom(255)
  puts message
end
