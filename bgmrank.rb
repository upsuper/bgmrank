#!/usr/bin/env ruby

require 'rubygems'
require 'bundler/setup'

require 'optparse'
require 'net/http'
require 'nokogiri'

CATEGORIES = [:anime, :book, :music, :game, :real]
STATES = [:wish, :collect, :do, :on_hold, :dropped]

progress = true
options = {
  :category => [:anime],
  :state => [:collect],
  :tags => false,
  :min_num => 1,
  :width => 70,
}
OptionParser.new do |opts|
  opts.banner = "Usage: bgmrank.rb [options] username"
  opts.summary_width = 25

  def map_check!(list, set)
    list = list.map do |i|
      sym = i.to_sym
      if sym == :all; set; else; sym end
    end.flatten.uniq
    if list.any? { |i| !set.include?(i) }
      raise OptionParser::InvalidArgument
    else
      list
    end
  end

  opts.separator ""
  opts.separator "Stats options:"
  opts.on("-c", "--category CAT,...", Array,
          "Categories (#{CATEGORIES.join ', '})") do |list|
    options[:category] = map_check!(list, CATEGORIES)
  end
  opts.on("-s", "--state STATE,...", Array,
          "States (#{STATES.join ', '})") do |list|
    options[:state] = map_check!(list, STATES)
  end
  opts.on("-t", "--[no-]tags", "Stats score of tags") do |t|
    options[:tags] = t
  end
  opts.on("-m", "--min-number N", Integer,
          "Only show tags with at least N ranked") do |m|
    options[:min_num] = m
  end

  opts.separator ""
  opts.separator "Display options:"
  opts.on("-p", "--[no-]progress", "Display progress") do |p|
    progress = p
  end
  opts.on("-w", "--max-width WIDTH", Integer, "Max output width") do |w|
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
if options[:tags]
  tags = Hash.new do |h, k|
    h[k] = {:total => 0, :ranks => Array.new(11, 0)}
  end
end

Net::HTTP.start 'bgm.tv' do |http|
  options[:category].product(options[:state]) do |(category, state)|
    base_url = "/#{category}/list/#{bgm_id}/#{state}"
    $stderr.puts base_url if progress
    for i in 1..Float::INFINITY
      url = "#{base_url}?page=#{i}"
      $stderr.print "fetching page ##{i}... " if progress

      content = http.request_get(url).body
      doc = Nokogiri::HTML(content)
      items = doc.css('#browserItemList>li')
      items.each do |item|
        starsinfo = item.css('.starsinfo').first
        score = if starsinfo
                  starsinfo[:class].split[0][6..-1].to_i
                else; 0 end
        ranks[score] += 1 if score > 0
        if options[:tags]
          taginfo = item.css('.collectInfo>.tip').first
          if taginfo
            taginfo.content.split[1..-1].each do |tag|
              tags[tag][:total] += 1
              tags[tag][:ranks][score] += 1 if score > 0
            end
          end
        end
      end
      $stderr.puts items.size if progress
      total += items.size

      break if items.size < 24
    end
  end
end
$stderr.puts if progress

def stat_ranks(ranks)
  ranked = ranks.inject(:+)
  sum = ranks.each_with_index.inject(0) do |sum, (count, rank)|
    sum + count * rank
  end
  return ranked, sum.to_f / ranked
end

def nan_to_ninf(f)
  f.nan? ? -Float::INFINITY : f
end

def info_key(info)
  [nan_to_ninf(info[:avg_rank]), info[:ranked], info[:total]]
end

tags.map do |tag, info|
  ranked, avg_rank = stat_ranks(info[:ranks])
  {:tag => tag, :total => info[:total], :ranks => info[:ranks],
   :ranked => ranked, :avg_rank => avg_rank}
end.sort do |a, b|
  info_key(b) <=> info_key(a)
end.each do |info|
  if info[:ranked] >= options[:min_num]
    line = "%.2f " % info[:avg_rank]
    line << "#{info[:tag]}: "
    line << "#{info[:ranked]}/#{info[:total]} "
    puts line
  end
end if options[:tags]
puts if options[:tags]

max_num = [ranks.max.to_f, 1].max
max_len = options[:width] - max_num.to_s.length - 5
max_len = max_num if max_num < max_len

ranks.each_index do |i|
  if !i.zero?
    rank = ranks[i]
    num = (rank / max_num * max_len).round
    line = "#{i.to_s.rjust 2}: " << '#' * num
    line << " " unless num.zero?
    line << rank.to_s
    puts line
  end
end
ranked, avg_rank = stat_ranks(ranks)
puts "ranked: #{ranked}/#{total}"
puts "average: #{avg_rank.round 2}"
