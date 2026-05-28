{ pkgs, ... }:

let
  fpvm = pkgs.rustPlatform.buildRustPackage {
    pname = "fpvm";
    version = "0.1.0";
    src = ../.;
    cargoLock.lockFile = ../Cargo.lock;
    doCheck = false;
  };
in
pkgs.runCommand "fpvm-fp-run-test" { nativeBuildInputs = [ fpvm ]; } ''
  mkdir -p $out
  failed=0

  for src in ${../tests/code}/*.fpvm; do
    name=$(basename "$src" .fpvm)
    golden="${../tests/golden}/$name.golden"

    if [ ! -f "$golden" ]; then
      echo "SKIP $name (no golden)"
      continue
    fi

    result=$(fp-run "$src" 2>&1 || true)

    actual=$(printf '%s' "$result" | sed 's/[[:space:]]*$//')
    expected=$(sed 's/[[:space:]]*$//' "$golden")
    if [ "$actual" = "$expected" ]; then
      echo "PASS $name"
    else
      echo "FAIL $name"
      echo "--- expected ---"
      cat "$golden"
      echo "--- actual ---"
      printf '%s' "$result"
      echo ""
      echo "---"
      failed=1
    fi
  done

  if [ $failed -ne 0 ]; then
    exit 1
  fi
''
