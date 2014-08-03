require "spec_helper"

describe "false" do
  it "exits with status '1'" do
    expect("target/false").to emit(exit_code: 1)
  end
end
