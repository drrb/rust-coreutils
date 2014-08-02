require "spec_helper"

describe "hostname" do
  it "returns the hostname" do
    expect("target/hostname").to match_the_output_of("hostname")
  end

  describe "-V" do
    it "prints the program name and version" do
      expect("target/hostname -V").to emit(exit_code: 0, stdout: "hostname 1.0.0\n")
    end
  end

  describe "--version" do
    it "prints the program name and version" do
      expect("target/hostname --version").to emit(exit_code: 0, stdout: "hostname 1.0.0\n")
    end
  end

  describe "-h" do
    it "prints the help message" do
      expect("target/hostname -h").to emit(exit_code: 0, stdout: <<-EOF)
Usage: target/hostname [options] [HOSTNAME]

Options:
    -V --version        Print the version number and exit
    -h --help           Print this help message

EOF
    end
  end

  describe "[HOSTNAME]" do
    context "when not root" do
      it "fails" do
        expect("target/hostname xxx").to emit(exit_code: 1, stderr: "hostname: you must be root to change the host name\n")
      end
    end
  end
end
