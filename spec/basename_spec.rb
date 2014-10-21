require "spec_helper"

describe "basename" do
  it "prints the last segment of the provided path" do
    expect("target/basename /path/to/file.txt").to emit(stdout: "file.txt\n")
  end

  context "with a suffix provided" do
    context "when the basename ends with the suffix" do
      it "prints the basename without the suffix" do
        expect("target/basename /path/to/file.txt .txt").to emit(stdout: "file\n")
      end
    end

    context "when the basename doesn't end with the suffix" do
      it "prints the basename" do
        expect("target/basename /path/to/file.txt .bin").to emit(stdout: "file.txt\n")
      end
    end

    context "when the basename exactly matches the suffix" do
      it "prints the basename" do
        expect("target/basename /path/to/.txt .txt").to emit(stdout: ".txt\n")
      end
    end
  end
end
