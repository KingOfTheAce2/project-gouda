# Critical Window Crash Fixes - Quick Action List

## URGENT (Apply Immediately)

### 1. Fix tauri.conf.json CSP (CRITICAL)
**File**: `src-tauri/tauri.conf.json`
**Current**: `"csp": null`
**Problem**: Disables Content Security Policy entirely, causes blank screen

**Required Change**:
```json
"app": {
  "security": {
    "csp": "default-src 'self' tauri: http://tauri.localhost https://tauri.localhost; img-src 'self' asset: https://asset.localhost http://asset.localhost tauri: http://tauri.localhost blob: data:; style-src 'self' 'unsafe-inline' tauri: http://tauri.localhost; script-src 'self' tauri: http://tauri.localhost https://tauri.localhost; font-src 'self' asset: https://asset.localhost http://asset.localhost tauri: http://tauri.localhost data:; connect-src 'self' tauri: http://tauri.localhost; object-src 'none'; base-uri 'self'; form-action 'self'"
  }
}
```

---

## HIGH PRIORITY (Apply Before Release)

### 2. Add Loading Screen During Initialization
**File**: `src/lib/providers.tsx`
**Problem**: Shows blank screen during startup
**Current**: Returns `null` instead of loading UI

**Change**: Replace the loading return with a visible loading screen:
```typescript
// Show loading screen instead of blank screen
return (
  <div className="flex items-center justify-center h-screen w-screen bg-background">
    <div className="flex flex-col items-center gap-4">
      <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-primary" />
      <p className="text-sm text-muted-foreground">Initializing...</p>
    </div>
  </div>
);
```

### 3. Add Initialization Timeout
**File**: `src/lib/providers.tsx`
**Problem**: App hangs indefinitely if backend is slow
**Add**: Timeout protection
```typescript
useEffect(() => {
  const timer = setTimeout(() => {
    if (!initialized) {
      console.warn('Initialization timeout - proceeding with defaults');
      setInitialized(true);
    }
  }, 5000); // 5 second timeout
  return () => clearTimeout(timer);
}, [initialized]);
```

### 4. Add Error Handling to main.tsx
**File**: `src/main.tsx`
**Problem**: Silent failures if React fails to initialize
**Add**: Global error handlers at the top:
```typescript
// Global error handlers
window.addEventListener('error', (event) => {
  console.error('🔴 Window error:', event.error);
});

window.addEventListener('unhandledrejection', (event) => {
  console.error('🔴 Unhandled promise rejection:', event.reason);
});

// Add try-catch around React.createRoot
try {
  ReactDOM.createRoot(rootElement).render(/* ... */);
} catch (error) {
  console.error('🔴 Fatal error:', error);
  rootElement.innerHTML = '<div>Fatal error. Check console.</div>';
}
```

---

## MEDIUM PRIORITY (Polish)

### 5. Audit Windows Console Output
**Files**: All `src-tauri/src/*.rs`
**Action**: Find all `println!` and `eprintln!` debug statements
```bash
grep -r "println!\|eprintln!" src-tauri/src/ --include="*.rs"
```
**Replace** debug output with logging framework:
```rust
log::info!("message");      // for info level
tracing::debug!("message"); // for debug level
```

### 6. Verify CREATE_NO_WINDOW is Used
**Status**: Already implemented in `process_helper.rs`
**Action**: Verify it's imported and used in:
- Any system command spawning
- Database initialization
- External process calls

Example usage:
```rust
use crate::process_helper::ProcessCommandExt;

tokio::process::Command::new("cmd")
    .arg("/c")
    .arg("some_command")
    .no_window()  // Add this!
    .output()
    .await?;
```

---

## Testing Checklist

After applying fixes:
- [ ] Build MSI installer
- [ ] Test fresh installation on Windows 10/11
- [ ] Verify window opens with UI (not blank)
- [ ] Check developer console (F12) has no errors
- [ ] Test with backend offline (should timeout gracefully)
- [ ] Check AppData logs folder exists and has logs
- [ ] No console windows should flash during startup

---

## Impact Summary

| Fix | Impact | Effort | Time |
|-----|--------|--------|------|
| CSP Fix | Prevents blank screen | 5 min | Critical |
| Loading UI | Better UX, prevents hang perception | 10 min | High |
| Timeout | Prevents indefinite hangs | 5 min | High |
| Error Handling | Better debugging | 15 min | High |
| Console Cleanup | Polish, user experience | 20 min | Medium |

**Total Estimated Time: 1 hour**

---

## Resources

- Full Analysis: `/docs/WINDOW_CRASH_FIXES.md`
- BEAR-LLM Repository: https://github.com/KingOfTheAce2/BEAR-LLM
- Tauri Security Guide: https://tauri.app/v1/guides/distribution/windows/

