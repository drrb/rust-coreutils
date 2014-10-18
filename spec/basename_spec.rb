require "spec_helper"

describe "basename" do
  it "prints the last segment of the provided path" do
    expect("target/basename /path/to/file.txt").to emit(stdout: "file.txt\n")
  end
end
