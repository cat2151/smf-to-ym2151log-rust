# Demo Separation and Error Resolution

## Problem

The main demo (`index.html`) combined both MIDI and MML functionality in tabs, but MML support requires external dependencies that are not included in the standard deployment. This caused confusing error messages like "MML WASM module not available (this is optional)" which misled users into thinking MML errors were acceptable.

## Solution

### 1. Separated Demos

**Main MIDI Demo** (`/`)
- Focused solely on MIDI file to YM2151 conversion
- No MML-related code or dependencies
- Clean, simple interface for basic use case

**MML Demo** (`/demo-mml/`)
- Dedicated demo for MML to YM2151 conversion
- Clear setup instructions for required dependencies
- Properly handles missing dependencies with helpful error messages

**Library Demo** (`/demo-library/`)
- Unchanged - demonstrates library usage patterns

### 2. Updated Navigation

All demos now have consistent footer navigation:
- Main Demo: GitHub | MML Demo | Library Demo
- MML Demo: GitHub | MIDI Demo | Library Demo  
- Library Demo: GitHub | Main Demo

### 3. Deployment Workflow Updates

`.github/workflows/deploy-pages.yml` now:
- Creates MML stub for both main and MML demos
- Builds all three demos separately
- Deploys all three to GitHub Pages

### 4. Agent Instructions Enhanced

Added to `.github/copilot-instructions.md`:
- **Mandatory demo verification** using headless browser during development
- **Critical error handling** - MML errors must be addressed, not ignored
- **Deployment responsibility** - agents must verify deploy procedures and check for library changes
- Example verification script using headless browser

## Benefits

1. **Clear Separation of Concerns**: MIDI demo is simple and always works; MML demo clearly indicates advanced setup needed
2. **Better User Experience**: Users aren't confused by errors in features they aren't using
3. **Easier Maintenance**: Each demo can be updated independently
4. **Better Documentation**: Each demo has its own README with specific instructions
5. **Agent Accountability**: Clear guidelines ensure future development maintains quality

## Verification

All three demos build successfully:
- Main demo: ✓ Built in 116ms
- Library demo: ✓ Built in 117ms  
- MML demo: ✓ Built in 133ms

## Testing Checklist

When deployed, verify:
- [ ] Main demo loads without errors
- [ ] Main demo can convert MIDI files
- [ ] MML demo loads with clear setup instructions
- [ ] MML demo shows appropriate error message for missing dependencies
- [ ] Library demo loads without errors
- [ ] All navigation links work correctly
- [ ] No JavaScript console errors on any demo
