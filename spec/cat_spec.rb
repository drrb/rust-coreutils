require "spec_helper"

describe "cat" do
  it "prints stdin" do
    expect("target/cat <<< hello").to emit(stdout: "hello\n")
  end

  context "when given a file to read" do
    it "prints the file" do
      file = Tempfile.new("myfile")
      file.puts "hello"
      file.close

      expect("target/cat #{file.path}").to emit(stdout: "hello\n")
    end
  end
end
