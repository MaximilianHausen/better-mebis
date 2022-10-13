// On-Load darkmode detection
chrome.storage.local.set({
    darkmode: window.document.documentElement.classList.contains('dark')
})

// Detect darkmode changes without reload
new MutationObserver(() => {
    chrome.storage.local.set({
        darkmode: window.document.documentElement.classList.contains('dark')
    })
}).observe(window.document.documentElement, {
    attributeFilter: ['class'],
});
