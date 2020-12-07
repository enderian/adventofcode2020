require 'set'
require 'pry'

BAG = /^(\w+)\s([\w\s]+?)\sbags?.?$/
EXTRA = ARGV.any? { |x| x == '--extra' }

file = File.read("inputs/dec07.txt")
$tree = {}
file.split("\n").each do |rule|
  key = rule.split(' bags contain ')[0]
  next if rule.split(' bags contain ')[1] == 'no other bags.'

  contents = rule.split(' bags contain ')[1].split(',')
    .map(&:strip)
    .map do |x|
      g = x.match(BAG) { |m| m.captures }
      [g.drop(1), g.first.to_i].flatten
    end

  if EXTRA
    contents.each do |k|
      $tree[key] ||= Set.new
      $tree[key] << k
    end
  else
    contents.each do |k|
      $tree[k] ||= Set.new
      $tree[k] << key
    end
  end
end

if EXTRA
  def contains(key)
    return 1 unless $tree.key?(key)
  
    a = 1 + $tree[key]&.map { |x, c| c * contains(x) }.sum
  end

  puts contains('shiny gold') - 1
else
  def contained(key)
    return [key] unless $tree.any {|x| }.key?(key)
  
    $tree[key].to_a + $tree[key]&.map{ |x| contained(x) }.flatten
  end

  puts contained('shiny gold').uniq.size
end
