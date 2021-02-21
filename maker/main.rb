require 'open-uri'
require 'nokogiri'

url = 'https://developer.twitter.com/en/docs/twitter-api/v1/data-dictionary/object-model/user'

doc = Nokogiri::HTML(URI.open(url))

doc.xpath('//table[1]//tr').each_with_index do |tr, index|
    next if index == 0
    puts index
    tr = tr.xpath('td[1]')
    puts tr.text
  end