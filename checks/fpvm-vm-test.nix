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
pkgs.runCommand "fpvm-vm-test" { nativeBuildInputs = [ fpvm ]; } ''
  mkdir -p $out
  failed=0

  for src in ${../tests/code}/*.fpvm; do
    name=$(basename "$src" .fpvm)
    golden="${../tests/golden}/$name.golden"

    if [ ! -f "$golden" ]; then
      echo "SKIP $name (no golden)"
      continue
    fi

    mkdir -p /tmp/vmtest/$name
    dump_tool "$src" /tmp/vmtest/$name --codegen > /dev/null 2>&1
    bytecode="/tmp/vmtest/$name/$name.output"

    if [ ! -f "$bytecode" ]; then
      echo "FAIL $name (codegen failed)"
      failed=1
      continue
    fi

    result=$(fpvm "$bytecode" 2>&1 || true)

    if diff -q <(echo "$result") "$golden" > /dev/null 2>&1; then
      echo "PASS $name"
    else
      echo "FAIL $name"
      echo "--- expected ---"
      cat "$golden"
      echo "--- actual ---"
      echo "$result"
      echo "---"
      failed=1
    fi
  done

  if [ $failed -ne 0 ]; then
    exit 1
  fi
''
