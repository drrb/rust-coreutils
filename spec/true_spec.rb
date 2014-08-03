require "spec_helper"

describe "true" do
  it "exits with status '0'" do
    expect("target/true").to emit(exit_code: 0)
  end
end
