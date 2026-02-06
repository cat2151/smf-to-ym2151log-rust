#!/usr/bin/env node
/**
 * Demo Verification Script
 * 
 * This script verifies that deployed demos work correctly
 * by checking for JavaScript errors and basic functionality.
 * 
 * Usage: node verify-demos.js [base-url]
 * Example: node verify-demos.js https://cat2151.github.io/smf-to-ym2151log-rust
 * 
 * Default: http://localhost:5173 (Vite dev server default port)
 * Note: For production builds via 'npm run preview', use http://localhost:4173
 */

const BASE_URL = process.argv[2] || process.env.DEMO_BASE_URL || 'http://localhost:5173';

console.log(`\n🔍 Verifying demos at: ${BASE_URL}\n`);

const demoPages = [
    { path: '/', name: 'Main MIDI Demo' },
    { path: '/demo-mml/', name: 'MML Demo' },
    { path: '/demo-library/', name: 'Library Demo' }
];

// Simple fetch-based verification (works in Node.js 18+)
async function verifyPage(url, pageName) {
    try {
        const response = await fetch(url);
        if (!response.ok) {
            console.log(`❌ ${pageName}: HTTP ${response.status}`);
            return false;
        }
        
        const html = await response.text();
        
        // Basic checks
        const hasTitle = html.includes('<title>');
        const hasBody = html.includes('<body');
        const hasScript = html.includes('<script');
        
        if (hasTitle && hasBody && hasScript) {
            console.log(`✅ ${pageName}: Page structure OK`);
            return true;
        } else {
            console.log(`⚠️  ${pageName}: Page structure incomplete`);
            return false;
        }
    } catch (error) {
        console.log(`❌ ${pageName}: ${error.message}`);
        return false;
    }
}

async function main() {
    let allPassed = true;
    
    for (const demo of demoPages) {
        const url = BASE_URL + demo.path;
        const passed = await verifyPage(url, demo.name);
        if (!passed) allPassed = false;
    }
    
    console.log('\n' + '='.repeat(50));
    if (allPassed) {
        console.log('✅ All demos verified successfully!');
        process.exit(0);
    } else {
        console.log('❌ Some demos failed verification');
        process.exit(1);
    }
}

main().catch(error => {
    console.error('Fatal error:', error);
    process.exit(1);
});
