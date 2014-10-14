require "spec_helper"

describe "base64" do
  it "encodes stdin" do
    pending
    expect("target/base64 <<< hello").to emit(stdout: "aGVsbG8K\n")
  end

  describe "--decode" do
    it "decodes stdin" do
      expect("target/base64 --decode <<< aGVsbG8K").to emit(stdout: "hello\n")
    end

    context "when stdin can't be decoded" do
      it "prints question marks for invalid blocks" do
        pending
        expect("target/base64 --decode <<< hello").to emit(stdout: "??e\n")
      end
    end
  end
end
