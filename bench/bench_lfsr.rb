require_relative '../lib/lfsr'

require 'benchmark'
include Benchmark

Benchmark.benchmark(CAPTION, 8, FORMAT, '>total') do |x|
  state = []

  t_unroll = x.report("unroll") do
    state = unroll_lfsr(data_size: 32, state_size: 16, polynomial: 0xA02B)
  end
  t_reduce = x.report("reduce") { reduce(state) }

  [t_unroll + t_reduce]
end

Benchmark.benchmark(CAPTION, 8, FORMAT, '>total') do |x|
  x.report("16 => 8") do
    state = unroll_lfsr(data_size: 16, state_size: 8, polynomial: 0x07)
    reduce(state)
  end
  x.report("32 => 16") do
    state = unroll_lfsr(data_size: 32, state_size: 16, polynomial: 0xA02B)
    reduce(state)
  end
  x.report("64 => 32") do
    state = unroll_lfsr(data_size: 64, state_size: 32, polynomial: 0x04C11DB7)
    reduce(state)
  end
end
