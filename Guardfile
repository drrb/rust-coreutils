notification :terminal_notifier

guard :rspec, cmd: 'cargo build && bundle exec rspec --color' do
  watch(%r{^spec/.+_spec\.rb$})
  watch(%r{^src/(.+)\.rs$}) { |m| "spec" }
  watch('spec/spec_helper.rb')  { "spec" }
end

