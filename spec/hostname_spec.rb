require "spec_helper"

describe "hostname" do
  it "returns the hostname" do
    expect(output_of("target/hostname")).to eq(output_of("hostname"))
  end

  context "-V" do
    it "prints the program name and version" do
      expect(output_of("target/hostname -V")).to eq(output_with(exit_code: 0, stdout: "hostname 1.0.0\n"))
    end
  end

  context "--version" do
    it "prints the program name and version" do
      expect(output_of("target/hostname --version")).to eq(output_with(exit_code: 0, stdout: "hostname 1.0.0\n"))
    end
  end

  context "[HOSTNAME]" do
    context "when not root" do
      it "fails" do
        expect(output_of("target/hostname xxx")).to eq(output_with(exit_code: 1, stderr: "hostname: you must be root to change the host name\n"))
      end
    end
  end
end
