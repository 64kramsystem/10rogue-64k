#!/usr/bin/env ruby

require_relative 'shared'

=begin
Example in referencee:

    extern "C" {
        static mut terse: bool;
    }

Referenced:

    pub static mut terse: bool = 0 as libc::c_int != 0;

=end

class VarData < Struct.new(:name_type, :definition_file, :declaration_files); end

class DeexternVars
  RESERVED_MODULE_NAMES = ['extern']

  def execute(var_name_filter: //, output: $stdout)
    files_content = read_files
    definitions = find_definitions(files_content, output, var_name_filter)
    deextern_vars!(definitions, files_content, output)
    write_files(files_content)
  end

  private

  def read_files
    # Cache the files. This is not actually for optimization purposes, rather, because opening/writing
    # the files is noisy.
    #
    files_content = Dir
      .glob(File.join(SOURCE_DIR, '**/*.rs'))
      .map { |filename| [filename, IO.read(filename)] }
      .to_h
  end

  # Returns {filename => definitions}
  #
  def find_definitions(files_content, output, var_name_filter)
    files_content.map do |filename, content|
      definitions = content
        .scan(/^pub static mut (\w+): ([\w:]+) = .+;\n/)
        .select { |var_name, _| var_name =~ var_name_filter }

      definitions.each do |var_name, var_type|
        output&.puts "Found definition '#{var_name}: #{var_type}' (#{filename})"
      end

      [filename, definitions]
    end.to_h
  end

  def deextern_vars!(all_definitions, files_content, output)
    all_definitions.each do |definition_filename, file_definitions|
      file_definitions.each do |var_name, var_type|
        files_content.each do |declaration_filename, declaration_file_content|
          declaration_regex = /^    static mut #{var_name}: #{var_type};\n/

          if declaration_file_content.match(declaration_regex)
            declaration_file_content.sub!(declaration_regex, '')
            definition_module = File.basename(definition_filename.chomp('.rs'))
            definition_module = "r##{definition_module}" if RESERVED_MODULE_NAMES.include?(definition_module)
            declaration_file_content.sub!(/\A/, "use crate::#{definition_module}::#{var_name};\n")
          end
        end
      end
    end
  end

  def write_files(files_content)
    files_content.each do |filename, content|
      IO.write(filename, content)
    end
  end
end

if __FILE__ == $0
  var_name_filter = ARGV[0] ? /^#{ARGV[0]}$/ : //

  DeexternVars.new.execute(var_name_filter: var_name_filter)
end
