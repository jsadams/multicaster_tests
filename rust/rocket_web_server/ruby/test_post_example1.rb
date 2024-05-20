#!/usr/bin/env ruby

# nethttp3.rb
require 'uri'
require 'net/http'

#uri = URI('https://jsonplaceholder.typicode.com/posts')
uri = URI('http://127.0.0.1/api')

res = Net::HTTP.post_form(uri, 'title' => 'foo', 'body' => 'bar', 'userID' => 1)
puts res.body  if res.is_a?(Net::HTTPSuccess)
