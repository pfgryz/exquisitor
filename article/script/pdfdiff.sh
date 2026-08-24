#!/usr/bin/env bash
# Build a PDF showing LaTeX text differences between two git commits.
#
# Usage:
#   ./script/pdfdiff.sh <old-commit> [new-commit] [options]
#
# Examples:
#   ./script/pdfdiff.sh 1d1eaec
#   ./script/pdfdiff.sh 1d1eaec HEAD
#   ./script/pdfdiff.sh ca857ab HEAD -o /tmp/article-diff
#
# Options:
#   -o, --output DIR   Output directory (default: article/build/pdfdiff/<old>_vs_<new>)
#   -h, --help         Show this help
#
# Requires: git, latexdiff, latexmk, pdflatex, bibtex, rsync
set -euo pipefail

ARTICLE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROOT="$(git -C "${ARTICLE_DIR}" rev-parse --show-toplevel)"
ARTICLE_REL="$(realpath --relative-to="${ROOT}" "${ARTICLE_DIR}")"
MAIN_TEX="main.tex"

OLD_REV=""
NEW_REV=""
OUT_DIR=""
POSITIONAL=()

usage() {
  sed -n '2,16p' "$0" | sed 's/^# \?//'
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    -o|--output)
      OUT_DIR="${2:?missing value for $1}"
      shift 2
      ;;
    -*)
      echo "error: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
    *)
      POSITIONAL+=("$1")
      shift
      ;;
  esac
done

if [[ ${#POSITIONAL[@]} -lt 1 || ${#POSITIONAL[@]} -gt 2 ]]; then
  echo "error: expected <old-commit> [new-commit]" >&2
  usage >&2
  exit 2
fi

OLD_REV="${POSITIONAL[0]}"
NEW_REV="${POSITIONAL[1]:-HEAD}"

cd "${ROOT}"

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "error: not a git repository: ${ROOT}" >&2
  exit 1
fi

for cmd in git latexdiff latexmk pdflatex bibtex rsync; do
  if ! command -v "${cmd}" >/dev/null 2>&1; then
    echo "error: required command not found: ${cmd}" >&2
    exit 1
  fi
done

OLD_FULL="$(git rev-parse --verify "${OLD_REV}^{commit}")"
NEW_FULL="$(git rev-parse --verify "${NEW_REV}^{commit}")"
OLD_SHORT="$(git rev-parse --short "${OLD_FULL}")"
NEW_SHORT="$(git rev-parse --short "${NEW_FULL}")"

if [[ -z "${OUT_DIR}" ]]; then
  OUT_DIR="${ARTICLE_DIR}/build/pdfdiff/${OLD_SHORT}_vs_${NEW_SHORT}"
fi

if ! git cat-file -e "${OLD_FULL}:${ARTICLE_REL}/${MAIN_TEX}" 2>/dev/null; then
  echo "error: ${ARTICLE_REL}/${MAIN_TEX} missing in ${OLD_SHORT}" >&2
  exit 1
fi
if ! git cat-file -e "${NEW_FULL}:${ARTICLE_REL}/${MAIN_TEX}" 2>/dev/null; then
  echo "error: ${ARTICLE_REL}/${MAIN_TEX} missing in ${NEW_SHORT}" >&2
  exit 1
fi

WORKDIR="$(mktemp -d "${TMPDIR:-/tmp}/article-pdfdiff.XXXXXX")"
cleanup() { rm -rf "${WORKDIR}"; }
trap cleanup EXIT

OLD_TREE="${WORKDIR}/old"
NEW_TREE="${WORKDIR}/new"
mkdir -p "${OLD_TREE}" "${NEW_TREE}"

echo "==> Exporting ${ARTICLE_REL}/ from ${OLD_SHORT} and ${NEW_SHORT}"
git archive "${OLD_FULL}" "${ARTICLE_REL}" | tar -x -C "${OLD_TREE}"
git archive "${NEW_FULL}" "${ARTICLE_REL}" | tar -x -C "${NEW_TREE}"

OLD_ARTICLE="${OLD_TREE}/${ARTICLE_REL}"
NEW_ARTICLE="${NEW_TREE}/${ARTICLE_REL}"

mkdir -p "${OUT_DIR}"
DIFF_TEX="${OUT_DIR}/main-diff.tex"

LATEXDIFF_OPTS=(
  --flatten
  --type=UNDERLINE
  --encoding=utf8
  --graphics-markup=none
)

echo "==> Running latexdiff (${OLD_SHORT} → ${NEW_SHORT})"
latexdiff "${LATEXDIFF_OPTS[@]}" \
  "${OLD_ARTICLE}/${MAIN_TEX}" \
  "${NEW_ARTICLE}/${MAIN_TEX}" \
  > "${DIFF_TEX}"

echo "==> Preparing build tree in ${OUT_DIR}"
rsync -a --delete \
  --exclude 'build/' \
  --exclude 'work/' \
  --exclude 'venv/' \
  --exclude 'script/' \
  --exclude 'main-diff.tex' \
  --exclude 'main-diff.pdf' \
  --exclude 'README.txt' \
  "${NEW_ARTICLE}/" "${OUT_DIR}/work/"

# Keep figures removed in the new revision available if the diff still references them.
shopt -s nullglob
OLD_FIGS=("${OLD_ARTICLE}"/picture_*.{png,pdf,jpg,jpeg,eps})
if [[ ${#OLD_FIGS[@]} -gt 0 ]]; then
  mkdir -p "${OUT_DIR}/work"
  rsync -a --ignore-existing "${OLD_FIGS[@]}" "${OUT_DIR}/work/"
fi
shopt -u nullglob

cp "${DIFF_TEX}" "${OUT_DIR}/work/main-diff.tex"

if [[ ! -f "${OUT_DIR}/work/.latexmkrc" ]]; then
  cat > "${OUT_DIR}/work/.latexmkrc" <<'EOF'
$pdf_mode = 1;
$out_dir  = 'build';
$aux_dir  = 'build';
$bibtex_use = 1;
$pdflatex = 'pdflatex -interaction=nonstopmode -file-line-error %O %S';
$bibtex   = 'bibtex %O %S';
EOF
fi

mkdir -p "${OUT_DIR}/work/build"
cd "${OUT_DIR}/work"

echo "==> Compiling diff PDF (latexmk)"
set +e
latexmk -pdf -f -interaction=nonstopmode main-diff.tex
latexmk_status=$?
set -e

if [[ ! -f "build/main-diff.pdf" ]]; then
  echo "error: expected PDF not found at ${OUT_DIR}/work/build/main-diff.pdf (latexmk exit: ${latexmk_status})" >&2
  exit 1
fi

cp -f "build/main-diff.pdf" "${OUT_DIR}/main-diff.pdf"

DIFF_PUBLISH_DIR="${ARTICLE_DIR}/diff/${OLD_SHORT}_vs_${NEW_SHORT}"
mkdir -p "${DIFF_PUBLISH_DIR}"
cp -f "${OUT_DIR}/main-diff.tex" "${DIFF_PUBLISH_DIR}/main-diff.tex"
cp -f "${OUT_DIR}/main-diff.pdf" "${DIFF_PUBLISH_DIR}/main-diff.pdf"

cat > "${OUT_DIR}/README.txt" <<EOF
LaTeX PDF diff
==============
old: ${OLD_FULL} (${OLD_SHORT})
new: ${NEW_FULL} (${NEW_SHORT})
generated: $(date -Iseconds)

PDF: ${OUT_DIR}/main-diff.pdf
published: ${DIFF_PUBLISH_DIR}/
EOF

echo "==> Done: ${OUT_DIR}/main-diff.pdf"
echo "    copied to ${DIFF_PUBLISH_DIR}/"
echo "    old=${OLD_SHORT}  new=${NEW_SHORT}"
