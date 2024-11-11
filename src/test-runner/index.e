function log_summary(total: Number, passed: Number) {
    console.log('Total tests run: ', total)
    console.log('Tests passed: ', passed)
    console.log('Tests failed: ', (total - passed))
    if passed == total {
        console.log('All tests passed! 🎉')
    } else {
        console.log('Some tests failed.')
    }
}

log_summary(100, 100)
