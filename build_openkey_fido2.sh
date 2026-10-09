#!/usr/bin/env bash
# build_openkey_fido2.sh
# Build unificado do openkey-fido2 para Linux/macOS/WSL.
#
# Requisitos:
#   - Rust 1.85+
#   - cargo / rustc
#
# Uso:
#   ./build_openkey_fido2.sh
#   ./build_openkey_fido2.sh --release
#   ./build_openkey_fido2.sh --sim
#   ./build_openkey_fido2.sh --rp2350
#   ./build_openkey_fido2.sh --rp2350-uf2
#   ./build_openkey_fido2.sh --nrf52840
#   ./build_openkey_fido2.sh --check
#   ./build_openkey_fido2.sh --test
#   ./build_openkey_fido2.sh --clippy
#   ./build_openkey_fido2.sh --fmt
#   ./build_openkey_fido2.sh --all
#   ./build_openkey_fido2.sh --clean
#
# Artefatos:
#   target/debug/
#   target/release/
#   target/thumbv8m.main-none-eabihf/{debug,release}/rp2350-firmware*

set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

TARGET_RP2350="thumbv8m.main-none-eabihf"
TARGET_NRF52840="thumbv7em-none-eabihf"
RP2350_DIR="$ROOT/targets/rp2350"
WORKSPACE_TARGET_DIR="$ROOT/target"

MODE="debug"
ACTION="workspace"

log() {
    printf '[openkey-fido2] %s\n' "$*"
}

warn() {
    printf '[openkey-fido2] WARNING: %s\n' "$*" >&2
}

die() {
    printf '[openkey-fido2] ERROR: %s\n' "$*" >&2
    exit 1
}

usage() {
    cat <<'EOF'
============================================================
 openkey-fido2 build
============================================================

Uso:
  ./build_openkey_fido2.sh [opção]

Build:
  --debug
      Build do workspace em debug.

  --release
      Build do workspace em release.

  --sim
      Build do fido2-simulator.

  --rp2350
      Build do firmware RP2350.

  --rp2350-uf2
      Build do firmware RP2350 e gerar UF2.

  --nrf52840
      Build do firmware nRF52840.

Verificação:
  --test
      Executar cargo test --workspace.

  --clippy
      Executar cargo clippy com warnings como erro.

  --fmt
      Verificar rustfmt.

  --check
      Verificar targets embedded.

  --all
      Executar build + testes + clippy + fmt.

Limpeza:
  --clean
      Executar cargo clean e limpar targets dos firmwares.

Outros:
  --help
      Mostrar esta ajuda.

Exemplos:

  ./build_openkey_fido2.sh
  ./build_openkey_fido2.sh --release
  ./build_openkey_fido2.sh --sim --release
  ./build_openkey_fido2.sh --rp2350 --release
  ./build_openkey_fido2.sh --rp2350-uf2 --release
  ./build_openkey_fido2.sh --nrf52840
  ./build_openkey_fido2.sh --all

============================================================
EOF
}

require_command() {
    command -v "$1" >/dev/null 2>&1 || \
        die "'$1' não encontrado no PATH."
}

record_and_verify_sha256() {
    local artifact="$1"
    local checksum="${artifact}.sha256"
    local artifact_dir
    local artifact_name
    local checksum_name

    [[ -f "$artifact" ]] || die "Artefato não encontrado para checksum: $artifact"

    artifact_dir="$(dirname "$artifact")"
    artifact_name="$(basename "$artifact")"
    checksum_name="$(basename "$checksum")"

    if command -v sha256sum >/dev/null 2>&1; then
        (
            cd "$artifact_dir"
            sha256sum "$artifact_name" > "$checksum_name"
            sha256sum --check "$checksum_name"
        ) || die "Falha na validação SHA-256: $artifact"
    elif command -v shasum >/dev/null 2>&1; then
        (
            cd "$artifact_dir"
            shasum -a 256 "$artifact_name" > "$checksum_name"
            shasum -a 256 --check "$checksum_name"
        ) || die "Falha na validação SHA-256: $artifact"
    else
        die "Nem sha256sum nem shasum foram encontrados no PATH."
    fi

    log "SHA-256 validado:"
    log "  $checksum"
}

cargo_workspace() {
    log "Build workspace: $MODE"

    if [[ "$MODE" == "release" ]]; then
        cargo build --workspace --release --locked
    else
        cargo build --workspace --locked
    fi
}

build_simulator() {
    log "Build fido2-simulator: $MODE"

    if [[ "$MODE" == "release" ]]; then
        cargo build -p fido2-simulator --release --locked
    else
        cargo build -p fido2-simulator --locked
    fi
}

build_rp2350() {
    local dir="$RP2350_DIR"

    [[ -f "$dir/Cargo.toml" ]] || \
        die "Firmware RP2350 não encontrado: $dir"

    log "Build firmware RP2350: $MODE"

    (
        cd "$dir"

        if [[ "$MODE" == "release" ]]; then
            cargo build \
                --features firmware \
                --target "$TARGET_RP2350" \
                --release \
                --locked
        else
            cargo build \
                --features firmware \
                --target "$TARGET_RP2350" \
                --locked
        fi
    )

    local profile="$MODE"
    local elf="$WORKSPACE_TARGET_DIR/$TARGET_RP2350/$profile/rp2350-firmware"

    [[ -f "$elf" ]] || die "ELF não foi gerado: $elf"

    log "ELF gerado:"
    log "  $elf"

    if command -v arm-none-eabi-size >/dev/null 2>&1; then
        arm-none-eabi-size "$elf" || true
    fi

    record_and_verify_sha256 "$elf"
}

build_rp2350_uf2() {
    build_rp2350

    local dir="$RP2350_DIR"
    local profile="$MODE"

    local elf="$WORKSPACE_TARGET_DIR/$TARGET_RP2350/$profile/rp2350-firmware"
    local uf2="$WORKSPACE_TARGET_DIR/$TARGET_RP2350/$profile/rp2350-firmware.uf2"

    [[ -f "$elf" ]] || \
        die "ELF não encontrado: $elf"

    log "Convertendo ELF para UF2..."

    if command -v picotool >/dev/null 2>&1; then
        log "Usando picotool"

        picotool uf2 convert \
            "$elf" \
            -t elf \
            "$uf2" \
            -t uf2

    elif command -v elf2uf2-rs >/dev/null 2>&1; then
        log "Usando elf2uf2-rs"

        elf2uf2-rs "$elf" "$uf2"

    elif [[ -x "$HOME/.cargo/bin/elf2uf2-rs" ]]; then
        log "Usando ~/.cargo/bin/elf2uf2-rs"

        "$HOME/.cargo/bin/elf2uf2-rs" "$elf" "$uf2"

    else
        die "Não encontrei picotool nem elf2uf2-rs.
Instale um deles para gerar UF2."
    fi

    [[ -f "$uf2" ]] || \
        die "UF2 não foi gerado."

    record_and_verify_sha256 "$uf2"

    log "UF2 gerado:"
    log "  $uf2"

    ls -lh "$uf2" 2>/dev/null || true
}

build_nrf52840() {
    local dir="$ROOT/targets/nrf52840"

    if [[ ! -f "$dir/Cargo.toml" ]]; then
        warn "Firmware nRF52840 não encontrado:"
        warn "  $dir"
        return 0
    fi

    log "Build firmware nRF52840"

    (
        cd "$dir"
        cargo build \
            --locked \
            --target "$TARGET_NRF52840"
    )
}

check_targets() {
    require_command rustup

    log "Verificando target RP2350..."

    if ! rustup target list --installed | grep -qx "$TARGET_RP2350"; then
        die "Target ausente: $TARGET_RP2350

Instale com:
  rustup target add $TARGET_RP2350"
    fi

    log "Verificando target nRF52840..."

    if ! rustup target list --installed | grep -qx "$TARGET_NRF52840"; then
        die "Target ausente: $TARGET_NRF52840

Instale com:
  rustup target add $TARGET_NRF52840"
    fi

    log "Executando cargo check para RP2350..."

    cargo check \
        -p transport \
        --target "$TARGET_RP2350" \
        --features embedded \
        --no-default-features \
        --locked

    log "Executando cargo check para nRF52840..."

    cargo check \
        -p transport \
        --target "$TARGET_NRF52840" \
        --features embedded \
        --no-default-features \
        --locked
}

run_tests() {
    log "Executando testes..."

    cargo test --workspace --locked
}

run_clippy() {
    log "Executando Clippy..."

    cargo clippy \
        --workspace \
        --all-targets \
        --locked \
        -- -D warnings
}

run_fmt() {
    log "Verificando rustfmt..."

    cargo fmt --all -- --check
}

clean_all() {
    log "Limpando workspace..."

    cargo clean

    log "Limpeza concluída."
}

# ------------------------------------------------------------
# Argumentos
# ------------------------------------------------------------
while (($# > 0)); do
    case "$1" in
        --debug)
            MODE="debug"
            ;;
        --release)
            MODE="release"
            ;;
        --sim|--simulator)
            ACTION="sim"
            ;;
        --rp2350|--firmware)
            ACTION="rp2350"
            ;;
        --rp2350-uf2|--uf2)
            ACTION="rp2350-uf2"
            ;;
        --nrf52840|--nrf)
            ACTION="nrf52840"
            ;;
        --check|--check-targets)
            ACTION="check"
            ;;
        --test|--tests)
            ACTION="test"
            ;;
        --clippy)
            ACTION="clippy"
            ;;
        --fmt|--fmt-check)
            ACTION="fmt"
            ;;
        --all|--ci)
            ACTION="all"
            ;;
        --clean)
            ACTION="clean"
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            die "Opção desconhecida: $1
Use:
  ./build_openkey_fido2.sh --help"
            ;;
    esac
    shift
done

cd "$ROOT"

require_command cargo
require_command rustc

log "Root: $ROOT"
log "Rust: $(rustc --version)"
log "Cargo: $(cargo --version)"

# ------------------------------------------------------------
# Execução
# ------------------------------------------------------------

case "$ACTION" in

    workspace)
        cargo_workspace
        ;;

    sim)
        build_simulator
        ;;

    rp2350)
        build_rp2350
        ;;

    rp2350-uf2)
        build_rp2350_uf2
        ;;

    nrf52840)
        build_nrf52840
        ;;

    check)
        check_targets
        ;;

    test)
        run_tests
        ;;

    clippy)
        run_clippy
        ;;

    fmt)
        run_fmt
        ;;

    clean)
        clean_all
        ;;

    all)
        cargo_workspace
        run_tests
        run_clippy
        run_fmt
        ;;

esac

log "============================================================"
log "BUILD CONCLUÍDO"
log "Ação : $ACTION"
log "Modo : $MODE"
log "============================================================"
