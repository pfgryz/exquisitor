#!/usr/bin/env bash
# Spell-check article TeX sources with aspell (English only).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PERSONAL_EN="${ROOT}/script/aspell.en.pws"
INTERACTIVE=0
EXIT=0

usage() {
  cat <<'EOF'
Usage: script/spellcheck.sh [-i] [file.tex ...]

  Without arguments, checks main.tex.
  -i  interactive mode (aspell check; edits files in place)

Domain terms live in script/aspell.en.pws — append accepted words there.
Tokens containing digits (e.g. Kraken2) are ignored automatically.
EOF
}

if ! command -v aspell >/dev/null 2>&1; then
  echo "error: aspell not found (apt: aspell aspell-en)" >&2
  exit 1
fi
if ! aspell dump dicts 2>/dev/null | grep -qx en; then
  echo "error: English aspell dictionary not found (apt: aspell-en)" >&2
  exit 1
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help) usage; exit 0 ;;
    -i|--interactive) INTERACTIVE=1; shift ;;
    --) shift; break ;;
    -*) echo "error: unknown option: $1" >&2; usage >&2; exit 2 ;;
    *) break ;;
  esac
done

aspell_args() {
  local args=(--mode=tex --lang=en --encoding=utf-8)
  if [[ -f "${PERSONAL_EN}" ]]; then
    args+=(--home-dir="${ROOT}/script" --personal=aspell.en.pws)
  fi
  printf '%s\n' "${args[@]}"
}

# Drop tokens with digits (tool names like Kraken2) — aspell personal dicts reject them.
filter_misses() {
  grep -Ev '[[:digit:]]' || true
}

escape_ere() {
  # Escape Extended Regular Expression metacharacters in a literal token.
  printf '%s' "$1" | sed -e 's/[][\\.^$*+?(){}|]/\\&/g'
}

# Lines where word appears as a whole token (not a substring of a longer word).
grep_word_lines() {
  local file="$1"
  local word="$2"
  local pat
  pat="(^|[^[:alpha:][:digit:]_])$(escape_ere "$word")([^[:alpha:][:digit:]_]|$)"
  grep -n -E -- "$pat" "$file" || true
}

check_list() {
  local file="$1"

  local mapfile_args=()
  while IFS= read -r a; do mapfile_args+=("$a"); done < <(aspell_args)

  local err
  err="$(mktemp)"
  local misses
  set +e
  misses="$(aspell "${mapfile_args[@]}" list < "${file}" 2>"${err}" \
    | filter_misses \
    | sort -u)"
  local status=$?
  set -e

  if [[ ${status} -ne 0 ]]; then
    echo "  ERR ${file#"${ROOT}/"}" >&2
    cat "${err}" >&2
    rm -f "${err}"
    return 1
  fi
  rm -f "${err}"

  if [[ -z "${misses}" ]]; then
    echo "  OK  ${file#"${ROOT}/"}"
    return 0
  fi

  local rel="${file#"${ROOT}/"}"
  echo "  !!  ${rel}"
  while IFS= read -r word; do
    [[ -z "$word" ]] && continue
    local lines=()
    while IFS= read -r hit; do
      [[ -z "$hit" ]] && continue
      lines+=("${hit%%:*}")
    done < <(grep_word_lines "$file" "$word")
    if ((${#lines[@]} == 0)); then
      printf '      - %s\n' "$word"
    else
      local joined
      printf -v joined '%s, ' "${lines[@]}"
      printf '      - %s | %s\n' "$word" "${joined%, }"
    fi
  done <<< "${misses}"
  return 1
}

check_interactive() {
  local file="$1"

  local mapfile_args=()
  while IFS= read -r a; do mapfile_args+=("$a"); done < <(aspell_args)

  echo "==> Checking ${file#"${ROOT}/"} (en)"
  aspell "${mapfile_args[@]}" check "${file}"
}

FILES=()
if [[ $# -gt 0 ]]; then
  for arg in "$@"; do
    if [[ -f "$arg" ]]; then
      FILES+=("$(cd "$(dirname "$arg")" && pwd)/$(basename "$arg")")
    elif [[ -f "${ROOT}/${arg}" ]]; then
      FILES+=("${ROOT}/${arg}")
    else
      echo "error: file not found: ${arg}" >&2
      exit 1
    fi
  done
else
  if [[ -f "${ROOT}/main.tex" ]]; then
    FILES+=("${ROOT}/main.tex")
  fi
fi

if [[ ${#FILES[@]} -eq 0 ]]; then
  echo "error: no .tex files to check" >&2
  exit 1
fi

echo "==> Spellcheck ($( [[ ${INTERACTIVE} -eq 1 ]] && echo interactive || echo list ))"
echo

for file in "${FILES[@]}"; do
  if [[ ${INTERACTIVE} -eq 1 ]]; then
    check_interactive "${file}" || EXIT=1
  else
    check_list "${file}" || EXIT=1
  fi
done

echo
if [[ ${EXIT} -eq 0 ]]; then
  echo "==> No unknown words"
else
  echo "==> Unknown words found (add legitimate terms to script/aspell.en.pws)"
fi
exit "${EXIT}"
