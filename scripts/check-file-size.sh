#!/usr/bin/env bash
set -euo pipefail

warning_lines=500
error_lines=1000
warnings=0
errors=0

is_governed_source() {
  case "$1" in
    *.c|*.cc|*.cpp|*.cs|*.go|*.h|*.hpp|*.java|*.js|*.jsx|*.kt|*.kts|*.md|*.py|*.rb|*.rs|*.sh|*.swift|*.toml|*.ts|*.tsx|*.yaml|*.yml)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

while IFS= read -r -d '' path; do
  is_governed_source "$path" || continue
  lines=$(awk 'END { print NR }' "$path")

  if (( lines > error_lines )); then
    printf 'ERROR %s: %d lines (limit: %d)\n' "$path" "$lines" "$error_lines"
    if [[ ${GITHUB_ACTIONS:-} == "true" ]]; then
      printf '::error file=%s::File has %d lines; split it below the %d-line limit.\n' \
        "$path" "$lines" "$error_lines"
    fi
    ((errors += 1))
  elif (( lines > warning_lines )); then
    printf 'WARNING %s: %d lines (warning threshold: %d)\n' \
      "$path" "$lines" "$warning_lines"
    if [[ ${GITHUB_ACTIONS:-} == "true" ]]; then
      printf '::warning file=%s::File has %d lines; consider splitting it before it reaches %d lines.\n' \
        "$path" "$lines" "$error_lines"
    fi
    ((warnings += 1))
  fi
done < <(git ls-files -z --cached --others --exclude-standard)

printf 'File-size gate: %d warning(s), %d error(s).\n' "$warnings" "$errors"
(( errors == 0 ))
