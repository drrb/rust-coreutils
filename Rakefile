task :default => [:test]

task :build do
  system "cargo build" or fail "Cargo build failed"
end

task :test => :build do
  system "rspec --color"
end
