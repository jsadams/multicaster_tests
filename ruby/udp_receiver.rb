#!/usr/bin/ruby
require 'socket'
require 'optparse'
require 'optparse/time'
require 'ostruct'
#require 'pp'


class Options_parser

   def self.parse(args)
      options = OpenStruct.new
      options.port = 7091

      opt_parser = OptionParser.new do |opts|
         opts.on("-p", "--port N", Integer, "udp listen port") do |p|
            options.port = p
         end
      end

      opt_parser.parse!(args)
      options
   end  # parse()

end  # class OptparseExample


class Udp_receiver

   attr_accessor :read_counter
   attr_accessor :t_last
   attr_accessor :socket

   def initialize(port=8888,host="<any>")
      #port = 8888

      #host = "localhost"
      #host = "192.168.1.3"

      #host = "<any>"
      #host = "255.255.255.255"
      #host = ""
      #host = nil

      @socket = UDPSocket.open
      @socket.setsockopt(Socket::SOL_SOCKET, Socket::SO_BROADCAST, true)

      @socket.bind(host, port)


      @read_counter=0

      printf("\n listening for udp data on port #{port}")
      #super()
      @t_last=Time.now

   end

   def read_data(n_bytes_requested=1)


      s=@socket.recvfrom(n_bytes_requested)

      t_now=Time.now
      dt=t_now-@t_last
      f_s=1.0/dt;
      data=s[0]
      metadata=s[1]
      n_bytes=data.size
      rate=n_bytes/dt;

      printf("\n%d) ",@read_counter)
      printf("fs=%1.2f Hz, rate=%1.2g kbps ",f_s,rate/1e3)
      printf("n_bytes_requested=%d, n_bytes=%d",n_bytes_requested,n_bytes)

      #      printf("%d) fs=%1.2 Hz, rate=%1.2f bps, n_bytes_requested=%d, n_bytes=%d",@read_counter,f_s,rate,n_bytes_requested,n_bytes)

      printf("\n payload: %s",data)
      data_bin=data.unpack("c*")
      print "\n payload:<#{data_bin}>"
      # printf("\n data= ")
      #  data_bin.each do | byte|
      #    printf("\t%08b ",byte)
      #    printf("(b7=%d)",byte[6])
      #  end

      @t_last=t_now
      return data
   end

end

########################################
#
#
#
#
########################################


#load "udp_receiver_8888.rb"
#require "udp_receiver_8888"

if __FILE__ == $0
   options = Options_parser.parse(ARGV)
   #  pp options
   #  pp ARGV

   port = options.port


   the_udp_receiver1=Udp_receiver.new(port)

   packet_size=1024

   i=0



   loop do


      i=i+1

      # print "\n #{i} \t listening on port #{port} "

      data=the_udp_receiver1.read_data(packet_size);

      # $stdout.flush

   end

end
