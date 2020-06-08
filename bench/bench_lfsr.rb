require_relative '../lib/lfsr'

require 'benchmark'
include Benchmark

Benchmark.benchmark(CAPTION, 8, FORMAT, '>total') do |x|
  x.report("16 => 8") do
    unroll_lfsr(data_size: 16, state_size: 8, polynomial: 0x07)
  end
  x.report("32 => 16") do
    unroll_lfsr(data_size: 32, state_size: 16, polynomial: 0xA02B)
  end
  x.report("64 => 32") do
    unroll_lfsr(data_size: 64, state_size: 32, polynomial: 0x04C11DB7)
  end
  x.report("256 => 32") do
    unroll_lfsr(data_size: 256, state_size: 32, polynomial: 0x04C11DB7)
  end
  x.report("1024 => 32") do
    unroll_lfsr(data_size: 1024, state_size: 32, polynomial: 0x04C11DB7)
  end
end
