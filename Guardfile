notification :terminal_notifier

guard :rspec, cmd: 'cargo build && bundle exec rspec --color' do
  watch(%r{^spec/.+_spec\.rb$})
  watch(%r{^src/(.+)\.rs$}) { |m| "spec/#{m[1]}_spec.rb" }
  watch('spec/spec_helper.rb')  { "spec" }
end

