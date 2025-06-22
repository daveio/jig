# frozen_string_literal: true

require 'commander/import'
require 'httparty'
require 'json'
require 'csv'
require 'uri'
require 'yaml'

module EnvHunter
  module_function

  GITHUB_TOKEN = ENV['GITHUB_TOKEN'] || abort('Error: GITHUB_TOKEN environment variable is not set.')
  ENTROPY_THRESHOLD = 4.0
  KEYWORDS = /KEY|TOKEN/

  def entropy(str)
    return 0.0 if str.nil? || str.empty?

    chars = str.each_char.group_by(&:itself).transform_values(&:count)
    probs = chars.values.map { |c| c.to_f / str.length }
    -probs.map { |p| p * Math.log2(p) }.reduce(:+) || 0.0
  end

  def extract_env_keys(content)
    results = {}
    content.each_line do |line|
      next unless line =~ /^\s*[\w-]+=(.+)$/

      key, value = line.strip.split('=', 2)
      next unless key =~ KEYWORDS
      next if value.nil?

      ent = entropy(value.gsub(/['"]/, ''))
      results[key] = value if ent > ENTROPY_THRESHOLD
    end
    results
  end

  def github_search(query, mode, page = 1, per_page = 30)
    url = if mode == 'gists'
            "https://api.github.com/gists/public?page=#{page}&per_page=#{per_page}"
          else
            "https://api.github.com/search/code?q=#{URI.encode_www_form_component(query)}&page=#{page}&per_page=#{per_page}"
          end
    headers = {
      'Authorization' => "token #{GITHUB_TOKEN}",
      'User-Agent' => 'EnvHunter'
    }
    response = HTTParty.get(url, headers: headers)
    JSON.parse(response.body)
  end

  def process_results(data, mode, output_file)
    items = if mode == 'gists'
              data.is_a?(Array) ? data : []
            else
              data['items'] || []
            end
    found_count = 0
    results = []

    items.each do |item|
      raw_url, user, repo, file = if mode == 'gists'
                                    next unless item['files']

                                    file_obj = item['files'].values.find { |f| f['filename'] =~ /\.env$/ }
                                    next unless file_obj

                                    [file_obj['raw_url'], item['owner']['login'], "Gist: #{item['id']}",
                                     file_obj['filename']]
                                  else
                                    raw_url = item['html_url'].gsub('github.com', 'raw.githubusercontent.com').gsub(
                                      '/blob/', '/'
                                    )
                                    [raw_url, item['repository']['owner']['login'], item['repository']['full_name'],
                                     item['name']]
                                  end

      # Skip files that are likely not of interest
      next if file.downcase.include?('example')
      next if file.downcase.include?('sample')
      next if file.downcase.include?('template')
      next if file.downcase.include?('.ex.')
      next if file.downcase.include?('.bak')
      next if file.downcase.include?('.bkp')
      next if file.downcase.include?('staging')
      next if file.downcase.include?('copy')
      next if file.downcase.include?('backup')
      next if file.downcase.include?('old')
      next if file.downcase.include?('archive')

      begin
        raw = HTTParty.get(raw_url).body
        matches = extract_env_keys(raw)
        if matches.any?
          result = {
            user: user,
            repo: repo,
            file: file,
            matches: matches
          }
          found_count += 1

          # Always output to terminal
          key_names = result[:matches].keys.join(', ')
          puts "#{result[:repo]} (#{result[:file]}) - Found: #{key_names}"

          # Store result for YAML output if needed
          results << result if output_file
        end
      rescue StandardError => e
        warn "Error fetching #{raw_url}: #{e}"
      end
    end

    [found_count, results]
  end

  def write_yaml_file(results, filename)
    # Transform matches into a dictionary with env keys as keys and values as values
    yaml_results = results.map do |result|
      {
        repo: result[:repo],
        file: result[:file],
        matches: result[:matches]
      }
    end

    # Custom YAML dump to avoid colons before keys
    yaml_content = "---\n"
    yaml_results.each do |result|
      yaml_content += "- repo: #{result[:repo]}\n"
      yaml_content += "  file: #{result[:file]}\n"
      yaml_content += "  matches:\n"
      result[:matches].each do |key, value|
        yaml_content += "    #{key}: #{value}\n"
      end
    end

    File.write(filename, yaml_content)
    puts "\nResults written to #{filename}"
  end

  def run_cli
    program :name, 'EnvHunter'
    program :version, '1.0.0'
    program :description, 'Search GitHub for secrets in .env files'

    command :scan do |c|
      c.syntax = 'envhunter scan [options]'
      c.description = 'Scan GitHub code or gists for secrets'
      c.option '--mode MODE', String, 'Search mode: repos or gists (default: repos)'
      c.option '--output OUTPUT', String, 'Output YAML file (optional)'
      c.option '--limit LIMIT', Integer, 'Maximum number of responses to process (default: 100)'
      c.action do |_args, options|
        mode = options.mode == 'gists' ? 'gists' : 'code'
        output_file = options.output
        limit = options.limit || 100
        query = 'filename:.env'

        page = 1
        total_found = 0
        all_results = []

        begin
          loop do
            data = github_search(query, mode, page)
            break if data.nil? || (mode == 'gists' ? data.empty? : data['items'].empty?)

            found_in_page, page_results = process_results(data, mode, output_file)
            total_found += found_in_page
            all_results.concat(page_results) if output_file

            break if limit && total_found >= limit

            page += 1
            print '.' # Progress indicator
            $stdout.flush
          end
        rescue Interrupt
          puts "\nSearch interrupted by user"
        end

        puts "\nSearch completed. Found #{total_found} matches."

        # Write YAML file if output file is specified
        write_yaml_file(all_results, output_file) if output_file && all_results.any?
      end
    end
  end
end

EnvHunter.run_cli
