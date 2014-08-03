require "open4"

class String
  def outdent(spaces = -1)
    if spaces == -1
      self.gsub(/^\s+/, "")
    else
      self.gsub(/^\s+{#{spaces}}/, "")
    end
  end
end

class Array
  def to_sentence
    if size == 1
      first.to_s
    else
      [self[0..-2].join(", "), self[-1]].join(" and ")
    end
  end
end

def emit(parts = {})
  OutputMatcher.new(parts)
end

class OutputMatcher
  include RSpec::Mocks::ArgumentMatchers

  def initialize(parts)
    @expected_stdout = parts[:stdout] || /.*/
    @expected_stderr = parts[:stderr] || /.*/
    @expected_exit_code = parts[:exit_code] || 0
  end

  def diffable?
    true
  end

  def failure_message
    differing_parts = []
    diffs = []
    unless exit_code_maches?
      differing_parts << "exit code"
      diffs << [ "exit code", @expected_exit_code, @actual_exit_code ]
    end
    unless stdout_matches?
      differing_parts << "stdout"
      diffs << [ "stdout", @expected_stdout, @actual_stdout ]
    end
    unless stderr_matches?
      differing_parts << "stderr"
      diffs << [ "stderr", @expected_stderr, @actual_stderr ]
    end
    <<-EOF
OUTPUT MISMATCH:
The #{differing_parts.to_sentence} differ#{differing_parts.size == 1 ? "s" : ""}.
#{diffs.map { |type, e, a| "expected #{type}: #{e.inspect}\nactual #{type}:   #{a.inspect}"}.join("\n\n")}

EXPECTED OUTPUT:
#{expected}

ACTUAL OUTPUT:
#{actual}
EOF
  end

  def expected
    output_to_s(@expected_exit_code, @expected_stdout, @expected_stderr)
  end

  def actual
    output_to_s(@actual_exit_code, @actual_stdout, @actual_stderr)
  end

  def output_to_s(x, o, e)
    <<-EOF.outdent(6)
      ===========================
      exit_code:
      ---------------------------
      #{x}
      ===========================
      stdout:
      ---------------------------
      #{o}
      ===========================
      stderr:
      ---------------------------
      #{e}
      ===========================
    EOF
  end

  def matches?(command)
    @actual_stdout, @actual_stderr, @actual_exit_code = run(command)
    exit_code_maches? && stdout_matches? && stderr_matches?
  end

  private
  def exit_code_maches?
    @expected_exit_code == @actual_exit_code
  end

  def stdout_matches?
    @expected_stdout.is_a?(Regexp) ? @expected_stdout =~ @actual_stdout : @expected_stdout == @actual_stdout
  end

  def stderr_matches?
    @expected_stderr.is_a?(Regexp) ? @expected_stderr =~ @actual_stderr : @expected_stderr == @actual_stderr
  end
end

def match_the_output_of(command)
  o, e, s = run command
  emit(stdout: o, stderr: e, exit_code: s)
end

def run(command)
  pid, stdin, stdout, stderr = Open4.popen4(command)
  ignored, status = Process::waitpid2 pid
  return [ stdout.read, stderr.read, status.exitstatus ]
end

