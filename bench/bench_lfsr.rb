require_relative '../lib/lfsr'

require 'benchmark'
include Benchmark

Benchmark.benchmark(CAPTION, 8, FORMAT, '>total') do |x|
  state = []
  
  t_unroll = x.report("unroll") do
    state = unroll_lfsr(data_size: 32, state_size: 8, polynomial: 0x07)
  end
  t_reduce = x.report("reduce") { reduce(state) }

  [t_unroll + t_reduce]
end
