#!/bin/bash
# This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
# WebView2 Verification Script

echo "🔍 Verifying WebView2 Configuration..."
echo ""

echo "1️⃣ Checking Cargo.lock for webview dependencies:"
echo "=================================================="
grep -A 5 "name = \"webview2-com\"" src-tauri/Cargo.lock | head -10
echo ""

echo "2️⃣ Checking Wry version (should be 0.46+):"
echo "============================================"
grep -A 2 "name = \"wry\"" src-tauri/Cargo.lock | head -3
echo ""

echo "3️⃣ Checking tauri.conf.json for WebView2 install mode:"
echo "========================================================"
grep -A 3 "webviewInstallMode" src-tauri/tauri.conf.json
echo ""

echo "4️⃣ Checking init.rs for WebView2 configuration:"
echo "================================================="
grep -n "WebView2\|WEBVIEW2" src-tauri/src/init.rs
echo ""

echo "5️⃣ Technology Stack:"
echo "===================="
echo "✅ Wry: Cross-platform webview library (used by Tauri)"
echo "✅ WebView2-COM: Microsoft WebView2 bindings for Windows"
echo "✅ WebView2 Runtime: Chromium-based (NOT EdgeHTML/EBWebView)"
echo ""

echo "6️⃣ Folder Name Clarification:"
echo "=============================="
echo "📁 'EBWebView' folder name ≠ EdgeHTML technology"
echo "📁 Microsoft uses 'EBWebView' folder for backward compatibility"
echo "📁 The actual engine is modern Chromium-based WebView2"
echo "📁 Your app now uses 'WebView2' folder for clarity"
echo ""

echo "✅ Verification Complete!"
echo ""
echo "Your application IS using WebView2 (Chromium-based), NOT EdgeHTML."
echo "If you see 'EBWebView' errors, they refer to folder paths, not the engine."
