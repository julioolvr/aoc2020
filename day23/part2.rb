STARTING_VALUES = [2, 8, 4, 5, 7, 3, 9, 6, 1]

class List
  include Enumerable
  attr_accessor :head, :tail

  def initialize(head)
    @head = head
    @tail = nil
  end

  def each(&block)
    if block_given?
      next_list = self

      until next_list.nil?
        block.call(next_list)
        next_list = next_list.tail
      end
    else
      to_enum(:each)
    end
  end

  def <=>(other)
    self.head <=> other.head
  end

  def push(value)
    if @tail.nil?
      @tail = List.new(value)
    else
      @tail.push(value)
    end

    @tail
  end

  def pop
    new_list = @tail
    @tail = nil
    [self, new_list]
  end

  def remove(amount)
    extracted_list = @tail
    @tail = nth(amount + 1)
    extracted_list.shrink_to(amount)

    extracted_list
  end

  def nth(n)
    if n == 0
      self
    elsif @tail
      @tail.nth(n - 1)
    else
      nil
    end
  end

  def shrink_to(size)
    if size == 1
      @tail = nil
    elsif @tail
      @tail.shrink_to(size - 1)
    end
  end

  def append(other)
    last_list = self
    last_list = last_list.tail while last_list && last_list.tail

    if last_list
      last_list.tail = other
    end
  end

  # Quick cheat to quickly add a bunch of numbers at the end of the list
  def append_numbers(numbers)
    return @tail.append_numbers(numbers) if @tail != nil

    next_tail = push(numbers.next)

    while next_tail
      next_number = numbers.next rescue break
      next_tail = next_tail.push(next_number)
    end
  end

  def insert_next(other)
    old_tail = @tail
    @tail = other
    other.append(old_tail)
  end

  def last
    last_list = self

    until last_list.tail.nil?
      last_list = last_list.tail
    end

    last_list
  end
end

list = List.new(STARTING_VALUES.first)
STARTING_VALUES.drop(1).each { |n| list.push(n) }
list.append_numbers((10..1_000_000).each)

min, max = list.minmax
min = min.head
max = max.head

current = list

# Lists are mutated but the instances don't change, this gives me
# faster lookups without having to go through a million items each
# time
lists_by_value = {}
next_list = list
while next_list
  lists_by_value[next_list.head] = next_list
  next_list = next_list.tail
end

# Keep track of the last list for more efficient appending
last_list = current.last

10_000_000.times do |i|
  if i % 100_000 == 0
    puts "#{(i.to_f / 10_000_000 * 100).to_i}%"
  end

  # Since `current` is always the first element, then removing 3
  # * Will never get to the end of the list
  # * Will never change `last_list`
  removed_list = current.remove(3)

  destination_value = current.head > min ? current.head - 1 : max

  while removed_list.any? { |list| list.head == destination_value }
    if destination_value > min
      destination_value -= 1
    else
      destination_value = max
    end
  end

  destination_list = lists_by_value[destination_value]
  destination_list.insert_next(removed_list)

  if destination_list == last_list
    # Inserted at the end, need to update the `last_list` reference
    last_list = removed_list.last
  end

  previous_current, current = current.pop

  last_list.append(previous_current)
  last_list = previous_current
end

result = current
         .drop_while { |list| list.head != 1 }
         .drop(1)
         .take(2)
         .inject(1) { |acc, list| acc * list.head }
puts "Part 2: #{result}"
