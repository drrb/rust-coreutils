require "spec_helper"

describe "echo" do
  context "with no arguments" do
    it "prints a newline character" do
      expect("target/echo").to emit(stdout: "\n")
    end
  end

  context "with one argument" do
    it "prints the argument" do
      expect("target/echo hello ").to emit(stdout: "hello\n")
    end
  end

  context "with multiple arguments" do
    it "prints the arguments with spaces in between" do
      expect("target/echo hello 1 2 3 goodbye").to emit(stdout: "hello 1 2 3 goodbye\n")
    end
  end

  describe "-n" do
    it "suppresses the newline character" do
      expect("target/echo -n hello").to emit(stdout: "hello")
    end
  end
end
