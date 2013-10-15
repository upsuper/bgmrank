#!/usr/bin/env ruby

require 'optparse'
require 'net/http'

CATEGORIES = [:anime, :book, :music, :game, :real]
STATES = [:wish, :collect, :do, :on_hold, :dropped]

progress = true
options = {
  :category => [:anime],
  :state => [:collect],
  :width => 70,
}
OptionParser.new do |opts|
  opts.banner = "Usage: bgmrank.rb [options] username"

  def map_check!(list, set)
    list.map! { |i| i.to_sym }
    if list.any? { |i| !set.include?(i) }
      raise OptionParser::InvalidArgument
    else
      list
    end
  end

  opts.separator ""
  opts.separator "Stats options:"
  opts.on("-c CAT", "--category CAT", Array,
          "Select category (#{CATEGORIES.join ', '})") do |list|
    options[:category] = map_check!(list, CATEGORIES)
  end
  opts.on("-s STATE", "--state STATE", Array,
          "Select state (#{STATES.join ', '})") do |list|
    options[:state] = map_check!(list, STATES)
  end

  opts.separator ""
  opts.separator "Display options:"
  opts.on("-p", "--[no-]progress", "Display progress") do |p|
    progress = p
  end
  opts.on("-w WIDTH", "--max-width WIDTH", Integer, "Max output width") do |w|
    options[:width] = w
  end

  opts.separator ""
  opts.separator "Common options:"
  opts.on_tail("-h", "--help", "Show this message") do
    puts opts
    exit
  end
end.parse!

bgm_id = ARGV[0]

total = 0
ranks = Array.new(11, 0)

Net::HTTP.start 'bgm.tv' do |http|
  options[:category].product(options[:state]) do |(category, state)|
    base_url = "/#{category}/list/#{bgm_id}/#{state}"
    $stderr.puts base_url if progress
    for i in 1..Float::INFINITY
      url = "#{base_url}?page=#{i}"
      $stderr.print "fetching page ##{i}... " if progress

      content = http.request_get(url).body
      count = content.scan(/<p class="collectInfo">/).length
      $stderr.puts count.to_s if progress
      total += count
      content.scan(/<span class="sstars(\d+) starsinfo">/) do |stars, |
        ranks[stars.to_i] += 1
      end

      break if count < 24
    end
  end
end
$stderr.puts if progress

max_num = [ranks.max.to_f, 1].max
max_len = options[:width] - max_num.to_s.length - 5
max_len = max_num if max_num < max_len
ranked = ranks.inject(:+)

rank_sum = 0
ranks.each_index do |i|
  if !i.zero?
    rank = ranks[i]
    rank_sum += rank * i

    num = (rank / max_num * max_len).round
    line = "#{i.to_s.rjust 2}: " << '#' * num
    line << " " unless num.zero?
    line << rank.to_s
    puts line
  end
end
puts "ranked: #{ranked}/#{total}"
puts "average: #{(rank_sum.to_f / ranked).round 2}"
