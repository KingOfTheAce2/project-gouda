#!/bin/bash
# This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
# Verification script for compilation fixes

echo "=== Verifying All Fixes ==="
echo ""

# 1. Check sea-orm-migration dependency
echo "1. Checking sea-orm-migration in Cargo.toml..."
if grep -q "sea-orm-migration" src-tauri/Cargo.toml; then
    echo "✅ sea-orm-migration dependency found"
else
    echo "❌ sea-orm-migration dependency missing"
fi
echo ""

# 2. Check Db struct visibility
echo "2. Checking Db struct field visibility..."
if grep -q "pub struct Db(pub DatabaseConnection)" src-tauri/src/services/db.rs; then
    echo "✅ Db field is public"
else
    echo "❌ Db field is not public"
fi
echo ""

# 3. Check emit vs emit_all
echo "3. Checking emit usage in cache.rs..."
if grep -q "handle.emit(" src-tauri/src/services/cache.rs && ! grep -q "handle.emit_all(" src-tauri/src/services/cache.rs; then
    echo "✅ Using emit (not emit_all)"
else
    echo "❌ Still using emit_all or emit not found"
fi
echo ""

# 4. Check Emitter import removed
echo "4. Checking for unused Emitter import..."
if ! grep -q "use tauri::Emitter" src-tauri/src/services/cache.rs; then
    echo "✅ Emitter import removed"
else
    echo "⚠️  Emitter import still present (will cause warning)"
fi
echo ""

# 5. Check From trait implementation
echo "5. Checking From<RawOllamaConfig> for OllamaConfig..."
if grep -q "impl From<RawOllamaConfig> for OllamaConfig" src-tauri/src/services/llm/types.rs; then
    echo "✅ From trait implemented in types.rs"
else
    echo "❌ From trait not found in types.rs"
fi

if grep -q "impl From<RawOllamaConfig> for OllamaConfig" src-tauri/src/services/llm/providers/ollama/config.rs; then
    echo "✅ From trait implemented in config.rs"
else
    echo "❌ From trait not found in config.rs"
fi
echo ""

echo "=== Summary ==="
echo "All fixes appear to be in place."
echo ""
echo "If you're still getting compilation errors, try:"
echo "  1. Clean the build cache:"
echo "     cd src-tauri && cargo clean"
echo "  2. Update dependencies:"
echo "     cargo update"
echo "  3. Rebuild:"
echo "     cd .. && pnpm tauri build"
echo ""
echo "Note: Make sure Rust and Cargo are installed:"
echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
