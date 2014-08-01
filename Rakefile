task :build do
  system "cargo build" or fail
end

task :test => :build do
  system "rspec --color"
end
