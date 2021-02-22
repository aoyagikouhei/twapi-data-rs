require 'open-uri'
require 'nokogiri'


def user
  url = 'https://developer.twitter.com/en/docs/twitter-api/v1/data-dictionary/object-model/user'

  doc = Nokogiri::HTML(URI.open(url))

  doc.xpath('//table[1]//tr').each_with_index do |tr, index|
    next if index == 0
    td = tr.xpath('td[1]')
    atribute = td.text
    td = tr.xpath('td[2]')
    type = td.text
    puts "    #{atribute}: #{change_type(type)},"
  end
end

def tweet
  url = 'https://developer.twitter.com/en/docs/twitter-api/v1/data-dictionary/object-model/tweet'

  doc = Nokogiri::HTML(URI.open(url))

  doc.xpath('//table[1]//tr').each_with_index do |tr, index|
    next if index == 0
    td = tr.xpath('td[1]')
    atribute = td.text
    td = tr.xpath('td[2]')
    type = td.text
    puts "    #{atribute}: #{change_type(type)},"
  end
end

def change_type(src)
  case src
  when "Int64" then
    "u64"
  when "String" then
    "String"
  when "Boolean" then
    "bool"
  when "Int" then
    "u64"
  when "Array of String" then
    "Vec<String>"
  else
    "XXXXXXXXXXX"
  end
end

tweet