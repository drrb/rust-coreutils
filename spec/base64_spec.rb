require "spec_helper"

describe "base64" do
  it "encodes stdin" do
    expect("target/base64 <<< hello").to emit(stdout: "aGVsbG8K")
  end
end
