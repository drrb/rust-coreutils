require "open4"

def output_with(parts = {})
  Output.new(parts)
end

class Output
  attr_reader :exit_code, :stdout, :stderr

  def initialize(parts)
    @stdout = parts[:stdout] || ""
    @stderr = parts[:stderr] || ""
    @exit_code = parts[:exit_code] || 0
  end

  def ==(output)
    ( output.exit_code.nil? || @exit_code.nil? || @exit_code == output.exit_code) &&
      ( output.stdout.nil? || @stdout.nil? || @stdout == output.stdout) &&
      ( output.stderr.nil? || @stderr.nil? || @stderr == output.stderr)
  end
end

def output_of(command)
  out, err, status = run(command)
  output_with(stdout: out, stderr: err, exit_code: status.exitstatus)
end

def run(command)
  pid, stdin, stdout, stderr = Open4.popen4(command)
  ignored, status = Process::waitpid2 pid
  return [ stdout.read, stderr.read, status ]
ensure
  stdin.close
  stdout.close
  stderr.close
end

