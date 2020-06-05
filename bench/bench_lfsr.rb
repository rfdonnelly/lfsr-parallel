require_relative '../lib/lfsr'

require 'benchmark'

time = Benchmark.bmbm do |x|
  state = []
  
  x.report("unroll") do
    state = unroll_lfsr(data_size: 32, state_size: 8, polynomial: 0x07)
  end
  x.report("reduce") { reduce(state) }
end

puts time
