require 'rake/testtask'

Rake::TestTask.new do |t|
  t.libs << 'test'
end

task :bench do
  ruby 'bench/bench_lfsr.rb'
end

task :default => :test
